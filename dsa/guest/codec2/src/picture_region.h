// Copyright 2026 The DSA Authors
// SPDX-License-Identifier: Apache-2.0
//
// The memory the host shares with the guest for decoded pictures, mapped read only.
//
// The host's decoder writes each picture into a slot of a region of host memory the VM sees
// as physical memory of its own, and the video port carries only the slot's number and where
// it is (VideoFrame.slot). DSA's kernel maps the region for this HAL alone through
// /dev/dsa_video_pictures, once the guest's control daemon has told it where the region is.
// The HAL maps all of it once, reads each picture from there into the app's buffer (a single
// copy: the app's buffers come from its surface, which allocates them in the guest's own
// memory, so a slot cannot be handed to the app as its buffer), and gives the slot back.
//
// The region starts with a header the host writes: the eight bytes "DSAVPIC1", then the
// region's id and its length, each eight bytes little endian. The HAL names the id when it
// opens a decoder; the host sends that stream's pictures in slots only when it is the id of
// the region it writes, so both are known to see the same memory.

#pragma once

#include <cstddef>
#include <cstdint>
#include <mutex>

namespace dsa::codec2 {

// The device DSA's kernel maps the region through.
constexpr const char* kPictureRegionDevice = "/dev/dsa_video_pictures";

// The id in the header at `base`, or 0 when the `size` bytes there hold no header, or one
// that says the region is longer than they are.
uint64_t pictureRegionId(const uint8_t* base, size_t size);

// The `length` bytes `offset` bytes into the `size` bytes at `base`, or nullptr when they do
// not lie wholly inside them, or inside the header.
const uint8_t* pictureRegionSlice(const uint8_t* base, size_t size, uint64_t offset, size_t length);

// The process's mapping of the region.
class PictureRegion {
   public:
    // The one mapping this process has.
    static PictureRegion& instance();

    // The region's id, mapping it first if it is not yet: 0 while there is none (an image
    // without the driver, or a guest not told where the region is yet), and the reason is
    // logged once. A failed map is tried again next time, since the region may be told later.
    uint64_t id();

    // The `length` bytes at `offset`, or nullptr when they are not inside the region.
    const uint8_t* slice(uint64_t offset, size_t length);

    // Takes `size` bytes at `base` as the region instead of the device, or none for nullptr:
    // what the tests run with.
    void useForTest(const uint8_t* base, size_t size);

   private:
    PictureRegion() = default;
    void mapLocked();

    std::mutex mLock;
    const uint8_t* mBase = nullptr;
    size_t mSize = 0;
    bool mTest = false;
    bool mLogged = false;
};

}  // namespace dsa::codec2
