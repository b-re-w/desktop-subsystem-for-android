package io.github.thisisthepy.dsa.agent.tasks

/** A change of a task's windowing mode the agent makes, which the framework then carries out. */
sealed interface WindowingChange {
    val taskId: Int

    /** Back to the freeform windowing mode at [bounds], the bounds the task had there. */
    data class ToFreeform(override val taskId: Int, val bounds: TaskBounds) : WindowingChange

    /** To the fullscreen windowing mode the app asked for. */
    data class ToFullscreen(override val taskId: Int) : WindowingChange
}

/**
 * Keeps a task whose app leaves freeform for the fullscreen windowing mode (an activity's
 * requestFullscreenMode) inside its host window when the user's setting for the app says so
 * ([hostSays], from the host's SetTaskFullScreen).
 *
 * Such a task cannot be resized while it is in fullscreen, and its frames are the display's
 * size, so its window would change size. The agent puts it back in freeform at the bounds it
 * had there ([WindowingChange.ToFreeform]) and reports it at those bounds from the moment it
 * left, so the host never hears of another size. It is still reported as asking for full
 * screen, which keeps its title band away, until another activity comes to its top: the app
 * cannot tell the framework it is done, since it is in freeform again. When the host says the
 * whole screen after all, a task kept that way goes to fullscreen again
 * ([WindowingChange.ToFullscreen]).
 */
class FullScreenHolds {
    /** Whether the host keeps each task's clear full screen request inside its window. */
    private val inWindow = HashMap<Int, Boolean>()

    /** The bounds each task last had in freeform. */
    private val freeformBounds = HashMap<Int, TaskBounds>()

    /** The tasks kept in freeform, with the top activity they asked from. */
    private val held = HashMap<Int, String?>()

    /** The tasks a move back to freeform was asked for, while it has not happened. */
    private val moving = HashSet<Int>()

    /** The host says where the task's clear full screen request shows. */
    fun hostSays(taskId: Int, inWindow: Boolean) {
        this.inWindow[taskId] = inWindow
    }

    /** Whether the task is kept in freeform for its window now. */
    fun holds(taskId: Int): Boolean = taskId in held

    /**
     * The tasks as the host is to hear of them, and the changes of windowing mode to make.
     * Called with every read of the running tasks.
     */
    fun apply(tasks: List<TaskSnapshot>): Pair<List<TaskSnapshot>, List<WindowingChange>> {
        val present = tasks.map { it.taskId }.toSet()
        inWindow.keys.retainAll(present)
        freeformBounds.keys.retainAll(present)
        held.keys.retainAll(present)
        moving.retainAll(present)
        val changes = mutableListOf<WindowingChange>()
        val reported = tasks.map { task ->
            val id = task.taskId
            val keep = inWindow[id] == true
            val top = task.stack?.top
            if (task.leftFreeform) {
                val back = freeformBounds[id]
                if (!keep || back == null) {
                    held.remove(id)
                    moving.remove(id)
                    return@map task
                }
                if (moving.add(id)) changes += WindowingChange.ToFreeform(id, back)
                held[id] = top
                return@map task.copy(bounds = back, fullscreenRequest = FullScreenRequest.EXPLICIT)
            }
            moving.remove(id)
            if (!task.bounds.isEmpty()) freeformBounds[id] = task.bounds
            if (id !in held) return@map task
            when {
                // Another screen of the app is on top: the request was that activity's.
                held[id] != top -> {
                    held.remove(id)
                    task
                }
                !keep -> {
                    held.remove(id)
                    changes += WindowingChange.ToFullscreen(id)
                    task.copy(fullscreenRequest = FullScreenRequest.EXPLICIT)
                }
                else -> task.copy(fullscreenRequest = FullScreenRequest.EXPLICIT)
            }
        }
        return reported to changes
    }
}
