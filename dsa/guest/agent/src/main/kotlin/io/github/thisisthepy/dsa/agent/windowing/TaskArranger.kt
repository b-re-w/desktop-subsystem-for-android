package io.github.thisisthepy.dsa.agent.windowing

import io.github.thisisthepy.dsa.agent.Log
import io.github.thisisthepy.dsa.agent.requests.ResizeOutcome
import io.github.thisisthepy.dsa.agent.requests.TaskActions
import io.github.thisisthepy.dsa.agent.tasks.ScreenDp
import io.github.thisisthepy.dsa.agent.tasks.TaskBounds
import io.github.thisisthepy.dsa.agent.tasks.TaskSnapshot

/**
 * The monitor the window of the next task the host starts opens on: apps draw at [densityDpi]
 * there, a window has a client area of at most [room] pixels, and the whole monitor is
 * [monitor] pixels.
 */
data class NewTaskPlacement(val densityDpi: Int, val room: Size, val monitor: Size)

/**
 * Keeps the guest's app tasks from overlapping, by moving them on the guest display, and
 * gives each task the size and density its host window asks for.
 *
 * A new task starts at the density of the monitor its window opens on, so that the app's first
 * frame is already right and Android never relaunches it for a new density right after it
 * opens. As soon as the agent sees a task it did not see before, it gives the task that density
 * and the size Android chose for it in density-independent pixels, at that density, in one
 * change: for an app whose process has to start, before its activity exists. The monitor is the
 * one the host named last ([placeLaunches]), when the host named it just before the task
 * appeared, which it does before every request that starts a task; otherwise (an app started
 * another app in a new task) it is the monitor of the task that had focus, whose window the new
 * one opens from. Until the task is seen at that density, it is reported at it
 * ([densityToReport]), so its window opens at its size 1:1.
 *
 * Android's freeform launch puts each new task at the same default place, so two apps
 * started one after the other cover each other exactly. Each time the agent reads the
 * tasks, the arranger plans where every app task has to be ([TaskPlacement.plan]) and asks
 * the window manager to move the ones that are in the wrong place.
 *
 * The host asks for a task's size when the user resizes its window ([resize]), and for its
 * density when the window moves to a monitor of another scale ([setDensity]). Only the
 * latest of each counts. The task keeps its top left corner, within the display, and takes
 * the new size, never less than the smallest size the framework gives it; if it then overlaps
 * another task, the usual plan moves whichever of the two is newer. A size and a density asked
 * for together are applied in one change. While a resize is under way, the next one waits for
 * it to land, so the window manager never works through a backlog of sizes the user has
 * already dragged past. It waits [RESIZE_PATIENCE_MILLIS] at most: a size that does not land
 * by then must not stop the task following the user's drag for the whole of [giveUpMillis].
 *
 * A size the host marks live is a step of a drag the user has not finished. A change of a
 * task's screen size in dp recreates its activities, which takes far longer than a frame, so
 * at the first live step the arranger takes the screen size the task's configuration has
 * then, and every step of the drag gives the task its new bounds while its configuration
 * keeps that screen size ([TaskActions.resizeTaskLive]); the app only lays itself out again.
 * The first size that is not live ends the drag, even when it is the size the task already
 * has: the task takes it and its configuration follows its bounds again
 * ([TaskActions.endLiveResize]), with the density the host last asked for, so the app is
 * reconfigured once, at its final size. The end of a drag waits its turn like any size, but
 * a live size that arrives after it never replaces it. A density that arrives during a drag
 * ends the hold as well, because a screen size held at the old density would be wrong at the
 * new one; the next live step holds the screen size the task has by then. A size that is not
 * live and comes outside a drag is applied as it always was ([TaskActions.resizeTask]).
 *
 * The window manager may apply a move or a resize later (it queues it behind a running
 * transition), and it does not always call the task stack listener. So the arranger reports
 * a task that is changing at the bounds it is going to; the agent reads the tasks again
 * every [checkIntervalMillis] while a change is under way, and [tick] counts the time. A
 * change that has not landed after [giveUpMillis] is given up: the task is reported where
 * it is, and the arranger does not try to move it again until its bounds change.
 *
 * Not thread safe: the agent calls it from one thread.
 */
