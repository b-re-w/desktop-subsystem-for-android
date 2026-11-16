"""Tests for the framework patches that keep an app and SurfaceFlinger alive when the graphics
renderer's stream is lost or the renderer is made again.

On the owner's PC the window manager (and the renderer it starts) restarted while an app opened.
The app's render thread crashed in libEGL's egl_cache_t::initialize (strlen of a NULL extension
string, reached from hwui's retry after a device loss), and SurfaceFlinger aborted in RenderEngine
("Failed to create a valid texture") on a buffer the new renderer did not know. Either one ends
every app, because Android's user space restarts."""

import re
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
NATIVE = HERE / "patches/frameworks/native"
VERSIONS = {"android16": ("0005", "0006"), "android17": ("0006", "0007")}


def patch(android, number):
    found = list((NATIVE / android).glob(f"{number}-*.patch"))
    assert len(found) == 1, (android, number)
    return found[0].read_text(encoding="utf-8").replace("\r\n", "\n")


def lines_of(text):
    added = [l[1:] for l in text.splitlines() if l.startswith("+") and not l.startswith("+++")]
    removed = [l[1:] for l in text.splitlines() if l.startswith("-") and not l.startswith("---")]
    return added, removed


class LostRendererPatchTests(unittest.TestCase):
    def test_nfr5_the_egl_cache_does_not_take_the_length_of_a_missing_extension_string(self):
        for android, (_, number) in VERSIONS.items():
            text = patch(android, number)
            self.assertIn("+++ b/opengl/libs/EGL/egl_cache.cpp", text, android)
            added, removed = lines_of(text)
            self.assertEqual(removed, [], android)
            joined = "\n".join(added)
            self.assertIn("if (exts == nullptr) {", joined, android)
            self.assertIn("return;", joined, android)
            # The check comes before anything reads the string.
            self.assertLess(text.index("exts == nullptr"), text.index("strlen(exts)"), android)

    def test_nfr5_surfaceflinger_does_not_abort_on_a_buffer_the_gpu_cannot_wrap(self):
        for android, (number, _) in VERSIONS.items():
            text = patch(android, number)
            self.assertIn("+++ b/libs/renderengine/skia/compat/GaneshBackendTexture.cpp", text, android)
            added, _ = lines_of(text)
            joined = "\n".join(added)
            # A buffer with a size that the GPU cannot wrap is warned about, not fatal.
            self.assertIn("} else if (!mBackendTexture.isValid() && desc.width && desc.height) {", joined, android)
            self.assertIn("ALOGW(\"Failed to create a valid texture.", joined, android)
            self.assertNotIn("LOG_ALWAYS_FATAL(\"Failed to create a valid texture", joined, android)
            # The invalid texture gives nothing to draw, as an image and as a surface.
            self.assertEqual(joined.count("if (!mBackendTexture.isValid()) {"), 2, android)
            self.assertEqual(joined.count("return nullptr;"), 2, android)
            # A buffer with no size is still a programming error: its fatal line is context.
            self.assertIn(" LOG_ALWAYS_FATAL(\"Failed to create a valid texture.", text.replace("        LOG_ALWAYS", " LOG_ALWAYS"), android)

    def test_nfr5_every_hunk_line_starts_with_a_diff_marker(self):
        # `git apply` on Windows rejects a context line that lost its leading space.
        for android, numbers in VERSIONS.items():
            for number in numbers:
                in_hunk = False
                for line in patch(android, number).splitlines():
                    if line.startswith("@@"):
                        in_hunk = True
                    elif line.startswith("diff --git"):
                        in_hunk = False
                    elif in_hunk:
                        self.assertIn(line[:1], (" ", "+", "-"), (android, number, line))

    def test_nfr5_the_patches_come_after_the_ones_they_build_on(self):
        for android, numbers in VERSIONS.items():
            names = sorted(p.name[:4] for p in (NATIVE / android).glob("*.patch"))
            self.assertEqual(names, sorted(set(names)), android)
            for number in numbers:
                self.assertIn(number, names, android)
            # 0001 changes GaneshBackendTexture.cpp, which the SurfaceFlinger patch builds on.
            self.assertLess(names.index("0001"), names.index(numbers[0]), android)


if __name__ == "__main__":
    unittest.main()
