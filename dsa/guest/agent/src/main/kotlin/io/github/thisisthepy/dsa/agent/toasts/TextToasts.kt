package io.github.thisisthepy.dsa.agent.toasts

import io.github.thisisthepy.dsa.agent.Log
import io.github.thisisthepy.dsa.agent.Scheduler
import io.github.thisisthepy.dsa.agent.tasks.TaskBounds
import io.github.thisisthepy.dsa.agent.tasks.TaskKind
import io.github.thisisthepy.dsa.agent.tasks.TaskSnapshot

/**
 * One text toast, as the notification manager hands it to the status bar
 * (IStatusBar.showToast). The binders are opaque here: [token] names the toast, [windowToken]
 * is the window token the notification manager made for its window, and [callback] is the
 * app's ITransientNotificationCallback, null when the app listens to nothing.
 */
data class ToastRequest(
    val uid: Int,
    val packageName: String,
    val token: Any?,
    val text: CharSequence,
    val windowToken: Any?,
    /** Toast.LENGTH_SHORT (0) or Toast.LENGTH_LONG (1). */
    val duration: Int,
    val callback: Any?,
    val displayId: Int,
)

/** The status bar's two toast calls, as they arrive on a binder thread. */
interface ToastCalls {
    fun show(request: ToastRequest)

    fun hide(packageName: String, token: Any?)
}

/** A toast that is drawn. */
interface ShownToast {
    /** Puts the toast where it belongs in [task], whose bounds changed. */
    fun place(task: TaskSnapshot)

    /** Removes the toast, tells its app and gives its token back to the notification manager. */
    fun hide()
}

/** Draws toasts: the framework side. */
interface ToastWindows {
    /**
     * Draws [request] inside [task]'s bounds and tells its app it is shown. Null when it cannot
     * be drawn.
     */
    fun show(request: ToastRequest, task: TaskSnapshot): ShownToast?
}

/**
 * The text toasts of the guest's apps, each drawn inside a task's window.
 *
 * Since Android 11 the text toast of an app that targets Android 11 or later is not drawn by
 * the app: the notification manager queues it and asks the status bar, which on a phone is
 * SystemUI, to show it and later to hide it. The guest has no SystemUI, because the host is the
 * shell, and the agent is the registered status bar, so the two calls come here.
 *
 * A toast belongs in the window of the app that showed it ([toastTask]), where Android would
 * put it on that app's screen. The notification manager keeps the queue and the durations: it
 * shows one toast at a time and asks for each to be hidden when its time is up, so nothing here
 * times a toast. As SystemUI does, a new toast first hides the one still showing, and a request
 * to hide is followed only for the toast that is showing.
 *
 * While a toast shows, its task is read every [FOLLOW_MILLIS]: a task that was resized or
 * moved on the display takes its toast with it, since a toast that straddles its task's edge is
 * in no host window, and a task that ended takes its toast away.
 *
 * Not thread safe: every method runs on [scheduler]'s thread.
 */
class TextToasts(
    private val windows: ToastWindows,
    private val tasks: () -> List<TaskSnapshot>,
    private val scheduler: Scheduler,
    private val log: Log,
    private val ownPackage: String,
) {
    private class Current(val request: ToastRequest, var task: TaskSnapshot, val window: ShownToast, val number: Long)

    private var current: Current? = null
    private var shown = 0L

    fun show(request: ToastRequest) {
        current?.let(::end)
        val task = toastTask(request.packageName, tasks(), ownPackage)
        if (task == null) {
            // The notification manager drops it from its queue when its time is up.
            log.info("no app window is open for the toast of ${request.packageName}; it is not drawn")
            return
        }
        val window = try {
            windows.show(request, task)
        } catch (e: RuntimeException) {
            log.warn("could not draw the toast of ${request.packageName}", e)
            null
        } ?: return
        val now = Current(request, task, window, ++shown)
        current = now
        log.info("the toast of ${request.packageName} is shown in task ${task.taskId}")
        scheduler.postDelayed(FOLLOW_MILLIS) { follow(now.number, FOLLOW_MILLIS) }
    }

    fun hide(packageName: String, token: Any?) {
        val showing = current ?: return
        // A toast that was not drawn, or one a newer toast already replaced.
        if (showing.request.packageName != packageName || showing.request.token != token) return
        end(showing)
    }

    private fun end(toast: Current) {
        current = null
        try {
            toast.window.hide()
        } catch (e: RuntimeException) {
            log.warn("could not hide the toast of ${toast.request.packageName}", e)
        }
    }

    /** Keeps toast [number] inside its task for as long as it shows. */
    private fun follow(number: Long, elapsed: Long) {
        val showing = current?.takeIf { it.number == number } ?: return
        if (elapsed >= MAX_SHOWN_MILLIS) {
            // The notification manager never asked for it to be hidden.
            log.warn("the toast of ${showing.request.packageName} was never hidden; hiding it")
            end(showing)
            return
        }
        val task = tasks().firstOrNull { it.taskId == showing.task.taskId && holdsToasts(it, ownPackage) }
        if (task == null) {
            end(showing)
            return
        }
        if (task.bounds != showing.task.bounds) {
            showing.task = task
            try {
                showing.window.place(task)
            } catch (e: RuntimeException) {
                log.warn("could not move the toast of ${showing.request.packageName} with its window", e)
            }
        }
        scheduler.postDelayed(FOLLOW_MILLIS) { follow(number, elapsed + FOLLOW_MILLIS) }
    }

    companion object {
        /** How often a shown toast's task is read. */
        const val FOLLOW_MILLIS = 100L

        /**
         * Longer than the notification manager shows any toast, with the longest accessibility
         * timeout a user can set (two minutes): a toast still showing then is hidden, so its
         * task is not read for ever.
         */
        const val MAX_SHOWN_MILLIS = 5 * 60_000L
    }
}