class TaskArranger(
    private val actions: TaskActions,
    /** The guest display's size; may throw. */
    private val displaySize: () -> Size,
    private val log: Log,
    private val giveUpMillis: Long = 2000,
    /** Milliseconds on a monotonic clock. */
    private val nowMillis: () -> Long = { System.nanoTime() / 1_000_000 },
) {
    /** A new task being given the density of its window's monitor, until it is seen at it. */
    private class Placing(val densityDpi: Int, var millisLeft: Long)

    /** The monitor the host named last for the tasks it starts, and when. */
    private var launchPlacement: NewTaskPlacement? = null
    private var placedAt = 0L

    /** The tasks seen so far; null until the first [arrange], whose tasks are not new. */
    private var known: MutableSet<Int>? = null

    /** The density of the task that had focus at the last [arrange]. */
    private var parentDensity: Int? = null

    private val placing = HashMap<Int, Placing>()

    private class Pending(val target: TaskBounds, var millisLeft: Long, val resize: Boolean)

    /** What the host last asked of a task and the arranger has not yet asked the framework. */
    private class Wanted(
        var size: Size? = null,
        var densityDpi: Int? = null,
        /** Whether the latest size is a step of a drag. */
        var live: Boolean = false,
        /** Whether a size that ends the task's drag came in; a later live size keeps it. */
        var endsDrag: Boolean = false,
        /** Whether the latest density ends a move, so the task's activity is to be recreated at it. */
        var recreate: Boolean = false,
    )

    /** A task whose activity is to be recreated once it draws at [densityDpi]. */
    private class Recreate(val densityDpi: Int, var millisLeft: Long)

    private val pending = HashMap<Int, Pending>()
    private val wanted = HashMap<Int, Wanted>()

    /** Tasks given a density that ended a move, until they are seen at it. */
    private val recreating = HashMap<Int, Recreate>()

    /**
     * Tasks whose window the user is dragging, and the screen size their configuration keeps
     * meanwhile: null when the framework did not say what it was, and the task is then
     * resized plainly.
     */
    private val dragging = HashMap<Int, ScreenDp?>()

    /** The density the host last asked for each task, which the end of a drag keeps. */
    private val densities = HashMap<Int, Int>()

    /** Tasks the window manager would not move, and the bounds they stayed at. */
    private val stuck = HashMap<Int, TaskBounds>()
    private var warnedNoDisplay = false
    private var warnedNoDensity = false

    /** The display's size when the tasks were last arranged, if it could be read. */
    var display: Size? = null
        private set

    /**
     * The bounds the last [arrange] asked the framework to give each task it resized: the size
     * the host asked for, kept within the task's smallest size and the display.
     */
    val resizedTo: Map<Int, TaskBounds> get() = lastResized

    private val lastResized = HashMap<Int, TaskBounds>()

    /**
     * Whether a change has been asked for and not yet seen to land, or is waiting its turn, or
     * a task waits to be seen at a density whose activity is to be recreated at it.
     */
    val waiting: Boolean
        get() = pending.isNotEmpty() || wanted.isNotEmpty() || recreating.isNotEmpty() || placing.isNotEmpty()

    /**
     * The host named the monitor the window of the next task it starts opens on; null when it
     * names none, and new tasks keep Android's own density unless they open from another task.
     */
    fun placeLaunches(placement: NewTaskPlacement?) {
        launchPlacement = placement
        placedAt = nowMillis()
    }

    /**
     * The density to report for task [taskId], which Android says draws at [read]: the density
     * of its window's monitor while a new task is on its way to it, else [read].
     */
    fun densityToReport(taskId: Int, read: Int): Int = placing[taskId]?.densityDpi ?: read

    /**
     * How soon the agent reads the tasks again while [waiting]: every frame while the user
     * resizes a window, so the next size goes out as soon as the last one lands.
     */
    val checkIntervalMillis: Long
        get() = if (wanted.isNotEmpty() || pending.values.any { it.resize }) RESIZE_CHECK_MILLIS else MOVE_CHECK_MILLIS

    /**
     * The host asked for the task to be [size] pixels: [live] while the user is still
     * dragging the window's edge.
     */
    fun resize(taskId: Int, size: Size, live: Boolean = false) {
        val want = wanted.getOrPut(taskId) { Wanted() }
        want.size = size
        want.live = live
        if (!live && taskId in dragging) want.endsDrag = true
    }

    /**
     * The host asked for the task to draw at [densityDpi]: with [recreate], because its window's
     * move ended, so once the task is seen at that density its top activity is recreated
     * ([TaskActions.recreateActivities]). Android recreates an activity that does not say it
     * handles a change of density on its own, so only one that says it does is asked for. The
     * recreate waits for the density to land: an activity recreated before then would take
     * the density afterwards as a change it handles itself, the half done layout the recreate is
     * there to avoid.
     */
    fun setDensity(taskId: Int, densityDpi: Int, recreate: Boolean = false) {
        val want = wanted.getOrPut(taskId) { Wanted() }
        want.densityDpi = densityDpi
        want.recreate = recreate
    }

    /**
     * Applies the sizes the host asked for, moves the [apps] that overlap one another or lie
     * outside the display, and returns the bounds to report for each: where the task is, or
     * where it is going.
     */
    fun arrange(apps: List<TaskSnapshot>): Map<Int, TaskBounds> {
        lastResized.clear()
        val current = LinkedHashMap<Int, TaskBounds>()
        val screens = HashMap<Int, ScreenDp?>()
        val smallest = HashMap<Int, Size>()
        for (task in apps) {
            if (current.putIfAbsent(task.taskId, task.bounds) == null) {
                screens[task.taskId] = task.screenDp
                smallest[task.taskId] = Size(task.minWidth, task.minHeight)
            }
        }
        settle(current)
        wanted.keys.retainAll(current.keys)
        dragging.keys.retainAll(current.keys)
        densities.keys.retainAll(current.keys)
        recreateLanded(apps)
        placeNew(apps)

        val size = try {
            displaySize().also { warnedNoDisplay = false }
        } catch (e: Exception) {
            if (!warnedNoDisplay) log.warn("could not read the display's size; tasks are not moved apart", e)
            warnedNoDisplay = true
            null
        }
        display = size
        if (size != null) {
            val layout = LinkedHashMap(expected(current))
            val resized = LinkedHashMap<Int, Wanted>()
            for ((taskId, want) in wanted.entries.toList()) {
                // One resize at a time: the next waits for this one to land. What it asks
                // for stays in wanted meanwhile, so the end of a drag is never lost.
                val under = pending[taskId]
                if (under != null && under.resize && giveUpMillis - under.millisLeft < RESIZE_PATIENCE_MILLIS) continue
                val now = layout.getValue(taskId)
                want.size?.let { layout[taskId] = sized(now, it, size, smallest[taskId]) }
                resized[taskId] = want
                wanted.remove(taskId)
            }
            val fixed = (stuck.keys + pending.keys) - resized.keys
            val moves = TaskPlacement.plan(layout, size, fixed).associateBy { it.taskId }
            for ((taskId, want) in resized) {
                val target = moves[taskId]?.to ?: layout.getValue(taskId)
                startResize(taskId, current.getValue(taskId), target, want, screens[taskId])
            }
            for (move in moves.values) {
                if (move.taskId !in resized) start(move, size)
            }
        }
        return expected(current)
    }

    /** [millis] have passed while a change was under way. */
    fun tick(millis: Long) {
        for (change in pending.values) change.millisLeft -= millis
        for (recreate in recreating.values) recreate.millisLeft -= millis
        for (place in placing.values) place.millisLeft -= millis
    }

    /**
     * Gives each task in [apps] that was not there at the last [arrange] the density of the
     * monitor its window opens on, and its size at that density, unless it has them.
     */
    private fun placeNew(apps: List<TaskSnapshot>) {
        val ids = apps.map { it.taskId }.toSet()
        placing.entries.removeAll { (taskId, place) ->
            val task = apps.firstOrNull { it.taskId == taskId }
            when {
                task == null -> true
                task.densityDpi == place.densityDpi -> true
                place.millisLeft <= 0 -> {
                    log.warn("task $taskId did not take ${place.densityDpi} dpi when it started; it draws at ${task.densityDpi} dpi")
                    true
                }
                else -> false
            }
        }
        val seen = known
        if (seen == null) {
            // The tasks there when the agent starts are not new: their windows exist already.
            known = ids.toMutableSet()
            parentDensity = focusedDensity(apps)
            return
        }
        val hosts = launchPlacement?.takeIf { nowMillis() - placedAt <= LAUNCH_PLACEMENT_MILLIS }
        for (task in apps) {
            if (!seen.add(task.taskId)) continue
            val densityDpi = hosts?.densityDpi ?: parentDensity ?: continue
            place(task, densityDpi, hosts)
        }
        seen.retainAll(ids)
        focusedDensity(apps)?.let { parentDensity = it }
    }

    /** The density the task that has focus draws at, or is on its way to. */
    private fun focusedDensity(apps: List<TaskSnapshot>): Int? = apps.firstOrNull { it.focused }
        ?.let { densityToReport(it.taskId, densities[it.taskId] ?: it.densityDpi) }
        ?.takeIf { it > 0 }

    /**
     * Asks for [task] at [densityDpi] and at the size Android gave it in density-independent
     * pixels, rescaled to that density: within the room the host's window has on the monitor
     * ([monitor], when the host named it), or the whole monitor for an app that asks to fill the
     * screen. Applied by [arrange] in one change with the density.
     */
    private fun place(task: TaskSnapshot, densityDpi: Int, monitor: NewTaskPlacement?) {
        val from = task.densityDpi
        val now = Size(task.bounds.right - task.bounds.left, task.bounds.bottom - task.bounds.top)
        val size = when {
            task.bounds.isEmpty() -> null
            monitor != null && task.fullscreen -> monitor.monitor
            from > 0 && from != densityDpi -> rescale(now, from, densityDpi).let { if (monitor != null) fitWithin(it, monitor.room) else it }
            monitor != null -> fitWithin(now, monitor.room)
            else -> now
        }
        val newDensity = from != densityDpi
        if (!newDensity && (size == null || size == now)) return
        val want = wanted.getOrPut(task.taskId) { Wanted() }
        if (want.size == null && size != null && size != now) want.size = size
        if (newDensity && want.densityDpi == null) {
            want.densityDpi = densityDpi
            placing[task.taskId] = Placing(densityDpi, giveUpMillis)
        }
        log.info(
            "task ${task.taskId} (${task.packageName}) starts at $densityDpi dpi" +
                (size?.let { ", ${it.width}x${it.height}" } ?: "") + ", the density of the monitor its window opens on",
        )
    }

    /**
     * Recreates the top activity of each task in [apps] now seen at the density a move that
     * ended gave it, if that activity says it handles a change of density; Android recreated
     * any other already. A task not seen at it within [giveUpMillis] keeps what it has.
     */
    private fun recreateLanded(apps: List<TaskSnapshot>) {
        recreating.keys.retainAll(apps.map { it.taskId }.toSet())
        for (task in apps) {
            val recreate = recreating[task.taskId] ?: continue
            if (task.densityDpi != recreate.densityDpi) {
                if (recreate.millisLeft <= 0) {
                    log.warn(
                        "task ${task.taskId} did not take ${recreate.densityDpi} dpi; its activity is not recreated " +
                            "and draws at ${task.densityDpi} dpi",
                    )
                    recreating.remove(task.taskId)
                }
                continue
            }
            recreating.remove(task.taskId)
            val packageName = task.packageName
            if (!task.handlesDensityChange || packageName == null) continue
            val recreated = try {
                actions.recreateActivities(task.taskId, packageName)
            } catch (e: Exception) {
                log.warn("recreating the activity of task ${task.taskId} failed", e)
                false
            }
            if (recreated) {
                log.info("recreating the activity of task ${task.taskId} ($packageName) at ${task.densityDpi} dpi")
            } else {
                log.warn(
                    "could not recreate the activity of task ${task.taskId} ($packageName); " +
                        "it redraws at ${task.densityDpi} dpi on its own",
                )
            }
        }
    }

    private fun settle(current: Map<Int, TaskBounds>) {
        pending.keys.retainAll(current.keys)
        stuck.entries.removeAll { (taskId, bounds) -> current[taskId] != bounds }
        for ((taskId, change) in pending.entries.toList()) {
            val now = current.getValue(taskId)
            if (now == change.target) {
                pending.remove(taskId)
            } else if (change.millisLeft <= 0) {
                val what = if (change.resize) "take" else "move to"
                log.warn("task $taskId did not $what ${describe(change.target)}; it stays at ${describe(now)}")
                pending.remove(taskId)
                stuck[taskId] = now
            }
        }
    }

    private fun start(move: Move, display: Size) {
        if (move.overlap > 0) {
            log.warn(
                "the ${display.width}x${display.height} display has no free place for task ${move.taskId} " +
                    "(${move.to.right - move.to.left}x${move.to.bottom - move.to.top}); it goes to " +
                    "${describe(move.to)}, where it shares the fewest pixels (${move.overlap}) with other tasks",
            )
        }
        val asked = try {
            actions.moveTask(move.taskId, move.to)
        } catch (e: Exception) {
            log.warn("moving task ${move.taskId} failed", e)
            false
        }
        if (asked) {
            log.info("moving task ${move.taskId} from ${describe(move.from)} to ${describe(move.to)}")
            pending[move.taskId] = Pending(move.to, giveUpMillis, resize = false)
        } else {
            stuck[move.taskId] = move.from
        }
    }

    /**
     * Asks the framework for [to], the bounds [want] comes to, in the way the drag calls for:
     * a live step holds the screen size [screen] the task had when the drag began, the end of
     * a drag lets the configuration follow the bounds again, and anything else is a plain
     * resize.
     */
    private fun startResize(taskId: Int, from: TaskBounds, to: TaskBounds, want: Wanted, screen: ScreenDp?) {
        want.densityDpi?.let { densities[taskId] = it }
        val endsDrag = taskId in dragging && (want.endsDrag || !want.live || want.densityDpi != null)
        var densityDpi = want.densityDpi
        val outcome = try {
            when {
                endsDrag -> {
                    dragging.remove(taskId)
                    densityDpi = densities[taskId]
                    actions.endLiveResize(taskId, to, densityDpi)
                }
                want.live && want.densityDpi == null -> {
                    val held = if (taskId in dragging) dragging[taskId] else screen.also { dragging[taskId] = it }
                    if (held != null) actions.resizeTaskLive(taskId, to, held) else actions.resizeTask(taskId, to, null)
                }
                else -> actions.resizeTask(taskId, to, densityDpi)
            }
        } catch (e: Exception) {
            log.warn("resizing task $taskId failed", e)
            ResizeOutcome.REFUSED
        }
        // A new task the framework did not give its density to is reported at the one it has.
        if (outcome != ResizeOutcome.APPLIED) placing.remove(taskId)
        if (outcome == ResizeOutcome.REFUSED) {
            log.warn("the framework refused to give task $taskId ${describe(to)}; it stays at ${describe(from)}")
            stuck[taskId] = from
            return
        }
        if (outcome == ResizeOutcome.BOUNDS_ONLY && densityDpi != null && !warnedNoDensity) {
            log.warn("the framework cannot set a task's density; task $taskId draws at the display's density, not $densityDpi dpi")
            warnedNoDensity = true
        }
        val asked = want.densityDpi
        if (want.recreate && asked != null && outcome == ResizeOutcome.APPLIED) {
            recreating[taskId] = Recreate(asked, giveUpMillis)
        }
        stuck.remove(taskId)
        pending[taskId] = Pending(to, giveUpMillis, resize = true)
        lastResized[taskId] = to
    }

    private fun expected(current: Map<Int, TaskBounds>): Map<Int, TaskBounds> =
        current.mapValues { (taskId, bounds) -> pending[taskId]?.target ?: bounds }

    private fun describe(b: TaskBounds) = "[${b.left},${b.top}][${b.right},${b.bottom}]"

    companion object {
        const val MOVE_CHECK_MILLIS = 100L

        /** About one frame at 60 Hz. */
        const val RESIZE_CHECK_MILLIS = 16L

        /** The longest a task's next size waits for its last one to land. */
        const val RESIZE_PATIENCE_MILLIS = 250L

        /**
         * How long after the host names a monitor a new task is taken to be one the host
         * started: the host names it just before it asks for a task, and an app's process can
         * take seconds to start on a busy guest.
         */
        const val LAUNCH_PLACEMENT_MILLIS = 10_000L

        /** [size] drawn at density [from], at density [to]: the same size in dp. */
        fun rescale(size: Size, from: Int, to: Int): Size {
            if (from <= 0 || to <= 0 || from == to) return size
            val at = { length: Int -> ((length.toLong() * to + from / 2) / from).toInt().coerceAtLeast(1) }
            return Size(at(size.width), at(size.height))
        }

        /** [size], made smaller with the same shape until it fits [room]. */
        fun fitWithin(size: Size, room: Size): Size {
            if (size.width <= room.width && size.height <= room.height) return size
            val scale = minOf(room.width.toDouble() / size.width, room.height.toDouble() / size.height)
            return Size((size.width * scale).toInt().coerceAtLeast(1), (size.height * scale).toInt().coerceAtLeast(1))
        }

        /**
         * [now] given [size], keeping its top left corner unless that would put it past the
         * display's edge, never larger than the display and never smaller than [smallest],
         * the size the framework would give it anyway.
         */
        fun sized(now: TaskBounds, size: Size, display: Size, smallest: Size? = null): TaskBounds {
            val maxWidth = maxOf(1, display.width)
            val maxHeight = maxOf(1, display.height)
            val width = size.width.coerceIn((smallest?.width ?: 1).coerceIn(1, maxWidth), maxWidth)
            val height = size.height.coerceIn((smallest?.height ?: 1).coerceIn(1, maxHeight), maxHeight)
            val left = if (now.isEmpty()) 0 else now.left.coerceIn(0, maxOf(0, display.width - width))
            val top = if (now.isEmpty()) 0 else now.top.coerceIn(0, maxOf(0, display.height - height))
            return TaskBounds(left, top, left + width, top + height)
        }
    }
}
