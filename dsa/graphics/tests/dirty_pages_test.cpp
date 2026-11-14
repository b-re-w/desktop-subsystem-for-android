// Copyright 2026 The DSA Authors
// SPDX-License-Identifier: Apache-2.0
//
// How the guest's Vulkan driver learns which pages of its copies of host-visible memory the
// app wrote, so a submit compares and sends only those (dsa_dirty_pages.h). A fake kernel
// stands in for userfaultfd, PAGEMAP_SCAN and the soft-dirty bits; the last tests ask the
// kernel the tests run on, where it allows.

#include <sys/mman.h>
#include <unistd.h>

#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <map>
#include <string>
#include <vector>

#include "dsa_dirty_pages.h"

namespace {

using dsa::DirtyPages;
using dsa::DirtyTracking;
using dsa::kShadowBlockSize;
using dsa::ShadowBlocks;
using Range = ShadowBlocks::Range;

int failures = 0;

void check(bool ok, const char* test, const char* what) {
    if (!ok) {
        std::fprintf(stderr, "%s: %s\n", test, what);
        failures++;
    }
}

constexpr uint64_t kPage = 4096;

// A kernel that keeps, for each page, whether it was written since it was protected (write
// protection) and its soft-dirty bit, and counts what it is asked.
class FakeKernel final : public dsa::DirtyKernel {
   public:
    int writeProtectError = 0;
    int softDirtyError = 0;
    int scanError = 0;
    int clearError = 0;
    // Each page its own run, as a kernel may report them, to show they are joined.
    bool runPerPage = false;
    int pid = 100;
    int scans = 0;
    int clears = 0;
    int pagemapReads = 0;
    int protects = 0;
    int unprotects = 0;
    std::vector<std::string> logs;

    uint64_t pageSize() override { return kPage; }
    int processId() override { return pid; }

    int openWriteProtect(std::string* reason) override {
        if (writeProtectError) *reason = "userfaultfd() was refused";
        return writeProtectError;
    }
    int protect(uint64_t start, uint64_t length) override {
        protects++;
        for (uint64_t at = start; at < start + length; at += kPage) {
            protected_[at] = true;
            written_[at] = false;
        }
        return 0;
    }
    void unprotect(uint64_t start, uint64_t length) override {
        unprotects++;
        for (uint64_t at = start; at < start + length; at += kPage) protected_.erase(at);
    }
    long takeWritten(uint64_t start, uint64_t end, dsa::WrittenRun* runs, size_t capacity,
                     uint64_t* scanned) override {
        scans++;
        if (scanError) return scanError;
        size_t count = 0;
        *scanned = end;
        for (uint64_t at = start; at < end; at += kPage) {
            if (!written_[at]) continue;
            if (!runPerPage && count > 0 && runs[count - 1].end == at) {
                runs[count - 1].end += kPage;
            } else if (count == capacity) {
                *scanned = at;
                return static_cast<long>(count);
            } else {
                runs[count++] = dsa::WrittenRun{at, at + kPage};
            }
            written_[at] = false;
        }
        return static_cast<long>(count);
    }

    int openSoftDirty(std::string* reason) override {
        if (softDirtyError) *reason = "the kernel keeps no soft-dirty bits";
        return softDirtyError;
    }
    int clearSoftDirty() override {
        clears++;
        if (clearError) return clearError;
        softDirty_.clear();
        return 0;
    }
    long readPagemap(uint64_t start, uint64_t* entries, size_t count) override {
        pagemapReads++;
        for (size_t i = 0; i < count; i++) {
            entries[i] = softDirty_[start + i * kPage] ? dsa::kPagemapSoftDirty : 0;
        }
        return static_cast<long>(count);
    }

    void log(const std::string& message) override { logs.push_back(message); }