/** Whether [task] has a host window a toast can be drawn in. */
private fun holdsToasts(task: TaskSnapshot, ownPackage: String): Boolean =
    task.kind == TaskKind.APP && task.packageName != null && task.packageName != ownPackage &&
        !task.bounds.isEmpty() && !task.pictureInPicture

/**
 * The task whose window shows a toast of [packageName], among [tasks] (every running task,
 * most recent first); null when no app window is open.
 *
 * The app's own task: the focused one, else its most recent. A minimized window still counts,
 * because what an app shows in its window stays with that window. An app with no task (a
 * background service) gets the focused app task, else the most recent one, as on a phone its
 * toast shows over the app in front. The agent's own tasks and home have no host window and
 * are never chosen.
 */
fun toastTask(packageName: String, tasks: List<TaskSnapshot>, ownPackage: String): TaskSnapshot? {
    val open = tasks.filter { holdsToasts(it, ownPackage) }
    val own = open.filter { it.packageName == packageName }
    return own.firstOrNull { it.focused } ?: own.firstOrNull()
        ?: open.firstOrNull { it.focused } ?: open.firstOrNull()
}

/**
 * Where the framework puts a toast in its container: Toast's gravity, already resolved for the
 * layout direction (Gravity.getAbsoluteGravity), its offsets in pixels, and its margins as
 * fractions of the container.
 */
data class ToastPlacement(
    val gravity: Int,
    val xOffset: Int,
    val yOffset: Int,
    val horizontalMargin: Float = 0f,
    val verticalMargin: Float = 0f,
)

/** Whether a toast of [gravity] is as wide as its container (Gravity.FILL_HORIZONTAL). */
fun fillsWidth(gravity: Int): Boolean = gravity and FILL_HORIZONTAL == FILL_HORIZONTAL

/** Whether a toast of [gravity] is as high as its container (Gravity.FILL_VERTICAL). */
fun fillsHeight(gravity: Int): Boolean = gravity and FILL_VERTICAL == FILL_VERTICAL

/**
 * The frame of a toast of [width] by [height] pixels in a task window at [window], in display
 * pixels: where the window manager's layout would put it on a display that is the window
 * (Gravity.apply with the offsets and the margins, as WindowLayout does), then moved, and cut
 * down if it must be, to lie wholly inside the window. The hwcomposer gives a task only the
 * layers its bounds contain, so a toast that crossed the window's edge would be in no window.
 */
fun toastFrame(window: TaskBounds, width: Int, height: Int, place: ToastPlacement): TaskBounds {
    val windowWidth = window.right - window.left
    val windowHeight = window.bottom - window.top
    val x = place.xOffset + (place.horizontalMargin * windowWidth).toInt()
    val y = place.yOffset + (place.verticalMargin * windowHeight).toInt()
    val (left, right) = fit(along(place.gravity and PULL, window.left, window.right, width, x), window.left, window.right)
    val (top, bottom) = fit(along((place.gravity shr Y_SHIFT) and PULL, window.top, window.bottom, height, y), window.top, window.bottom)
    return TaskBounds(left, top, right, bottom)
}

/**
 * Gravity.apply along one axis: the span of [size] in [start] to [end], pushed by [offset].
 * [pull] is the axis's pull bits: towards the start, towards the end, both (fill) or neither
 * (centered).
 */
private fun along(pull: Int, start: Int, end: Int, size: Int, offset: Int): Pair<Int, Int> =
    when (pull) {
        PULL_BEFORE -> Pair(start + offset, start + offset + size)
        PULL_AFTER -> Pair(end - offset - size, end - offset)
        PULL -> Pair(start + offset, end + offset)
        else -> (start + (end - start - size) / 2 + offset).let { Pair(it, it + size) }
    }

/** [span] moved into [start] to [end], and cut to it when it is longer. */
private fun fit(span: Pair<Int, Int>, start: Int, end: Int): Pair<Int, Int> {
    val (from, to) = span
    return when {
        to - from >= end - start -> Pair(start, end)
        from < start -> Pair(start, start + (to - from))
        to > end -> Pair(end - (to - from), end)
        else -> span
    }
}

/**
 * Whether Android shows the app's icon in a text toast of an app that targets [targetSdk]:
 * SystemUI shows it for an app that targets Android 12 or later, and for a system app only
 * when it has a launcher activity ([launchable]).
 */
fun toastShowsIcon(targetSdk: Int, systemApp: Boolean, launchable: Boolean): Boolean =
    targetSdk >= ANDROID_12 && (!systemApp || launchable)

/**
 * Whether a text toast of an app that targets [targetSdk] is kept to two lines: Android limits
 * the toasts of apps that target Android 12 or later, and shows an older app's in full.
 */
fun toastKeepsTwoLines(targetSdk: Int): Boolean = targetSdk >= ANDROID_12

// Build.VERSION_CODES.S.
private const val ANDROID_12 = 31

// Gravity's values, which are part of the SDK and never change: each axis has a bit that pulls
// to its start and one that pulls to its end, the vertical axis four bits above the horizontal.
private const val PULL_BEFORE = 2
private const val PULL_AFTER = 4
private const val PULL = PULL_BEFORE or PULL_AFTER
private const val Y_SHIFT = 4
private const val FILL_HORIZONTAL = 7
private const val FILL_VERTICAL = 112
