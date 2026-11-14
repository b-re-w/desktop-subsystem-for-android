// Copyright 2026 The DSA Authors
// SPDX-License-Identifier: Apache-2.0
//
// Which pages of the guest's copies of host-visible memory were written since the last flush,
// as the kernel tracks them.
//
// The guest Vulkan driver sends the host what the app wrote into its copies before each submit
// (dsa_shadow_blocks.h). It found that by hashing every block of every copy the app had mapped,
// so each submit read all of the app's mapped memory: a game that maps a few hundred megabytes
// spent two thirds of its render thread in the hash and never finished loading.
//
// The kernel knows which pages were written. The driver asks it, in this order:
//
// 1. Write protection in asynchronous mode (userfaultfd with UFFD_FEATURE_WP_ASYNC). Each copy
//    is registered and write protected once the app maps it. A write to a protected page goes
//    through at once: the kernel only marks the page written. At each flush the PAGEMAP_SCAN
//    ioctl lists the written pages and protects them again in the same step, so a write that
//    races the scan is listed now or at the next flush, never lost. A flush with nothing written
//    lists nothing, and no block is hashed or sent.
// 2. Soft-dirty bits, where the kernel keeps them (CONFIG_MEM_SOFT_DIRTY, x86_64 only): a page
//    written since the process's bits were last cleared shows bit 55 in /proc/self/pagemap.
//    Reading and clearing are two steps, and clearing is for the whole process, so a write that
//    lands between them is missed. A sweep therefore compares a bounded slice of the copies at
//    every flush (kSweepPages), and such a write still reaches the host within a few hundred
//    flushes. Clearing also makes the process's next write to each of its pages fault once.
// 3. Hashing every block, as before, when neither is there. The reason is logged once.
//
// A process the app forks has none of this tracking (the kernel drops it from the child), so
// a child compares every block. Copies are tracked from the app's first map of them, so memory
// the app never maps costs no page tables for the protection either.

#pragma once

#include <cerrno>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <map>
#include <mutex>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "dsa_shadow_blocks.h"

#if defined(__linux__)
#include <fcntl.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <sys/syscall.h>
#include <unistd.h>
#endif

namespace dsa {

// How this process learns which pages of its copies were written.
enum class DirtyTracking { kUnchosen, kWriteProtect, kSoftDirty, kHashing };

inline const char* dirtyTrackingName(DirtyTracking tracking) {
    switch (tracking) {
        case DirtyTracking::kWriteProtect:
            return "write protection (userfaultfd and PAGEMAP_SCAN)";
        case DirtyTracking::kSoftDirty:
            return "soft-dirty bits";
        case DirtyTracking::kHashing:
            return "hashing every block";
        case DirtyTracking::kUnchosen:
            break;
    }
    return "not chosen yet";
}

// Pages written, as addresses: [start, end).
struct WrittenRun {
    uint64_t start;
    uint64_t end;
};

// The bit of a /proc/self/pagemap entry that says the page is soft-dirty.
inline constexpr uint64_t kPagemapSoftDirty = 1ULL << 55;

// What the tracking asks of the kernel. LinuxDirtyKernel makes the calls; the tests fake them.
// Every call that can fail returns 0 or a count, or a negative errno.
class DirtyKernel {
   public:
    virtual ~DirtyKernel() = default;
    virtual uint64_t pageSize() = 0;
    virtual int processId() = 0;

    // Whether write protection works in this process: 0, or a negative errno with `reason`
    // saying what refused it.
    virtual int openWriteProtect(std::string* reason) = 0;
    // Starts protecting `length` bytes from `start`, every page counted clean.
    virtual int protect(uint64_t start, uint64_t length) = 0;
    virtual void unprotect(uint64_t start, uint64_t length) = 0;
    // The runs of pages in [start, end) written since they were last protected, written into
    // `runs`, and protected again in the same step. Returns how many. The scan stops early
    // when `runs` is full; `scanned` says where it stopped.
    virtual long takeWritten(uint64_t start, uint64_t end, WrittenRun* runs, size_t capacity,
                             uint64_t* scanned) = 0;

