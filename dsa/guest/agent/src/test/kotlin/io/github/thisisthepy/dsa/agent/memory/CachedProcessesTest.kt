package io.github.thisisthepy.dsa.agent.memory

import io.github.thisisthepy.dsa.agent.FakeScheduler
import io.github.thisisthepy.dsa.agent.RecordingLog
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

/**
 * The activity manager as far as ending cached processes needs it. Ending a package's
 * background processes does what the framework's killBackgroundProcesses does: it ends every
 * process that is not persistent, scores at least the service level, and runs the package in
 * its own app's uid or loaded its code.
 */
class FakeBackgroundProcesses : BackgroundProcesses {
    val processes = mutableListOf<AppProcess>()
    val dependencies = mutableMapOf<Int, Set<String>>()
    val scores = mutableMapOf<Int, Int>()
    val foreground = mutableSetOf<String>()
    val ended = mutableListOf<String>()
    var readable = true
    var playing: Set<String>? = emptySet()

    override fun playingPackages(): Set<String>? = playing

    fun score(process: AppProcess): Int = scores[process.pid] ?: when {
        process.persistent -> -800
        process.state >= CachedProcesses.STATE_CACHED_ACTIVITY -> 905
        process.state == 10 -> 500
        else -> 100
    }

    override fun running(): List<AppProcess> = processes.toList()

    override fun records(): List<ProcessRecord>? = if (!readable) {
        null
    } else {
        processes.map {
            ProcessRecord(it.pid, it.uid, it.name, it.packages.toSet(), dependencies[it.pid].orEmpty(), score(it), it.persistent)
        }
    }

    override fun memory(pids: List<Int>): List<ProcessPss> =
        pids.map { ProcessPss(pid = it, name = "", pssKb = 70L * 1024, swapPssKb = 42L * 1024) }

    override fun foregroundServicePackages(): Set<String> = foreground

    override fun endBackground(packageName: String) {
        ended += packageName
        val appIds = processes.filter { packageName in it.packages }.map { it.uid % 100_000 }.toSet()
        processes.removeAll { process ->
            !process.persistent && score(process) >= 500 &&
                ((packageName in process.packages && process.uid % 100_000 in appIds) ||
                    packageName in dependencies[process.pid].orEmpty())
        }
    }
}

class CachedProcessesTest {
    private val processes = FakeBackgroundProcesses()
    private val scheduler = FakeScheduler()
    private val log = RecordingLog()
    private var connected = true
    private var shown: Set<String> = emptySet()
    private val policy = CachedProcesses(
        processes,
        scheduler,
        log,
        clock = { scheduler.now },
        isConnected = { connected },
        ownPackage = AGENT,
        shownPackages = { shown },
    )

    private fun serving(pid: Int, uid: Int, name: String, vararg packages: String) =
        AppProcess(pid, uid, name, packages.toList(), state = CachedProcesses.STATE_SERVICE)

    /** Looks once now and once after the time a process may only run services. */
    private fun idleServing() {
        policy.check()
        scheduler.advance(CachedProcesses.SERVICE_IDLE_MILLIS)
        policy.check()
    }

    @Test
    fun nfr1_a_windowless_app_that_only_runs_a_service_is_ended_after_the_service_idle_time() {
        // What the owner's guest held after YouTube closed: APKPure with no window, its
        // started service keeping it out of the cached states, so the cached rule never saw it.
        processes.processes += serving(6454, 10117, "com.apkpure.aegon", "com.apkpure.aegon")
        policy.check()
        scheduler.advance(CachedProcesses.SERVICE_IDLE_MILLIS - 1)
        policy.check()
        assertTrue(processes.ended.isEmpty())
        scheduler.advance(1)
        policy.check()
        assertEquals(listOf("com.apkpure.aegon"), processes.ended)
        assertTrue(
            log.lines.toString(),
            log.lines.contains(
                "info: ended background com.apkpure.aegon (pid 6454, only services for 300 s, no task window, " +
                    "no foreground service, no playback): PSS 70 MiB, 28 MiB resident and 42 MiB swapped out to zram",
            ),
        )
    }

