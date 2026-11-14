package io.github.thisisthepy.dsa.agent.tasks

/**
 * One report of the system bars' appearance on one guest display ([displayId]), as the window
 * manager gives it to the status bar (IStatusBar.onSystemBarAttributesChanged) each time it
 * changes. The window manager keeps each display's bars apart, so each display reports its
 * own.
 *
 * [appearance] holds the WindowInsetsController appearance flags of the window that controls
 * the bars: the display's focused window, which an app changes with
 * setSystemBarsAppearance, screen by screen and tab by tab. [regions] are the parts of the
 * status bar each app window under it calls for (AppearanceRegion), when the window manager
 * computes them. [packageName] is the package of the controlling window, or null when the
 * report did not say.
 */
data class BarsReport(
    val appearance: Int,
    val regions: List<AppearanceArea> = emptyList(),
    val packageName: String? = null,
    val displayId: Int = DEFAULT_DISPLAY,
)

/**
 * What the framework says of a task's bars apart from the reports: the display the task is on
 * ([displayId]), and the appearance flags its task description carries ([described]: those of
 * its top opaque activity, which Android updates each time that activity's window asks for
 * another appearance, focused or not; null when the framework does not say).
 */
data class TaskBars(val displayId: Int = DEFAULT_DISPLAY, val described: Int? = null)

/** Display.DEFAULT_DISPLAY: the guest's own display. */
const val DEFAULT_DISPLAY = 0

/**
 * A part of the status bar ([bounds], in guest display pixels) and the appearance flags the
 * window under it calls for there (APPEARANCE_LIGHT_STATUS_BARS when it wants dark icons).
 */
data class AppearanceArea(val appearance: Int, val bounds: TaskBounds)

/**
 * The live status bar appearance of each task, which the host's caption buttons follow as
 * Android's own status bar icons follow it.
 *
 * The window manager reports the live appearance to the status bar ([BarsReport]); the agent
 * is the status bar in a guest without SystemUI, and each report lands here ([report]). A
 * report is the appearance of the window that controls one display's bars, its focused
 * window, so it belongs to a task on that display: the task the report's package runs in,
 * the focused one first when several do ([apply]). A report whose package runs in no app task
 * there (a system window) is left out. A region that covers the top of a task decides for that
 * task instead.
 *
 * Reports alone miss every task whose window does not control the bars. A phone shows one app
 * at a time, but the guest display shows many tasks side by side, one window each on the
 * host, and only the focused one is reported: an app that changes its appearance while
 * another task has focus, or a task not focused since the agent started, would never be
 * heard of. So each task's description counts too ([TaskBars.described]): Android updates it
 * whenever the window of the task's top opaque activity asks for another appearance. It does
 * not tell every screen apart (a window that is not the activity's own, or a top activity that
 * leaves every flag clear over one that set some, which the description then takes from
 * below), so it counts only when it changes, and the latest word of either source decides. A
 * description with no flags, before it ever had any, is an app that has not drawn yet, not one
 * that asked for dark.
 *
 * A task's appearance is light while the app asks for dark icons on its status bar
 * (APPEARANCE_LIGHT_STATUS_BARS) or caption bar (APPEARANCE_LIGHT_CAPTION_BARS, which an app
 * on a desktop window may set apart; either one set makes the bar light). A task the window
 * manager reported with both clear is dark, whether or not it ever asked for light: on a phone
 * the status bar then draws light icons, which is what an app that leaves the flag clear is
 * designed for (FLO's player screen draws its own icons white and never sets the flag). Only a
 * task neither source has spoken of yet is unknown, and the host then reads the tone from its
 * pixels.
 *
 * Thread safe: the window manager reports on a binder thread, and the agent reads the tasks
 * on its own.
 */
class SystemBarAppearances {
    private class Seen {
        var light = false

        fun take(light: Boolean) {
            this.light = light
        }

        val appearance: StatusBarAppearance
            get() = if (light) StatusBarAppearance.LIGHT else StatusBarAppearance.DARK
    }

    private val pending = ArrayDeque<BarsReport>()
    private val seen = HashMap<Int, Seen>()

    /** What each task's description said when the tasks were last read. */
    private val described = HashMap<Int, Int>()

