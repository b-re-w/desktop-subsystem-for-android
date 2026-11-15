"""Tests for making an RGB ColorBuffer linear where a driver keeps an app's fast clears per device.

On an Intel Core i3-1005G1 (UHD Graphics 0x8A56, driver 31.0.101.2134, the last for Ice Lake)
every app window showed its new frame's text and icons over parts of earlier frames, and
`adb screencap` showed the same. Rows were never sheared, so no row pitch was wrong: whole blocks
kept an earlier frame where the app had cleared. The app clears through an image on its own
VkDevice, the driver keeps the clear in that image's compression state, and the renderer and
SurfaceFlinger read the memory through images on other devices.

Patch 0035 makes RGB ColorBuffers linear on Intel's Gen9 to Gen11 GPUs (no compression state, so
every clear is written), and makes every image the guest binds to a ColorBuffer's memory with the
ColorBuffer's tiling, and a linear window buffer image at the ColorBuffer's size, since a linear
image's row pitch follows its width."""

import re
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
PATCH = next((HERE / "patches").glob("0035-*.patch"))
NATIVE = "host/vulkan/VkAndroidNativeBuffer.cpp"
OPERATIONS = "host/vulkan/VkCommonOperations.cpp"
HEADER = "host/vulkan/VkCommonOperations.h"
DECODER = "host/vulkan/VkDecoderGlobalState.cpp"
TILING = HERE / "include" / "dsa_color_buffer_tiling.h"


def sides(patch, name):
    """The added and removed lines of one file."""
    text = patch.read_text(encoding="utf-8")
    for part in re.split(r"^diff --git ", text, flags=re.M):
        if part.startswith(f"a/{name} "):
            lines = part.splitlines()
            added = [l[1:] for l in lines if l.startswith("+") and not l.startswith("+++")]
            removed = [l[1:] for l in lines if l.startswith("-") and not l.startswith("---")]
            return "\n".join(added), "\n".join(removed)
    raise AssertionError(f"{patch.name} does not change {name}")