    @Test
    fun nfr1_a_service_app_that_plays_audio_shows_a_notification_or_has_a_widget_is_kept() {
        processes.processes += serving(300, 10150, "org.example.radio", "org.example.radio")
        processes.processes += serving(301, 10151, "org.example.mail", "org.example.mail")
        processes.processes += serving(302, 10152, "org.example.weather", "org.example.weather")
        processes.playing = setOf("org.example.radio")
        shown = setOf("org.example.mail", "org.example.weather")
        idleServing()
        assertTrue(processes.ended.isEmpty())
    }

    @Test
    fun nfr1_the_audio_effect_app_is_kept_while_anything_plays_and_ended_after() {
        processes.processes += serving(400, 10050, "com.android.musicfx", "com.android.musicfx")
        processes.processes += AppProcess(401, 10160, "com.spotify.music", listOf("com.spotify.music"), state = 4)
        processes.foreground += "com.spotify.music"
        processes.playing = setOf("com.spotify.music")
        idleServing()
        assertTrue(processes.ended.isEmpty())
        // The music stops: the effect app goes at its next look.
        processes.playing = emptySet()
        scheduler.advance(CachedProcesses.BLOCKED_RETRY_MILLIS)
        policy.check()
        assertEquals(listOf("com.android.musicfx"), processes.ended)
        assertEquals(setOf("com.spotify.music"), names())
    }

    @Test
    fun nfr1_a_service_process_is_kept_when_playback_cannot_be_read() {
        processes.processes += serving(500, 10170, "org.example.sync", "org.example.sync")
        processes.playing = null
        idleServing()
        scheduler.advance(CachedProcesses.BLOCKED_RETRY_MILLIS)
        policy.check()
        assertTrue(processes.ended.isEmpty())
        assertEquals(1, log.lines.count { it.startsWith("warn: could not read which apps play audio") })
    }

    @Test
    fun nfr1_a_system_persistent_foreground_or_bound_service_process_is_kept() {
        processes.processes += serving(600, 1000, "com.android.systemui.helper", "com.android.helper")
        processes.processes += serving(601, 10180, "org.example.keeper", "org.example.keeper").copy(persistent = true)
        processes.processes += serving(602, 10181, "org.example.upload", "org.example.upload")
        processes.foreground += "org.example.upload"
        // Bound by a perceptible client: its score is above the service level.
        val bound = serving(603, 10182, "org.example.provider", "org.example.provider")
        processes.processes += bound
        processes.scores[bound.pid] = 200
        processes.processes += serving(604, 10183, "org.example.notes:sync", "org.example.notes")
        policy.tasks(setOf("org.example.notes"))
        idleServing()
        assertTrue(processes.ended.toString(), processes.ended.isEmpty())
    }

    @Test
    fun nfr1_a_service_process_that_comes_back_waits_longer_each_time() {
        processes.processes += serving(700, 10190, "org.example.sticky", "org.example.sticky")
        idleServing()
        assertEquals(1, processes.ended.size)
        // The system restarts its sticky service a minute later.
        scheduler.advance(60_000)
        processes.processes += serving(701, 10190, "org.example.sticky", "org.example.sticky")
        idleServing()
        assertEquals(1, processes.ended.size)
        scheduler.advance(CachedProcesses.SERVICE_IDLE_MILLIS)
        policy.check()
        assertEquals(2, processes.ended.size)
    }

    private fun cachedEmpty(pid: Int, uid: Int, name: String, vararg packages: String) =
        AppProcess(pid, uid, name, packages.toList(), state = CACHED_EMPTY)

    /** Looks once now and once after the idle time. */
    private fun idle() {
        policy.check()
        scheduler.advance(CachedProcesses.IDLE_MILLIS)
        policy.check()
    }

