package io.github.thisisthepy.dsa.agent.tasks

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class ImmersiveTest {
    /** Only the IME and the caption bar asked to be visible: both system bars hidden. */
    private val barsHidden = 8 or 4

    /** Every bar asked to be visible, the framework's default. */
    private val barsShown = 503

    @Test
    fun fr30_an_app_that_hides_both_its_bars_fills_the_screen() {
        assertTrue(hidesBothSystemBars(barsHidden))
        assertTrue(hidesBothSystemBars(0))
    }

    @Test
    fun fr30_an_app_that_keeps_either_bar_or_says_nothing_does_not() {
        assertFalse(hidesBothSystemBars(STATUS_BARS))
        assertFalse(hidesBothSystemBars(NAVIGATION_BARS or 8))
        assertFalse(hidesBothSystemBars(STATUS_BARS or NAVIGATION_BARS))
        assertFalse(hidesBothSystemBars(null))
    }

    @Test
    fun fr4_hiding_the_bars_on_a_portrait_feed_keeps_the_window() {
        // YouTube's Shorts feed hides both bars on every visit, in portrait or with no
        // orientation asked for: the window keeps its title bar and its size.
        for (orientation in listOf(TaskOrientation.UNSPECIFIED, TaskOrientation.PORTRAIT)) {
            assertFalse(
                asksForFullScreen(
                    fullscreenMode = false,
                    requestedVisibleTypes = barsHidden,
                    fullscreenTheme = false,
                    orientation = orientation,
                ),
            )
        }
    }

    @Test
    fun fr30_hiding_the_bars_while_asking_for_landscape_fills_the_screen() {
        // A player's full screen button on a phone: the bars go and the phone turns on its side.
        assertTrue(
            asksForFullScreen(
                fullscreenMode = false,
                requestedVisibleTypes = barsHidden,
                fullscreenTheme = false,
                orientation = TaskOrientation.LANDSCAPE,
            ),
        )
        // Landscape with its bars shown is an ordinary landscape window.
        assertFalse(
            asksForFullScreen(
                fullscreenMode = false,
                requestedVisibleTypes = barsShown,
                fullscreenTheme = false,
                orientation = TaskOrientation.LANDSCAPE,
            ),
        )
    }

    @Test
    fun fr30_a_game_whose_theme_or_windowing_mode_asks_fills_the_screen_in_any_orientation() {
        for (orientation in TaskOrientation.values()) {
            assertTrue(asksForFullScreen(false, barsHidden, fullscreenTheme = true, orientation = orientation))
            assertTrue(asksForFullScreen(true, barsHidden, fullscreenTheme = false, orientation = orientation))
        }
    }

    @Test
    fun fr30_an_orientation_asked_for_at_run_time_counts_until_another_activity_is_on_top() {
        val orientations = RequestedOrientations()
        val player = "com.example/.Player"
        // Nothing asked at run time: the manifest's orientation.
        assertEquals(TaskOrientation.PORTRAIT, orientations.of(5, player, TaskOrientation.PORTRAIT))

        // The full screen button asks for sensor landscape.
        orientations.requested(5, 6)
        assertEquals(TaskOrientation.LANDSCAPE, orientations.of(5, player, TaskOrientation.UNSPECIFIED))
        assertEquals(TaskOrientation.LANDSCAPE, orientations.of(5, player, TaskOrientation.UNSPECIFIED))
        // Leaving full screen asks for no orientation again, over the manifest's.
        orientations.requested(5, -1)
        assertEquals(TaskOrientation.UNSPECIFIED, orientations.of(5, player, TaskOrientation.PORTRAIT))

        // Another activity on top asks for its own manifest's orientation.
        orientations.requested(5, 0)
        assertEquals(TaskOrientation.LANDSCAPE, orientations.of(5, player, TaskOrientation.UNSPECIFIED))
        assertEquals(TaskOrientation.PORTRAIT, orientations.of(5, "com.example/.Comments", TaskOrientation.PORTRAIT))
        assertEquals(TaskOrientation.UNSPECIFIED, orientations.of(5, player, TaskOrientation.UNSPECIFIED))

        // An ended task's request is forgotten.
        orientations.requested(9, 0)
        orientations.retain(listOf(5))
        assertEquals(TaskOrientation.PORTRAIT, orientations.of(9, player, TaskOrientation.PORTRAIT))
    }
}