    // Whether soft-dirty bits work in this process, as openWriteProtect.
    virtual int openSoftDirty(std::string* reason) = 0;
    // Clears every soft-dirty bit of the process.
    virtual int clearSoftDirty() = 0;
    // Reads `count` pagemap entries, from the page at `start`. Returns how many it read.
    virtual long readPagemap(uint64_t start, uint64_t* entries, size_t count) = 0;

    virtual void log(const std::string& message) = 0;
};

// The process's tracking of its copies. Thread safe.
class DirtyPages {
   public:
    using Range = ShadowBlocks::Range;

    // Pages the soft-dirty sweep compares at each flush, across all copies: 1 MiB of 4 KiB
    // pages, a fraction of a millisecond of hashing.
    static constexpr uint64_t kSweepPages = 256;

    explicit DirtyPages(DirtyKernel& kernel) : kernel_(kernel), runs_(256), entries_(512) {}

    // The tracking this process uses, chosen on first use.
    DirtyTracking tracking() {
        std::lock_guard<std::mutex> lock(mutex_);
        chooseLocked();
        return tracking_;
    }

    // Starts tracking the copy at `base`, `length` bytes from a page boundary. Returns the id
    // its flushes name it by, or 0 when nothing tracks it and every block is compared.
    uint64_t track(const uint8_t* base, uint64_t length) {
        std::lock_guard<std::mutex> lock(mutex_);
        chooseLocked();
        if (forkedLocked() || length == 0) return 0;
        if (tracking_ != DirtyTracking::kWriteProtect && tracking_ != DirtyTracking::kSoftDirty) {
            return 0;
        }
        const uint64_t page = kernel_.pageSize();
        const uint64_t start = static_cast<uint64_t>(reinterpret_cast<uintptr_t>(base));
        if (page == 0 || start % page != 0) {
            noteLocked("a copy that does not start on a page boundary is compared block by block");
            return 0;
        }
        Region region;
        region.start = start;
        region.pages = (length + page - 1) / page;
        if (tracking_ == DirtyTracking::kWriteProtect) {
            const int error = kernel_.protect(start, region.pages * page);
            if (error != 0) {
                noteLocked("write protection refused a copy (" + errorName(error) +
                           "); it is compared block by block");
                return 0;
            }
        } else {
            region.pending.assign(static_cast<size_t>((region.pages + 63) / 64), 0);
        }
        const uint64_t id = nextId_++;
        regions_.emplace(id, std::move(region));
        return id;
    }

    // Ends the tracking of a copy, before its memory is unmapped.
    void untrack(uint64_t id) {
        std::lock_guard<std::mutex> lock(mutex_);
        auto it = regions_.find(id);
        if (it == regions_.end()) return;
        if (tracking_ == DirtyTracking::kWriteProtect && !forkedLocked() && !it->second.broken) {
            kernel_.unprotect(it->second.start, it->second.pages * kernel_.pageSize());
        }
        regions_.erase(it);
    }

    // A flush of several copies starts and ends: soft-dirty bits are read and cleared once
    // for all of them, not once for each.
    void beginFlush() {
        std::lock_guard<std::mutex> lock(mutex_);
        inFlush_ = true;
        collected_ = false;
    }
    void endFlush() {
        std::lock_guard<std::mutex> lock(mutex_);
        inFlush_ = false;
        collected_ = false;
    }

    // The bytes of copy `id` (offsets from its start, whole pages, in order, neighbours joined)
    // written since the last call. False when that is not known: then every block is compared.
    bool written(uint64_t id, std::vector<Range>* ranges) {
        ranges->clear();
        std::lock_guard<std::mutex> lock(mutex_);
        if (id == 0) return false;
        auto it = regions_.find(id);
        if (it == regions_.end() || forkedLocked() || it->second.broken) return false;
        switch (tracking_) {
            case DirtyTracking::kWriteProtect:
                return takeWrittenLocked(it->second, ranges);
            case DirtyTracking::kSoftDirty:
                if (!inFlush_ || !collected_) {
                    if (!collectSoftDirtyLocked()) return false;
                    collected_ = inFlush_;
                }
                if (it->second.broken) return false;
                drainLocked(it->second, ranges);
                return true;
            case DirtyTracking::kHashing:
            case DirtyTracking::kUnchosen:
                break;
        }
        return false;
    }

    size_t tracked() {
        std::lock_guard<std::mutex> lock(mutex_);
        return regions_.size();
    }

