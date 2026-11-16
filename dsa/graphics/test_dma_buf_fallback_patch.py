"""Tests for host patch 0039: memory is exported as an opaque fd when a dma-buf is refused.

lavapipe lists VK_EXT_external_memory_dma_buf but makes a dma-buf through /dev/udmabuf, so on a
host without it every exported allocation failed and the renderer could not find a memory type
for its ColorBuffers. Patch 0039 makes such an allocation again without the dma-buf handle type,
and exports what was made."""

import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
PATCHES = HERE / "patches"
PATCH = next(PATCHES.glob("0039-*.patch"))


class DmaBufFallbackPatchTests(unittest.TestCase):
    def setUp(self):
        self.text = PATCH.read_text(encoding="utf-8")
        self.added = "\n".join(l[1:] for l in self.text.splitlines()
                               if l.startswith("+") and not l.startswith("+++"))

    def test_pr3_a_refused_dma_buf_allocation_is_made_again_without_it(self):
        self.assertIn("exportsDmaBuf = false;", self.added)
        self.assertIn("&= ~VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT", self.added)
        self.assertIn("continue;", self.added)

    def test_pr3_the_export_follows_the_handle_type_the_memory_was_made_with(self):
        self.assertIn("+        if (exportsDmaBuf) {\n+            vkHandleType", self.text)
        self.assertNotIn("+        if (mDeviceInfo.supportsDmaBuf)", self.text)

    def test_pr3_every_context_line_starts_with_a_space(self):
        in_hunk = False
        for line in self.text.splitlines():
            if line.startswith("@@"):
                in_hunk = True
            elif line.startswith("diff --git"):
                in_hunk = False
            elif in_hunk:
                self.assertTrue(line[:1] in (" ", "+", "-"), repr(line))

    def test_pr3_the_fetch_applies_it_after_0038(self):
        self.assertIn(" 0038 0039 ", (HERE / "host/fetch.sh").read_text(encoding="utf-8"))
        self.assertIn('-Filter "0039-*.patch"',
                      (HERE / "host/fetch.ps1").read_text(encoding="utf-8"))


if __name__ == "__main__":
    unittest.main()
