package io.github.thisisthepy.dsa.agent.tasks

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class FullScreenHoldsTest {
    private val window = TaskBounds(100, 100, 900, 600)
    private val display = TaskBounds(0, 0, 2560, 1440)

    private fun task(
        bounds: TaskBounds = window,
        leftFreeform: Boolean = false,
        top: String = "org.example.player/.Watch",
    ) = TaskSnapshot(
        taskId = 7,
        kind = TaskKind.APP,
        packageName = "org.example.player",
        label = "Player",
        bounds = bounds,
        focused = true,
        fullscreenRequest = if (leftFreeform) FullScreenRequest.EXPLICIT else FullScreenRequest.NONE,
        leftFreeform = leftFreeform,
        stack = TaskStack(activities = 1, top = top, topShows = true, visible = true),
    )

    @Test
    fun fr30_a_task_that_leaves_freeform_stays_inside_its_window_when_the_host_says_so() {
        val holds = FullScreenHolds()
        holds.hostSays(7, inWindow = true)
        holds.apply(listOf(task()))
        // The app asks for the fullscreen windowing mode: the task goes back to freeform at its
        // window's bounds, and the host hears of those bounds and of the request from the start.
        val (asked, changes) = holds.apply(listOf(task(display, leftFreeform = true)))
        assertEquals(listOf(WindowingChange.ToFreeform(7, window)), changes)
        assertEquals(window, asked.single().bounds)
        assertEquals(FullScreenRequest.EXPLICIT, asked.single().fullscreenRequest)
        // Asked once while the framework carries it out.
        assertTrue(holds.apply(listOf(task(display, leftFreeform = true))).second.isEmpty())
        // Back in freeform, it still asks for full screen, so its title band stays away.
        val (kept, none) = holds.apply(listOf(task()))
        assertTrue(none.isEmpty())
        assertTrue(holds.holds(7))
        assertEquals(FullScreenRequest.EXPLICIT, kept.single().fullscreenRequest)
        // Another screen on top ends it.
        val (other, _) = holds.apply(listOf(task(top = "org.example.player/.Browse")))
        assertFalse(holds.holds(7))
        assertEquals(FullScreenRequest.NONE, other.single().fullscreenRequest)
    }

    @Test
    fun fr30_a_task_kept_in_its_window_fills_the_screen_again_when_the_host_says_the_whole_screen() {
        val holds = FullScreenHolds()
        holds.hostSays(7, inWindow = true)
        holds.apply(listOf(task()))
        holds.apply(listOf(task(display, leftFreeform = true)))
        holds.apply(listOf(task()))
        holds.hostSays(7, inWindow = false)
        val (shown, changes) = holds.apply(listOf(task()))
        assertEquals(listOf(WindowingChange.ToFullscreen(7)), changes)
        assertEquals(FullScreenRequest.EXPLICIT, shown.single().fullscreenRequest)
        // In fullscreen, it is left there and reported as it is.
        val (full, none) = holds.apply(listOf(task(display, leftFreeform = true)))
        assertTrue(none.isEmpty())
        assertEquals(display, full.single().bounds)
    }

    @Test
    fun fr30_without_a_word_from_the_host_a_task_that_leaves_freeform_fills_the_screen() {
        val holds = FullScreenHolds()
        holds.apply(listOf(task()))
        val (full, changes) = holds.apply(listOf(task(display, leftFreeform = true)))
        assertTrue(changes.isEmpty())
        assertEquals(display, full.single().bounds)
    }
}