   private:
    struct Region {
        uint64_t start = 0;
        uint64_t pages = 0;
        // A scan or a read failed: compared block by block from then on.
        bool broken = false;
        // Soft-dirty: the pages seen written and not yet taken, one bit each.
        std::vector<uint64_t> pending;
    };

    static std::string errorName(int error) {
        const int code = error < 0 ? -error : error;
        return std::string(std::strerror(code)) + ", errno " + std::to_string(code);
    }

    // Each distinct message once, so a fallback is said once rather than at every flush.
    void noteLocked(const std::string& message) {
        if (noted_.insert(message).second) kernel_.log(message);
    }

    void chooseLocked() {
        if (tracking_ != DirtyTracking::kUnchosen) return;
        pid_ = kernel_.processId();
        std::string writeProtect;
        if (kernel_.openWriteProtect(&writeProtect) == 0) {
            tracking_ = DirtyTracking::kWriteProtect;
            noteLocked(std::string("host-visible memory: the pages written are found with ") +
                       dirtyTrackingName(tracking_));
            return;
        }
        std::string softDirty;
        if (kernel_.openSoftDirty(&softDirty) == 0) {
            tracking_ = DirtyTracking::kSoftDirty;
            noteLocked(std::string("host-visible memory: the pages written are found with ") +
                       dirtyTrackingName(tracking_) +
                       ", since write protection is not available (" + writeProtect + ")");
            return;
        }
        tracking_ = DirtyTracking::kHashing;
        noteLocked("host-visible memory: every block of each mapped copy is hashed at each submit, "
                   "since write protection is not available (" +
                   writeProtect + ") and soft-dirty bits are not either (" + softDirty + ")");
    }

    // A forked child has the parent's bookkeeping but none of its tracking, and the files it
    // inherited still name the parent's memory, so it never asks the kernel again.
    bool forkedLocked() {
        if (forked_) return true;
        if (tracking_ == DirtyTracking::kUnchosen || kernel_.processId() == pid_) return false;
        forked_ = true;
        noteLocked("host-visible memory: a forked process hashes every block of its copies");
        return true;
    }

    static void addRange(std::vector<Range>* ranges, uint64_t offset, uint64_t size) {
        if (!ranges->empty() && ranges->back().offset + ranges->back().size == offset) {
            ranges->back().size += size;
        } else {
            ranges->push_back(Range{offset, size});
        }
    }

    bool takeWrittenLocked(Region& region, std::vector<Range>* ranges) {
        const uint64_t end = region.start + region.pages * kernel_.pageSize();
        for (uint64_t at = region.start; at < end;) {
            uint64_t scanned = end;
            const long count = kernel_.takeWritten(at, end, runs_.data(), runs_.size(), &scanned);
            if (count < 0) {
                region.broken = true;
                noteLocked("PAGEMAP_SCAN failed on a copy (" + errorName(static_cast<int>(count)) +
                           "); it is compared block by block");
                ranges->clear();
                return false;
            }
            for (long i = 0; i < count; i++) {
                const uint64_t from = runs_[i].start < at ? at : runs_[i].start;
                const uint64_t to = runs_[i].end > end ? end : runs_[i].end;
                if (from < to) addRange(ranges, from - region.start, to - from);
            }
            // A full list means the scan stopped early, where it says.
            if (static_cast<size_t>(count) < runs_.size() || scanned <= at) break;
            at = scanned;
        }
        return true;
    }

    static void setBit(std::vector<uint64_t>& bits, uint64_t index) {
        bits[static_cast<size_t>(index / 64)] |= 1ULL << (index % 64);
    }

