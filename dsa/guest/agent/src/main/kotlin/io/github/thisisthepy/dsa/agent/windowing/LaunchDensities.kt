package io.github.thisisthepy.dsa.agent.windowing

import io.github.thisisthepy.dsa.agent.Log
import io.github.thisisthepy.dsa.agent.memory.AppProcess
import io.github.thisisthepy.dsa.agent.memory.BackgroundProcesses
import io.github.thisisthepy.dsa.agent.memory.ProcessRecord

/**
 * Makes an app that the host starts on a monitor draw at that monitor's density in its process
 * too, not only in its new task.
 *
 * The arranger starts a new task at the density of its window's monitor ([TaskArranger]). An app
 * whose process is already running (it stayed cached after its window closed, or a job or a
 * broadcast started it in the background) keeps what it worked out at the density its process
 * started at: Android binds an app's process with the configuration of the activity it starts
 * for, or with the display's when it starts for no activity, and an app that reads its
 * resources once in its process keeps those values for good. YouTube does: started at 262 dpi
 * (the 200% main monitor) and opened on a 125% monitor at 164 dpi, its watch page left the
 * comments and the video's details blank below the player, and its mini player stayed a green
 * box. Only a new process drew it right.
 *
 * So before the host's request starts an app at a density ([beforeLaunch]), the app's processes
 * that run in the background with no task are looked at. When one of them started at another
 * density, the app's background processes are ended, and the start that follows makes a new
 * process at the new density. Ending them is what the activity manager does to a cached process
 * when memory runs short; the app's jobs, alarms and notifications stay.
 *
 * A process's density is the one its app's tasks had when the last of them went ([tasks]), and
 * the display's ([displayDensity]) for a process this never saw with a task, since nothing but
 * the display gave it one. Nothing is ended when:
 * - the app has a task: its process draws a window now, and the start brings that task forward;
 * - one of its processes holds an activity, or the app runs a foreground service (music, a
 *   download the user sees);
 * - ending the app's background processes would reach a process of another app (one that loaded
 *   its code) or a persistent one;
 * - the density a process started at is not known.
 *
 * Not thread safe: the agent calls it from its one thread.
 */
class LaunchDensities(
    private val processes: BackgroundProcesses,
    /** The density of the guest display, which a process started for no activity has; 0 if unknown. */
    private val displayDensity: () -> Int,
    private val log: Log,
) {
    /** The density of each app's tasks as last read, for the apps that have a task. */
    private var withTasks: Map<String, Int> = emptyMap()

    /** The density each process (by pid) had when its app's last task went. */
    private val started = HashMap<Int, Int>()

    /**
     * The tasks as the agent read them: the density each app's tasks draw at, by package. When
     * an app's last task goes, its running processes are noted at that density.
     */
    fun tasks(densities: Map<String, Int>) {
        val gone = withTasks.filterKeys { it !in densities }
        withTasks = densities.toMap()
        if (gone.isEmpty()) return
        val running = try {
            processes.running()
        } catch (e: Exception) {
            log.warn("could not list the app processes; an app opened on another monitor may keep its old density", e)
            return
        }
        started.keys.retainAll(running.map { it.pid }.toSet())
        for ((packageName, densityDpi) in gone) {
            if (densityDpi <= 0) continue
            for (process in running) {
                if (packageName in process.packages) started[process.pid] = densityDpi
            }
        }
    }

    /**
     * The host is about to start [packageName], whose new task draws at [densityDpi] (null when
     * the host named no monitor). Ends the app's background processes first when they started at
     * another density, and says so in the log. Returns whether it ended them.
     */
    fun beforeLaunch(packageName: String, densityDpi: Int?): Boolean {
        if (densityDpi == null || densityDpi <= 0 || packageName in withTasks) return false
        val running = try {
            processes.running()
        } catch (e: Exception) {
            log.warn("could not list the app processes; $packageName starts in the process it has", e)
            return false
        }
        started.keys.retainAll(running.map { it.pid }.toSet())
        val own = running.filter { packageName in it.packages }
        if (own.isEmpty()) return false
        val display = displayDensity()
        val at = own.associate { it.pid to (started[it.pid] ?: display) }
        val stale = own.filter { at.getValue(it.pid) != densityDpi }
        if (stale.isEmpty()) return false
        val kept = keepReason(packageName, own, at)
        if (kept != null) {
            log.info("$packageName starts at $densityDpi dpi in the process it has, ${describe(stale, at)}: $kept")
            return false
        }
        try {
            processes.endBackground(packageName)
        } catch (e: Exception) {
            log.warn("could not end the background processes of $packageName; it starts in the process it has", e)
            return false
        }
        for (process in own) started.remove(process.pid)
        log.info(
            "ended the background processes of $packageName (${describe(stale, at)}) so that it starts at " +
                "$densityDpi dpi in a new process: an app keeps what it worked out at its process's first density",
        )
        return true
    }

    /** Why the app's processes stay, or null when its background processes may be ended. */
    private fun keepReason(packageName: String, own: List<AppProcess>, at: Map<Int, Int>): String? {
        if (own.any { at.getValue(it.pid) <= 0 }) return "the density its process started at is not known"
        if (own.any { it.persistent }) return "its process is persistent"
        if (own.any { it.hasActivities }) return "its process holds an activity"
        if (own.any { it.state < STATE_SERVICE }) return "its process is in use"
        val foreground = try {
            processes.foregroundServicePackages()
        } catch (e: Exception) {
            return "which apps run a foreground service could not be read"
        }
        if (packageName in foreground) return "it runs a foreground service"
        val records = try {
            processes.records()
        } catch (e: Exception) {
            null
        } ?: return "the activity manager's process records could not be read"
        val mine = own.map { it.pid }.toSet()
        val appIds = own.map { appId(it.uid) }.toSet()
        val other = reach(packageName, appIds, records).firstOrNull { it.pid !in mine }
        if (other != null) return "ending them would also end ${other.name} (pid ${other.pid})"
        return null
    }

    /**
     * The processes that ending [packageName]'s background processes could reach, as
     * [io.github.thisisthepy.dsa.agent.memory.CachedProcesses] counts them: those that run it
     * in its app's uid or loaded its code, not persistent, scoring [REACH_ADJ] or more (or with
     * no score).
     */
    private fun reach(packageName: String, appIds: Set<Int>, records: List<ProcessRecord>): List<ProcessRecord> =
        records.filter { record ->
            !record.persistent &&
                (record.setAdj == null || record.setAdj >= REACH_ADJ) &&
                ((packageName in record.packages && appId(record.uid) in appIds) || packageName in record.dependencies)
        }

    private fun describe(stale: List<AppProcess>, at: Map<Int, Int>): String =
        stale.joinToString(", ") { "${it.name} pid ${it.pid} started at ${at.getValue(it.pid)} dpi" }

    companion object {
        /** ActivityManager.PROCESS_STATE_SERVICE: from here on a process runs no activity the user sees. */
        const val STATE_SERVICE = 10

        /** The lowest score a process is counted as reached from, as the cached processes' rule counts it. */
        const val REACH_ADJ = 250

        private const val PER_USER_RANGE = 100_000

        private fun appId(uid: Int): Int = uid % PER_USER_RANGE
    }
}