    private fun names() = processes.processes.map { it.name }.toSet()

    @Test
    fun nfr2_cached_play_processes_end_after_their_work_and_the_core_stays() {
        // What the owner's guest held: FLO shown, APKPure in the background, Play's one-off
        // processes and Settings cached, Google Play services' core bound by the system.
        processes.processes += listOf(
            AppProcess(742, 1000, "system", listOf("android"), state = 0, persistent = true),
            AppProcess(1051, 10068, AGENT, listOf(AGENT), state = 1, persistent = true),
            AppProcess(1233, 10110, "com.google.android.gms", listOf(GMS), state = 5),
            AppProcess(1414, 10119, "skplanet.musicmate", listOf("skplanet.musicmate"), state = 2, hasActivities = true),
            AppProcess(6454, 10117, "com.apkpure.aegon", listOf("com.apkpure.aegon"), state = 3),
            cachedEmpty(7232, 10112, "com.android.vending", VENDING),
            cachedEmpty(7346, 10112, "com.android.vending:background", VENDING),
            cachedEmpty(7529, 10110, "com.google.android.gms.unstable", GMS),
            cachedEmpty(4762, 1000, "com.android.settings", "com.android.settings"),
        )
        policy.tasks(setOf("skplanet.musicmate", "com.apkpure.aegon"))
        policy.check()
        assertTrue(processes.ended.isEmpty())

        scheduler.advance(CachedProcesses.IDLE_MILLIS)
        policy.check()

        assertEquals(setOf("system", AGENT, "com.google.android.gms", "skplanet.musicmate", "com.apkpure.aegon"), names())
        assertEquals(setOf(VENDING, GMS, "com.android.settings"), processes.ended.toSet())
        val lines = log.lines.filter { it.startsWith("info: ended cached ") }
        assertEquals(lines.toString(), 4, lines.size)
        assertTrue(
            lines.toString(),
            lines.contains(
                "info: ended cached com.google.android.gms.unstable (pid 7529, cached 30 s, no task window): " +
                    "PSS 70 MiB, 28 MiB resident and 42 MiB swapped out to zram",
            ),
        )
    }

    @Test
    fun nfr2_google_play_services_core_is_never_ended_even_when_cached() {
        processes.processes += cachedEmpty(1233, 10110, "com.google.android.gms", GMS)
        processes.processes += cachedEmpty(1300, 10110, "com.google.android.gms.persistent", GMS)
        idle()
        assertTrue(processes.ended.isEmpty())
    }

    @Test
    fun nfr2_a_package_whose_end_would_reach_the_core_is_kept() {
        // The core's score dropped to the service level: ending Play services' cached process
        // through its package would end the core as well.
        val core = AppProcess(1233, 10110, "com.google.android.gms", listOf(GMS), state = 10)
        processes.processes += core
        processes.scores[core.pid] = 500
        processes.processes += cachedEmpty(7529, 10110, "com.google.android.gms.unstable", GMS)
        idle()
        assertTrue(processes.ended.isEmpty())
        assertTrue(log.lines.toString(), log.lines.any { it.startsWith("info: kept cached com.google.android.gms.unstable (pid 7529): ending it would also end com.google.android.gms (pid 1233") })
    }

    @Test
    fun nfr2_a_process_whose_app_has_a_task_window_is_kept() {
        processes.processes += cachedEmpty(5000, 10120, "org.example.notes:sync", "org.example.notes")
        policy.tasks(setOf("org.example.notes"))
        idle()
        assertTrue(processes.ended.isEmpty())
    }

