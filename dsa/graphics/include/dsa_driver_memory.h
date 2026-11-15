// Copyright 2026 The DSA Authors
// SPDX-License-Identifier: Apache-2.0
//
// What the guest Vulkan driver holds of an app's host-visible memory, so a memory report can say
// it per app.
//
// Each host-visible allocation is either a grant of the graphics memory the host shares (the
// app maps the host's pages, dsa_graphics_memory.h) or a copy in the guest that is kept in step
// with the host (dsa_shadow_memory.h). The copies are anonymous mappings, which dumpsys meminfo
// counted under "Unknown" with everything else nobody named. Each copy is now named
// [anon:dsa-vk-copy] (needs a kernel with CONFIG_ANON_VMA_NAME), so smaps and meminfo show it
// on its own line for each app.
//
// The driver also counts, per process, the live copies and grants and why an allocation was
// copied rather than granted, and logs one line with those totals at most every few minutes,
// when an allocation is made or freed. A process that allocates nothing logs nothing, since
// nothing it holds changed.

#pragma once

#include <atomic>
#include <chrono>
#include <cinttypes>
#include <cstddef>
#include <cstdint>
#include <cstdio>

#if defined(__linux__)
#include <sys/prctl.h>
#endif

namespace dsa {

// The name every copy's mapping carries.
inline constexpr const char kCopyMappingName[] = "dsa-vk-copy";

// Names the anonymous mapping at `start` (`length` bytes) [anon:dsa-vk-copy]. False when the
// kernel refuses (no CONFIG_ANON_VMA_NAME); the mapping works the same either way.
inline bool nameCopyMapping(void* start, size_t length) {
#if defined(__linux__)
#ifndef PR_SET_VMA
#define PR_SET_VMA 0x53564d41
#endif
#ifndef PR_SET_VMA_ANON_NAME
#define PR_SET_VMA_ANON_NAME 0
#endif
    return prctl(PR_SET_VMA, PR_SET_VMA_ANON_NAME, reinterpret_cast<unsigned long>(start),
                 static_cast<unsigned long>(length),
                 reinterpret_cast<unsigned long>(kCopyMappingName)) == 0;
#else
    (void)start;
    (void)length;
    return false;
#endif
}

// Why a host-visible allocation got a copy rather than a grant.
enum class CopyReason : int {
    // The host shares no graphics memory with this guest, or this device cannot use it (no
    // kernel device, or an older renderer).
    NoSharedMemory = 0,
    // The host did not put the allocation in the shared memory (its region was full, or the
    // memory type is one it does not put there).
    HostKeptItOut = 1,
    // The host granted it, but the kernel would not map the grant.
    KernelWouldNotMap = 2,
};
inline constexpr int kCopyReasons = 3;

class DriverMemoryLog {
   public:
    // How often a process logs its line at most.
    static constexpr std::chrono::minutes kInterval{3};

    void copyMade(uint64_t bytes) {
        mCopies.fetch_add(1, std::memory_order_relaxed);
        mCopyBytes.fetch_add(bytes, std::memory_order_relaxed);
    }
    void copyFreed(uint64_t bytes) {
        mCopies.fetch_sub(1, std::memory_order_relaxed);
        mCopyBytes.fetch_sub(bytes, std::memory_order_relaxed);
    }
    void grantMapped(uint64_t bytes) {
        mGrants.fetch_add(1, std::memory_order_relaxed);
        mGrantBytes.fetch_add(bytes, std::memory_order_relaxed);
    }
    void grantUnmapped(uint64_t bytes) {
        mGrants.fetch_sub(1, std::memory_order_relaxed);
        mGrantBytes.fetch_sub(bytes, std::memory_order_relaxed);
    }
    void copied(CopyReason reason) {
        mReasons[static_cast<int>(reason)].fetch_add(1, std::memory_order_relaxed);
    }
    void named(bool ok) {
        if (!ok) mUnnamed.fetch_add(1, std::memory_order_relaxed);
    }

    // The line with this process's totals.
    void line(char* out, size_t size) const {
        const auto mib = [](uint64_t bytes) { return static_cast<double>(bytes) / 1048576.0; };
        std::snprintf(out, size,
                      "dsa-vk memory: %" PRIu64 " copies of host-visible memory, %.1f MiB "
                      "([anon:dsa-vk-copy], %" PRIu64 " left unnamed); %" PRIu64
                      " grants of the shared graphics memory, %.1f MiB; copied rather than "
                      "granted since start: %" PRIu64 " with no shared memory, %" PRIu64
                      " kept out by the host, %" PRIu64 " the kernel would not map",
                      mCopies.load(std::memory_order_relaxed),
                      mib(mCopyBytes.load(std::memory_order_relaxed)),
                      mUnnamed.load(std::memory_order_relaxed),
                      mGrants.load(std::memory_order_relaxed),
                      mib(mGrantBytes.load(std::memory_order_relaxed)),
                      mReasons[0].load(std::memory_order_relaxed),
                      mReasons[1].load(std::memory_order_relaxed),
                      mReasons[2].load(std::memory_order_relaxed));
    }

    // Writes the line through `log` when kInterval has passed since the last one (or there was
    // none), at `now`. One caller wins when several ask at once.
    template <typename Log>
    void maybeLog(std::chrono::steady_clock::time_point now, Log&& log) {
        const int64_t at = now.time_since_epoch().count();
        int64_t last = mLastLog.load(std::memory_order_relaxed);
        const int64_t interval =
            std::chrono::duration_cast<std::chrono::steady_clock::duration>(kInterval).count();
        if (last != kNever && at - last < interval) return;
        if (!mLastLog.compare_exchange_strong(last, at, std::memory_order_relaxed)) return;
        char text[400];
        line(text, sizeof(text));
        log(text);
    }

   private:
    static constexpr int64_t kNever = INT64_MIN;
    std::atomic<uint64_t> mCopies{0};
    std::atomic<uint64_t> mCopyBytes{0};
    std::atomic<uint64_t> mGrants{0};
    std::atomic<uint64_t> mGrantBytes{0};
    std::atomic<uint64_t> mUnnamed{0};
    std::atomic<uint64_t> mReasons[kCopyReasons] = {};
    std::atomic<int64_t> mLastLog{kNever};
};

// This process's.
inline DriverMemoryLog& driverMemoryLog() {
    static DriverMemoryLog log;
    return log;
}

}  // namespace dsa
