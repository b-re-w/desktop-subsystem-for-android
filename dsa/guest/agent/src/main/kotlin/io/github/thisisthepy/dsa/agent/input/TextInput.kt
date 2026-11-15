package io.github.thisisthepy.dsa.agent.input

import android.text.InputType
import android.view.inputmethod.EditorInfo
import io.github.thisisthepy.dsa.agent.Log
import io.github.thisisthepy.dsa.agent.tasks.TaskBounds
import io.github.thisisthepy.dsa.control.v1.Envelope
import io.github.thisisthepy.dsa.control.v1.ImeCommit
import io.github.thisisthepy.dsa.control.v1.ImeComposition
import io.github.thisisthepy.dsa.control.v1.ImeCursor
import io.github.thisisthepy.dsa.control.v1.ImeDeleteSurrounding
import io.github.thisisthepy.dsa.control.v1.ImeEditor
import io.github.thisisthepy.dsa.control.v1.ImeInputKind
import io.github.thisisthepy.dsa.control.v1.ImeSetComposingRegion
import io.github.thisisthepy.dsa.control.v1.ImeSetSelection
import io.github.thisisthepy.dsa.control.v1.Rect

/**
 * The input method's side of the focused editor's InputConnection: what the agent's input
 * method (platform/HostInputMethod.kt) does with the host's text.
 */
interface EditorConnection {
    /**
     * Makes [text] the editor's composition, in place of the one it has, with the cursor
     * [cursor] UTF-16 code units from the start of [text]. An empty [text] removes the
     * composition. False when no editor takes text now.
     */
    fun setComposing(text: String, cursor: Int): Boolean

    /** Types [text] in place of the composition, which ends. False when no editor takes text now. */
    fun commit(text: String): Boolean

    /**
     * Deletes [before] UTF-16 code units before the cursor and [after] after it, keeping the
     * composition. False when no editor takes text now.
     */
    fun deleteSurrounding(before: Int, after: Int): Boolean

    /**
     * Selects from [start] to [end], offsets in UTF-16 code units from the start of the
     * editor's text, keeping the composition. False when no editor takes text now.
     */
    fun setSelection(start: Int, end: Int): Boolean

    /**
     * Makes the text from [start] to [end] the composition, as the host's input method does
     * when it reconverts a word; equal offsets end the composition and keep its text. False
     * when no editor takes text now.
     */
    fun setComposingRegion(start: Int, end: Int): Boolean

    /**
     * Whether a composition shows in the editor: false when the input method has only the
     * app's fallback connection, which takes a composition and shows it nowhere (see
     * [EditorLink]).
     */
    val composesInline: Boolean
        get() = true

    /**
     * Input restarted on the editor in the middle of a composition, and the framework finished
     * the composition as the old connection closed, keeping its text. Makes that text the
     * composition again, when the editor still holds it where it was composed. The composing
     * region in the editor's whole text, or null when there is no composition to resume or the
     * editor no longer holds it: then it changed, and the composition is over.
     */
    fun resumeComposition(): IntRange? = null

    /** Input started on an editor: a new one, or the same one again when [restarting]. */
    fun inputStarted(restarting: Boolean) {}

    companion object {
        /** No editor, as before the input method starts. */
        val NONE = object : EditorConnection {
            override val composesInline = false
            override fun setComposing(text: String, cursor: Int) = false
            override fun commit(text: String) = false
            override fun deleteSurrounding(before: Int, after: Int) = false
            override fun setSelection(start: Int, end: Int) = false
            override fun setComposingRegion(start: Int, end: Int) = false
        }
    }
}

/**
 * The focused editor as the host's input method reads it: the kind of text it takes, and its
 * text around the selection. Offsets are UTF-16 code units from the start of the editor's
 * whole text; [selectionStart] is the anchor and [selectionEnd] the active end. A composition
 * whose start equals its end is none. [packageName] is the app the editor belongs to, as its
 * EditorInfo names it (empty when it does not), which tells the agent the task it is in.
 */