    @Test
    fun nfr2_a_process_that_loaded_a_minimized_apps_code_is_not_ended_with_it() {
        // A minimized window's app is cached too, and it loaded WebView's code: ending
        // WebView's cached service process through its package would end the window's app.
        processes.processes += AppProcess(6454, 10117, "com.apkpure.aegon", listOf("com.apkpure.aegon"), state = 16, hasActivities = true)
        processes.dependencies[6454] = setOf(WEBVIEW)
        processes.processes += cachedEmpty(3233, 10064, "com.android.webview:webview_service", WEBVIEW)
        policy.tasks(setOf("com.apkpure.aegon"))
        idle()
        assertTrue(processes.ended.isEmpty())
        assertEquals(setOf("com.apkpure.aegon", "com.android.webview:webview_service"), names())
        // Kept processes are looked at again later, not on every look.
        val kept = log.lines.count { it.startsWith("info: kept cached com.android.webview:webview_service") }
        scheduler.advance(CachedProcesses.INTERVAL_MILLIS)
        policy.check()
        assertEquals(kept, log.lines.count { it.startsWith("info: kept cached com.android.webview:webview_service") })
    }

    @Test
    fun nfr2_a_process_that_is_not_cached_holds_activities_or_is_persistent_is_kept() {
        processes.processes += AppProcess(100, 10130, "org.example.player", listOf("org.example.player"), state = 4)
        processes.processes += AppProcess(101, 10131, "org.example.sync", listOf("org.example.sync"), state = 10)
        processes.processes += AppProcess(102, 10132, "org.example.reader", listOf("org.example.reader"), state = 16, hasActivities = true)
        processes.processes += AppProcess(103, 10133, "org.example.phone", listOf("org.example.phone"), state = CACHED_EMPTY, persistent = true)
        processes.processes += cachedEmpty(104, 10068, "$AGENT:helper", AGENT)
        idle()
        assertTrue(processes.ended.isEmpty())
    }

    @Test
    fun nfr2_an_app_with_a_foreground_service_keeps_its_cached_processes() {
        processes.processes += AppProcess(200, 10140, "org.example.music", listOf("org.example.music"), state = 4)
        processes.processes += cachedEmpty(201, 10140, "org.example.music:cache", "org.example.music")
        processes.foreground += "org.example.music"
        idle()
        assertTrue(processes.ended.isEmpty())
    }

    @Test
    fun nfr2_a_process_cached_for_less_than_the_idle_time_is_kept() {
        processes.processes += cachedEmpty(7232, 10112, "com.android.vending", VENDING)
        policy.check()
        scheduler.advance(CachedProcesses.IDLE_MILLIS - 1)
        policy.check()
        assertTrue(processes.ended.isEmpty())
        // Work that brings it out of the cached state starts the count again.
        processes.processes[0] = processes.processes[0].copy(state = 10)
        policy.check()
        processes.processes[0] = processes.processes[0].copy(state = CACHED_EMPTY)
        scheduler.advance(1)
        policy.check()
        assertTrue(processes.ended.isEmpty())
        scheduler.advance(CachedProcesses.IDLE_MILLIS)
        policy.check()
        assertEquals(listOf(VENDING), processes.ended)
    }

    @Test
    fun nfr2_a_process_that_comes_back_soon_waits_longer_before_it_is_ended_again() {
        processes.processes += cachedEmpty(7232, 10112, "com.android.vending", VENDING)
        idle()
        assertEquals(1, processes.ended.size)
        // A job starts it again a minute later, and it is cached once the job is done.
        scheduler.advance(60_000)
        processes.processes += cachedEmpty(7400, 10112, "com.android.vending", VENDING)
        idle()
        assertEquals(1, processes.ended.size)
        scheduler.advance(CachedProcesses.IDLE_MILLIS)
        policy.check()
        assertEquals(2, processes.ended.size)
    }

    @Test
    fun nfr2_nothing_is_ended_while_the_host_is_not_connected_or_the_records_are_unreadable() {
        processes.processes += cachedEmpty(7232, 10112, "com.android.vending", VENDING)
        connected = false
        idle()
        assertTrue(processes.ended.isEmpty())
        connected = true
        processes.readable = false
        idle()
        idle()
        assertTrue(processes.ended.isEmpty())
        assertEquals(1, log.lines.count { it.startsWith("warn: could not read the activity manager's process records") })
    }

