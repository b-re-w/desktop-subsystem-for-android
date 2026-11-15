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
 */
class LinkedEditor(private val link: () -> EditorLink?) : EditorConnection {
    override val composesInline: Boolean
        get() = link()?.started != null

    private fun target(): EditorCalls? = link()?.let { it.started ?: it.fallback }

    override fun setComposing(text: String, cursor: Int): Boolean {
        val ic = target() ?: return false
        if (text.isEmpty()) {
            // Removes the composition and what it showed.
            ic.setComposingText("", 1)
            ic.finishComposingText()
            return true
        }
        ic.beginBatchEdit()
        try {
            // 1 puts the cursor after the text, 0 before it; anywhere else is set after.
            ic.setComposingText(text, if (cursor == 0) 0 else 1)
            if (cursor in 1 until text.length) {
                val end = ic.selectionEnd()
                if (end != null) {
                    val at = end - (text.length - cursor)
                    ic.setSelection(at, at)
                }
            }
        } finally {
            ic.endBatchEdit()
        }
        return true
    }

    override fun commit(text: String): Boolean = target()?.commitText(text, 1) ?: false

    override fun deleteSurrounding(before: Int, after: Int): Boolean =
        target()?.deleteSurroundingText(before, after) ?: false

    override fun setSelection(start: Int, end: Int): Boolean = target()?.setSelection(start, end) ?: false

    override fun setComposingRegion(start: Int, end: Int): Boolean {
        val ic = target() ?: return false
        // An empty region ends the composition with its text as it is.
        return if (start == end) ic.finishComposingText() else ic.setComposingRegion(start, end)
    }
}