    // Reads the soft-dirty bits of every tracked copy and clears them, as one step as far as
    // the kernel allows: each copy's written pages wait in its pending bits until its flush.
    bool collectSoftDirtyLocked() {
        const uint64_t page = kernel_.pageSize();
        for (auto& [id, region] : regions_) {
            (void)id;
            if (region.broken) continue;
            for (uint64_t first = 0; first < region.pages; first += entries_.size()) {
                const uint64_t left = region.pages - first;
                const size_t count =
                    left < entries_.size() ? static_cast<size_t>(left) : entries_.size();
                const long read =
                    kernel_.readPagemap(region.start + first * page, entries_.data(), count);
                if (read != static_cast<long>(count)) {
                    region.broken = true;
                    noteLocked("/proc/self/pagemap could not be read for a copy (" +
                               (read < 0 ? errorName(static_cast<int>(read)) : "short read") +
                               "); it is compared block by block");
                    break;
                }
                for (size_t i = 0; i < count; i++) {
                    if (entries_[i] & kPagemapSoftDirty) setBit(region.pending, first + i);
                }
            }
        }
        const int error = kernel_.clearSoftDirty();
        if (error != 0) {
            tracking_ = DirtyTracking::kHashing;
            noteLocked("host-visible memory: clearing the soft-dirty bits failed (" +
                       errorName(error) +
                       "); every block of each mapped copy is hashed at each submit from now on");
            return false;
        }
        sweepLocked();
        return true;
    }

    // Marks the next kSweepPages pages of the copies, in turn, as written, for a write that
    // landed between a read of the bits and their clearing.
    void sweepLocked() {
        uint64_t budget = kSweepPages;
        size_t visited = 0;
        auto it = regions_.lower_bound(sweepRegion_);
        while (budget > 0 && !regions_.empty() && visited <= regions_.size()) {
            if (it == regions_.end()) it = regions_.begin();
            if (it->first != sweepRegion_) {
                sweepRegion_ = it->first;
                sweepPage_ = 0;
            }
            Region& region = it->second;
            const uint64_t left = region.pages > sweepPage_ ? region.pages - sweepPage_ : 0;
            const uint64_t count = left < budget ? left : budget;
            if (!region.broken) {
                for (uint64_t i = 0; i < count; i++) setBit(region.pending, sweepPage_ + i);
            }
            budget -= count;
            sweepPage_ += count;
            if (sweepPage_ >= region.pages) {
                ++it;
                visited++;
                sweepRegion_ = it == regions_.end() ? 0 : it->first;
                sweepPage_ = 0;
            }
        }
    }

    // Hands a copy's pending pages over as ranges and forgets them.
    void drainLocked(Region& region, std::vector<Range>* ranges) {
        const uint64_t page = kernel_.pageSize();
        for (size_t word = 0; word < region.pending.size(); word++) {
            uint64_t bits = region.pending[word];
            region.pending[word] = 0;
            while (bits != 0) {
                const int bit = __builtin_ctzll(bits);
                bits &= bits - 1;
                const uint64_t index =
                    static_cast<uint64_t>(word) * 64 + static_cast<uint64_t>(bit);
                if (index < region.pages) addRange(ranges, index * page, page);
            }
        }
    }

