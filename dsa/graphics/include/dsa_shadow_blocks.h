// Copyright 2026 The DSA Authors
// SPDX-License-Identifier: Apache-2.0
//
// Which parts of a guest's copy of host-visible memory changed since the host last had them.
//
// A Hyper-V VM has no memory the guest and the host share, so the guest driver keeps a copy of
// host-visible memory and the stream carries it to the host before each submit. An app's
// allocations add up to tens of megabytes, and a frame changes a few kilobytes of them.
// Sending all of it for every submit makes every frame wait for tens of megabytes to cross a
// Hyper-V socket, so only the blocks that changed are sent.
//
// Each block's hash is kept, 8 bytes per 4 KiB block rather than a second copy of the memory,
// and a block goes only when its hash changed, so a block written with what it held already
// is not sent. Which blocks are compared at a flush depends on what the kernel tracks
// (dsa_dirty_pages.h): only the blocks it saw written (changedIn), or every block (changed)
// where it tracks nothing. Comparing every block reads all of the app's mapped memory at
// every submit, which for a game that maps hundreds of megabytes is most of its render time.
//
// A new allocation counts as all zero, which is what the driver's copy starts as, so blocks
// the app never wrote are never sent. The host's memory is undefined there until written.

#pragma once

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <vector>

namespace dsa {

inline constexpr uint64_t kShadowBlockSize = 4096;

// One block's hash. A change confined to one 8-byte word always changes it: each step is a
// bijection of the lane and of the word.
inline uint64_t shadowBlockHash(const uint8_t* data, size_t size) {
    constexpr uint64_t kPrime = 0x9E3779B97F4A7C15ULL;
    uint64_t lanes[4] = {0x243F6A8885A308D3ULL, 0x13198A2E03707344ULL, 0xA4093822299F31D0ULL,
                         0x082EFA98EC4E6C89ULL};
    size_t at = 0;
    uint64_t words[4];
    for (; size - at >= sizeof(words); at += sizeof(words)) {
        std::memcpy(words, data + at, sizeof(words));
        for (int lane = 0; lane < 4; lane++) {
            lanes[lane] = (lanes[lane] ^ words[lane]) * kPrime;
        }
    }
    // Up to 31 bytes are left: whole words go into the lanes one by one, and the last few
    // bytes into a word of their own. The tail word holds 8 bytes, so it never takes more.
    for (int lane = 0; size - at >= sizeof(uint64_t); at += sizeof(uint64_t), lane++) {
        uint64_t word;
        std::memcpy(&word, data + at, sizeof(word));
        lanes[lane] = (lanes[lane] ^ word) * kPrime;
    }
    uint64_t tail = 0;
    if (at != size) {
        std::memcpy(&tail, data + at, size - at);
    }
    uint64_t hash = (lanes[0] ^ tail) * kPrime;
    for (int lane = 1; lane < 4; lane++) {
        hash = (hash ^ lanes[lane]) * kPrime;
    }
    return hash ^ (hash >> 29);
}

class ShadowBlocks {
   public:
    // A byte range of the memory.
    struct Range {
        uint64_t offset;
        uint64_t size;
    };

    explicit ShadowBlocks(uint64_t size)
        : size_(size), hashes_(blockCount(size), zeroHash(kShadowBlockSize)) {
        if (size % kShadowBlockSize != 0) {
            hashes_.back() = zeroHash(size % kShadowBlockSize);
        }
    }

    // The ranges of `memory` that changed since the last call, each as large as it can be, in
    // order. Each comes back once: the caller is taken to send them.
    std::vector<Range> changed(const uint8_t* memory) {
        std::vector<Range> ranges;
        for (size_t block = 0; block < hashes_.size(); block++) {
            compare(memory, block, &ranges);
        }
        return ranges;
    }

    // As changed(), when the kernel says which bytes of `memory` were written since the last
    // call (`written`, in order, as offsets from its start): only the blocks they touch are
    // compared, since every other block holds what it held at the last call. Nothing written
    // compares and sends nothing. `compared`, when given, grows by the bytes hashed.
    std::vector<Range> changedIn(const uint8_t* memory, const std::vector<Range>& written,
                                 uint64_t* compared = nullptr) {
        std::vector<Range> ranges;
        forEachBlock(written, [&](size_t block) {
            const uint64_t length = compare(memory, block, &ranges);
            if (compared) *compared += length;
        });
        return ranges;
    }

