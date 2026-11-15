"""Tests for host patch 0038: the renderer's headers include <cstdint> for the types they use.

GCC 15's libstdc++ (Ubuntu 26.04, now the Linux runners' default) no longer includes <cstdint>
from <memory>, <string> or <functional>. Four gfxstream headers use uint32_t, uint8_t or uint64_t
with only those, or nothing, in front of them, and the Linux renderer stopped compiling at
AstcCpuDecompressor.h. Patch 0038 adds the include and nothing else."""

import re
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
PATCHES = HERE / "patches"
PATCH = next(PATCHES.glob("0038-*.patch"))
HEADERS = [
    "host/BorrowedImage.h",
    "host/ReadbackWorker.h",
    "host/compressedTextureFormats/AstcCpuDecompressor.h",
    "host/gl/gl-host-common/include/host-common/opengl/GLProcessPipe.h",
]


class FixedWidthTypesPatchTests(unittest.TestCase):
    def test_pr3_each_header_that_uses_the_types_includes_cstdint(self):
        text = PATCH.read_text(encoding="utf-8")
        files = re.findall(r"^diff --git a/(\S+) ", text, flags=re.M)
        self.assertEqual(files, HEADERS)
        for part in re.split(r"^diff --git ", text, flags=re.M)[1:]:
            lines = part.splitlines()
            added = [l[1:] for l in lines if l.startswith("+") and not l.startswith("+++")]
            removed = [l for l in lines if l.startswith("-") and not l.startswith("---")]
            self.assertIn("#include <cstdint>", added, lines[0])
            self.assertEqual([l for l in added if l.strip()], ["#include <cstdint>"], lines[0])
            self.assertEqual(removed, [], lines[0])

    def test_pr3_the_fetch_applies_it_after_0037(self):
        names = sorted(p.name for p in PATCHES.glob("00*.patch"))
        self.assertEqual(names.index(PATCH.name),
                         names.index(next(n for n in names if n.startswith("0037-"))) + 1)
        self.assertRegex( (HERE / "host/fetch.sh").read_text(encoding="utf-8"), r" 0037 0038[ ;]")
        self.assertIn('-Filter "0038-*.patch"',
                      (HERE / "host/fetch.ps1").read_text(encoding="utf-8"))


if __name__ == "__main__":
    unittest.main()
