// Copyright 2026 The DSA Authors
// SPDX-License-Identifier: Apache-2.0

#include "picture_region.h"

#include <fcntl.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <unistd.h>

#include <cerrno>
#include <cstring>

#include "log.h"

namespace dsa::codec2 {

namespace {

// The header's size, and where the slots may start: the header has the first 64 KiB.
constexpr size_t kHeaderBytes = 24;
constexpr uint64_t kSlotsStart = 64 * 1024;

// The driver's one ioctl, the region's length as a 64-bit number: _IOR('D', 0x50, __u64).
constexpr unsigned long kSizeIoctl = 0x80084450UL;

uint64_t littleEndian64(const uint8_t* bytes) {
    uint64_t value = 0;
    for (int i = 7; i >= 0; --i) {
        value = (value << 8) | bytes[i];
    }
    return value;
}

}  // namespace

uint64_t pictureRegionId(const uint8_t* base, size_t size) {
    if (base == nullptr || size < kHeaderBytes || std::memcmp(base, "DSAVPIC1", 8) != 0) {
        return 0;
    }
    const uint64_t length = littleEndian64(base + 16);
    if (length > size) {
        return 0;
    }
    return littleEndian64(base + 8);
}

const uint8_t* pictureRegionSlice(const uint8_t* base, size_t size, uint64_t offset, size_t length) {
    if (base == nullptr || length == 0 || offset < kSlotsStart || offset > size || length > size - offset) {
        return nullptr;
    }
    return base + offset;
}

PictureRegion& PictureRegion::instance() {
    static PictureRegion* region = new PictureRegion();
    return *region;
}

void PictureRegion::mapLocked() {
    if (mBase != nullptr || mTest) {
        return;
    }
    const int fd = open(kPictureRegionDevice, O_RDONLY | O_CLOEXEC);
    if (fd < 0) {
        if (!mLogged) {
            CODEC_LOGI("pictures come through the video port: %s cannot be opened (%s)", kPictureRegionDevice,
                       std::strerror(errno));
            mLogged = true;
        }
        return;
    }
    uint64_t size = 0;
    if (ioctl(fd, kSizeIoctl, &size) != 0 || size == 0) {
        if (!mLogged) {
            CODEC_LOGW("pictures come through the video port: %s does not say its size (%s)",
                       kPictureRegionDevice, std::strerror(errno));
            mLogged = true;
        }
        close(fd);
        return;
    }
    void* mapped = mmap(nullptr, static_cast<size_t>(size), PROT_READ, MAP_SHARED, fd, 0);
    close(fd);
    if (mapped == MAP_FAILED) {
        if (!mLogged) {
            CODEC_LOGW("pictures come through the video port: %s cannot be mapped (%s)", kPictureRegionDevice,
                       std::strerror(errno));
            mLogged = true;
        }
        return;
    }
    mBase = static_cast<const uint8_t*>(mapped);
    mSize = static_cast<size_t>(size);
    CODEC_LOGI("pictures come through %zu MiB of memory shared with the host", mSize >> 20);
}

uint64_t PictureRegion::id() {
    std::lock_guard<std::mutex> lock(mLock);
    mapLocked();
    // Read each time: a new window manager writes the header of a new region.
    return pictureRegionId(mBase, mSize);
}

const uint8_t* PictureRegion::slice(uint64_t offset, size_t length) {
    std::lock_guard<std::mutex> lock(mLock);
    return pictureRegionSlice(mBase, mSize, offset, length);
}

void PictureRegion::useForTest(const uint8_t* base, size_t size) {
    std::lock_guard<std::mutex> lock(mLock);
    mBase = base;
    mSize = base == nullptr ? 0 : size;
    mTest = true;
}

}  // namespace dsa::codec2
