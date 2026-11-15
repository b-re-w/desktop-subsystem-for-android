"""Tests for host patch 0040: the compositor composes YUV ColorBuffers' layers.

The hwcomposer handed every YUV layer (a video's pictures) back to SurfaceFlinger because the
compositor sampled RGB images only, and SurfaceFlinger kept two display-sized client target
buffers for it. Patch 0040 reads a YUV layer through a sampler YCbCr conversion made from the
matrix, range and chroma order the guest puts in the layer's colour, and lists an extension so
the guest hands it such layers only when it does."""

import re
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
PATCH = next((HERE / "patches").glob("0040-*.patch"))
HEADER = (HERE / "include/dsa_shared_color_buffer.h").read_text(encoding="utf-8")
POLICY = (HERE.parent / "guest/hwcomposer/src/cpu_upload_policy.cpp").read_text(encoding="utf-8")


class YuvCompositionPatchTests(unittest.TestCase):
    def setUp(self):
        self.text = PATCH.read_text(encoding="utf-8")
        self.added = "\n".join(l[1:] for l in self.text.splitlines()
                               if l.startswith("+") and not l.startswith("+++"))

    def test_pr3_only_the_compositor_and_its_extension_change(self):
        files = re.findall(r"^diff --git a/(\S+) ", self.text, flags=re.M)
        self.assertEqual(sorted(files), ["host/FrameBuffer.cpp", "host/FrameBuffer.h",
                                         "host/RenderControl.cpp", "host/vulkan/CompositorVk.cpp",
                                         "host/vulkan/CompositorVk.h"])

    def test_pr3_host_and_guest_name_the_same_extension(self):
        name = re.search(r'kDsaYuvComposition = "([^"]+)"', self.added).group(1)
        self.assertIn(f'#define DSA_YUV_COMPOSITION_EXTENSION "{name}"', HEADER)
        self.assertIn("return m_useVulkanComposition && supportsDsaYuvColorBuffers() &&\n"
                      "           m_emulationVk->supportsSamplerYcbcrConversion();", self.added)

    def test_pr3_the_host_reads_the_colour_the_guest_writes(self):
        # Marker 'Y' in alpha; red 1, 2, 3 is BT.601, BT.709, BT.2020; green 1 full, 2 limited.
        self.assertIn("kDsaYuvColorMarker = 0x59;", self.added)
        self.assertIn("kYuvColorMarker = 0x59;",
                      (HERE.parent / "guest/hwcomposer/src/cpu_upload_policy.h")
                      .read_text(encoding="utf-8"))
        self.assertIn("color.r == 1   ? VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601\n"
                      "                : color.r == 2 ? VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709\n"
                      "                               : VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020;",
                      self.added)
        self.assertIn("range = color.g == 1 ? VK_SAMPLER_YCBCR_RANGE_ITU_FULL : "
                      "VK_SAMPLER_YCBCR_RANGE_ITU_NARROW;", self.added)
        self.assertIn("Bt601 = 1, Bt709 = 2, Bt2020 = 3", (HERE.parent /
                      "guest/hwcomposer/src/cpu_upload_policy.h").read_text(encoding="utf-8"))
        self.assertIn("Full = 1, Limited = 2", (HERE.parent /
                      "guest/hwcomposer/src/cpu_upload_policy.h").read_text(encoding="utf-8"))
        self.assertIn("out[3] = kYuvColorMarker;", POLICY)

    def test_pr3_v_then_u_chroma_swaps_red_and_blue_in_the_conversion(self):
        self.assertIn(".components = {swap ? VK_COMPONENT_SWIZZLE_B : VK_COMPONENT_SWIZZLE_IDENTITY,",
                      self.added)
        self.assertIn("swap ? VK_COMPONENT_SWIZZLE_R : VK_COMPONENT_SWIZZLE_IDENTITY,", self.added)

    def test_pr3_the_view_and_sampler_share_one_conversion(self):
        self.assertIn(".conversion = sampling->conversion,", self.added)
        self.assertIn(".pImmutableSamplers = &sampler,", self.added)
        self.assertIn(".addressModeU = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,", self.added)

    def test_pr3_a_yuv_layers_views_and_sets_are_freed_with_its_frame(self):
        self.assertIn("releaseYuvFrameResources(frameResources);", self.added)
        self.assertIn("m_vk.vkDestroyImageView(m_vkDevice, view, nullptr);", self.added)
        self.assertIn("VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT", self.added)

    def test_pr3_every_context_line_starts_with_a_space(self):
        in_hunk = False
        for line in self.text.splitlines():
            if line.startswith("@@"):
                in_hunk = True
            elif line.startswith("diff --git"):
                in_hunk = False
            elif in_hunk:
                self.assertTrue(line[:1] in (" ", "+", "-", "\\"), repr(line))

    def test_pr3_the_fetch_applies_it_after_0039(self):
        self.assertIn(" 0039 0040", (HERE / "host/fetch.sh").read_text(encoding="utf-8"))
        self.assertIn('-Filter "0040-*.patch"',
                      (HERE / "host/fetch.ps1").read_text(encoding="utf-8"))


if __name__ == "__main__":
    unittest.main()
