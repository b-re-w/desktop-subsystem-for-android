"""Tests for giving back a large staging grant once its copy is done (patches/guest/mesa/0018).

Each guest Vulkan device kept one staging grant of the graphics memory the host shares, for
gralloc buffers' CPU uploads and read backs (0015), sized to the largest copy so far and held
until the device went. SurfaceFlinger's device lives as long as Android, and every screen
capture is read back for the CPU, so one capture of the 4096x4096 display held 64 MiB of the
shared memory from then on. The rule (keep up to 16 MiB) is in include/dsa_graphics_memory.h,
tested by tests/graphics_memory_test.cpp."""

import re
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
PATCHES = HERE / "patches/guest/mesa"
PATCH = next(PATCHES.glob("0018-*.patch"))
HEADER = HERE / "include/dsa_graphics_memory.h"
TESTS = HERE / "tests/test.sh"
UNIT = HERE / "tests/graphics_memory_test.cpp"


def added(name):
    """The added lines of one file in the patch."""
    text = PATCH.read_text(encoding="utf-8")
    for part in re.split(r"^diff --git ", text, flags=re.M):
        if part.startswith(f"a/{name} "):
            return [l[1:] for l in part.splitlines() if l.startswith("+") and not l.startswith("+++")]
    raise AssertionError(f"{PATCH.name} does not change {name}")


class StagingGrantTests(unittest.TestCase):
    def test_nfr1_a_grant_over_16_mib_is_not_kept(self):
        header = HEADER.read_text(encoding="utf-8")
        self.assertIn("inline constexpr uint64_t kStagingKeepBytes = uint64_t(16) << 20;", header)
        self.assertIn("inline constexpr bool keepStagingGrant(uint64_t grantBytes)", header)
        self.assertIn("nfr1_a_staging_grant_larger_than_16_mib_is_not_kept();",
                      UNIT.read_text(encoding="utf-8"))
        self.assertIn('"$here/graphics_memory_test.cpp"', TESTS.read_text(encoding="utf-8"))

    def test_nfr1_the_grant_is_sized_and_remembered_by_the_shared_rule(self):
        cpp = "\n".join(added("src/gfxstream/guest/vulkan_enc/ResourceTracker.cpp"))
        self.assertIn("dsa::stagingGrantBytes(size), typeIndex};", cpp)
        self.assertIn("staging.allocationSize = info.allocationSize;", cpp)
        header = added("src/gfxstream/guest/vulkan_enc/ResourceTracker.h")
        self.assertIn("        uint64_t allocationSize = 0;", header)
        self.assertIn("    void releaseLargeStagingLocked(VkEncoder* enc, VkDevice device);", header)

    def test_nfr1_a_large_grant_goes_once_the_read_back_or_upload_is_done(self):
        cpp = "\n".join(added("src/gfxstream/guest/vulkan_enc/ResourceTracker.cpp"))
        self.assertIn("void ResourceTracker::releaseLargeStagingLocked(VkEncoder* enc, VkDevice device) {",
                      cpp)
        self.assertIn("dsa::keepStagingGrant(found->second.allocationSize)", cpp)
        # Unmapped here before the host frees it, so the granules come back at once.
        self.assertLess(cpp.index("shared->unmap(found->second.grant.ptr"),
                        cpp.index("enc->vkFreeMemory(device, found->second.memory"))
        self.assertIn("mSharedStaging.erase(found);", cpp)
        # Both copies let go of it, after the copy, under the staging lock they already hold.
        self.assertIn("        releaseLargeStagingLocked(enc, device);", cpp)
        self.assertIn("    if (device != VK_NULL_HANDLE) releaseLargeStagingLocked(enc, device);", cpp)
        text = PATCH.read_text(encoding="utf-8")
        self.assertLess(text.index("mGralloc->readBackForCpu(mGralloc->getNativeHandle(ahb), staging);"),
                        text.index("+        releaseLargeStagingLocked(enc, device);"))
        self.assertLess(text.index("mGralloc->uploadForGpu(ahb, staging);"),
                        text.index("+    if (device != VK_NULL_HANDLE) releaseLargeStagingLocked"))

    def test_nfr1_the_fetch_applies_it_after_0017(self):
        names = sorted(p.name for p in PATCHES.glob("00*.patch"))
        self.assertEqual(names.index(PATCH.name),
                         names.index(next(n for n in names if n.startswith("0017-"))) + 1)
        fetch = (HERE / "fetch.sh").read_text(encoding="utf-8")
        self.assertIn('for patch in "$patch_dir"/guest/mesa/*.patch; do', fetch)


if __name__ == "__main__":
    unittest.main()
