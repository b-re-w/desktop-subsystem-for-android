package io.github.thisisthepy.dsa.agent.memory

import io.github.thisisthepy.dsa.agent.Log
import io.github.thisisthepy.dsa.agent.Scheduler

/**
 * Compiles the apps the user opens with ART's speed filter while the guest is idle.
 *
 * An app installed from a store runs code compiled from its cloud profile (speed-profile) and
 * JIT-compiles the rest as it runs. The JIT's code cache and the profiling data that drive it
 * are anonymous memory in each process, which the kernel can only swap; code compiled ahead of
 * time is clean pages of the app's .oat file, which the kernel drops under pressure and reads
 * back from the disk. So the apps the user actually opens (a task of theirs was shown) are
 * compiled with speed, which compiles all their code ahead of time; an app that runs for hours
 * (a music or video player) gains the most. Every other app keeps speed-profile.
 *
 * The guest counts as idle once no window has been shown for [QUIET_MILLIS], while the host
 * runs on AC power (or has no battery) and its battery saver is off: a compile takes a core for
 * tens of seconds to minutes. One package is compiled at a time, the most opened first, and only
 * one whose current install or update has not been compiled yet, so an update is compiled again
 * the next time the guest is idle. A package whose APKs are larger than [LARGE_APK_BYTES] is
 * compiled only once it has been opened [OFTEN_OPENS] times: speed takes two to three times its
 * code's size on the disk. Each package is looked at once per boot at most, since the compile
 * script leaves a package already compiled with speed alone.
 *
 * What was compiled, when and how it ended is kept ([AppMemoryPort.save]) and logged:
 * `compiled <package> with speed (opened N times, M MiB of APKs): <result>`.
 *
 * Not thread safe: every method runs on the agent's scheduler thread.
 */
