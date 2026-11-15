"""Tests for the renderer's and the guest Vulkan driver's memory diagnostics.

Host patch 0041 lets the renderer describe every ColorBuffer (who made it, who holds it open,
whether it is a task window's frame). Guest gfxstream patch 0003 has each guest process say who it
is on its process pipe, so the renderer's report names it. Guest Mesa patch 0016 names the
driver's copies of host-visible memory [anon:dsa-vk-copy] and logs each process's totals."""

import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
PATCHES = HERE / "patches"
INCLUDE = HERE / "include"
HOST = HERE / "host"


def read(path):
    return path.read_text(encoding="utf-8")


def added(text):
    return "\n".join(l[1:] for l in text.splitlines() if l.startswith("+") and not l.startswith("+++"))


class MemoryDiagnosticsPatchTests(unittest.TestCase):
    def setUp(self):
        self.host = read(next(PATCHES.glob("0041-*.patch")))
        self.identity = read(next((PATCHES / "guest/gfxstream").glob("0003-*.patch")))
        self.driver = read(next((PATCHES / "guest/mesa").glob("0016-*.patch")))

    def test_nfr1_every_color_buffer_is_described_under_the_tables_locks(self):
        body = added(self.host)
        self.assertIn("std::vector<ColorBufferDescription> describeColorBuffers();", body)
        self.assertIn("AutoLock colorBufferMapLock(m_colorBufferMapLock);", body)
        self.assertIn("m_procOwnedColorBuffers", body)
        self.assertIn("getColorBufferAllocationInfo", body)

    def test_nfr1_a_color_buffer_remembers_its_creator_and_last_post(self):
        body = added(self.host)
        self.assertIn("history.creatorPuid = creator ? creator->m_puid : 0;", body)
        self.assertIn("found->second.lastPostUs = android::base::getUnixTimeUs();", body)

    def test_nfr1_the_backend_lists_color_buffers_and_names_guest_devices(self):
        header = read(HOST / "dsa_gfx_backend.h")
        self.assertIn("dsa_gfx_describe_color_buffers(char* out, size_t capacity)", header)
        self.assertIn("dsa_gfx_get_guest_devices(", header)
        memory = read(HOST / "dsa_gfx_memory.cpp")
        self.assertIn("case VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT:\n"
                      "                return Import::Grant;", memory)
        self.assertIn('role = "client target";', memory)
        self.assertIn("task frame (display", memory)

    def test_nfr1_a_guest_process_says_its_pid_and_name_on_the_process_pipe(self):
        body = added(self.identity)
        self.assertIn("identity.op = dsa::kIdentifyRequest;", body)
        self.assertIn("identity.pid = static_cast<uint32_t>(getpid());", body)
        backend = read(HOST / "dsa_gfx_backend.cpp")
        self.assertIn("if (op == dsa::kIdentifyRequest) {", backend)
        self.assertIn("dsa_gfx_memory_process_forget(puid);", backend)
        shared = read(INCLUDE / "dsa_graphics_memory.h")
        self.assertIn('static_assert(sizeof(ProcessIdentity) == 64', shared)

    def test_nfr1_the_drivers_copies_are_named_and_counted(self):
        body = added(self.driver)
        self.assertIn("counts.named(dsa::nameCopyMapping(", body)
        self.assertIn("counts.copied(why);", body)
        self.assertIn("dsa::driverMemoryLog().copyFreed(", body)
        self.assertIn("dsa::driverMemoryLog().grantUnmapped(info.sharedLength);", body)
        header = read(INCLUDE / "dsa_driver_memory.h")
        self.assertIn('kCopyMappingName[] = "dsa-vk-copy"', header)
        self.assertIn("PR_SET_VMA, PR_SET_VMA_ANON_NAME", header)
        self.assertIn("kInterval{3}", header)

    def test_pr3_every_context_line_starts_with_a_space(self):
        for text in (self.host, self.identity, self.driver):
            in_hunk = False
            for line in text.splitlines():
                if line.startswith("@@"):
                    in_hunk = True
                elif line.startswith("diff --git"):
                    in_hunk = False
                elif in_hunk:
                    self.assertTrue(line[:1] in (" ", "+", "-"), repr(line))

    def test_pr3_the_fetch_applies_0041_after_0040(self):
        self.assertIn(" 0040 0041;", read(HOST / "fetch.sh"))
        self.assertIn('-Filter "0041-*.patch"', read(HOST / "fetch.ps1"))


if __name__ == "__main__":
    unittest.main()