    DirtyKernel& kernel_;
    std::mutex mutex_;
    DirtyTracking tracking_ = DirtyTracking::kUnchosen;
    int pid_ = 0;
    bool forked_ = false;
    bool inFlush_ = false;
    bool collected_ = false;
    uint64_t nextId_ = 1;
    std::map<uint64_t, Region> regions_;
    uint64_t sweepRegion_ = 0;
    uint64_t sweepPage_ = 0;
    std::vector<WrittenRun> runs_;
    std::vector<uint64_t> entries_;
    std::set<std::string> noted_;
};

// What of a copy changed since the last flush: the blocks the kernel saw written whose bytes
// differ, or every block that differs where the copy is not tracked. `compared`, when given,
// grows by the bytes hashed.
inline std::vector<ShadowBlocks::Range> changedShadow(DirtyPages& pages, uint64_t region,
                                                      ShadowBlocks& blocks, const uint8_t* memory,
                                                      uint64_t* compared = nullptr) {
    std::vector<ShadowBlocks::Range> written;
    if (pages.written(region, &written)) return blocks.changedIn(memory, written, compared);
    if (compared) *compared += blocks.size();
    return blocks.changed(memory);
}

// The host and the copy agree on all of it now (a read back before the app had it): the
// hashes of the blocks written since the last flush are taken again, or of every block where
// the copy is not tracked.
inline void refreshShadow(DirtyPages& pages, uint64_t region, ShadowBlocks& blocks,
                          const uint8_t* memory) {
    std::vector<ShadowBlocks::Range> written;
    if (pages.written(region, &written)) {
        blocks.refreshIn(memory, written);
    } else {
        blocks.refresh(memory);
    }
}

#if defined(__linux__)

// The Linux calls. The structures and numbers are the kernel's user API (linux/userfaultfd.h
// and linux/fs.h from Linux 6.7), spelt out here because the Android NDK's headers predate
// asynchronous write protection and PAGEMAP_SCAN.
namespace linux_uapi {
inline constexpr uint64_t kUffdApi = 0xAA;
inline constexpr uint64_t kUffdFeatureWpUnpopulated = 1ULL << 13;
inline constexpr uint64_t kUffdFeatureWpAsync = 1ULL << 15;
inline constexpr int kUffdUserModeOnly = 1;
inline constexpr uint64_t kUffdioRegisterModeWp = 1ULL << 1;
inline constexpr uint64_t kUffdioWriteProtectModeWp = 1ULL << 0;
struct UffdioApi {
    uint64_t api;
    uint64_t features;
    uint64_t ioctls;
};
struct UffdioRange {
    uint64_t start;
    uint64_t len;
};
struct UffdioRegister {
    UffdioRange range;
    uint64_t mode;
    uint64_t ioctls;
};
struct UffdioWriteProtect {
    UffdioRange range;
    uint64_t mode;
};
inline constexpr uint64_t kPageIsWritten = 1ULL << 1;
inline constexpr uint64_t kPmScanWpMatching = 1ULL << 0;
inline constexpr uint64_t kPmScanCheckWpAsync = 1ULL << 1;
struct PageRegion {
    uint64_t start;
    uint64_t end;
    uint64_t categories;
};
struct PmScanArg {
    uint64_t size;
    uint64_t flags;
    uint64_t start;
    uint64_t end;
    uint64_t walkEnd;
    uint64_t vec;
    uint64_t vecLen;
    uint64_t maxPages;
    uint64_t categoryInverted;
    uint64_t categoryMask;
    uint64_t categoryAnyofMask;
    uint64_t returnMask;
};
static_assert(sizeof(UffdioApi) == 24 && sizeof(UffdioRegister) == 32 &&
                  sizeof(UffdioWriteProtect) == 24 && sizeof(PageRegion) == 24 &&
                  sizeof(PmScanArg) == 96,
              "the kernel's structures are laid out as the kernel lays them out");
// The kernel's request numbers, as the unsigned int bionic's ioctl takes for one.
inline unsigned int uffdioApi() { return _IOWR(0xAA, 0x3F, UffdioApi); }
inline unsigned int uffdioRegister() { return _IOWR(0xAA, 0x00, UffdioRegister); }
inline unsigned int uffdioUnregister() { return _IOR(0xAA, 0x01, UffdioRange); }
inline unsigned int uffdioWriteProtect() { return _IOWR(0xAA, 0x06, UffdioWriteProtect); }
inline unsigned int pagemapScan() { return _IOWR('f', 16, PmScanArg); }
}  // namespace linux_uapi

class LinuxDirtyKernel final : public DirtyKernel {
   public:
    // `log` gets each message once; stderr when it is null.
    explicit LinuxDirtyKernel(void (*log)(const char*) = nullptr) : log_(log) {}
    ~LinuxDirtyKernel() override {
        closeWriteProtect();
        if (clearRefs_ >= 0) close(clearRefs_);
        if (pagemap_ >= 0) close(pagemap_);
    }
    LinuxDirtyKernel(const LinuxDirtyKernel&) = delete;
    LinuxDirtyKernel& operator=(const LinuxDirtyKernel&) = delete;

    uint64_t pageSize() override {
        static const uint64_t page = static_cast<uint64_t>(sysconf(_SC_PAGESIZE));
        return page;
    }

    int processId() override { return static_cast<int>(getpid()); }

