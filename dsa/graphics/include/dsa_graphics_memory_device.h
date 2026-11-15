// Copyright 2026 The DSA Authors
// SPDX-License-Identifier: Apache-2.0
//
// The guest's side of the graphics memory the host shares (dsa_graphics_memory.h): the
// kernel's /dev/dsa_graphics_memory, through which a process maps the grants the renderer
// gives it, and only those.
//
// A process opens the device once. For each grant (offset, length, key) the renderer gave it,
// it binds the grant to its file, maps it, and uses the pages the renderer imported into
// Vulkan. When it frees the memory it unmaps the pages and releases the grant before it tells
// the renderer, so the renderer takes the granules back at once. A process that ends lets go
// of every grant it held when its file closes.

#pragma once

#include <cerrno>
#include <cstdint>
#include <mutex>

#if defined(__linux__)
#include <fcntl.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <unistd.h>
#endif

namespace dsa {

struct GraphicsMemoryInfo {
    uint64_t length;
    uint64_t granule;
    uint64_t regionId;
    uint64_t ackBytes;
};

struct GraphicsMemoryGrantArg {
    uint64_t offset;
    uint64_t length;
    uint64_t key;
};

inline constexpr const char kGraphicsMemoryDevice[] = "/dev/dsa_graphics_memory";

#if defined(__linux__)
// The driver's ioctls (drivers/misc/dsa_graphics_memory.c).
inline constexpr unsigned long kGraphicsMemoryInfoIoctl = _IOR('D', 0x51, GraphicsMemoryInfo);
inline constexpr unsigned long kGraphicsMemoryBindIoctl = _IOW('D', 0x52, GraphicsMemoryGrantArg);
inline constexpr unsigned long kGraphicsMemoryReleaseIoctl = _IOW('D', 0x53, uint64_t);

// The process's one file of the device. Thread safe.
class GraphicsMemoryDevice {
   public:
    // The device, opened the first time it is asked for; null when this guest has none (an
    // image or kernel without it, or a host that shares no graphics memory). `why` says why,
    // the first time.
    static GraphicsMemoryDevice* get(const char** why = nullptr) {
        static GraphicsMemoryDevice device;
        static const char* failure = device.open();
        if (why) *why = failure;
        return failure ? nullptr : &device;
    }

    const GraphicsMemoryInfo& info() const { return mInfo; }

    // Binds and maps the grant. Null, with the grant left unbound, when the driver refuses it
    // (the key is not this grant's, another file holds it, or it has been revoked).
    uint8_t* map(uint64_t offset, uint64_t length, uint64_t key) {
        GraphicsMemoryGrantArg grant{offset, length, key};
        if (ioctl(mFd, kGraphicsMemoryBindIoctl, &grant) != 0) return nullptr;
        void* at = mmap(nullptr, length, PROT_READ | PROT_WRITE, MAP_SHARED, mFd,
                        static_cast<off_t>(offset));
        if (at == MAP_FAILED) {
            uint64_t start = offset;
            ioctl(mFd, kGraphicsMemoryReleaseIoctl, &start);
            return nullptr;
        }
        return static_cast<uint8_t*>(at);
    }

    // Unmaps the grant and lets the driver release it, before the renderer is told.
    void unmap(uint8_t* at, uint64_t offset, uint64_t length) {
        if (at) munmap(at, length);
        uint64_t start = offset;
        ioctl(mFd, kGraphicsMemoryReleaseIoctl, &start);
    }

   private:
    const char* open() {
        mFd = ::open(kGraphicsMemoryDevice, O_RDWR | O_CLOEXEC);
        if (mFd < 0) {
            // Each refusal says what it means: the node is there whenever the kernel is DSA's.
            switch (errno) {
                case ENOENT:
                    return "the guest has no /dev/dsa_graphics_memory";
                case ENODEV:
                    return "/dev/dsa_graphics_memory could not be opened (No such device, errno "
                           "19): the kernel was not told where the host's graphics memory is, "
                           "or did not find it there";
                case EACCES:
                    return "/dev/dsa_graphics_memory could not be opened (Permission denied, errno "
                           "13): SELinux or the node's mode refuses this process";
                default:
                    return "/dev/dsa_graphics_memory could not be opened";
            }
        }
        if (ioctl(mFd, kGraphicsMemoryInfoIoctl, &mInfo) != 0 || mInfo.length == 0) {
            ::close(mFd);
            mFd = -1;
            return "the guest kernel shares no graphics memory with the host";
        }
        return nullptr;
    }

    int mFd = -1;
    GraphicsMemoryInfo mInfo{};
};
#else
// No such device off Linux: every allocation keeps its copy.
class GraphicsMemoryDevice {
   public:
    static GraphicsMemoryDevice* get(const char** why = nullptr) {
        if (why) *why = "the guest is not Linux";
        return nullptr;
    }
    const GraphicsMemoryInfo& info() const { return mInfo; }
    uint8_t* map(uint64_t, uint64_t, uint64_t) { return nullptr; }
    void unmap(uint8_t*, uint64_t, uint64_t) {}

   private:
    GraphicsMemoryInfo mInfo{};
};
#endif

}  // namespace dsa
