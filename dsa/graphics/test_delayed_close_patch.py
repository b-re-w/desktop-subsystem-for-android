"""Tests for closing the color buffers a guest closed last.

A ColorBuffer whose last reference the guest closes waits on a delayed close list, which was
looked at only inside the next closeColorBuffer. A video's whole queue closes within one second,
so it stayed until another buffer closed, which with one still app open never came. Patch 0034
adds FrameBuffer::closeDelayedColorBuffers, and the renderer's once-a-second trim calls it."""

import re
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
PATCH = next((HERE / "patches").glob("0034-*.patch"))
TRIM = HERE / "host" / "dsa_gfx_memory.cpp"


def added(patch, name):
    """The lines one file gains."""
    text = patch.read_text(encoding="utf-8")
    for part in re.split(r"^diff --git ", text, flags=re.M):
        if part.startswith(f"a/{name} "):
            return "\n".join(
                l[1:] for l in part.splitlines() if l.startswith("+") and not l.startswith("+++")
            )
    raise AssertionError(f"{patch.name} does not change {name}")


class DelayedClosePatchTests(unittest.TestCase):
    def test_nfr1_only_the_frame_buffer_changes_and_nothing_is_removed(self):
        text = PATCH.read_text(encoding="utf-8")
        files = re.findall(r"^diff --git a/(\S+) ", text, flags=re.M)
        self.assertEqual(files, ["host/FrameBuffer.h", "host/FrameBuffer.cpp"])
        removed = [l for l in text.splitlines() if l.startswith("-") and not l.startswith("---")]
        self.assertEqual(removed, [])

    def test_nfr1_the_delayed_close_runs_under_the_locks_a_close_takes(self):
        body = added(PATCH, "host/FrameBuffer.cpp")
        self.assertIn("int FrameBuffer::closeDelayedColorBuffers() {", body)
        # The frame buffer's lock first, as closeColorBuffer takes it, then the map's.
        self.assertLess(body.index("AutoLock mutex(m_lock);"),
                        body.index("AutoLock colorBufferMapLock(m_colorBufferMapLock);"))
        # The delay stays: only what waited for it goes.
        self.assertIn("performDelayedColorBufferCloseLocked(false);", body)
        self.assertNotIn("performDelayedColorBufferCloseLocked(true);", body)
        self.assertIn("if (m_refCountPipeEnabled) {", body)

    def test_nfr1_the_method_is_public_beside_close_color_buffer(self):
        header = added(PATCH, "host/FrameBuffer.h")
        self.assertIn("int closeDelayedColorBuffers();", header)
        text = PATCH.read_text(encoding="utf-8")
        hunk = text[text.index("+++ b/host/FrameBuffer.h"):text.index("diff --git a/host/FrameBuffer.cpp")]
        self.assertIn(" void closeColorBuffer(HandleType p_colorbuffer);", hunk)

    def test_nfr1_the_trim_closes_them_every_second_before_freeing_retired(self):
        trim = TRIM.read_text(encoding="utf-8")
        run = trim[trim.index("void trim_run() {"):trim.index("void start_trim() {")]
        self.assertIn("fb->closeDelayedColorBuffers()", run)
        self.assertLess(run.index("closeDelayedColorBuffers"), run.index("freeFinishedRetired"))
        self.assertIn("kTrimInterval{1}", trim)

    def test_nfr1_the_fetch_applies_it_after_0033(self):
        names = sorted(p.name for p in (HERE / "patches").glob("00*.patch"))
        self.assertEqual(names.index(PATCH.name),
                         names.index(next(n for n in names if n.startswith("0033-"))) + 1)
        for script in (HERE / "host/fetch.ps1", HERE / "host/fetch.sh"):
            self.assertIn("0034", script.read_text(encoding="utf-8"))


if __name__ == "__main__":
    unittest.main()
