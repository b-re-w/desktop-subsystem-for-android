// Copyright 2026 The DSA Authors
// SPDX-License-Identifier: Apache-2.0
//
// Which GPUs get linear RGB ColorBuffers: Intel's Gen9 to Gen11, whose last driver keeps an
// app's fast clears where the renderer and SurfaceFlinger do not see them, and no other. And
// the size a window buffer's image takes on a linear ColorBuffer, checked against drivers whose
// linear rows are aligned to 64, 256 and 512 bytes.

#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <initializer_list>

#include "dsa_color_buffer_tiling.h"

namespace {

int failures = 0;

void check(bool ok, const char* test, const char* what) {
    if (!ok) {
        std::fprintf(stderr, "%s: %s\n", test, what);
        failures++;
    }
}

void pr3_ice_lake_uhd_graphics_gets_linear_color_buffers() {
    const char* test = __func__;
    // The GPU of the report: Intel UHD Graphics, Core i3-1005G1.
    check(dsa::rgbColorBuffersAreLinear(0x8086, 0x8A56), test, "0x8A56");
    check(dsa::rgbColorBuffersAreLinear(0x8086, 0x8A52), test, "Iris Plus G7");
    check(dsa::rgbColorBuffersAreLinear(0x8086, 0x8A5A), test, "Iris Plus G4");
}

void pr3_every_gen9_to_gen11_intel_gpu_gets_linear_color_buffers() {
    const char* test = __func__;
    const uint32_t ids[] = {
        0x1912, 0x1916, 0x193B,          // Skylake
        0x5916, 0x5917, 0x591B,          // Kaby Lake
        0x87C0, 0x87CA,                  // Amber Lake
        0x3E92, 0x3E9B, 0x3EA0,          // Coffee Lake, Whiskey Lake
        0x9BC4, 0x9B41, 0x9BA8,          // Comet Lake
        0x5A85, 0x0A84, 0x3185,          // Apollo Lake, Broxton, Gemini Lake
        0x4E55, 0x4E61, 0x4571, 0x4541,  // Jasper Lake, Elkhart Lake
    };
    for (uint32_t id : ids) {
        if (!dsa::rgbColorBuffersAreLinear(0x8086, id)) {
            std::fprintf(stderr, "%s: 0x%04X is Gen9 to Gen11\n", test, id);
            failures++;
        }
    }
}

void pr3_newer_intel_and_other_vendors_keep_optimal_color_buffers() {
    const char* test = __func__;
    const uint32_t ids[] = {
        0x9A49, 0x9A78,  // Tiger Lake
        0x4C8A,          // Rocket Lake
        0x4680, 0x46A6,  // Alder Lake
        0xA7A0,          // Raptor Lake
        0x4905,          // DG1
        0x5690, 0x56A0,  // Arc A-series
        0x7D55, 0x7DD5,  // Meteor Lake (Arc Graphics)
        0x64A0,          // Lunar Lake
        0xE20B,          // Battlemage
    };
    for (uint32_t id : ids) {
        if (dsa::rgbColorBuffersAreLinear(0x8086, id)) {
            std::fprintf(stderr, "%s: 0x%04X is Gen12 or later\n", test, id);
            failures++;
        }
    }
    // The same device IDs from another vendor say nothing about the GPU.
    check(!dsa::rgbColorBuffersAreLinear(0x10DE, 0x8A56), test, "NVIDIA");
    check(!dsa::rgbColorBuffersAreLinear(0x1002, 0x1912), test, "AMD");
    check(!dsa::rgbColorBuffersAreLinear(0x10DE, 0x28E1), test, "RTX 4050");
}

// A driver's row pitch for a linear image: the row's bytes rounded up to its alignment.
uint64_t rowPitch(uint32_t width, uint32_t alignment) {
    const uint64_t row = uint64_t(width) * 4;
    return (row + alignment - 1) / alignment * alignment;
}

// Writes a `cb` sized RGBA frame into linear memory as the ColorBuffer's image lays it out,
// then reads it back through an image of `reader` size on the same memory, as the driver lays
// that one out. True when every pixel of the guest's `frame` reads back as written.
bool readsBack(dsa::ImageSize cb, dsa::ImageSize reader, dsa::ImageSize frame,
               uint32_t alignment) {
    const uint64_t writePitch = rowPitch(cb.width, alignment);
    const uint64_t readPitch = rowPitch(reader.width, alignment);
    for (uint32_t y = 0; y < frame.height; ++y) {
        for (uint32_t x = 0; x < frame.width; ++x) {
            const uint64_t written = y * writePitch + uint64_t(x) * 4;
            const uint64_t read = y * readPitch + uint64_t(x) * 4;
            if (written != read) {
                return false;
            }
        }
    }
    return true;
}

void pr3_a_linear_window_image_reads_rows_at_the_color_buffers_pitch() {
    const char* test = __func__;
    // The report's window: a 422x916 frame in a ColorBuffer whose width the guest rounded up,
    // on drivers whose linear rows are aligned to 64, 256 or 512 bytes. A 422-pixel row is
    // 1688 bytes; a 512-pixel one 2048.
    const dsa::ImageSize frame{422, 916};
    const dsa::ImageSize cb{512, 1024};
    for (uint32_t alignment : {64u, 256u, 512u}) {
        const dsa::ImageSize linear = dsa::imageSizeOnColorBuffer(true, frame, cb);
        check(linear.width == cb.width && linear.height == cb.height, test,
              "a linear image takes the ColorBuffer's size");
        check(readsBack(cb, linear, frame, alignment), test,
              "every row read back where the ColorBuffer wrote it");
        // The guest's own size steps by a shorter pitch wherever the alignment does not round
        // both widths to the same one (64 and 256 bytes here, not 512): every row after the
        // first would read from the wrong place.
        const bool samePitch = rowPitch(frame.width, alignment) == rowPitch(cb.width, alignment);
        check(readsBack(cb, frame, frame, alignment) == samePitch, test,
              "the guest's narrower size reads the wrong rows when its pitch differs");
    }
    check(!readsBack(cb, frame, frame, 256), test, "256-byte rows: 1792 against 2048");
    check(rowPitch(422, 256) == 1792 && rowPitch(422, 512) == 2048 && rowPitch(512, 256) == 2048,
          test, "pitches of the report's sizes");
    // An optimal image keeps the guest's size, as it always has.
    const dsa::ImageSize optimal = dsa::imageSizeOnColorBuffer(false, frame, cb);
    check(optimal.width == frame.width && optimal.height == frame.height, test,
          "an optimal image keeps the guest's size");
}

}  // namespace

int main() {
    pr3_ice_lake_uhd_graphics_gets_linear_color_buffers();
    pr3_every_gen9_to_gen11_intel_gpu_gets_linear_color_buffers();
    pr3_newer_intel_and_other_vendors_keep_optimal_color_buffers();
    pr3_a_linear_window_image_reads_rows_at_the_color_buffers_pitch();
    if (failures) {
        return EXIT_FAILURE;
    }
    std::puts("color_buffer_tiling_test: ok");
    return EXIT_SUCCESS;
}