    /** A report from the window manager, applied at the next [apply]. */
    @Synchronized
    fun report(report: BarsReport) {
        if (pending.size >= MAX_PENDING) pending.removeFirst()
        pending.addLast(report)
    }

    /**
     * [tasks] as they are now, each with its live status bar appearance, after each task whose
     * description changed takes what it says, and the reports that came since the last call
     * are given to the tasks they belong to. [bars] says, by task ID, which display each task
     * is on and what its description says; a task it leaves out is on the default display and
     * its description says nothing. A task that is gone is forgotten.
     */
    @Synchronized
    fun apply(tasks: List<TaskSnapshot>, bars: Map<Int, TaskBars> = emptyMap()): List<TaskSnapshot> {
        val running = tasks.map { it.taskId }.toSet()
        seen.keys.retainAll(running)
        described.keys.retainAll(running)
        for (task in tasks) {
            if (task.kind == TaskKind.APP) describe(task.taskId, bars[task.taskId]?.described)
        }
        while (pending.isNotEmpty()) attribute(pending.removeFirst(), tasks, bars)
        return tasks.map { task ->
            val appearance = seen[task.taskId]?.appearance ?: StatusBarAppearance.UNKNOWN
            if (appearance == task.statusBar) task else task.copy(statusBar = appearance)
        }
    }

    /**
     * The task's description says [appearance] now (null: it says nothing). A change is the
     * app's latest word; no flags before it ever had any is an app that has not drawn yet.
     */
    private fun describe(taskId: Int, appearance: Int?) {
        if (appearance == null) return
        val last = described.put(taskId, appearance)
        if (last == appearance) return
        if (appearance == 0 && (last == null || last == 0)) return
        seen.getOrPut(taskId, ::Seen).take(appearance and LIGHT_BARS != 0)
    }

    private fun attribute(report: BarsReport, tasks: List<TaskSnapshot>, bars: Map<Int, TaskBars>) {
        // Each display's bars are its own: only the tasks on it share them.
        val apps = tasks.filter {
            it.kind == TaskKind.APP && (bars[it.taskId]?.displayId ?: DEFAULT_DISPLAY) == report.displayId
        }
        val covered = HashSet<Int>()
        for (task in apps) {
            if (task.bounds.isEmpty()) continue
            val x = (task.bounds.left + task.bounds.right) / 2
            val y = task.bounds.top
            val area = report.regions.firstOrNull { it.bounds.covers(x, y) } ?: continue
            seen.getOrPut(task.taskId, ::Seen).take(area.appearance and APPEARANCE_LIGHT_STATUS_BARS != 0)
            covered += task.taskId
        }
        val owner = controlling(report.packageName, apps) ?: return
        if (owner.taskId in covered) return
        seen.getOrPut(owner.taskId, ::Seen).take(report.appearance and LIGHT_BARS != 0)
    }

    /**
     * The task of the window that controls the bars: one the package runs in (its root or
     * its top activity's), the focused one first; the focused task when the report names no
     * package; none when the package runs in no app task, or in several of which none is
     * focused.
     */
    private fun controlling(packageName: String?, apps: List<TaskSnapshot>): TaskSnapshot? {
        if (packageName.isNullOrEmpty()) return apps.firstOrNull { it.focused }
        val runs = apps.filter { it.packageName == packageName || it.topPackage() == packageName }
        return runs.firstOrNull { it.focused } ?: runs.singleOrNull()
    }

    private fun TaskSnapshot.topPackage(): String? = stack?.top?.substringBefore('/')

    private fun TaskBounds.covers(x: Int, y: Int) = x >= left && x < right && y >= top && y < bottom

    companion object {
        /** WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS. */
        const val APPEARANCE_LIGHT_STATUS_BARS = 8

        /** WindowInsetsController.APPEARANCE_LIGHT_CAPTION_BARS. */
        const val APPEARANCE_LIGHT_CAPTION_BARS = 256

        private const val LIGHT_BARS = APPEARANCE_LIGHT_STATUS_BARS or APPEARANCE_LIGHT_CAPTION_BARS

        /** Reports kept between two reads of the tasks; older ones are outdated by then. */
        private const val MAX_PENDING = 64
    }
}
