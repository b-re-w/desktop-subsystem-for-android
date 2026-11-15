package io.github.thisisthepy.dsa.agent.windowing

/**
 * Which tasks start below the title bar of their host window.
 *
 * A host window draws its title bar over the top of its task, and the task's app draws under
 * it, as under a phone's status bar ([CaptionInsets]). Some apps handle that top inset badly:
 * they give it as padding to a bar of a fixed height, which is then squeezed or hidden under
 * the title bar. The user can set such an app to start below the title bar instead, and the
 * host says which apps those are ([hostSays]). A task of one of them gets no status bar inset,
 * and a caption bar inset the framework consumes for the app, so the app's content begins
 * under the band and the app has no top inset to handle, as on Android's own desktop windows.
 *
 * The choice is settled when the agent first sees a task ([tasks]) and stays for as long as
 * the task lives. Many apps read their insets once, when a screen is created, so an inset
 * that changed kind under a running task would leave that screen with a doubled gap or a bar
 * under the caption. A change of the user's setting therefore counts for the tasks that start
 * after it: the next time the app's window opens.
 *
 * Not thread safe: the agent calls it from one thread.
 */
class BelowTitleBar {
    private var packages: Set<String> = emptySet()

    /** Each running task, and whether it started below the title bar. */
    private val settled = HashMap<Int, Boolean>()

    /** The host says which apps start below the title bar. The list replaces the one before. */
    fun hostSays(packages: Collection<String>) {
        this.packages = packages.toSet()
    }

    /**
     * The tasks among [running] (each with the package of its app, null while unknown) that
     * start below the title bar. A task seen for the first time is settled by the list the
     * host last gave; a task whose package is not known yet waits until it is. Forgets the
     * tasks that ended.
     */
    fun tasks(running: Map<Int, String?>): Set<Int> {
        settled.keys.retainAll(running.keys)
        for ((taskId, packageName) in running) {
            if (taskId !in settled && packageName != null) settled[taskId] = packageName in packages
        }
        return settled.filterValues { it }.keys.toSet()
    }
}