    int openWriteProtect(std::string* reason) override {
        using namespace linux_uapi;
#if defined(__NR_userfaultfd)
        int fd = static_cast<int>(
            syscall(__NR_userfaultfd, O_CLOEXEC | O_NONBLOCK | kUffdUserModeOnly));
        if (fd < 0 && errno == EINVAL) {
            // A kernel before 5.11 knows no user-mode-only flag.
            fd = static_cast<int>(syscall(__NR_userfaultfd, O_CLOEXEC | O_NONBLOCK));
        }
        if (fd < 0) return failed(reason, "userfaultfd() was refused", errno);
        UffdioApi api = {kUffdApi, kUffdFeatureWpAsync | kUffdFeatureWpUnpopulated, 0};
        if (ioctl(fd, uffdioApi(), &api) != 0) {
            const int error = errno;
            close(fd);
            return failed(reason, "the kernel has no asynchronous write protection (UFFDIO_API)",
                          error);
        }
        if ((api.features & (kUffdFeatureWpAsync | kUffdFeatureWpUnpopulated)) !=
            (kUffdFeatureWpAsync | kUffdFeatureWpUnpopulated)) {
            close(fd);
            *reason = "the kernel has no asynchronous write protection of anonymous memory";
            return -ENOTSUP;
        }
        const int pagemap = openPagemap(reason);
        if (pagemap < 0) {
            close(fd);
            return pagemap;
        }
        uffd_ = fd;
        // One page through the whole cycle, so a refusal (SELinux on the ioctls, a kernel
        // without PAGEMAP_SCAN) shows here and not at the app's first flush.
        const uint64_t page = pageSize();
        void* probe = mapProbe(page);
        if (probe == MAP_FAILED) {
            const int error = errno;
            closeWriteProtect();
            return failed(reason, "no memory for a probe", error);
        }
        const uint64_t start = static_cast<uint64_t>(reinterpret_cast<uintptr_t>(probe));
        WrittenRun runs[4];
        uint64_t scanned = 0;
        int error = protect(start, page);
        const char* step = "UFFDIO_REGISTER or UFFDIO_WRITEPROTECT was refused";
        if (error == 0) {
            static_cast<volatile uint8_t*>(probe)[1] = 1;
            const long first = takeWritten(start, start + page, runs, 4, &scanned);
            const long second =
                first < 0 ? first : takeWritten(start, start + page, runs, 4, &scanned);
            if (first < 0 || second < 0) {
                error = static_cast<int>(first < 0 ? first : second);
                step = "PAGEMAP_SCAN on /proc/self/pagemap was refused";
            } else if (first != 1 || second != 0) {
                error = -EIO;
                step = "PAGEMAP_SCAN did not report the probe's write exactly once";
            }
            unprotect(start, page);
        }
        munmap(probe, page);
        if (error != 0) {
            closeWriteProtect();
            return failed(reason, step, -error);
        }
        return 0;
#else
        *reason = "this build has no userfaultfd system call";
        return -ENOSYS;
#endif
    }

    int protect(uint64_t start, uint64_t length) override {
        using namespace linux_uapi;
#if defined(MADV_NOHUGEPAGE)
        // Page by page: a huge page would count as written whole for one byte.
        madvise(reinterpret_cast<void*>(static_cast<uintptr_t>(start)),
                static_cast<size_t>(length), MADV_NOHUGEPAGE);
#endif
        UffdioRegister request = {{start, length}, kUffdioRegisterModeWp, 0};
        if (ioctl(uffd_, uffdioRegister(), &request) != 0) return -errno;
        UffdioWriteProtect protection = {{start, length}, kUffdioWriteProtectModeWp};
        if (ioctl(uffd_, uffdioWriteProtect(), &protection) != 0) {
            const int error = errno;
            unprotect(start, length);
            return -error;
        }
        return 0;
    }

    void unprotect(uint64_t start, uint64_t length) override {
        linux_uapi::UffdioRange range = {start, length};
        ioctl(uffd_, linux_uapi::uffdioUnregister(), &range);
    }

    long takeWritten(uint64_t start, uint64_t end, WrittenRun* runs, size_t capacity,
                     uint64_t* scanned) override {
        using namespace linux_uapi;
        if (regions_.size() < capacity) regions_.resize(capacity);
        PmScanArg arg = {};
        arg.size = sizeof(arg);
        arg.flags = kPmScanWpMatching | kPmScanCheckWpAsync;
        arg.start = start;
        arg.end = end;
        arg.vec = static_cast<uint64_t>(reinterpret_cast<uintptr_t>(regions_.data()));
        arg.vecLen = capacity;
        arg.categoryMask = kPageIsWritten;
        arg.returnMask = kPageIsWritten;
        const long count = ioctl(pagemap_, pagemapScan(), &arg);
        if (count < 0) return -errno;
        for (long i = 0; i < count; i++) runs[i] = WrittenRun{regions_[i].start, regions_[i].end};
        *scanned = arg.walkEnd;
        return count;
    }