data class EditorState(
    val kind: ImeInputKind,
    val text: String,
    val textStart: Int,
    val selectionStart: Int,
    val selectionEnd: Int,
    val compositionStart: Int,
    val compositionEnd: Int,
    val packageName: String = "",
) {
    companion object {
        /** No editor takes text: the focus is on a view that reads keys, or on nothing. */
        val NONE = EditorState(ImeInputKind.IME_INPUT_KIND_NONE, "", 0, 0, 0, 0, 0)

        /** The most text around the selection the host takes, in UTF-16 code units. */
        const val MAX_TEXT = 4096

        /** How far the text reaches on each side of the selection, where the editor's goes on. */
        const val REACH = 1024
    }
}

/**
 * The kind of text an editor takes, from its EditorInfo's input type and options, so the
 * host's input method offers the keys for it and keeps composition and prediction out of a
 * password.
 */
fun inputKind(inputType: Int, imeOptions: Int): ImeInputKind {
    val variation = inputType and InputType.TYPE_MASK_VARIATION
    return when (inputType and InputType.TYPE_MASK_CLASS) {
        InputType.TYPE_NULL -> ImeInputKind.IME_INPUT_KIND_NONE
        InputType.TYPE_CLASS_NUMBER ->
            if (variation == InputType.TYPE_NUMBER_VARIATION_PASSWORD) {
                ImeInputKind.IME_INPUT_KIND_NUMBER_PASSWORD
            } else {
                ImeInputKind.IME_INPUT_KIND_NUMBER
            }
        InputType.TYPE_CLASS_PHONE -> ImeInputKind.IME_INPUT_KIND_PHONE
        InputType.TYPE_CLASS_DATETIME -> when (variation) {
            InputType.TYPE_DATETIME_VARIATION_DATE -> ImeInputKind.IME_INPUT_KIND_DATE
            InputType.TYPE_DATETIME_VARIATION_TIME -> ImeInputKind.IME_INPUT_KIND_TIME
            else -> ImeInputKind.IME_INPUT_KIND_TEXT
        }
        InputType.TYPE_CLASS_TEXT -> when (variation) {
            InputType.TYPE_TEXT_VARIATION_PASSWORD,
            InputType.TYPE_TEXT_VARIATION_VISIBLE_PASSWORD,
            InputType.TYPE_TEXT_VARIATION_WEB_PASSWORD,
            -> ImeInputKind.IME_INPUT_KIND_PASSWORD
            InputType.TYPE_TEXT_VARIATION_EMAIL_ADDRESS,
            InputType.TYPE_TEXT_VARIATION_WEB_EMAIL_ADDRESS,
            -> ImeInputKind.IME_INPUT_KIND_EMAIL
            InputType.TYPE_TEXT_VARIATION_URI -> ImeInputKind.IME_INPUT_KIND_URL
            InputType.TYPE_TEXT_VARIATION_PERSON_NAME -> ImeInputKind.IME_INPUT_KIND_PERSON_NAME
            else ->
                if (imeOptions and EditorInfo.IME_MASK_ACTION == EditorInfo.IME_ACTION_SEARCH) {
                    ImeInputKind.IME_INPUT_KIND_SEARCH
                } else {
                    ImeInputKind.IME_INPUT_KIND_TEXT
                }
        }
        else -> ImeInputKind.IME_INPUT_KIND_TEXT
    }
}

/** The key events that type some text where no editor takes text, such as a game's view. */
fun interface TextKeys {
    /** The presses and releases that type [text] in order; null when a character has no key. */
    fun keysFor(text: String): List<InjectedKey>?
}

/**
 * Puts the host's text into the guest: what the host's input method composes and commits in a
 * task's window, and the characters its keys type.
 *
 * The guest has no keyboard of its own: the agent's input method is the system's only one,
 * shows nothing, and is the input method side of whichever editor has the focus. A
 * composition becomes the editor's composing text, underlined where the user types it, with
 * the cursor where the host's input method has it; a commit replaces it. Before text goes to
 * a task that does not hold the input focus, the task is given it, as for a key.
 *
 * Text typed where no editor takes text (a game, a view of its own that reads keys) goes as
 * the key events of the guest's virtual keyboard for each character, so typing still works
 * there; a composition there has nowhere to go and is dropped.
 *
 * The other way, the input method reports the editor and where its cursor is on the display,
 * and this class tells the host which task's window they are in, so the host's input method
 * composes in that window's document and its candidate list appears beside the cursor.
 *
 * Which task an editor is in does not rest on the window manager's focus alone. The window
 * manager can name no focused task at all while an app's editor has the input method (on
 * Android 17, with every task freeform, its focused app was null on every display while the
 * input dispatcher still sent the keys to the app's window), and an editor told to no task
 * never reaches the host: the window then had no editor to compose in, so the host's input
 * method showed its composition in a window of its own in the corner of the screen, and the
 * app received the text only when it was committed. The editor's own app names its task: the
 * one task of that app, else the one of them that holds the focus or that the host last sent
 * input to. An editor of an app with no task (a dialog of another app shown in the task) goes
 * to the focused task, else to the one the host sent input to last. When the editor is in a
 * task that does not hold the focus, that task is given it, once, so the window manager's
 * input method target follows the editor again.
 *
 * Not thread safe: the agent calls it from one thread.
 */
