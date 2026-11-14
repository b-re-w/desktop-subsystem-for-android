package io.github.thisisthepy.dsa.agent.toasts

import io.github.thisisthepy.dsa.agent.AGENT
import io.github.thisisthepy.dsa.agent.FakeScheduler
import io.github.thisisthepy.dsa.agent.RecordingLog
import io.github.thisisthepy.dsa.agent.appTask
import io.github.thisisthepy.dsa.agent.tasks.TaskBounds
import io.github.thisisthepy.dsa.agent.tasks.TaskKind
import io.github.thisisthepy.dsa.agent.tasks.TaskSnapshot
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class TextToastsTest {
    private val scheduler = FakeScheduler()
    private val log = RecordingLog()
    private var running = listOf<TaskSnapshot>()

    /** What the fake framework side was asked to do, in order. */
    private val events = mutableListOf<String>()

    private val windows = object : ToastWindows {
        var fails = false

        override fun show(request: ToastRequest, task: TaskSnapshot): ShownToast? {
            if (fails) return null
            events += "show ${request.text} in ${task.taskId} at ${task.bounds}"
            return object : ShownToast {
                override fun place(task: TaskSnapshot) {
                    events += "place ${request.text} at ${task.bounds}"
                }

                override fun hide() {
                    events += "hide ${request.text}"
                }
            }
        }
    }

    private val toasts = TextToasts(windows, { running }, scheduler, log, AGENT)

    private val mail = "org.example.mail"
    private val music = "org.example.music"

    private fun toast(packageName: String, text: String, token: Any = Any()) =
        ToastRequest(10_100, packageName, token, text, Any(), 0, null, 0)

    @Test
    fun fr40_a_text_toast_is_shown_in_its_apps_own_window() {
        val other = appTask(1, packageName = music, focused = true)
        val own = appTask(2, packageName = mail)
        running = listOf(other, own)
        toasts.show(toast(mail, "Sent"))
        assertEquals(listOf("show Sent in 2 at ${own.bounds}"), events)

        // Of an app's several windows: the focused one, else the most recent (the first).
        val recent = appTask(3, packageName = mail)
        val older = appTask(4, packageName = mail)
        assertEquals(3, toastTask(mail, listOf(other, recent, older), AGENT)?.taskId)
        assertEquals(4, toastTask(mail, listOf(other, recent, older.copy(focused = true)), AGENT)?.taskId)
    }

    @Test
    fun fr40_a_toast_of_an_app_with_no_window_goes_to_the_focused_window() {
        val recent = appTask(1, packageName = music)
        val focused = appTask(2, packageName = "org.example.maps", focused = true)
        assertEquals(2, toastTask(mail, listOf(recent, focused), AGENT)?.taskId)
        // No app task has the focus (the user is in a host program): the most recent one.
        assertEquals(1, toastTask(mail, listOf(recent, focused.copy(focused = false)), AGENT)?.taskId)

        // Home, the agent's own tasks, a task with no bounds yet and a picture-in-picture
        // window have no host window to draw in.
        val none = listOf(
            appTask(3, packageName = "com.android.launcher", kind = TaskKind.OTHER, focused = true),
            appTask(4, packageName = AGENT),
            appTask(5, packageName = mail, bounds = TaskBounds.EMPTY),
            appTask(6, packageName = mail).copy(pictureInPicture = true),
        )
        assertNull(toastTask(mail, none, AGENT))

        // With no window open nothing is drawn, and the later request to hide is not an error.
        running = none
        val request = toast(mail, "Synced")
        toasts.show(request)
        toasts.hide(mail, request.token)
        assertEquals(emptyList<String>(), events)
        assertTrue(log.lines.single().contains("no app window is open"))
    }

    @Test
    fun fr40_the_toast_lies_inside_the_window_at_the_frameworks_place() {
        val window = TaskBounds(100, 50, 500, 650)
        // Android's default: bottom, centered, the y offset above the bottom edge.
        val default = ToastPlacement(gravity = 81, xOffset = 0, yOffset = 48)
        assertEquals(TaskBounds(200, 562, 400, 602), toastFrame(window, 200, 40, default))
        // An odd remainder rounds down, as Gravity.apply does.
        assertEquals(TaskBounds(199, 562, 400, 602), toastFrame(window, 201, 40, default))

        // Left and top, right and bottom, centered both ways, and fill.
        assertEquals(TaskBounds(110, 70, 310, 110), toastFrame(window, 200, 40, ToastPlacement(3 or 48, 10, 20)))
        assertEquals(TaskBounds(290, 590, 490, 630), toastFrame(window, 200, 40, ToastPlacement(5 or 80, 10, 20)))
        assertEquals(TaskBounds(210, 350, 410, 390), toastFrame(window, 200, 40, ToastPlacement(17, 10, 20)))
        assertEquals(TaskBounds(100, 590, 500, 630), toastFrame(window, 400, 40, ToastPlacement(7 or 80, 0, 20)))
        assertTrue(fillsWidth(7 or 80))
        assertFalse(fillsWidth(81))
        assertTrue(fillsHeight(112 or 1))
        assertFalse(fillsHeight(81))

        // Margins are fractions of the window, added to the offsets.
        assertEquals(
            TaskBounds(240, 502, 440, 542),
            toastFrame(window, 200, 40, ToastPlacement(81, 0, 48, horizontalMargin = 0.1f, verticalMargin = 0.1f)),
        )

        // Never across the window's edge: pushed back inside, and cut to the window when larger.
        assertEquals(TaskBounds(300, 610, 500, 650), toastFrame(window, 200, 40, ToastPlacement(81, 900, -900)))
        assertEquals(TaskBounds(100, 50, 300, 90), toastFrame(window, 200, 40, ToastPlacement(81, -900, 900)))
        assertEquals(TaskBounds(100, 50, 500, 650), toastFrame(window, 900, 900, default))
    }

    @Test
    fun fr40_one_toast_at_a_time_and_only_the_current_one_is_hidden() {
        running = listOf(appTask(1, packageName = mail, focused = true))
        val first = toast(mail, "First")
        val second = toast(mail, "Second")
        toasts.show(first)
        // Another toast's token, and another app with the same token, hide nothing.
        toasts.hide(mail, second.token)
        toasts.hide(music, first.token)
        assertEquals(1, events.size)

        // A new toast hides the one still showing first, as SystemUI does.
        toasts.show(second)
        assertEquals("hide First", events[1])
        assertTrue(events[2].startsWith("show Second in 1"))
        // The late request to hide the first one does not hide the second.
        toasts.hide(mail, first.token)
        assertEquals(3, events.size)

        toasts.hide(mail, second.token)
        assertEquals("hide Second", events[3])
        // Hidden once: a second request finds nothing.
        toasts.hide(mail, second.token)
        assertEquals(4, events.size)
    }

    @Test
    fun fr40_a_toast_that_cannot_be_drawn_leaves_nothing_showing() {
        running = listOf(appTask(1, packageName = mail, focused = true))
        windows.fails = true
        val request = toast(mail, "Lost")
        toasts.show(request)
        toasts.hide(mail, request.token)
        scheduler.advance(1_000)
        assertEquals(emptyList<String>(), events)
    }

    @Test
    fun fr40_a_toast_follows_its_window_and_ends_with_it() {
        val task = appTask(1, packageName = mail, focused = true)
        running = listOf(task)
        val request = toast(mail, "Saved")
        toasts.show(request)
        scheduler.advance(1_000)
        // Nothing moved: nothing is placed again.
        assertEquals(1, events.size)

        val resized = task.copy(bounds = TaskBounds(10, 20, 810, 620))
        running = listOf(resized)
        scheduler.advance(TextToasts.FOLLOW_MILLIS)
        assertEquals("place Saved at ${resized.bounds}", events[1])
        scheduler.advance(1_000)
        assertEquals(2, events.size)

        // The task ended: its toast goes with it, and is hidden once.
        running = emptyList()
        scheduler.advance(TextToasts.FOLLOW_MILLIS)
        assertEquals("hide Saved", events[2])
        toasts.hide(mail, request.token)
        scheduler.advance(1_000)
        assertEquals(3, events.size)
    }

    @Test
    fun fr40_a_hidden_toast_is_followed_no_longer() {
        val task = appTask(1, packageName = mail, focused = true)
        running = listOf(task)
        val request = toast(mail, "Saved")
        toasts.show(request)
        toasts.hide(mail, request.token)
        running = listOf(task.copy(bounds = TaskBounds(0, 0, 300, 300)))
        scheduler.advance(1_000)
        assertEquals(listOf("show Saved in 1 at ${task.bounds}", "hide Saved"), events)

        // A toast nothing ever asks to hide does not keep its task read for ever.
        toasts.show(toast(mail, "Stuck"))
        scheduler.advance(TextToasts.MAX_SHOWN_MILLIS)
        assertEquals("hide Stuck", events.last())
    }

    @Test
    fun fr40_the_app_icon_shows_only_where_android_shows_it() {
        // An app that targets Android 12 or later, unless it is a system app with no launcher
        // activity.
        assertTrue(toastShowsIcon(targetSdk = 31, systemApp = false, launchable = true))
        assertTrue(toastShowsIcon(targetSdk = 36, systemApp = false, launchable = false))
        assertTrue(toastShowsIcon(targetSdk = 36, systemApp = true, launchable = true))
        assertFalse(toastShowsIcon(targetSdk = 36, systemApp = true, launchable = false))
        // An app that targets Android 11 has no icon and no limit of two lines.
        assertFalse(toastShowsIcon(targetSdk = 30, systemApp = false, launchable = true))
        assertFalse(toastKeepsTwoLines(30))
        assertTrue(toastKeepsTwoLines(31))
    }
}