class UsedAppsCompile(
    private val port: AppMemoryPort,
    private val scheduler: Scheduler,
    private val log: Log,
    private val clock: () -> Long,
    private val ownPackage: String,
) {
    private var known: MutableMap<String, CompileRecord>? = null
    private var withTasks: Set<String> = emptySet()
    private var screenOn = true
    private var screenOffAt = 0L
    private var powerAllows = true
    private var inFlight: Pair<Long, String>? = null
    private var sentAt = 0L
    private var lastNumber = 0L
    private val lookedAt = HashSet<String>()
    private val toldTooLarge = HashSet<String>()
    private var generation = 0

    private fun records(): MutableMap<String, CompileRecord> =
        known ?: try {
            port.records().toMutableMap()
        } catch (e: Exception) {
            log.warn("could not read which apps were compiled; they are looked at again", e)
            mutableMapOf()
        }.also { known = it }

    /** Takes the apps that have a task now: each that was not there before was opened. */
    fun tasks(packages: Set<String>) {
        val opened = packages - withTasks
        withTasks = packages
        for (packageName in opened) {
            if (packageName.isEmpty() || packageName == ownPackage) continue
            val record = records()[packageName] ?: CompileRecord()
            keep(packageName, record.copy(opens = record.opens + 1))
        }
    }

    /** An app was installed or updated: its new code is compiled at the next idle time. */
    fun packageChanged(packageName: String) {
        lookedAt -= packageName
    }

    /** Takes whether the host shows any task window. */
    fun screen(on: Boolean) {
        if (on == screenOn) return
        screenOn = on
        if (!on) screenOffAt = clock()
        waitForQuiet()
    }

    /** Takes the host's power: the compile waits for AC power and the battery saver off. */
    fun power(acOnline: Boolean, batteryPresent: Boolean, powerSaver: Boolean) {
        val allows = (acOnline || !batteryPresent) && !powerSaver
        if (allows == powerAllows) return
        powerAllows = allows
        waitForQuiet()
    }

    private fun waitForQuiet() {
        val ticket = ++generation
        if (screenOn || !powerAllows) return
        val left = (QUIET_MILLIS - (clock() - screenOffAt)).coerceAtLeast(0)
        scheduler.postDelayed(left) { if (ticket == generation) next() }
    }

    private fun idle(): Boolean = !screenOn && powerAllows && clock() - screenOffAt >= QUIET_MILLIS

    /** Compiles the next app that needs it, if the guest is idle and none is being compiled. */
    fun next() {
        if (inFlight != null || !idle()) return
        val candidates = records().entries
            .filter { (packageName, record) -> record.opens > 0 && packageName !in lookedAt }
            .sortedWith(compareByDescending<Map.Entry<String, CompileRecord>> { it.value.opens }.thenBy { it.key })
        for ((packageName, record) in candidates) {
            val installed = try {
                port.installed(packageName)
            } catch (e: Exception) {
                null
            }
            lookedAt += packageName
            if (installed == null) continue
            if (record.compiledUpdate == installed.updatedAt) continue
            if (installed.apkBytes > LARGE_APK_BYTES && record.opens < OFTEN_OPENS) {
                if (toldTooLarge.add(packageName)) {
                    log.info(
                        "left $packageName at speed-profile: its APKs are ${installed.apkBytes / MIB} MiB, " +
                            "over ${LARGE_APK_BYTES / MIB}, and it was opened ${record.opens} times, " +
                            "fewer than $OFTEN_OPENS",
                    )
                }
                continue
            }
            if (send(packageName, record, installed)) return
        }
    }

    private fun send(packageName: String, record: CompileRecord, installed: InstalledPackage): Boolean {
        val number = clock().let { if (it <= lastNumber) lastNumber + 1 else it }
        val value = "$number $packageName"
        if (value.length > MAX_PROPERTY_LENGTH) {
            log.info("left $packageName at speed-profile: its name is too long for the compile request")
            return false
        }
        lastNumber = number
        try {
            port.requestCompile(value)
        } catch (e: Exception) {
            log.warn("could not ask for $packageName to be compiled", e)
            return false
        }
        inFlight = number to packageName
        sentAt = clock()
        log.info(
            "compiling $packageName with speed while the guest is idle (opened ${record.opens} times, " +
                "${installed.apkBytes / MIB} MiB of APKs)",
        )
        scheduler.postDelayed(POLL_MILLIS) { answer(installed) }
        return true
    }

    private fun answer(installed: InstalledPackage) {
        val (number, packageName) = inFlight ?: return
        val done = try {
            port.compileDone().trim()
        } catch (e: Exception) {
            ""
        }
        val result = done.takeIf { it.startsWith("$number ") }?.substringAfter(' ')
        if (result == null) {
            if (clock() - sentAt < COMPILE_TIMEOUT_MILLIS) {
                scheduler.postDelayed(POLL_MILLIS) { answer(installed) }
                return
            }
            log.warn(
                "compiling $packageName gave no answer in ${COMPILE_TIMEOUT_MILLIS / 60_000} min; " +
                    "it is looked at again after the next boot",
            )
            inFlight = null
            next()
            return
        }
        inFlight = null
        val record = records()[packageName] ?: CompileRecord()
        val compiled = result == "compiled" || result == "current"
        keep(
            packageName,
            record.copy(
                compiledUpdate = if (compiled) installed.updatedAt else record.compiledUpdate,
                result = result,
                compiledAtMillis = port.wallMillis(),
            ),
        )
        log.info(
            "compiled $packageName with speed (opened ${record.opens} times, " +
                "${installed.apkBytes / MIB} MiB of APKs): $result",
        )
        next()
    }

    private fun keep(packageName: String, record: CompileRecord) {
        records()[packageName] = record
        try {
            port.save(packageName, record)
        } catch (e: Exception) {
            log.warn("could not keep what is known of $packageName's compile", e)
        }
    }

    companion object {
        /** How long no window must have been shown before a compile starts. */
        const val QUIET_MILLIS = 5 * 60_000L

        /** APKs larger than this are compiled only for an app opened [OFTEN_OPENS] times. */
        const val LARGE_APK_BYTES = 150L * 1024 * 1024

        /** How many times a large app must have been opened to be compiled. */
        const val OFTEN_OPENS = 5

        /** How often the compile's answer is looked for. */
        const val POLL_MILLIS = 5_000L

        /** How long one compile may take before the next app is looked at. */
        const val COMPILE_TIMEOUT_MILLIS = 30 * 60_000L

        /** The longest value a property that is not read-only holds (PROP_VALUE_MAX less one). */
        const val MAX_PROPERTY_LENGTH = 91

        private const val MIB = 1024L * 1024
    }
}