class TextInput(
    private val editor: () -> EditorConnection,
    private val injector: InputInjector,
    private val keys: TextKeys,
    /** Whether the host was told about the task. */
    private val isReported: (Int) -> Boolean,
    /** Whether the task holds the input focus in the guest, as last read. */
    private val isFocused: (Int) -> Boolean,
    /** Gives the task the input focus; false when the framework refused. */
    private val focus: (Int) -> Boolean,
    /** Where the host was told the task is on the display. */
    private val bounds: (Int) -> TaskBounds?,
    /** The task the host was told holds the input focus, if any. */
    private val focusedTask: () -> Int?,
    /** Sends one control frame to the host. */
    private val send: (ByteArray) -> Unit,
    private val log: Log,
    /** The tasks the host was told about, in the order it was told. */
    private val reportedTasks: () -> Collection<Int> = { emptyList() },
    /** The package the host was told the task is of; null when unknown. */
    private val packageOf: (Int) -> String? = { null },
) {
    private val unknownTasks = HashSet<Int>()
    private var lastCursor: Pair<Int, Rect>? = null
    private var lastEditor: Pair<Int, EditorState>? = null
    private var failing = false

    /** The task the host last sent input to: its window is the one the user types in. */
    private var addressed: Int? = null

    /** The task last given the focus because its editor had the input method. */
    private var focusedForEditor: Int? = null

    /** The task a composition is being typed in, and how many updates it has had. */
    private var composing: Pair<Int, Int>? = null

    /**
     * The composition that could not show because the input method had only the app's
     * fallback connection: it goes into the editor once its own connection comes.
     */
    private var waiting: ImeComposition? = null

    /** The task told it has no connection of its own, once for each time it lacks one. */
    private var unconnected: Int? = null

    /**
     * The host sent the task input (a key, a press, text): the user is at its window. Tasks
     * the host was never told about are ignored.
     */
    fun addressed(taskId: Int) {
        if (isReported(taskId)) addressed = taskId
    }

    /**
     * Acts on a composition from the host. The log hears when a composition begins and how
     * it ends, not each key of it.
     */
    fun handle(composition: ImeComposition) {
        val taskId = composition.taskId
        if (!reach(taskId)) return
        val text = composition.text
        val cursor = composition.cursor.coerceIn(0, text.length)
        val target = editor()
        val taken = target.setComposing(text, cursor)
        if (text.isEmpty()) {
            waiting = null
            ended(taskId, "was removed")
            return
        }
        val (task, updates) = composing ?: (null to 0)
        if (task != taskId) {
            composing = taskId to 1
            log.info(
                when {
                    !taken -> "a composition began in task $taskId, but no editor there takes text"
                    target.composesInline -> "a composition began in task $taskId, inline in its editor"
                    else -> "a composition began in task $taskId, which its editor cannot show yet"
                },
            )
            if (!taken) log.warn("dropped a composition for task $taskId: no editor there takes text")
        } else {
            composing = taskId to updates + 1
        }
        if (taken && !target.composesInline) {
            waiting = composition
            if (unconnected != taskId) {
                unconnected = taskId
                log.warn(
                    "the editor in task $taskId has no input connection of its own, only the app's " +
                        "fallback, which shows no composition; asking for the task's focus so the " +
                        "framework starts input on the editor",
                )
                if (!focus(taskId)) log.warn("could not give task $taskId the input focus its editor needs")
            }
        } else {
            waiting = null
            if (taken) unconnected = null
        }
    }

    /** The composition in [taskId] ended as [how] says: one line for the log. */
    private fun ended(taskId: Int, how: String) {
        val (task, updates) = composing ?: return
        if (task != taskId) return
        composing = null
        log.info("the composition in task $taskId $how after $updates updates")
    }

    /** Acts on committed text from the host. */
    fun handle(commit: ImeCommit) {
        val taskId = commit.taskId
        if (!reach(taskId)) return
        val text = commit.text
        waiting = null
        ended(taskId, "was committed")
        if (text.isEmpty() || editor().commit(text)) return
        val typed = keys.keysFor(text)
        if (typed == null) {
            log.warn("dropped text for task $taskId: no editor there takes text, and no key types it")
            return
        }
        for (key in typed) inject(key)
    }

    /** Deletes text around the editor's cursor, as the host's input method asks. */
    fun handle(delete: ImeDeleteSurrounding) {
        val taskId = delete.taskId
        if (!reach(taskId)) return
        val before = delete.beforeLength.coerceAtLeast(0)
        val after = delete.afterLength.coerceAtLeast(0)
        if (before == 0 && after == 0) return
        if (!editor().deleteSurrounding(before, after)) {
            log.warn("dropped a deletion for task $taskId: no editor there takes text")
        }
    }

    /** Selects text in the editor, as the host's input method asks. */
    fun handle(selection: ImeSetSelection) {
        val taskId = selection.taskId
        if (!reach(taskId)) return
        if (selection.start < 0 || selection.end < 0) return
        if (!editor().setSelection(selection.start, selection.end)) {
            log.warn("dropped a selection for task $taskId: no editor there takes text")
        }
    }

    /** Makes text in the editor the composition again, as the host's input method asks. */
    fun handle(region: ImeSetComposingRegion) {
        val taskId = region.taskId
        if (!reach(taskId)) return
        if (region.start < 0 || region.end < region.start) return
        if (!editor().setComposingRegion(region.start, region.end)) {
            log.warn("dropped a composing region for task $taskId: no editor there takes text")
        }
    }

    /**
     * The focused editor is now [state]. The host hears it for the task the editor is in
     * ([editorTask]), once for each state, so its input method reads the editor's own text.
     * A task that does not hold the focus while its editor takes text is given it, once.
     */
    fun editorChanged(state: EditorState) {
        val taskId = editorTask(state.packageName) ?: return
        if (isFocused(taskId)) {
            // Held now: should it lose the focus again, it is given it again.
            if (focusedForEditor == taskId) focusedForEditor = null
        } else if (state.kind != ImeInputKind.IME_INPUT_KIND_NONE && focusedForEditor != taskId) {
            focusedForEditor = taskId
            if (!focus(taskId)) log.warn("could not give task $taskId the input focus its editor has")
        }
        // The editor has its own connection now: the composition that could not show goes in.
        val held = waiting
        if (held != null && held.taskId == taskId && state.kind != ImeInputKind.IME_INPUT_KIND_NONE) {
            val target = editor()
            if (target.composesInline && target.setComposing(held.text, held.cursor.coerceIn(0, held.text.length))) {
                waiting = null
                unconnected = null
                log.info("the composition in task $taskId went into its editor once the editor's own connection came")
            }
        }
        if (lastEditor == taskId to state) return
        lastEditor = taskId to state
        send(
            Envelope.newBuilder()
                .setImeEditor(
                    ImeEditor.newBuilder()
                        .setTaskId(taskId)
                        .setKind(state.kind)
                        .setText(state.text)
                        .setTextStart(state.textStart)
                        .setSelectionStart(state.selectionStart)
                        .setSelectionEnd(state.selectionEnd)
                        .setCompositionStart(state.compositionStart)
                        .setCompositionEnd(state.compositionEnd),
                )
                .build()
                .toByteArray(),
        )
    }

    /**
     * Input restarted on the focused editor, which is now [state]. Its app restarts input as it
     * sets the editor's text or state, and the framework finishes the composition as the old
     * connection closes, keeping its text. A composition still being typed in the editor's task
     * goes on: its text becomes the composition again, and the host hears the editor with it,
     * so neither side commits the syllable half made. Only when the editor no longer holds the
     * text where it was composed did the editor really change; the composition is then over,
     * and the host hears the editor as it is.
     */
    fun editorRestarted(state: EditorState) {
        val (task, _) = composing ?: return editorChanged(state)
        if (state.kind == ImeInputKind.IME_INPUT_KIND_NONE || editorTask(state.packageName) != task) {
            return editorChanged(state)
        }
        val region = editor().resumeComposition()
        if (region == null) {
            waiting = null
            ended(task, "ended as input restarted on an editor that no longer holds it,")
            return editorChanged(state)
        }
        if (!resumeNoted) {
            resumeNoted = true
            log.info("input restarted in task $task in the middle of a composition, which goes on in its editor")
        }
        val end = region.last + 1
        val within = region.first >= state.textStart && end <= state.textStart + state.text.length
        editorChanged(if (within) state.copy(compositionStart = region.first, compositionEnd = end) else state)
    }

    /** Whether the log was told a composition went on as input restarted. */
    private var resumeNoted = false

    /**
     * The focused editor's cursor is at these display pixels. The host hears where it is in
     * the task of the editor last described, else the focused task, else the one the host last
     * sent input to, once for each place it moves to.
     */
    fun cursorMoved(left: Float, top: Float, right: Float, bottom: Float) {
        val taskId = lastEditor?.first?.takeIf(isReported) ?: focusedTask() ?: addressed?.takeIf(isReported) ?: return
        val task = bounds(taskId) ?: return
        val rect = Rect.newBuilder()
            .setLeft(Math.round(left) - task.left)
            .setTop(Math.round(top) - task.top)
            .setRight(Math.round(right) - task.left)
            .setBottom(Math.round(bottom) - task.top)
            .build()
        if (lastCursor == taskId to rect) return
        lastCursor = taskId to rect
        send(
            Envelope.newBuilder()
                .setImeCursor(ImeCursor.newBuilder().setTaskId(taskId).setCursor(rect))
                .build()
                .toByteArray(),
        )
    }

    /** Forgets the tasks not in [taskIds], as tasks end. */
    fun retainTasks(taskIds: Set<Int>) {
        unknownTasks.removeAll(taskIds)
        if (lastCursor?.first !in taskIds) lastCursor = null
        if (lastEditor?.first !in taskIds) lastEditor = null
        if (addressed !in taskIds) addressed = null
        if (focusedForEditor !in taskIds) focusedForEditor = null
    }

    /**
     * The task an editor of [packageName] is in: the focused task when it is that app's (or
     * the package is not known), else the app's one task, else the one of its tasks the host
     * last sent input to, held the editor before or comes first. An editor of an app with no
     * task goes to the focused task, else to the task that held the editor before, else to the
     * one the host last sent input to. Null when there is none of these.
     */
    private fun editorTask(packageName: String): Int? {
        val focused = focusedTask()
        val before = lastEditor?.first?.takeIf(isReported)
        val sentTo = addressed?.takeIf(isReported)
        if (packageName.isEmpty()) return before ?: focused ?: sentTo
        if (focused != null && packageOf(focused) == packageName) return focused
        val owners = reportedTasks().filter { packageOf(it) == packageName }
        return when {
            owners.isEmpty() -> focused ?: before ?: sentTo
            owners.size == 1 -> owners.single()
            sentTo in owners -> sentTo
            before in owners -> before
            else -> owners.first()
        }
    }

    /**
     * Whether text for the task can go to it: the host knows the task, and it holds the focus,
     * or its editor has the input method already. A task that holds neither is given the focus.
     */
    private fun reach(taskId: Int): Boolean {
        if (!isReported(taskId)) {
            if (unknownTasks.add(taskId)) {
                log.warn("dropped text input for task $taskId, which the host was never told about")
            }
            return false
        }
        unknownTasks.remove(taskId)
        addressed = taskId
        // An editor of the task has the input method: asking for the focus on every keystroke
        // of a composition, while the window manager names no focused task, only moves the
        // task to the front again and again.
        val editorThere = lastEditor?.let { (task, state) -> task == taskId && state.kind != ImeInputKind.IME_INPUT_KIND_NONE } == true
        if (!isFocused(taskId) && !editorThere && !focus(taskId)) {
            log.warn("could not give task $taskId the input focus; the text may reach another task")
        }
        return true
    }

    private fun inject(key: InjectedKey) {
        val injected = try {
            injector.injectKey(key)
        } catch (e: RuntimeException) {
            if (!failing) log.warn("the framework refused the keys of typed text", e)
            failing = true
            return
        }
        if (!injected) {
            if (!failing) log.warn("the framework refused the keys of typed text")
            failing = true
        } else {
            failing = false
        }
    }
}
