package io.github.thisisthepy.dsa.agent.tasks

import io.github.thisisthepy.dsa.agent.decode
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import io.github.thisisthepy.dsa.control.v1.FullScreenRequest as WireRequest

class TaskWindowModeTest {
    private fun task(
        orientation: TaskOrientation = TaskOrientation.UNSPECIFIED,
        fullscreenRequest: FullScreenRequest = FullScreenRequest.NONE,
        pictureInPicture: Boolean = false,
    ) = TaskSnapshot(
        taskId = 4,
        kind = TaskKind.APP,
        packageName = "org.example.reader",
        label = "Reader",
        bounds = TaskBounds(0, 0, 600, 1000),
        focused = true,
        orientation = orientation,
        fullscreenRequest = fullscreenRequest,
        pictureInPicture = pictureInPicture,
    )

    @Test
    fun fr30_an_activitys_screen_orientation_says_which_way_its_task_is_locked() {
        assertEquals(TaskOrientation.PORTRAIT, TaskOrientation.of(1))
        assertEquals(TaskOrientation.PORTRAIT, TaskOrientation.of(7))
        assertEquals(TaskOrientation.LANDSCAPE, TaskOrientation.of(0))
        assertEquals(TaskOrientation.LANDSCAPE, TaskOrientation.of(6))
        // Unspecified, sensor, user, full sensor and locked do not lock it one way.
        for (free in listOf(-1, 2, 4, 10, 14)) {
            assertEquals(TaskOrientation.UNSPECIFIED, TaskOrientation.of(free))
        }
    }

    @Test
    fun fr30_the_host_hears_a_tasks_orientation_and_whether_it_fills_the_screen_or_floats() {
        val portrait = decode(Messages.taskUpdated(task(TaskOrientation.PORTRAIT))).taskUpdated
        assertEquals(
            io.github.thisisthepy.dsa.control.v1.TaskOrientation.TASK_ORIENTATION_PORTRAIT,
            portrait.orientation,
        )
        assertFalse(portrait.fullscreen)
        assertEquals(WireRequest.FULL_SCREEN_REQUEST_NONE, portrait.fullscreenRequest)
        val full = decode(Messages.taskUpdated(task(fullscreenRequest = FullScreenRequest.EXPLICIT))).taskUpdated
        assertTrue(full.fullscreen)
        assertEquals(WireRequest.FULL_SCREEN_REQUEST_EXPLICIT, full.fullscreenRequest)
        val pip = decode(Messages.taskUpdated(task(pictureInPicture = true))).taskUpdated
        assertTrue(pip.pictureInPicture)
    }

    @Test
    fun fr30_the_agent_reports_which_kind_of_full_screen_an_app_asks_for() {
        val barsHidden = 0x1F8 // every insets type but the status and navigation bars
        val shown = 0x1FF
        // Leaving freeform and a full screen theme are clear requests in any orientation.
        assertEquals(FullScreenRequest.EXPLICIT, fullScreenRequest(true, shown, false))
        assertEquals(FullScreenRequest.EXPLICIT, fullScreenRequest(false, null, true, TaskOrientation.PORTRAIT))
        // Both bars hidden with landscape is a player's full screen button.
        assertEquals(
            FullScreenRequest.EXPLICIT,
            fullScreenRequest(false, barsHidden, false, TaskOrientation.LANDSCAPE),
        )
        // Both bars hidden without landscape is told apart, so the host can keep the window.
        for (orientation in listOf(TaskOrientation.UNSPECIFIED, TaskOrientation.PORTRAIT)) {
            assertEquals(
                FullScreenRequest.BARS_HIDDEN,
                fullScreenRequest(false, barsHidden, false, orientation),
            )
        }
        assertEquals(FullScreenRequest.NONE, fullScreenRequest(false, shown, false, TaskOrientation.LANDSCAPE))
        assertEquals(FullScreenRequest.NONE, fullScreenRequest(false, null, false))
        // The host hears the kind, and that the app fills the screen in either case.
        val immersive = decode(Messages.taskUpdated(task(fullscreenRequest = FullScreenRequest.BARS_HIDDEN))).taskUpdated
        assertEquals(WireRequest.FULL_SCREEN_REQUEST_BARS_HIDDEN, immersive.fullscreenRequest)
        assertTrue(immersive.fullscreen)
    }

    @Test
    fun fr30_a_change_to_a_tasks_mode_is_news_to_the_host() {
        val sent = mutableListOf<ByteArray>()
        val reporter = TaskReporter("io.github.thisisthepy.dsa.agent") { frame ->
            sent += frame
            true
        }
        reporter.update(listOf(task()))
        reporter.update(listOf(task(fullscreenRequest = FullScreenRequest.BARS_HIDDEN)))
        reporter.update(listOf(task(fullscreenRequest = FullScreenRequest.EXPLICIT)))
        assertEquals(3, sent.count { decode(it).hasTaskUpdated() })
        assertEquals(WireRequest.FULL_SCREEN_REQUEST_EXPLICIT, decode(sent.last()).taskUpdated.fullscreenRequest)
    }
}
