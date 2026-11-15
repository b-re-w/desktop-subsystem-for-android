package io.github.thisisthepy.dsa.agent.windowing

import io.github.thisisthepy.dsa.agent.RecordingLog
import io.github.thisisthepy.dsa.agent.memory.AppProcess
import io.github.thisisthepy.dsa.agent.memory.FakeBackgroundProcesses
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

/**
 * An app the host starts on a monitor draws at that monitor's density in its process too. The
 * owner's PC (dev build 0.2.4-dev.23, a 200% main monitor at 262 dpi and a 125% one at 164 dpi):
 * YouTube opened on the 125% monitor in a process that had started at 262 dpi left its watch
 * page's comments and details blank and its mini player green, as it did when it was moved
 * between the monitors.
 */
class LaunchDensitiesTest {
    private val processes = FakeBackgroundProcesses()
    private val log = RecordingLog()
    private var display = 262
    private val densities = LaunchDensities(processes, { display }, log)

    private fun youtube(pid: Int = 4210, state: Int = CACHED, hasActivities: Boolean = false) =
        AppProcess(pid, 10_143, YOUTUBE, listOf(YOUTUBE), state = state, hasActivities = hasActivities)

    @Test
    fun fr4_an_app_whose_background_process_started_at_the_display_density_gets_a_new_process_on_another_monitor() {
        // A job started YouTube's process in the background: it has the display's 262 dpi.
        processes.processes += youtube(state = SERVICE)

        assertTrue(densities.beforeLaunch(YOUTUBE, 164))

        assertEquals(listOf(YOUTUBE), processes.ended)
        assertTrue(processes.processes.isEmpty())
        assertTrue(log.lines.toString(), log.lines.any { "ended the background processes of $YOUTUBE" in it && "started at 262 dpi" in it })
    }

    @Test
    fun fr4_an_app_whose_process_has_the_launch_density_keeps_it() {
        processes.processes += youtube()

        assertFalse(densities.beforeLaunch(YOUTUBE, 262))
        assertTrue(processes.ended.isEmpty())
    }

    @Test
    fun fr4_a_cached_process_keeps_the_density_its_last_task_had() {
        // YouTube's window was on the 125% monitor; it closed and its process stayed cached.
        processes.processes += youtube()
        densities.tasks(mapOf(YOUTUBE to 164))
        densities.tasks(emptyMap())

        assertFalse("opened on the same monitor again, it keeps its process", densities.beforeLaunch(YOUTUBE, 164))
        assertTrue("opened on the 200% monitor, it gets a new one", densities.beforeLaunch(YOUTUBE, 262))
        assertEquals(listOf(YOUTUBE), processes.ended)
    }

    @Test
    fun fr4_a_new_process_of_the_app_is_not_taken_for_the_one_before() {
        processes.processes += youtube(pid = 4210)
        densities.tasks(mapOf(YOUTUBE to 164))
        densities.tasks(emptyMap())
        // The process went, and a job started another one, at the display's density.
        processes.processes.clear()
        processes.processes += youtube(pid = 5120, state = SERVICE)

        assertTrue(densities.beforeLaunch(YOUTUBE, 164))
    }

    @Test
    fun fr4_an_app_with_a_task_or_in_use_keeps_its_process() {
        processes.processes += youtube()
        densities.tasks(mapOf(YOUTUBE to 262))
        assertFalse("its window is open: the start brings it forward", densities.beforeLaunch(YOUTUBE, 164))

        densities.tasks(emptyMap())
        processes.processes[0] = youtube(hasActivities = true)
        assertFalse("it holds an activity", densities.beforeLaunch(YOUTUBE, 164))

        processes.processes[0] = youtube(state = TOP)
        assertFalse("the user sees it", densities.beforeLaunch(YOUTUBE, 164))

        processes.processes[0] = youtube(state = SERVICE)
        processes.foreground += YOUTUBE
        assertFalse("it plays in the background", densities.beforeLaunch(YOUTUBE, 164))
        assertTrue(processes.ended.isEmpty())
        assertTrue(log.lines.toString(), log.lines.any { "foreground service" in it })
    }

    @Test
    fun fr4_an_app_whose_end_would_reach_another_apps_process_keeps_its_process() {
        processes.processes += youtube()
        val host = AppProcess(4300, 10_200, "org.example.host", listOf("org.example.host"), state = SERVICE)
        processes.processes += host
        processes.dependencies[host.pid] = setOf(YOUTUBE)

        assertFalse(densities.beforeLaunch(YOUTUBE, 164))
        assertTrue(processes.ended.isEmpty())
        assertTrue(log.lines.toString(), log.lines.any { "org.example.host" in it })
    }

    @Test
    fun fr4_nothing_is_ended_without_a_monitor_a_known_density_or_a_process() {
        assertFalse("no process: it starts at the new density anyway", densities.beforeLaunch(YOUTUBE, 164))
        processes.processes += youtube()
        assertFalse("the host named no monitor", densities.beforeLaunch(YOUTUBE, null))
        display = 0
        assertFalse("the display's density is not known", densities.beforeLaunch(YOUTUBE, 164))
        processes.readable = false
        display = 262
        assertFalse("the process records cannot be read", densities.beforeLaunch(YOUTUBE, 164))
        assertTrue(processes.ended.isEmpty())
    }

    private companion object {
        const val YOUTUBE = "com.google.android.youtube"
        const val TOP = 2
        const val SERVICE = 10
        const val CACHED = 16
    }
}