    @Test
    fun nfr2_the_policy_looks_at_the_processes_periodically_once_boot_settled() {
        processes.processes += cachedEmpty(7529, 10110, "com.google.android.gms.unstable", GMS)
        policy.start()
        scheduler.advance(CachedProcesses.FIRST_DELAY_MILLIS)
        assertTrue(processes.ended.isEmpty())
        scheduler.advance(CachedProcesses.IDLE_MILLIS + CachedProcesses.INTERVAL_MILLIS)
        assertEquals(listOf(GMS), processes.ended)
    }

    @Test
    fun nfr2_the_activity_managers_process_records_are_read_from_its_dump() {
        val records = ProcessRecords.parse(DUMP)!!
        assertEquals(listOf(7529, 6454, 1050, 4762), records.map { it.pid })
        val unstable = records[0]
        assertEquals("com.google.android.gms.unstable", unstable.name)
        assertEquals(10110, unstable.uid)
        assertEquals(setOf(GMS), unstable.packages)
        assertEquals(905, unstable.setAdj)
        assertEquals(setOf(WEBVIEW), records[1].dependencies)
        assertEquals(100, records[1].setAdj)
        assertTrue(records[2].persistent)
        assertEquals(setOf("com.android.networkstack", "com.android.cellbroadcastservice", "com.android.networkstack.tethering"), records[2].packages)
        assertEquals(-800, records[2].setAdj)
        assertEquals(905, records[3].setAdj)
        assertNull(ProcessRecords.parse("Permission Denial: can't dump ActivityManager"))
    }

    private companion object {
        const val AGENT = "io.github.thisisthepy.dsa.agent"
        const val GMS = "com.google.android.gms"
        const val VENDING = "com.android.vending"
        const val WEBVIEW = "com.android.webview"
        const val CACHED_EMPTY = 19

        // From the owner's guest (dumpsys activity processes), shortened.
        val DUMP = """
            ACTIVITY MANAGER RUNNING PROCESSES (dumpsys activity processes)
              All known processes:
              *APP* UID 10110 ProcessRecord{b268d09 7529:com.google.android.gms.unstable/u0a110}
                user #0 uid=10110 gids={3003, 50110, 20110, 9997}
                packageList={com.google.android.gms}
                pid=7529
                oom adj: max=1001 curRaw=905 setRaw=905 cur=905 set=905
                curProcState=19 mRepProcState=19 setProcState=19 lastStateTime=-21s

                cached=true empty=true
                 Configuration={1.0 ?mcc0mnc}
              *APP* UID 10117 ProcessRecord{a8e488a 6454:com.apkpure.aegon/u0a117}
                packageList={com.apkpure.aegon}
                packageDependencies={com.android.webview}
                oom adj: max=1001 curRaw=100 setRaw=100 cur=100 set=100
              *PERS* UID 1073 ProcessRecord{dca66fa 1050:com.android.networkstack.process/1073}
                packageList={com.android.networkstack, com.android.cellbroadcastservice, com.android.networkstack.tethering}
                packageDependencies={com.android.cellbroadcastservice}
                oom adj: max=-800 curRaw=-800 setRaw=-800 cur=-800 set=-800
              *APP* UID 1000 ProcessRecord{ec69723 4762:com.android.settings/1000}
                packageList={com.android.settings}
                oom adj: max=1001 curRaw=905 setRaw=905 cur=905 set=905

              Isolated process list (sorted by uid):
                Isolated #0: ProcessRecord{14639b3 6657:com.android.webview:sandboxed_process0:org.chromium.content.app.SandboxedProcessService0:0/u0a117i2}
              Process LRU list (sorted by oom_adj, 14 total, non-act at 2, non-svc at 2):
                PERS #13: sys      F/ /PER  LCMNFUATI  t: 0 742:system/1000 (provider)
        """.trimIndent()
    }
}
