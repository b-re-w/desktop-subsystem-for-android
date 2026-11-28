"""Tests for naming each large grant of the shared graphics memory (patches/guest/mesa/0019 and
host/dsa_gfx_memory.cpp).

The renderer's memory report listed a 64 MiB grant on SurfaceFlinger's device with its process,
size and memory type, but nothing said what the guest made it for. The guest driver now logs a
line for every grant over 16 MiB when it is made and when it goes, with its purpose, and the
renderer's report names the same grants above its list with their age, so one run on the PC
matches the two. The line formats are in include/dsa_driver_memory.h and
include/dsa_allocation_note.h, tested by tests/driver_memory_test.cpp and
tests/allocation_note_test.cpp."""

import re
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
PATCHES = HERE / "patches/guest/mesa"
PATCH = next(PATCHES.glob("0019-*.patch"))
DRIVER_HEADER = HERE / "include/dsa_driver_memory.h"
NOTE_HEADER = HERE / "include/dsa_allocation_note.h"
HOST = HERE / "host/dsa_gfx_memory.cpp"
TESTS = HERE / "tests/test.sh"


def added(name):
    """The added lines of one file in the patch."""
    text = PATCH.read_text(encoding="utf-8")
    for part in re.split(r"^diff --git ", text, flags=re.M):
        if part.startswith(f"a/{name} "):
            return [l[1:] for l in part.splitlines() if l.startswith("+") and not l.startswith("+++")]
    raise AssertionError(f"{PATCH.name} does not change {name}")


class LargeGrantLogTests(unittest.TestCase):
    def test_nfr1_the_line_formats_are_tested_on_the_build_machine(self):
        driver = DRIVER_HEADER.read_text(encoding="utf-8")
        self.assertIn("inline constexpr uint64_t kLargeGrantBytes = uint64_t(16) << 20;", driver)
        self.assertIn('"dsa-vk large grant: %s %s %.1f MiB (%" PRIu64', driver)
        note = NOTE_HEADER.read_text(encoding="utf-8")
        self.assertIn('return "# Large grant: " + process', note)
        tests = TESTS.read_text(encoding="utf-8")
        self.assertIn('"$here/driver_memory_test.cpp"', tests)
        self.assertIn('"$here/allocation_note_test.cpp"', tests)

    def test_nfr1_every_grant_over_16_mib_is_logged_when_made_and_when_it_goes(self):
        cpp = "\n".join(added("src/gfxstream/guest/vulkan_enc/ResourceTracker.cpp"))
        self.assertIn('#include "dsa_allocation_note.h"', cpp)
        self.assertIn("if (!dsa::isLargeGrant(bytes)) return;", cpp)
        self.assertIn("dsa::largeGrantLine(line, sizeof(line), made, dsa::thisProcessName()", cpp)
        # An app's allocation, made and freed.
        self.assertIn("logLargeGrant(true, sharedLength, hostAllocationInfo.memoryTypeIndex,", cpp)
        self.assertIn("logLargeGrant(false, info.sharedLength, info.memoryTypeIndex,", cpp)
        # The staging grant, with what the copy was, made and let go of.
        self.assertIn('"a CPU read back of a %ux%u gralloc buffer (staging)"', cpp)
        self.assertIn('"a CPU upload of a %ux%u gralloc buffer (staging)"', cpp)
        self.assertIn("sharedStagingLocked(enc, device, largest, purpose)", cpp)
        self.assertIn("logLargeGrant(true, staging.grant.length, typeIndex, typeFlags, "
                      "staging.purpose.c_str());", cpp)
        self.assertIn("void ResourceTracker::forgetStagingLocked(const DeviceStaging& staging) {", cpp)
        self.assertIn("logLargeGrant(false, staging.grant.length, staging.typeIndex, staging.typeFlags,",
                      cpp)

    def test_nfr1_staging_grants_count_among_the_process_grants(self):
        cpp = "\n".join(added("src/gfxstream/guest/vulkan_enc/ResourceTracker.cpp"))
        self.assertIn("dsa::driverMemoryLog().grantMapped(staging.grant.length);", cpp)
        self.assertIn("dsa::driverMemoryLog().grantUnmapped(staging.grant.length);", cpp)
        # Every path that lets a staging grant go goes through forgetStagingLocked.
        self.assertGreaterEqual(cpp.count("forgetStagingLocked(found->second);"), 2)
        self.assertIn("forgetStagingLocked(staging->second);", cpp)

    def test_nfr1_the_renderer_report_names_large_grants_with_their_age(self):
        host = HOST.read_text(encoding="utf-8")
        self.assertIn("allocation.created_us = android::base::getUnixTimeUs();", host)
        self.assertIn("if (dsa::isLargeGrantRecord(record)) {", host)
        self.assertIn("notes += dsa::largeGrantNote(label, record, age) + \"\\n\";", host)
        self.assertIn('"device\\tprocess\\tkind\\tbytes\\tmemory_type\\ttype_flags\\tdedicated\\twhy\\tage_s\\n"',
                      host)
        # The notes come before the column header, the lines after it.
        self.assertLess(host.index("text += notes;"), host.index("text += body;"))

    def test_nfr1_the_fetch_applies_it_after_0018(self):
        names = sorted(p.name for p in PATCHES.glob("00*.patch"))
        self.assertEqual(names.index(PATCH.name),
                         names.index(next(n for n in names if n.startswith("0018-"))) + 1)


if __name__ == "__main__":
    unittest.main()