    // The host sent `size` bytes (`data`) of a read back for `offset` of `memory`. Each block
    // the device changed since the host last had it (its bytes differ from what this copy
    // last exchanged) takes the host's bytes. Each other block keeps this copy's, so what the
    // app wrote there since the last flush is not lost and still goes at the next one. A block
    // the range covers only in part takes the host's bytes and goes at the next flush whole.
    // Returns how many bytes were taken.
    uint64_t receive(uint8_t* memory, uint64_t offset, const uint8_t* data, uint64_t size) {
        if (offset >= size_ || size == 0) {
            return 0;
        }
        const uint64_t end = size > size_ - offset ? size_ : offset + size;
        uint64_t taken = 0;
        for (uint64_t at = offset; at < end;) {
            const size_t block = static_cast<size_t>(at / kShadowBlockSize);
            const uint64_t start = block * kShadowBlockSize;
            const uint64_t length = blockSize(start);
            const uint64_t stop = start + length < end ? start + length : end;
            if (start == at && stop == start + length) {
                const uint64_t hash = shadowBlockHash(data + (at - offset), length);
                if (hash != hashes_[block]) {
                    std::memcpy(memory + at, data + (at - offset), length);
                    hashes_[block] = hash;
                    taken += length;
                }
            } else {
                std::memcpy(memory + at, data + (at - offset), stop - at);
                taken += stop - at;
                // Not the hash of any contents: the block differs from the host's at the
                // next flush and is sent whole.
                hashes_[block] = ~shadowBlockHash(memory + start, length);
            }
            at = stop;
        }
        return taken;
    }

    // The host wrote `memory` (a read back), so it and the host agree on all of it now.
    void refresh(const uint8_t* memory) {
        for (size_t block = 0; block < hashes_.size(); block++) {
            const uint64_t offset = block * kShadowBlockSize;
            hashes_[block] = shadowBlockHash(memory + offset, blockSize(offset));
        }
    }

    // As refresh(), for the bytes the kernel saw written since the last call alone: every
    // other block holds what its hash says already.
    void refreshIn(const uint8_t* memory, const std::vector<Range>& written) {
        forEachBlock(written, [&](size_t block) {
            const uint64_t offset = block * kShadowBlockSize;
            hashes_[block] = shadowBlockHash(memory + offset, blockSize(offset));
        });
    }

    uint64_t size() const { return size_; }

   private:
    // Compares one block with its hash, takes the new hash, and adds the block to `ranges`
    // when it changed, joined to the range before it where they meet. Returns its length.
    uint64_t compare(const uint8_t* memory, size_t block, std::vector<Range>* ranges) {
        const uint64_t offset = block * kShadowBlockSize;
        const uint64_t length = blockSize(offset);
        const uint64_t hash = shadowBlockHash(memory + offset, length);
        if (hash == hashes_[block]) {
            return length;
        }
        hashes_[block] = hash;
        if (!ranges->empty() && ranges->back().offset + ranges->back().size == offset) {
            ranges->back().size += length;
        } else {
            ranges->push_back(Range{offset, length});
        }
        return length;
    }

    // Calls `visit` once for each block the ranges touch, in order, within the memory. Ranges
    // that share a block (two runs of pages smaller than a block) visit it once.
    template <typename Visit>
    void forEachBlock(const std::vector<Range>& written, Visit visit) {
        size_t next = 0;
        for (const Range& range : written) {
            if (range.size == 0 || range.offset >= size_) continue;
            const uint64_t end =
                range.size > size_ - range.offset ? size_ : range.offset + range.size;
            size_t block = static_cast<size_t>(range.offset / kShadowBlockSize);
            if (block < next) block = next;
            for (; block < hashes_.size() && block * kShadowBlockSize < end; block++) {
                visit(block);
            }
            if (block > next) next = block;
        }
    }

    static size_t blockCount(uint64_t size) {
        return static_cast<size_t>((size + kShadowBlockSize - 1) / kShadowBlockSize);
    }

    static uint64_t zeroHash(uint64_t size) {
        const std::vector<uint8_t> zeros(static_cast<size_t>(size), 0);
        return shadowBlockHash(zeros.data(), zeros.size());
    }

    uint64_t blockSize(uint64_t offset) const {
        return size_ - offset < kShadowBlockSize ? size_ - offset : kShadowBlockSize;
    }

    uint64_t size_;
    std::vector<uint64_t> hashes_;
};

}  // namespace dsa