class RgbLinearColorBufferPatchTests(unittest.TestCase):
    def setUp(self):
        self.native, self.native_removed = sides(PATCH, NATIVE)
        self.operations, self.operations_removed = sides(PATCH, OPERATIONS)
        self.header, self.header_removed = sides(PATCH, HEADER)
        self.decoder, self.decoder_removed = sides(PATCH, DECODER)
        start = self.operations.index("VkImageTiling VkEmulation::rgbColorBufferTiling(")
        self.rgb = self.operations[start:self.operations.index(
            "VkImageTiling VkEmulation::colorBufferTiling(", start)]

    def test_pr3_only_the_color_buffer_and_the_images_on_it_change(self):
        text = PATCH.read_text(encoding="utf-8")
        files = re.findall(r"^diff --git a/(\S+) ", text, flags=re.M)
        self.assertEqual(files, [NATIVE, OPERATIONS, HEADER, DECODER])
        self.assertIn("VkImageTiling colorBufferTiling(VkFormat format, uint32_t width, "
                      "uint32_t height) const;", self.header)
        self.assertIn("VkImageTiling colorBufferTiling(VkFormat format) const;",
                      self.header_removed)

    def test_pr3_only_gpus_whose_driver_keeps_clears_per_device_get_linear_rgb(self):
        self.assertIn('#include "dsa_color_buffer_tiling.h"', self.operations)
        self.assertIn("if (!dsa::rgbColorBuffersAreLinear(props.vendorID, props.deviceID) ||",
                      self.rgb)
        # YUV keeps 0032's rule; depth and the other YUV layouts stay optimal.
        self.assertIn("return rgbColorBufferTiling(format, width, height);", self.operations)
        self.assertIn("        return VK_IMAGE_TILING_OPTIMAL;", self.operations_removed)
        for kept in ("gfxstream::vk::formatIsDepthOrStencil(format)",
                     "VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16"):
            self.assertIn(kept, self.rgb)
        header = TILING.read_text(encoding="utf-8")
        self.assertIn("return vendorId == kIntelVendorId && isIntelGen9To11(deviceId);", header)

    def test_nfr5_linear_only_where_every_use_of_an_optimal_one_works_linear(self):
        # A ColorBuffer or a guest image the device cannot make would fail the app's window,
        # so a linear one must draw, sample, copy and blit as the optimal one does, at its
        # size, with exported memory.
        for feature in ("VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT",
                        "VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT",
                        "VK_FORMAT_FEATURE_TRANSFER_SRC_BIT",
                        "VK_FORMAT_FEATURE_TRANSFER_DST_BIT",
                        "VK_FORMAT_FEATURE_BLIT_SRC_BIT"):
            self.assertIn(feature, self.rgb)
        self.assertIn("VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT",
                      self.rgb)
        self.assertIn("maxExtent.width < width || maxExtent.height < height", self.rgb)
        self.assertIn("(mDeviceInfo.supportsExternalMemoryExport && "
                      "!supportInfo.supportsExternalMemory)", self.rgb)
        self.assertIn("const bool linear = known && (linearFeatures & optimalFeatures) == "
                      "optimalFeatures &&", self.rgb)
        self.assertIn("(linearUsage & neededUsage) == neededUsage;", self.rgb)

    def test_pr3_the_renderer_says_once_per_format_which_tiling_it_chose(self):
        self.assertIn("sLogged.insert(static_cast<int>(format)).second", self.rgb)
        self.assertIn("are linear on this GPU", self.rgb)
        self.assertIn("stay optimal on this GPU", self.rgb)

    def test_pr3_the_color_buffer_is_made_with_its_size(self):
        self.assertIn(": colorBufferTiling(vkFormat, infoPtr->width, infoPtr->height);",
                      self.operations)
        self.assertIn(": colorBufferTiling(vkFormat);", self.operations_removed)

    def test_pr3_an_hardware_buffer_image_asks_with_the_same_size(self):
        # The AHardwareBuffer's image decides with the buffer's size, which is the ColorBuffer's,
        # so both come out the same way.
        self.assertIn("imageCreateInfo.tiling = m_vkEmulation->colorBufferTiling(\n"
                      "                        resolvedFormat, imageCreateInfo.extent.width,\n"
                      "                        imageCreateInfo.extent.height);", self.decoder)

    def test_pr3_a_window_buffer_image_takes_the_color_buffers_tiling(self):
        # Upstream left a VkNativeBufferANDROID image at the guest's optimal tiling even on a
        # linear ColorBuffer; both the decoder's check and the image itself now follow it.
        self.assertIn("imageCreateInfo.tiling = colorBufferVkImageCi->tiling;", self.decoder)
        text = PATCH.read_text(encoding="utf-8")
        self.assertLess(text.index("imageCreateInfo.tiling = colorBufferVkImageCi->tiling;"),
                        text.index("imageCreateInfo.format = resolvedFormat;"))
        self.assertIn("createImageCi.tiling = colorBufferCi.tiling;", self.native)
        self.assertEqual(self.native_removed, "")

    def test_pr3_a_linear_window_buffer_image_reads_rows_at_the_color_buffers_pitch(self):
        # A linear image's row pitch follows its width, so a window buffer image narrower than
        # its ColorBuffer would read every row after the first from the wrong place.
        self.assertIn('#include "dsa_color_buffer_tiling.h"', self.native)
        self.assertIn("dsa::imageSizeOnColorBuffer(\n"
                      "            createImageCi.tiling == VK_IMAGE_TILING_LINEAR,", self.native)
        self.assertIn("createImageCi.extent.width = size.width;", self.native)
        self.assertIn("createImageCi.extent.height = size.height;", self.native)
        header = TILING.read_text(encoding="utf-8")
        self.assertIn("return linear ? colorBuffer : guest;", header)
        tests = (HERE / "tests" / "test.sh").read_text(encoding="utf-8")
        self.assertIn("color_buffer_tiling_test.cpp", tests)

    def test_pr3_the_fetch_applies_it_after_the_patch_it_extends(self):
        names = sorted(p.name for p in (HERE / "patches").glob("00*.patch"))
        self.assertEqual(names.index(PATCH.name),
                         names.index(next(n for n in names if n.startswith("0034-"))) + 1)
        for script in (HERE / "host/fetch.ps1", HERE / "host/fetch.sh"):
            self.assertIn("0035", script.read_text(encoding="utf-8"))


if __name__ == "__main__":
    unittest.main()