    // The app (or the driver's read back) writes into `memory` at `offset`.
    void write(uint8_t* memory, uint64_t offset, uint8_t value) {
        memory[offset] = value;
        const uint64_t page = reinterpret_cast<uintptr_t>(memory + offset) / kPage * kPage;
        written_[page] = true;
        softDirty_[page] = true;
    }

   private:
    std::map<uint64_t, bool> protected_;
    std::map<uint64_t, bool> written_;
    std::map<uint64_t, bool> softDirty_;
};

// Page-aligned memory for a copy, as allocateShadowMemory gives.
struct Copy {
    explicit Copy(uint64_t size) : size(size) {
        void* at = nullptr;
        if (posix_memalign(&at, kPage, size) != 0) std::abort();
        std::memset(at, 0, size);
        memory = static_cast<uint8_t*>(at);
    }
    ~Copy() { std::free(memory); }
    uint64_t size;
    uint8_t* memory;
};

void nfr2_a_flush_with_nothing_written_hashes_and_sends_nothing() {
    const char* test = __func__;
    FakeKernel kernel;
    DirtyPages pages(kernel);
    Copy copy(256 * kPage);
    ShadowBlocks blocks(copy.size);
    const uint64_t region = pages.track(copy.memory, copy.size);
    check(region != 0, test, "the copy is tracked");
    check(pages.tracking() == DirtyTracking::kWriteProtect, test, "with write protection");
    uint64_t compared = 0;
    const auto ranges = dsa::changedShadow(pages, region, blocks, copy.memory, &compared);
    check(ranges.empty(), test, "nothing to send");
    check(compared == 0, test, "and not one block hashed");
}

void nfr2_written_pages_become_ranges_joined_where_they_meet() {
    const char* test = __func__;
    FakeKernel kernel;
    kernel.runPerPage = true;
    DirtyPages pages(kernel);
    Copy copy(16 * kPage);
    ShadowBlocks blocks(copy.size);
    const uint64_t region = pages.track(copy.memory, copy.size);
    kernel.write(copy.memory, 1 * kPage + 3, 1);
    kernel.write(copy.memory, 2 * kPage + 9, 1);
    kernel.write(copy.memory, 5 * kPage, 1);
    std::vector<Range> written;
    check(pages.written(region, &written), test, "the kernel knows");
    check(written.size() == 2, test, "two ranges");
    check(written.size() == 2 && written[0].offset == kPage && written[0].size == 2 * kPage, test,
          "the neighbours join");
    check(written.size() == 2 && written[1].offset == 5 * kPage && written[1].size == kPage, test,
          "the other stays apart");
    check(pages.written(region, &written) && written.empty(), test,
          "each write is reported once");
}

void nfr2_only_written_blocks_whose_bytes_changed_are_sent() {
    const char* test = __func__;
    FakeKernel kernel;
    DirtyPages pages(kernel);
    Copy copy(64 * kPage);
    ShadowBlocks blocks(copy.size);
    const uint64_t region = pages.track(copy.memory, copy.size);
    kernel.write(copy.memory, 7 * kPage + 1, 5);
    // Written with what it held: the kernel says written, the hash says unchanged.
    kernel.write(copy.memory, 9 * kPage, 0);
    uint64_t compared = 0;
    const auto ranges = dsa::changedShadow(pages, region, blocks, copy.memory, &compared);
    check(ranges.size() == 1 && ranges[0].offset == 7 * kShadowBlockSize &&
              ranges[0].size == kShadowBlockSize,
          test, "the changed block alone goes");
    check(compared == 2 * kShadowBlockSize, test, "only the two written blocks are hashed");
}

void nfr2_a_full_list_of_runs_continues_where_the_scan_stopped() {
    const char* test = __func__;
    FakeKernel kernel;
    DirtyPages pages(kernel);
    const uint64_t count = 2000;
    Copy copy(count * kPage);
    const uint64_t region = pages.track(copy.memory, copy.size);
    // Every other page: more runs than one scan holds.
    for (uint64_t page = 0; page < count; page += 2) kernel.write(copy.memory, page * kPage, 1);
    std::vector<Range> written;
    check(pages.written(region, &written), test, "the kernel knows");
    check(written.size() == count / 2, test, "every written page is reported");
    check(kernel.scans > 1, test, "over more than one scan");
    bool exact = written.size() == count / 2;
    for (size_t i = 0; exact && i < written.size(); i++) {
        exact = written[i].offset == 2 * i * kPage && written[i].size == kPage;
    }
    check(exact, test, "each one where it was written");
}

void nfr2_write_protection_comes_first_then_soft_dirty_bits_then_hashing() {
    const char* test = __func__;
    {
        FakeKernel kernel;
        DirtyPages pages(kernel);
        check(pages.tracking() == DirtyTracking::kWriteProtect, test, "write protection first");
    }
    {
        FakeKernel kernel;
        kernel.writeProtectError = -EACCES;
        DirtyPages pages(kernel);
        check(pages.tracking() == DirtyTracking::kSoftDirty, test, "soft-dirty bits next");
        check(kernel.logs.size() == 1 &&
                  kernel.logs[0].find("userfaultfd() was refused") != std::string::npos,
              test, "saying why write protection is not used");
    }
    {
        FakeKernel kernel;
        kernel.writeProtectError = -EACCES;
        kernel.softDirtyError = -ENOTSUP;
        DirtyPages pages(kernel);
        check(pages.tracking() == DirtyTracking::kHashing, test, "hashing last");
        check(kernel.logs.size() == 1 &&
                  kernel.logs[0].find("userfaultfd() was refused") != std::string::npos &&
                  kernel.logs[0].find("no soft-dirty bits") != std::string::npos,
              test, "saying once why neither is used");
    }
}

void nfr2_without_tracking_every_block_is_compared_and_the_reason_is_logged_once() {
    const char* test = __func__;
    FakeKernel kernel;
    kernel.writeProtectError = -ENOSYS;
    kernel.softDirtyError = -ENOTSUP;
    DirtyPages pages(kernel);
    Copy copy(8 * kPage);
    ShadowBlocks blocks(copy.size);
    const uint64_t region = pages.track(copy.memory, copy.size);
    check(region == 0, test, "nothing tracks the copy");
    pages.track(copy.memory, copy.size);
    copy.memory[3 * kPage] = 1;
    uint64_t compared = 0;
    const auto ranges = dsa::changedShadow(pages, region, blocks, copy.memory, &compared);
    check(ranges.size() == 1 && ranges[0].offset == 3 * kShadowBlockSize, test,
          "a write is still found");
    check(compared == copy.size, test, "by hashing every block");
    check(kernel.logs.size() == 1, test, "the reason is logged once for the process");
}

void nfr2_a_forked_child_compares_every_block_and_never_asks_the_parents_kernel_state() {
    const char* test = __func__;
    FakeKernel kernel;
    DirtyPages pages(kernel);
    Copy copy(8 * kPage);
    ShadowBlocks blocks(copy.size);
    const uint64_t region = pages.track(copy.memory, copy.size);
    kernel.pid = 101;
    const int scans = kernel.scans;
    copy.memory[2 * kPage] = 9;
    uint64_t compared = 0;
    const auto ranges = dsa::changedShadow(pages, region, blocks, copy.memory, &compared);
    check(ranges.size() == 1 && ranges[0].offset == 2 * kShadowBlockSize, test,
          "the child's write is found");
    check(compared == copy.size, test, "by hashing every block");
    check(kernel.scans == scans, test, "without a scan of the parent's memory");
    check(pages.track(copy.memory, copy.size) == 0, test, "and it tracks no new copy");
    pages.untrack(region);
    check(kernel.unprotects == 0, test, "nor unprotects the parent's");
}

void nfr2_a_failed_scan_leaves_that_copy_to_hashing() {
    const char* test = __func__;
    FakeKernel kernel;
    DirtyPages pages(kernel);
    Copy copy(8 * kPage);
    ShadowBlocks blocks(copy.size);
    const uint64_t region = pages.track(copy.memory, copy.size);
    kernel.scanError = -EPERM;
    copy.memory[kPage] = 4;
    for (int flush = 0; flush < 3; flush++) {
        uint64_t compared = 0;
        const auto ranges = dsa::changedShadow(pages, region, blocks, copy.memory, &compared);
        check(compared == copy.size, test, "every block is compared");
        if (flush == 0) check(ranges.size() == 1, test, "the write is not lost");
    }
    check(kernel.scans == 1, test, "the scan is not tried again");
    check(kernel.logs.size() == 2, test, "the failure is logged once");
}

void nfr2_an_untracked_copy_is_unprotected_and_forgotten() {
    const char* test = __func__;
    FakeKernel kernel;
    DirtyPages pages(kernel);
    Copy copy(8 * kPage);
    const uint64_t region = pages.track(copy.memory, copy.size);
    check(pages.tracked() == 1 && kernel.protects == 1, test, "tracked and protected");
    pages.untrack(region);
    check(pages.tracked() == 0 && kernel.unprotects == 1, test, "unprotected before it is freed");
    std::vector<Range> written;
    check(!pages.written(region, &written), test, "a freed copy is not asked about");
}

void nfr2_soft_dirty_bits_are_read_and_cleared_once_for_a_flush_of_several_copies() {
    const char* test = __func__;
    FakeKernel kernel;
    kernel.writeProtectError = -EACCES;
    DirtyPages pages(kernel);
    Copy first(64 * kPage);
    Copy second(64 * kPage);
    ShadowBlocks firstBlocks(first.size);
    ShadowBlocks secondBlocks(second.size);
    const uint64_t a = pages.track(first.memory, first.size);
    const uint64_t b = pages.track(second.memory, second.size);
    // The sweep's first slice covers these copies; take it before the test.
    pages.beginFlush();
    dsa::changedShadow(pages, a, firstBlocks, first.memory);
    dsa::changedShadow(pages, b, secondBlocks, second.memory);
    pages.endFlush();
    kernel.write(first.memory, 10 * kPage, 1);
    kernel.write(second.memory, 20 * kPage, 2);
    const int clears = kernel.clears;
    pages.beginFlush();
    const auto one = dsa::changedShadow(pages, a, firstBlocks, first.memory);
    const auto two = dsa::changedShadow(pages, b, secondBlocks, second.memory);
    pages.endFlush();
    check(kernel.clears == clears + 1, test, "one clear for the flush");
    check(one.size() == 1 && one[0].offset == 10 * kShadowBlockSize, test, "the first copy's write");
    check(two.size() == 1 && two[0].offset == 20 * kShadowBlockSize, test, "the second copy's");
}

void nfr2_soft_dirty_bits_of_a_copy_another_flush_cleared_are_kept_for_it() {
    const char* test = __func__;
    FakeKernel kernel;
    kernel.writeProtectError = -EACCES;
    DirtyPages pages(kernel);
    Copy first(300 * kPage);
    Copy second(300 * kPage);
    ShadowBlocks firstBlocks(first.size);
    ShadowBlocks secondBlocks(second.size);
    const uint64_t a = pages.track(first.memory, first.size);
    const uint64_t b = pages.track(second.memory, second.size);
    kernel.write(second.memory, 299 * kPage, 7);
    // Another device's flush clears the process's bits; the second copy is not in it.
    pages.beginFlush();
    dsa::changedShadow(pages, a, firstBlocks, first.memory);
    pages.endFlush();
    pages.beginFlush();
    const auto ranges = dsa::changedShadow(pages, b, secondBlocks, second.memory);
    pages.endFlush();
    bool found = false;
    for (const Range& range : ranges) {
        found |= range.offset <= 299 * kShadowBlockSize &&
                 range.offset + range.size > 299 * kShadowBlockSize;
    }
    check(found, test, "the write survives the other flush's clear");
}

void nfr2_the_soft_dirty_sweep_compares_a_bounded_slice_and_reaches_every_page() {
    const char* test = __func__;
    FakeKernel kernel;
    kernel.writeProtectError = -EACCES;
    DirtyPages pages(kernel);
    const uint64_t count = 1000;
    Copy copy(count * kPage);
    ShadowBlocks blocks(copy.size);
    const uint64_t region = pages.track(copy.memory, copy.size);
    // A write the bits missed (it landed between a read and the clear).
    copy.memory[777 * kPage] = 3;
    bool sent = false;
    uint64_t most = 0;
    for (int flush = 0; flush < 8 && !sent; flush++) {
        uint64_t compared = 0;
        const auto ranges = dsa::changedShadow(pages, region, blocks, copy.memory, &compared);
        if (compared > most) most = compared;
        for (const Range& range : ranges) sent |= range.offset == 777 * kShadowBlockSize;
    }
    check(sent, test, "the missed write still reaches the host");
    check(most <= DirtyPages::kSweepPages * kPage, test, "no flush hashes more than the sweep");
}

void pr3_a_read_back_is_settled_by_hashing_only_what_it_wrote() {
    const char* test = __func__;
    FakeKernel kernel;
    DirtyPages pages(kernel);
    Copy copy(32 * kPage);
    ShadowBlocks blocks(copy.size);
    const uint64_t region = pages.track(copy.memory, copy.size);
    // The first map's read back writes two blocks of the copy, before the app has it.
    std::vector<uint8_t> host(copy.size, 0);
    host[4 * kPage] = 8;
    host[30 * kPage + 1] = 6;
    blocks.receive(copy.memory, 0, host.data(), host.size());
    kernel.write(copy.memory, 4 * kPage, 8);
    kernel.write(copy.memory, 30 * kPage + 1, 6);
    dsa::refreshShadow(pages, region, blocks, copy.memory);
    uint64_t compared = 0;
    check(dsa::changedShadow(pages, region, blocks, copy.memory, &compared).empty(), test,
          "what the host sent is not sent back");
    check(compared == 0, test, "and the next flush hashes nothing");
}

void pr3_an_apps_write_kept_through_a_read_back_still_goes() {
    const char* test = __func__;
    FakeKernel kernel;
    DirtyPages pages(kernel);
    Copy copy(4 * kPage);
    ShadowBlocks blocks(copy.size);
    const uint64_t region = pages.track(copy.memory, copy.size);
    kernel.write(copy.memory, 3, 4);  // the app wrote block 0, not flushed yet
    std::vector<uint8_t> host(copy.size, 0);
    host[2 * kPage + 5] = 9;  // the device wrote block 2
    blocks.receive(copy.memory, 0, host.data(), host.size());
    kernel.write(copy.memory, 2 * kPage + 5, 9);  // what receive copied in
    const auto ranges = dsa::changedShadow(pages, region, blocks, copy.memory);
    check(ranges.size() == 1 && ranges[0].offset == 0 && ranges[0].size == kShadowBlockSize, test,
          "only the app's write goes");
}

// The kernel the tests run on, when it lets this process use write protection.
void nfr2_the_kernel_reports_exactly_the_pages_written() {
    const char* test = __func__;
    std::string reason;
    if (dsa::LinuxDirtyKernel().openWriteProtect(&reason) != 0) {
        std::printf("%s: skipped, %s\n", test, reason.c_str());
        return;
    }
    dsa::LinuxDirtyKernel kernel;
    DirtyPages pages(kernel);
    check(pages.tracking() == DirtyTracking::kWriteProtect, test, "write protection is chosen");
    const uint64_t page = kernel.pageSize();
    const uint64_t length = 64 * page;
    void* at = mmap(nullptr, length, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    check(at != MAP_FAILED, test, "memory for a copy");
    if (at == MAP_FAILED) return;
    uint8_t* memory = static_cast<uint8_t*>(at);
    memory[0] = 1;  // written before tracking: counted clean once tracked
    const uint64_t region = pages.track(memory, length);
    check(region != 0, test, "the copy is tracked");
    std::vector<Range> written;
    check(pages.written(region, &written) && written.empty(), test, "nothing written yet");
    memory[3 * page + 100] = 1;
    memory[4 * page] = 1;
    memory[40 * page + page - 1] = 1;
    check(pages.written(region, &written), test, "the kernel knows");
    check(written.size() == 2 && written[0].offset == 3 * page && written[0].size == 2 * page &&
              written[1].offset == 40 * page && written[1].size == page,
          test, "exactly the pages written");
    check(pages.written(region, &written) && written.empty(), test, "and each only once");
    // Reading does not count as writing.
    volatile uint8_t sum = 0;
    for (uint64_t offset = 0; offset < length; offset += page) sum += memory[offset];
    (void)sum;
    check(pages.written(region, &written) && written.empty(), test, "a read is not a write");
    pages.untrack(region);
    munmap(at, length);
}

void nfr2_the_kernel_keeps_soft_dirty_bits_where_it_has_them() {
    const char* test = __func__;
    dsa::LinuxDirtyKernel kernel;
    std::string reason;
    if (kernel.openSoftDirty(&reason) != 0) {
        std::printf("%s: skipped, %s\n", test, reason.c_str());
        return;
    }
    const uint64_t page = kernel.pageSize();
    const uint64_t length = 8 * page;
    void* at = mmap(nullptr, length, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (at == MAP_FAILED) return;
    uint8_t* memory = static_cast<uint8_t*>(at);
    const uint64_t start = reinterpret_cast<uintptr_t>(memory);
    memory[0] = 1;
    check(kernel.clearSoftDirty() == 0, test, "the bits clear");
    memory[5 * page] = 1;
    uint64_t entries[8] = {};
    check(kernel.readPagemap(start, entries, 8) == 8, test, "pagemap reads");
    check(!(entries[0] & dsa::kPagemapSoftDirty), test, "a page untouched since the clear is clean");
    check(entries[5] & dsa::kPagemapSoftDirty, test, "a page written since is soft-dirty");
    munmap(at, length);
}

}  // namespace

int main() {
    nfr2_a_flush_with_nothing_written_hashes_and_sends_nothing();
    nfr2_written_pages_become_ranges_joined_where_they_meet();
    nfr2_only_written_blocks_whose_bytes_changed_are_sent();
    nfr2_a_full_list_of_runs_continues_where_the_scan_stopped();
    nfr2_write_protection_comes_first_then_soft_dirty_bits_then_hashing();
    nfr2_without_tracking_every_block_is_compared_and_the_reason_is_logged_once();
    nfr2_a_forked_child_compares_every_block_and_never_asks_the_parents_kernel_state();
    nfr2_a_failed_scan_leaves_that_copy_to_hashing();
    nfr2_an_untracked_copy_is_unprotected_and_forgotten();
    nfr2_soft_dirty_bits_are_read_and_cleared_once_for_a_flush_of_several_copies();
    nfr2_soft_dirty_bits_of_a_copy_another_flush_cleared_are_kept_for_it();
    nfr2_the_soft_dirty_sweep_compares_a_bounded_slice_and_reaches_every_page();
    pr3_a_read_back_is_settled_by_hashing_only_what_it_wrote();
    pr3_an_apps_write_kept_through_a_read_back_still_goes();
    nfr2_the_kernel_reports_exactly_the_pages_written();
    nfr2_the_kernel_keeps_soft_dirty_bits_where_it_has_them();
    if (failures != 0) {
        std::fprintf(stderr, "%d failed\n", failures);
        return EXIT_FAILURE;
    }
    std::puts("dirty pages: all passed");
    return EXIT_SUCCESS;
}