    int openSoftDirty(std::string* reason) override {
        const int pagemap = pagemap_ >= 0 ? pagemap_ : openPagemap(reason);
        if (pagemap < 0) return pagemap;
        clearRefs_ = open("/proc/self/clear_refs", O_WRONLY | O_CLOEXEC);
        if (clearRefs_ < 0) {
            return failed(reason, "/proc/self/clear_refs could not be opened", errno);
        }
        // One page written, cleared and written again, as each flush will see it.
        const uint64_t page = pageSize();
        void* probe = mapProbe(page);
        if (probe == MAP_FAILED) return closeSoftDirty(reason, "no memory for a probe", errno);
        const uint64_t start = static_cast<uint64_t>(reinterpret_cast<uintptr_t>(probe));
        static_cast<volatile uint8_t*>(probe)[1] = 1;
        uint64_t cleared = 0;
        uint64_t written = 0;
        const int error = clearSoftDirty();
        bool read = false;
        if (error == 0 && readPagemap(start, &cleared, 1) == 1) {
            static_cast<volatile uint8_t*>(probe)[1] = 2;
            read = readPagemap(start, &written, 1) == 1;
        }
        munmap(probe, page);
        if (error != 0) {
            return closeSoftDirty(reason, "writing /proc/self/clear_refs failed", -error);
        }
        if (!read) return closeSoftDirty(reason, "/proc/self/pagemap could not be read", EIO);
        if ((cleared & kPagemapSoftDirty) || !(written & kPagemapSoftDirty)) {
            return closeSoftDirty(
                reason, "the kernel keeps no soft-dirty bits (CONFIG_MEM_SOFT_DIRTY)", ENOTSUP);
        }
        return 0;
    }

    int clearSoftDirty() override {
        // 4 clears the soft-dirty bits of every page of the process.
        errno = 0;
        if (write(clearRefs_, "4", 1) != 1) return errno ? -errno : -EIO;
        return 0;
    }

    long readPagemap(uint64_t start, uint64_t* entries, size_t count) override {
        const uint64_t page = pageSize();
        const ssize_t read = pread(pagemap_, entries, count * sizeof(uint64_t),
                                   static_cast<off_t>(start / page * sizeof(uint64_t)));
        if (read < 0) return -errno;
        return static_cast<long>(read / static_cast<ssize_t>(sizeof(uint64_t)));
    }

    void log(const std::string& message) override {
        if (log_) {
            log_(message.c_str());
        } else {
            std::fprintf(stderr, "%s\n", message.c_str());
        }
    }

   private:
    static int failed(std::string* reason, const char* what, int error) {
        *reason = std::string(what) + ": " + std::strerror(error) + ", errno " +
                  std::to_string(error);
        return -error;
    }

    static void* mapProbe(uint64_t page) {
        return mmap(nullptr, static_cast<size_t>(page), PROT_READ | PROT_WRITE,
                    MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    }

    int openPagemap(std::string* reason) {
        pagemap_ = open("/proc/self/pagemap", O_RDONLY | O_CLOEXEC);
        if (pagemap_ < 0) return failed(reason, "/proc/self/pagemap could not be opened", errno);
        return pagemap_;
    }

    void closeWriteProtect() {
        if (uffd_ >= 0) close(uffd_);
        uffd_ = -1;
    }

    int closeSoftDirty(std::string* reason, const char* what, int error) {
        if (clearRefs_ >= 0) close(clearRefs_);
        clearRefs_ = -1;
        return failed(reason, what, error);
    }

    void (*log_)(const char*);
    int uffd_ = -1;
    int pagemap_ = -1;
    int clearRefs_ = -1;
    std::vector<linux_uapi::PageRegion> regions_;
};

// The process's one tracking, shared by every device's copies. `log` (the driver's log) is
// taken from the first call.
inline DirtyPages& dirtyPages(void (*log)(const char*) = nullptr) {
    static LinuxDirtyKernel* kernel = new LinuxDirtyKernel(log);
    static DirtyPages* pages = new DirtyPages(*kernel);
    return *pages;
}

#endif  // defined(__linux__)

}  // namespace dsa
