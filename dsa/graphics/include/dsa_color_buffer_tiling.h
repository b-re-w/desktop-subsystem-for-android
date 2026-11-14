// Copyright 2026 The DSA Authors
// SPDX-License-Identifier: Apache-2.0
//
// Which GPUs get linear RGB ColorBuffers, so that a clear an app makes reaches every reader.
//
// An app draws into its window buffer through an image on its own VkDevice, bound to the
// ColorBuffer's memory; the host's task window copy and readback (screencap) read the same
// memory through the ColorBuffer's image on the renderer's VkDevice, and SurfaceFlinger
// samples it through a third. Every image is optimal, made with the same parameters, which
// the Vulkan specification says is enough for them to see the same contents.
//
// On Intel UHD Graphics of the Ice Lake generation (device 0x8A56, driver 31.0.101.2134,
// Vulkan "Intel-0.404.2134", the last driver Intel ships for it) it was not. Every app window
// showed its new frame's text and icons over parts of earlier frames: the backgrounds an app
// clears before drawing never showed, while what it drew over them did, in blocks around
// the drawing. `adb screencap` showed the same, so the memory itself held the old frames. That
// is a fast clear: the driver marks the cleared blocks in the image's compression state, held
// per image (and per device), instead of writing the clear color into memory, and a reader on
// another device takes the memory as it lies. The same build on Intel Arc graphics (driver
// 101.9150) and on an NVIDIA RTX 4050 does not show it.
//
// A linear image has no compression state, so every clear and every draw goes to memory. RGB
// ColorBuffers are linear on the generations whose driver keeps clears per device: Intel's
// Gen9 to Gen11 GPUs (Skylake to Ice Lake, Jasper Lake and Elkhart Lake), which only that
// legacy driver supports. Every other GPU keeps optimal ColorBuffers, which draw faster.

#pragma once

#include <cstdint>

namespace dsa {

inline constexpr uint32_t kIntelVendorId = 0x8086;

// Whether `deviceId` is an Intel GPU of Gen9 to Gen11, by the PCI device IDs Intel gave them.
// Gen12 and later (Tiger Lake, Rocket Lake, Alder Lake, DG1, Arc, Meteor Lake and on) are not.
inline bool isIntelGen9To11(uint32_t deviceId) {
    const uint32_t high = deviceId >> 8;
    switch (high) {
        case 0x19:  // Skylake
        case 0x59:  // Kaby Lake
        case 0x3E:  // Coffee Lake, Whiskey Lake
        case 0x9B:  // Comet Lake
        case 0x8A:  // Ice Lake
        case 0x4E:  // Jasper Lake
        case 0x45:  // Elkhart Lake
            return true;
        default:
            break;
    }
    switch (deviceId) {
        case 0x0A84:  // Broxton, Apollo Lake
        case 0x1A84:
        case 0x1A85:
        case 0x5A84:
        case 0x5A85:
        case 0x3184:  // Gemini Lake
        case 0x3185:
        case 0x87C0:  // Amber Lake (Kaby Lake)
        case 0x87CA:  // Amber Lake (Coffee Lake)
            return true;
        default:
            return false;
    }
}

// Whether an RGB ColorBuffer on this GPU is made linear (when the device can draw into,
// sample, copy and export it that way), because its driver keeps an optimal image's fast
// clears where an image on another device does not see them.
inline bool rgbColorBuffersAreLinear(uint32_t vendorId, uint32_t deviceId) {
    return vendorId == kIntelVendorId && isIntelGen9To11(deviceId);
}

struct ImageSize {
    uint32_t width = 0;
    uint32_t height = 0;
};

// The size of an image the guest makes on a ColorBuffer's memory (a window buffer's image,
// whose size the guest gives and may be smaller than the ColorBuffer's). An optimal image keeps
// the guest's size, as it always has. A linear one takes the ColorBuffer's: the driver lays a
// linear image's rows out at a row pitch that follows its width (the row's bytes rounded up
// to the driver's alignment, 64 to 512 bytes), so a narrower image on the same memory would
// step from row to row by a shorter pitch than the ColorBuffer's image wrote them with, and
// read every row after the first from the wrong place.
inline ImageSize imageSizeOnColorBuffer(bool linear, ImageSize guest, ImageSize colorBuffer) {
    return linear ? colorBuffer : guest;
}

}  // namespace dsa
