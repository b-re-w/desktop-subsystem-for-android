package io.github.thisisthepy.dsa.agent.tasks

/**
 * The orientation each task's top activity asked for while it runs (Activity's
 * setRequestedOrientation), which its manifest does not say.
 *
 * A video player's full screen button asks for landscape this way: YouTube's turns a phone on
 * its side when it hides the bars. The framework does not put the request in the task's
 * description (TaskInfo.topActivityInfo is the manifest's entry, whose screenOrientation
 * stays as declared); it tells task stack listeners
 * (onActivityRequestedOrientationChanged, with the task and the orientation) instead. Each
 * request is kept for the activity that was on top when the tasks were next read, and stops
 * counting once another activity is on top, which asks for its own manifest's orientation
 * until it asks for another.
 *
 * Thread safe: requests arrive on a binder thread while the agent reads the tasks on its own.
 */
class RequestedOrientations {
    private class Request(val orientation: TaskOrientation, var activity: String?)

    private val requests = HashMap<Int, Request>()

    /** The top activity of task [taskId] asked for [screenOrientation] (ActivityInfo.SCREEN_ORIENTATION_*). */
    @Synchronized
    fun requested(taskId: Int, screenOrientation: Int) {
        requests[taskId] = Request(TaskOrientation.of(screenOrientation), null)
    }

    /**
     * The orientation task [taskId], whose top activity is [topActivity] and whose manifest
     * asks for [declared], asks for now: the last request of that activity, else [declared].
     */
    @Synchronized
    fun of(taskId: Int, topActivity: String?, declared: TaskOrientation): TaskOrientation {
        val request = requests[taskId] ?: return declared
        if (request.activity == null) request.activity = topActivity
        if (request.activity != topActivity) {
            requests.remove(taskId)
            return declared
        }
        return request.orientation
    }

    /** Forgets the tasks that are not in [taskIds]. */
    @Synchronized
    fun retain(taskIds: Collection<Int>) {
        requests.keys.retainAll(taskIds.toSet())
    }
}
