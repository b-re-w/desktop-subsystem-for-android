package io.github.thisisthepy.dsa.agent.input

/**
 * The calls an input method makes on an editor's InputConnection that the host's text needs.
 * The agent's input method (platform/HostInputMethod.kt) passes them to the connection as they
 * are, so the order and the arguments of the calls are [LinkedEditor]'s, and are tested
 * without Android.
 */
interface EditorCalls {
    fun beginBatchEdit(): Boolean

    fun endBatchEdit(): Boolean

    fun setComposingText(text: String, newCursorPosition: Int): Boolean

    fun finishComposingText(): Boolean

    fun commitText(text: String, newCursorPosition: Int): Boolean

    fun deleteSurroundingText(before: Int, after: Int): Boolean

    fun setSelection(start: Int, end: Int): Boolean

    fun setComposingRegion(start: Int, end: Int): Boolean

    /** Where the selection ends in the editor's whole text; null when the editor does not say. */
    fun selectionEnd(): Int?

    /** The [length] UTF-16 code units before the cursor, fewer at the start; null when the editor does not say. */
    fun textBeforeCursor(length: Int): String?

    /** The [length] UTF-16 code units after the cursor, fewer at the end; null when the editor does not say. */
    fun textAfterCursor(length: Int): String?
}

/**
 * What the input method holds of the focused editor.
 *
 * [started] is the connection the framework started input with for the editor. It is null
 * when the framework started none: it can start input on an editor and hand the input method
 * no connection, and once input finishes the input method keeps the editor's EditorInfo but no
 * longer its connection.
 *
 * [fallback] is the connection of the input method's binding to the app, which the input
 * method's current connection falls back to without one started: the app's fallback
 * BaseInputConnection. It keeps a composition in a buffer of its own that no view shows, and
 * sends what is committed to the focused view as key events. Through it a commit still types,
 * but a composition never shows.
 */
class EditorLink(val started: EditorCalls?, val fallback: EditorCalls?)

/**
 * The focused editor, through the connection [link] gives: the one the framework started input
 * with, else the app's fallback, so a commit types even then.
 *
 * Input can restart on the editor in the middle of a composition: its app restarts it as it
 * sets the editor's text or state, and the framework then hands the input method a new
 * connection. As the old connection closes, the framework finishes its composition, which keeps
 * the text and loses the composing span. The composition still goes on for the host, so before
 * the next edit on the new connection the editor's composing text is made the composition again,
 * if the editor still holds it around the cursor; the edit then replaces it rather than typing
 * beside it. An editor that no longer holds it changed for some other reason, and the next
 * composition begins where its cursor is.
 */
class LinkedEditor(private val link: () -> EditorLink?) : EditorConnection {
    override val composesInline: Boolean
        get() = link()?.started != null

    /** The connection the composition was last made in, its text, and its cursor in that text. */
    private var composedIn: EditorCalls? = null
    private var composed: String? = null
    private var composedCursor = 0

    private fun target(): EditorCalls? = link()?.let { it.started ?: it.fallback }

    private fun forget() {
        composedIn = null
        composed = null
        composedCursor = 0
    }

    /**
     * Where the composition made last stands in the editor [ic], which must hold its text on
     * both sides of the cursor as composed; the composing span is set there again when
     * [restore] says so or when [ic] is not the connection it was made in. Null, and the
     * composition forgotten, when there is none or the editor no longer holds it.
     */
    private fun carry(ic: EditorCalls, restore: Boolean): IntRange? {
        val text = composed ?: return null
        val cursor = composedCursor
        val end = ic.selectionEnd()
        val before = if (cursor > 0) ic.textBeforeCursor(cursor) else ""
        val after = if (cursor < text.length) ic.textAfterCursor(text.length - cursor) else ""
        if (end == null || before != text.substring(0, cursor) || after != text.substring(cursor)) {
            forget()
            return null
        }
        val start = end - cursor
        if (restore || composedIn !== ic) {
            ic.setComposingRegion(start, start + text.length)
            composedIn = ic
        }
        return start until start + text.length
    }

    /** Before an edit on [ic]: the composition made on another connection is carried over. */
    private fun carryOver(ic: EditorCalls) {
        forgetIfAnew()
        if (composed != null && composedIn !== ic) carry(ic, restore = false)
    }

    /**
     * Whether input started on another editor since the composition was made. The input
     * method's thread says so, and this editor's own thread acts on it.
     */
    @Volatile
    private var startedAnew = false

    private fun forgetIfAnew() {
        if (startedAnew) {
            startedAnew = false
            forget()
        }
    }

    override fun setComposing(text: String, cursor: Int): Boolean {
        val ic = target() ?: return false
        carryOver(ic)
        if (text.isEmpty()) {
            // Removes the composition and what it showed.
            ic.setComposingText("", 1)
            ic.finishComposingText()
            forget()
            return true
        }
        var at = text.length
        ic.beginBatchEdit()
        try {
            // 1 puts the cursor after the text, 0 before it; anywhere else is set after.
            ic.setComposingText(text, if (cursor == 0) 0 else 1)
            if (cursor == 0) at = 0
            if (cursor in 1 until text.length) {
                val end = ic.selectionEnd()
                if (end != null) {
                    val inside = end - (text.length - cursor)
                    ic.setSelection(inside, inside)
                    at = cursor
                }
            }
        } finally {
            ic.endBatchEdit()
        }
        composedIn = ic
        composed = text
        composedCursor = at
        return true
    }

    override fun commit(text: String): Boolean {
        val ic = target() ?: return false
        carryOver(ic)
        forget()
        return ic.commitText(text, 1)
    }

    override fun deleteSurrounding(before: Int, after: Int): Boolean =
        target()?.deleteSurroundingText(before, after) ?: false

    override fun setSelection(start: Int, end: Int): Boolean {
        val ic = target() ?: return false
        // The cursor no longer stands where it was in the composition.
        forget()
        return ic.setSelection(start, end)
    }

    override fun setComposingRegion(start: Int, end: Int): Boolean {
        val ic = target() ?: return false
        // The host says where the composition is from now on.
        forget()
        // An empty region ends the composition with its text as it is.
        return if (start == end) ic.finishComposingText() else ic.setComposingRegion(start, end)
    }

    override fun resumeComposition(): IntRange? {
        forgetIfAnew()
        val ic = target() ?: return null
        return carry(ic, restore = true)
    }

    /** Called on any thread. */
    override fun inputStarted(restarting: Boolean) {
        // Input started on another editor: what was composed belongs to the one before.
        if (!restarting) startedAnew = true
    }
}
