"""Tests for the guest Vulkan driver finding what an app wrote with the kernel's dirty tracking.

The driver keeps a copy of each host-visible allocation the app maps and sends the host what the
app wrote before each submit. It found that by hashing every block of every mapped copy, so a
submit read all of the app's mapped memory: a game that maps a few hundred megabytes spent two
thirds of its render thread in the hash (dsa::shadowBlockHash) and never finished loading.

Patch 0014 asks the kernel instead (dsa_dirty_pages.h, tested in tests/dirty_pages_test.cpp):
userfaultfd's asynchronous write protection with PAGEMAP_SCAN, then soft-dirty bits, then the
hashing as before. The guest kernel and the image's SELinux policy give it what it needs."""

import re
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
PATCH_DIR = HERE / "patches/guest/mesa"
PATCH = next(PATCH_DIR.glob("0014-*.patch"))
TRACKER = "src/gfxstream/guest/vulkan_enc/ResourceTracker.cpp"
HEADER = "src/gfxstream/guest/vulkan_enc/ResourceTracker.h"
DIRTY = (HERE / "include/dsa_dirty_pages.h").read_text(encoding="utf-8")
ROOT = HERE.parent.parent


def file_hunks(patch, name):
    """The added and removed lines of one file in a patch."""
    text = patch.read_text(encoding="utf-8")
    for part in re.split(r"^diff --git ", text, flags=re.M):
        if part.startswith(f"a/{name} "):
            lines = part.splitlines()
            return ([l[1:] for l in lines if l.startswith("+") and not l.startswith("+++")],
                    [l[1:] for l in lines if l.startswith("-") and not l.startswith("---")])
    raise AssertionError(f"{patch.name} does not change {name}")


