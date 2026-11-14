"""Tests for offering ASTC LDR textures to every app through the GPU decoder.

Games (Unity's among them) ship ASTC textures. Many desktop GPUs have no ASTC (NVIDIA and AMD
desktop GPUs, Intel Arc), and gfxstream's renderer then decodes ASTC on the GPU with a compute
shader: FrameBuffer sets astcLdrEmulationMode to AstcEmulationMode::Gpu, and DSA builds without
the CPU decoder (ASTC_CPU_DECODING OFF), which stays off by the owner's decision of 2026-11-15.

Upstream left ANGLE's Vulkan instances out of that ("let it do its own emulation"), but ANGLE
has no ASTC decoder: it offers GL_KHR_texture_compression_astc_ldr only for the ASTC formats the
Vulkan device samples. Every GLES app, which in the guest always runs on ANGLE, went without
ASTC on such a GPU. Patch 0035 offers it to ANGLE too, offers it only where the GPU can run the
decoder (no CPU fallback), keeps the guest's format lists and ASTC decode modes off the images
that stand in for the compressed ones, and logs once how ASTC reaches apps."""

import re
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
PATCHES = HERE / "patches"
PATCH = next(PATCHES.glob("0035-*.patch"))
OPERATIONS = "host/vulkan/VkCommonOperations.cpp"
HEADER = "host/vulkan/VkCommonOperations.h"
DECODER = "host/vulkan/VkDecoderGlobalState.cpp"


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


def changed_lines(patch):
    """Every added or removed line of a patch."""
    return [l[1:] for l in patch.read_text(encoding="utf-8").splitlines()
            if l[:1] in "+-" and not l.startswith(("+++", "---"))]


