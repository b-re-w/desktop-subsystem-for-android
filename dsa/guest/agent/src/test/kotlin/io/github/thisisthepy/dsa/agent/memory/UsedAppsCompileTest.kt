package io.github.thisisthepy.dsa.agent.memory

import io.github.thisisthepy.dsa.agent.FakeScheduler
import io.github.thisisthepy.dsa.agent.RecordingLog
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class UsedAppsCompileTest {
    private val port = FakeAppMemory()
    private val scheduler = FakeScheduler()
    private val log = RecordingLog()
    private val compile = UsedAppsCompile(port, scheduler, log, clock = { scheduler.now }, ownPackage = AGENT)

    private fun open(vararg packages: String) {
        compile.tasks(packages.toSet())
        compile.tasks(emptySet())
    }

    /** No window shown for the quiet time, on AC power. */
    private fun idle() {
        compile.power(acOnline = true, batteryPresent = true, powerSaver = false)
        compile.screen(false)
        scheduler.advance(UsedAppsCompile.QUIET_MILLIS)
    }

    @Test
    fun nfr1_used_apps_are_compiled_with_speed_one_at_a_time_while_idle() {
        port.packages[MUSIC] = InstalledPackage(updatedAt = 100, apkBytes = 90L shl 20)
        port.packages[NOTES] = InstalledPackage(updatedAt = 200, apkBytes = 20L shl 20)
        port.packages[UNUSED] = InstalledPackage(updatedAt = 300, apkBytes = 10L shl 20)
        open(MUSIC, AGENT)
        open(MUSIC, NOTES)
        // Nothing while a window is shown, or before the quiet time is up.
        scheduler.advance(60 * 60_000L)
        assertEquals(emptyList<String>(), port.compiles)
        compile.power(acOnline = true, batteryPresent = true, powerSaver = false)
        compile.screen(false)
        scheduler.advance(UsedAppsCompile.QUIET_MILLIS - 1)
        assertEquals(emptyList<String>(), port.compiles)
        // The most opened first, the next once the first is done; never an app not opened,
        // nor the agent.
        port.compileResult = null
        scheduler.advance(1)
        assertEquals(listOf(MUSIC), port.compiled())
        scheduler.advance(UsedAppsCompile.POLL_MILLIS * 3)
        assertEquals(listOf(MUSIC), port.compiled())
        port.compileResult = "compiled"
        // The fake answers the request it was given; answer the one in flight.
        port.requestCompile(port.compiles.removeAt(0))
        scheduler.advance(UsedAppsCompile.POLL_MILLIS)
        assertEquals(listOf(MUSIC, NOTES), port.compiled())
        scheduler.advance(UsedAppsCompile.POLL_MILLIS)
        assertEquals(listOf(MUSIC, NOTES), port.compiled())
        // What was compiled is kept, with the update it compiled.
        assertEquals(CompileRecord(opens = 2, compiledUpdate = 100, result = "compiled", compiledAtMillis = port.wallMillis()), port.saved[MUSIC])
        assertEquals(200L, port.saved[NOTES]!!.compiledUpdate)
        assertTrue(log.lines.any { it.startsWith("info: compiled $MUSIC with speed (opened 2 times, 90 MiB of APKs): compiled") })
        assertTrue(port.saved.keys.none { it == AGENT || it == UNUSED })
    }

    @Test
    fun nfr1_the_compile_waits_for_ac_power_and_the_battery_saver_off() {
        port.packages[MUSIC] = InstalledPackage(updatedAt = 100, apkBytes = 1L shl 20)
        open(MUSIC)
        compile.power(acOnline = false, batteryPresent = true, powerSaver = false)
        compile.screen(false)
        scheduler.advance(60 * 60_000L)
        assertEquals(emptyList<String>(), port.compiles)
        compile.power(acOnline = true, batteryPresent = true, powerSaver = true)
        scheduler.advance(60 * 60_000L)
        assertEquals(emptyList<String>(), port.compiles)
        // A desktop without a battery counts as on AC power.
        compile.power(acOnline = false, batteryPresent = false, powerSaver = false)
        scheduler.advance(1)
        assertEquals(listOf(MUSIC), port.compiled())
    }

    @Test
    fun nfr1_an_updated_app_is_compiled_again() {
        port.packages[MUSIC] = InstalledPackage(updatedAt = 100, apkBytes = 1L shl 20)
        open(MUSIC)
        idle()
        assertEquals(listOf(MUSIC), port.compiled())
        scheduler.advance(UsedAppsCompile.POLL_MILLIS)
        // Idle again later: already compiled, nothing runs.
        compile.screen(true)
        idle()
        assertEquals(1, port.compiles.size)
        // An update: its new code is compiled at the next idle time.
        port.packages[MUSIC] = InstalledPackage(updatedAt = 900, apkBytes = 1L shl 20)
        compile.packageChanged(MUSIC)
        compile.screen(true)
        idle()
        assertEquals(listOf(MUSIC, MUSIC), port.compiled())
        scheduler.advance(UsedAppsCompile.POLL_MILLIS)
        assertEquals(900L, port.saved[MUSIC]!!.compiledUpdate)
    }

    @Test
    fun nfr1_a_large_app_is_compiled_only_once_it_is_used_often() {
        port.packages[GAME] = InstalledPackage(updatedAt = 100, apkBytes = 400L shl 20)
        repeat(UsedAppsCompile.OFTEN_OPENS - 1) { open(GAME) }
        idle()
        assertEquals(emptyList<String>(), port.compiles)
        assertTrue(log.lines.any { "left $GAME at speed-profile: its APKs are 400 MiB" in it })
        // Opened once more, it is looked at again after the next boot (a new agent here).
        open(GAME)
        val again = UsedAppsCompile(port, scheduler, log, clock = { scheduler.now }, ownPackage = AGENT)
        again.power(acOnline = true, batteryPresent = true, powerSaver = false)
        again.screen(false)
        scheduler.advance(UsedAppsCompile.QUIET_MILLIS)
        assertEquals(listOf(GAME), port.compiled())
    }

    private companion object {
        const val AGENT = "io.github.thisisthepy.dsa.agent"
        const val MUSIC = "com.spotify.music"
        const val NOTES = "org.example.notes"
        const val UNUSED = "org.example.unused"
        const val GAME = "org.example.game"
    }
}