class DirtyTrackingPatchTests(unittest.TestCase):
    def test_nfr2_a_flush_compares_only_what_the_kernel_saw_written(self):
        added, removed = file_hunks(PATCH, TRACKER)
        text = "\n".join(added)
        self.assertIn("            for (const dsa::ShadowBlocks::Range& range : "
                      "info.shadowBlocks->changed(info.ptr)) {", removed)
        self.assertIn("            scanned += info.shadowBlocks->size();", removed)
        self.assertNotIn("shadowBlocks->changed(", text)
        self.assertIn("dsa::changedShadow(\n"
                      "                     dirty, info.shadowRegion, *info.shadowBlocks, info.ptr, "
                      "&scanned)", text)
        # Soft-dirty bits are read and cleared once for all of the device's copies.
        self.assertLess(text.index("dirty.beginFlush();"), text.index("dsa::changedShadow("))
        self.assertLess(text.index("dsa::changedShadow("), text.index("dirty.endFlush();"))
        # The trace's scanned bytes are the bytes hashed now.
        self.assertIn("&scanned)", text)

    def test_nfr2_a_copy_is_tracked_from_the_first_map_before_its_read_back(self):
        added, removed = file_hunks(PATCH, TRACKER)
        raw = PATCH.read_text(encoding="utf-8")
        text = "\n".join(added)
        self.assertIn("            deviceMemoryInfo.shadowRegion = dsa::dirtyPages(logDirtyTracking)", added)
        self.assertIn("dsa::shadowMemoryLength(", text)
        # Tracked once, before the read back writes the copy, so its writes count.
        track = raw.index("+            deviceMemoryInfo.shadowTracked = true;")
        self.assertLess(raw.index("+        if (!deviceMemoryInfo.shadowTracked) {"), track)
        self.assertLess(track, raw.index(" // The guest's copy was not kept in step while the app"))
        # The read back's call ends on this line, in the context of the next hunk.
        self.assertLess(track, raw.index("true /* do lock */);"))
        # The read back is settled by hashing what it wrote, not every block.
        self.assertIn("                deviceMemoryInfo.shadowBlocks->refresh(deviceMemoryInfo.ptr);",
                      removed)
        self.assertIn("                dsa::refreshShadow(dsa::dirtyPages(logDirtyTracking),", added)
        self.assertLess(raw.index("+                dsa::refreshShadow("),
                        raw.index("         deviceMemoryInfo.shadowMapping.mapped();"))

    def test_nfr2_a_freed_copy_is_untracked_before_its_pages_go(self):
        added, _ = file_hunks(PATCH, TRACKER)
        raw = PATCH.read_text(encoding="utf-8")
        self.assertIn("            if (info.shadowRegion) "
                      "dsa::dirtyPages(logDirtyTracking).untrack(info.shadowRegion);", added)
        self.assertLess(raw.index("untrack(info.shadowRegion);"),
                        raw.index("             dsa::freeShadowMemory(info.ptr, info.shadowSize);"))

    def test_nfr2_the_tracking_logs_through_the_drivers_log(self):
        added, _ = file_hunks(PATCH, TRACKER)
        self.assertIn('#include "dsa_dirty_pages.h"', added)
        self.assertIn('static void logDirtyTracking(const char* message) { mesa_logi("%s", message); }',
                      added)

    def test_nfr2_each_allocation_keeps_its_tracking_id(self):
        added, _ = file_hunks(PATCH, HEADER)
        self.assertIn("        bool shadowTracked = false;", added)
        self.assertIn("        uint64_t shadowRegion = 0;", added)

    def test_nfr2_write_protection_comes_first_then_soft_dirty_then_hashing(self):
        choose = DIRTY[DIRTY.index("void chooseLocked()"):DIRTY.index("bool forkedLocked()")]
        self.assertLess(choose.index("openWriteProtect("), choose.index("openSoftDirty("))
        self.assertLess(choose.index("openSoftDirty("), choose.index("DirtyTracking::kHashing"))
        # Each refusal is said once, with its reason.
        self.assertIn("if (noted_.insert(message).second) kernel_.log(message);", DIRTY)

    def test_nfr2_the_scan_lists_written_pages_and_protects_them_in_one_step(self):
        self.assertIn("arg.flags = kPmScanWpMatching | kPmScanCheckWpAsync;", DIRTY)
        self.assertIn("arg.categoryMask = kPageIsWritten;", DIRTY)
        self.assertIn("arg.returnMask = kPageIsWritten;", DIRTY)
        self.assertIn("kUffdFeatureWpAsync | kUffdFeatureWpUnpopulated", DIRTY)
        self.assertIn("kUffdUserModeOnly", DIRTY)
        # The kernel's numbers: UFFD_FEATURE_WP_UNPOPULATED, UFFD_FEATURE_WP_ASYNC,
        # PAGE_IS_WRITTEN, PM_SCAN_WP_MATCHING, PM_SCAN_CHECK_WPASYNC and the ioctls.
        for line in ["kUffdFeatureWpUnpopulated = 1ULL << 13;", "kUffdFeatureWpAsync = 1ULL << 15;",
                     "kPageIsWritten = 1ULL << 1;", "kPmScanWpMatching = 1ULL << 0;",
                     "kPmScanCheckWpAsync = 1ULL << 1;", "_IOWR(0xAA, 0x3F, UffdioApi)",
                     "_IOWR(0xAA, 0x00, UffdioRegister)", "_IOR(0xAA, 0x01, UffdioRange)",
                     "_IOWR(0xAA, 0x06, UffdioWriteProtect)", "_IOWR('f', 16, PmScanArg)",
                     "kPagemapSoftDirty = 1ULL << 55;"]:
            self.assertIn(line, DIRTY)
        # A forked child never asks the kernel with the parent's files.
        self.assertIn("kernel_.processId() == pid_", DIRTY)

    def test_nfr2_pr3_the_kernel_and_the_policy_give_the_driver_what_it_needs(self):
        config = (ROOT / "dsa/kernel/android.config").read_text(encoding="utf-8").splitlines()
        for option in ["CONFIG_USERFAULTFD=y", "CONFIG_PTE_MARKER_UFFD_WP=y",
                       "CONFIG_PROC_PAGE_MONITOR=y"]:
            self.assertIn(option, config)
        policy = (ROOT / "dsa/guest/image/sepolicy/graphics-memory.cil").read_text(encoding="utf-8")
        self.assertIn("(allowx appdomain self (ioctl file (0x6610)))", policy)

    def test_nfr2_it_applies_after_0013_and_its_unit_test_runs(self):
        names = sorted(p.name for p in PATCH_DIR.glob("*.patch"))
        self.assertEqual(names[names.index(PATCH.name) - 1][:5], "0013-")
        tests = (HERE / "tests/test.sh").read_text(encoding="utf-8")
        self.assertIn('"$out/dirty_pages_test"', tests)

    def test_nfr2_the_patch_hunks_count_their_lines(self):
        # git apply refuses a hunk whose header miscounts its lines.
        header = re.compile(r"^@@ -\d+(?:,(\d+))? \+\d+(?:,(\d+))? @@")
        old_left = new_left = 0
        current = None
        for line in PATCH.read_text(encoding="utf-8").splitlines():
            match = header.match(line)
            if match:
                self.assertEqual((old_left, new_left), (0, 0), current)
                current = line
                old_left = int(match.group(1) or 1)
                new_left = int(match.group(2) or 1)
                continue
            if old_left == 0 and new_left == 0:
                continue
            if line.startswith("+"):
                new_left -= 1
            elif line.startswith("-"):
                old_left -= 1
            else:
                old_left -= 1
                new_left -= 1
        self.assertEqual((old_left, new_left), (0, 0), current)


if __name__ == "__main__":
    unittest.main()