class AstcGpuDecodingPatchTests(unittest.TestCase):
    def setUp(self):
        self.operations, self.operations_removed = sides(PATCH, OPERATIONS)
        self.header, _ = sides(PATCH, HEADER)
        self.decoder, self.decoder_removed = sides(PATCH, DECODER)
        start = self.operations.index("bool canDecodeAstcOnGpu(")
        self.check = self.operations[start:self.operations.index("return lacks.empty();", start)]

    def test_pr3_only_the_emulation_and_the_decoder_change(self):
        text = PATCH.read_text(encoding="utf-8")
        files = re.findall(r"^diff --git a/(\S+) ", text, flags=re.M)
        self.assertEqual(files, [OPERATIONS, HEADER, DECODER])
        self.assertIn("bool canDecodeAstcOnGpu(VulkanDispatch* vk, VkPhysicalDevice physicalDevice,\n"
                      "                        std::string* missing);", self.header)

    def test_pr3_the_renderer_decodes_astc_on_the_gpu_and_never_on_the_cpu(self):
        # FrameBuffer's default (AstcEmulationMode::Gpu) is what DSA runs with: no patch may
        # change it, and the CPU decoder stays out of the build.
        for patch in sorted(PATCHES.rglob("*.patch")):
            for line in changed_lines(patch):
                self.assertNotIn("astcLdrEmulationMode", line, patch.name)
                self.assertNotIn("AstcEmulationMode::Cpu", line, patch.name)
                self.assertNotIn("AstcCpuDecompressor", line, patch.name)
        cmake = (HERE / "host/CMakeLists.txt").read_text(encoding="utf-8")
        self.assertIn('set(ASTC_CPU_DECODING OFF CACHE BOOL "" FORCE)', cmake)
        self.assertNotIn("ASTC_CPU_DECODING ON", cmake)

    def test_pr3_no_other_patch_hides_astc_from_apps(self):
        # The guest driver passes the host's features and format properties through; no host
        # or guest patch may filter ASTC out of them.
        for patch in sorted(PATCHES.rglob("*.patch")):
            if patch == PATCH:
                continue
            for line in changed_lines(patch):
                self.assertNotIn("textureCompressionASTC", line, patch.name)
                self.assertNotIn("enableEmulatedAstc", line, patch.name)
                self.assertNotIn("needEmulatedAstc", line, patch.name)

    def test_pr3_astc_is_offered_only_through_the_gpu_decoder(self):
        self.assertIn("if (m_vkEmulation->getAstcLdrEmulationMode() == AstcEmulationMode::Disabled) {",
                      self.decoder_removed)
        self.assertIn("+        if (m_vkEmulation->getAstcLdrEmulationMode() != AstcEmulationMode::Gpu) {\n"
                      "             return false;", PATCH.read_text(encoding="utf-8"))

    def test_pr3_angle_is_offered_astc_too(self):
        # Only ASTC's exclusion of ANGLE goes; ETC2 keeps it, since ANGLE decodes ETC2 itself.
        self.assertEqual(self.decoder_removed.count("return !isAngleInstanceLocked(physicalDevice, vk);"), 1)
        self.assertIn("// Don't enable ASTC emulation for ANGLE, let it do its own emulation.",
                      self.decoder_removed)
        self.assertNotIn("ETC2 emulation for ANGLE", self.decoder_removed)
        self.assertIn("return canDecodeAstcOnGpu(vk, physicalDevice, nullptr);", self.decoder)
        self.assertNotIn("isAngleInstanceLocked", self.decoder)

    def test_pr3_a_gpu_that_cannot_run_the_decoder_is_offered_no_astc(self):
        # What the compute decoder needs: Vulkan 1.1 for the extended-usage image it writes, a
        # queue that draws and computes, RGBA32UI and RGBA8UI storage images, and RGBA8 and
        # sRGB images that the app samples filtered.
        self.assertIn("props.apiVersion < VK_API_VERSION_1_1", self.check)
        self.assertIn("VK_QUEUE_GRAPHICS_BIT | VK_QUEUE_COMPUTE_BIT", self.check)
        for need in ("{VK_FORMAT_R32G32B32A32_UINT,\n"
                     "         VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT | VK_FORMAT_FEATURE_TRANSFER_DST_BIT,",
                     "{VK_FORMAT_R8G8B8A8_UINT, VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT,",
                     "{VK_FORMAT_R8G8B8A8_UNORM,\n"
                     "         VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT | "
                     "VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT |",
                     "{VK_FORMAT_R8G8B8A8_SRGB,\n"
                     "         VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT | "
                     "VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT |"):
            self.assertIn(need, self.check)
        self.assertIn("(formatProps.optimalTilingFeatures & need.features) != need.features",
                      self.check)
        # The features, the format properties and the device creation all go through
        # enableEmulatedAstcLocked, so a device that cannot decode is offered nothing.
        self.assertNotIn("textureCompressionASTC_LDR", self.decoder_removed)

    def test_pr3_the_renderer_says_once_how_astc_reaches_apps(self):
        start = self.operations.index("// Say once how apps get ASTC LDR textures")
        log = self.operations[start:]
        self.assertIn("if (physdevFeatures.textureCompressionASTC_LDR) {", log)
        self.assertIn('"ASTC LDR textures: native.', log)
        self.assertIn('"ASTC LDR textures: GPU-emulated.', log)
        # Unavailable, with the reason: the decoder is off, or the GPU lacks what it needs.
        self.assertEqual(log.count('"ASTC LDR textures: unavailable.'), 2)
        self.assertIn("string_AstcEmulationMode(mAstcLdrEmulationMode)", log)
        self.assertIn("canDecodeAstcOnGpu(mIvk, mPhysicalDevice, &missing)", log)
        self.assertIn("missing.c_str()", log)
        self.assertIn("Nothing decodes them on", log)
        # It is part of initFeatures, which runs once when the renderer starts.
        text = PATCH.read_text(encoding="utf-8")
        hunk = text[text.rindex("@@", 0, text.index("// Say once how apps get ASTC")):]
        self.assertIn("mUseDedicatedAllocations = features.useDedicatedAllocations;", hunk)

    def test_pr3_images_that_stand_in_for_astc_take_no_compressed_view_formats(self):
        # The decoded image and the block images are made and queried in other formats than
        # the app's ASTC one, so its format list (ANGLE gives one for the sRGB twin) and its
        # ASTC decode mode would make them invalid.
        self.assertIn("static void dropFromChain(void* head, VkStructureType sType) {",
                      self.decoder)
        self.assertIn("dropFromChain(&decompInfo, VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO);",
                      self.decoder)
        self.assertIn("dropFromChain(&imageFormatInfo, VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO);",
                      self.decoder)
        self.assertIn("dropFromChain(&createInfo, VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT);",
                      self.decoder)
        text = PATCH.read_text(encoding="utf-8")
        # Dropped before the decoded image and its block images are made from it.
        self.assertLess(text.index("dropFromChain(&decompInfo,"),
                        text.index("             pCreateInfo = &decompInfo;"))

    def test_pr3_the_fetch_applies_it_after_0034(self):
        names = sorted(p.name for p in PATCHES.glob("00*.patch"))
        self.assertEqual(names.index(PATCH.name),
                         names.index(next(n for n in names if n.startswith("0034-"))) + 1)
        for script in (HERE / "host/fetch.ps1", HERE / "host/fetch.sh"):
            self.assertIn("0035", script.read_text(encoding="utf-8"))


if __name__ == "__main__":
    unittest.main()
