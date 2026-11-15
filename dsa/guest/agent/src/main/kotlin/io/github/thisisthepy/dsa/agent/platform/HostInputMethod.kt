package io.github.thisisthepy.dsa.agent.platform

import android.graphics.Matrix
import android.graphics.RectF
import android.inputmethodservice.InputMethodService
import android.text.InputType
import android.view.inputmethod.CursorAnchorInfo
import android.view.inputmethod.EditorInfo
import android.view.inputmethod.ExtractedTextRequest
import android.view.inputmethod.InputConnection
import io.github.thisisthepy.dsa.agent.input.EditorCalls
import io.github.thisisthepy.dsa.agent.input.EditorConnection
import io.github.thisisthepy.dsa.agent.input.EditorLink
import io.github.thisisthepy.dsa.agent.input.EditorState
import io.github.thisisthepy.dsa.agent.input.LinkedEditor
import io.github.thisisthepy.dsa.agent.input.inputKind

/**
 * The guest's input method, which is not a keyboard: it shows nothing, ever, and types what
 * the host's input method composes and commits.
 *
 * The host is the shell, so the user types with the input method of the host, in a window of
 * the host. Android still delivers text to an editor only through an input method, so the
 * agent declares this one, makes it the system's only and default input method
 * ([AndroidInputMethodSetup]), and drives the focused editor's InputConnection from the host's
 * messages ([connection]). It never asks for a window: it has no input view, refuses to be
 * shown, and is never fullscreen, so no soft keyboard appears, with a hardware keyboard or
 * without one.
 *
 * The other way, it asks each editor it is bound to for its cursor and passes where it is on
 * the display to the agent, which tells the host, so the host's candidate list appears beside
 * it.
 * It also describes the editor: the kind of text it takes and its text around the selection,
 * each time the selection or the text changes, so the host's input method reconverts and
 * predicts from the editor's own text, and offers the keys for its kind.
 *
 * It runs in the agent's process (the activity manager binds it there), so the agent reaches
 * it directly.
 */
class HostInputMethod : InputMethodService() {
    override fun onCreate() {
        super.onCreate()
        current = this
    }

    override fun onDestroy() {
        if (current === this) current = null
        super.onDestroy()
    }

    override fun onEvaluateInputViewShown(): Boolean = false

    override fun onShowInputRequested(flags: Int, configChange: Boolean): Boolean = false

    override fun onEvaluateFullscreenMode(): Boolean = false

    override fun onStartInput(attribute: EditorInfo?, restarting: Boolean) {
        super.onStartInput(attribute, restarting)
        attribute?.let { info ->
            val through = if (link()?.started != null) {
                "the editor's own connection"
            } else {
                "only the app's fallback connection, in which no composition shows"
            }
            AndroidLog.info(
                "input started in ${info.packageName}: inputType=0x${Integer.toHexString(info.inputType)} " +
                    "imeOptions=0x${Integer.toHexString(info.imeOptions)} restarting=$restarting, through $through",
            )
        }
        // Where the cursor is now, and again whenever it moves.
        currentInputConnection?.requestCursorUpdates(
            InputConnection.CURSOR_UPDATE_IMMEDIATE or InputConnection.CURSOR_UPDATE_MONITOR,
        )
        val info = attribute ?: return
        describe(info.initialSelStart, info.initialSelEnd, -1, -1)
    }

    override fun onUpdateSelection(
        oldSelStart: Int,
        oldSelEnd: Int,
        newSelStart: Int,
        newSelEnd: Int,
        candidatesStart: Int,
        candidatesEnd: Int,
    ) {
        super.onUpdateSelection(oldSelStart, oldSelEnd, newSelStart, newSelEnd, candidatesStart, candidatesEnd)
        describe(newSelStart, newSelEnd, candidatesStart, candidatesEnd)
    }

    override fun onFinishInput() {
        AndroidLog.info("input finished in ${currentInputEditorInfo?.packageName}")
        super.onFinishInput()
        onEditor?.invoke(EditorState.NONE)
    }

    /**
     * Tells the agent what the focused editor is now: its kind, and its text around the
     * selection from [selStart] to [selEnd] with the composition from [composeStart] to
     * [composeEnd] (-1 for none), as the editor reported them.
     */
    private fun describe(selStart: Int, selEnd: Int, composeStart: Int, composeEnd: Int) {
        val report = onEditor ?: return
        val info = currentInputEditorInfo
        val ic = editor()
        if (info == null || ic == null) {
            report(EditorState.NONE)
            return
        }
        val kind = inputKind(info.inputType, info.imeOptions)
        // The editor's app, which tells the agent the task it is in even while the window
        // manager names no focused task.
        val owner = info.packageName.orEmpty()
        if (selStart < 0 || selEnd < 0) {
            report(EditorState(kind, "", 0, 0, 0, 0, 0, owner))
            return
        }
        val from = minOf(selStart, selEnd)
        val to = maxOf(selStart, selEnd)
        val selected = if (to > from) ic.getSelectedText(0)?.toString() else ""
        val fits = selected != null && selected.length == to - from &&
            selected.length <= EditorState.MAX_TEXT - 2 * EditorState.REACH
        if (selected == null || !fits) {
            // The selection is too long to describe with its text: the cursor at its end.
            report(EditorState(kind, "", selEnd, selEnd, selEnd, 0, 0, owner))
            return
        }
        val before = ic.getTextBeforeCursor(EditorState.REACH, 0)?.toString().orEmpty().takeLast(from)
        val after = ic.getTextAfterCursor(EditorState.REACH, 0)?.toString().orEmpty()
        val start = from - before.length
        val end = start + before.length + selected.length + after.length
        val composing = composeStart in start..end && composeEnd in composeStart..end
        report(
            EditorState(
                kind = kind,
                text = before + selected + after,
                textStart = start,
                selectionStart = selStart,
                selectionEnd = selEnd,
                compositionStart = if (composing) composeStart else 0,
                compositionEnd = if (composing) composeEnd else 0,
                packageName = owner,
            ),
        )
    }

    override fun onUpdateCursorAnchorInfo(info: CursorAnchorInfo) {
        val rect = cursorRect(info) ?: return
        onCursor?.invoke(rect)
    }

    /** The connection of the editor that takes text now; null when the focus is on no editor. */
    private fun editor(): InputConnection? {
        val info = currentInputEditorInfo ?: return null
        if (info.inputType and InputType.TYPE_MASK_CLASS == InputType.TYPE_NULL) return null
        return currentInputConnection
    }

    /**
     * The connections to the focused editor that takes text: the one the framework started
     * input with, which the input method's current connection is unless it falls back to its
     * binding's (the app's fallback BaseInputConnection), and that fallback. Null when the
     * focus is on no editor.
     */
    private fun link(): EditorLink? {
        val info = currentInputEditorInfo ?: return null
        if (info.inputType and InputType.TYPE_MASK_CLASS == InputType.TYPE_NULL) return null
        val bound = currentInputBinding?.connection
        val started = currentInputConnection?.takeIf { currentInputStarted && it !== bound }
        return EditorLink(started?.let(::calls), bound?.let(::calls))
    }

    /** The calls on [ic], made once for each connection. */
    private fun calls(ic: InputConnection): EditorCalls {
        wrapped?.let { (connection, calls) -> if (connection === ic) return calls }
        val calls = ConnectionCalls(ic)
        wrapped = ic to calls
        return calls
    }

    @Volatile
    private var wrapped: Pair<InputConnection, EditorCalls>? = null

    /** An editor's InputConnection, called as [LinkedEditor] asks. */
    private class ConnectionCalls(private val ic: InputConnection) : EditorCalls {
        override fun beginBatchEdit() = ic.beginBatchEdit()
        override fun endBatchEdit() = ic.endBatchEdit()
        override fun setComposingText(text: String, newCursorPosition: Int) = ic.setComposingText(text, newCursorPosition)
        override fun finishComposingText() = ic.finishComposingText()
        override fun commitText(text: String, newCursorPosition: Int) = ic.commitText(text, newCursorPosition)
        override fun deleteSurroundingText(before: Int, after: Int) = ic.deleteSurroundingText(before, after)
        override fun setSelection(start: Int, end: Int) = ic.setSelection(start, end)
        override fun setComposingRegion(start: Int, end: Int) = ic.setComposingRegion(start, end)
        override fun selectionEnd(): Int? =
            ic.getExtractedText(ExtractedTextRequest(), 0)?.let { it.startOffset + it.selectionEnd }
    }

    companion object {
        @Volatile
        private var current: HostInputMethod? = null

        /**
         * Whether a text field takes typed text now: the input method is bound to an editor of
         * the focused window. A window with no editor still starts input with the input method,
         * but with no connection, which the input method manager marks with a field ID of 0; an
         * editor that takes raw keys (a terminal, whose input type is TYPE_NULL) counts too, so
         * Backspace there stays Backspace.
         */
        val editorFocused: Boolean
            get() {
                val method = current ?: return false
                val info = method.currentInputEditorInfo ?: return false
                if (!method.currentInputStarted || method.currentInputConnection == null) return false
                return info.inputType and InputType.TYPE_MASK_CLASS != InputType.TYPE_NULL || info.fieldId != 0
            }

        /** Where the cursor of the focused editor goes, in display pixels; called on any thread. */
        @Volatile
        var onCursor: ((RectF) -> Unit)? = null

        /** Where the focused editor's kind and text go; called on the input method's thread. */
        @Volatile
        var onEditor: ((EditorState) -> Unit)? = null

        /** The focused editor, through the input method; the agent calls it on its own thread. */
        val connection: EditorConnection = LinkedEditor { current?.link() }

        /**
         * The cursor's rectangle in display pixels: the insertion marker if the editor gives
         * one, else the last character of the composition.
         */
        private fun cursorRect(info: CursorAnchorInfo): RectF? {
            val local = if (!info.insertionMarkerHorizontal.isNaN()) {
                val x = info.insertionMarkerHorizontal
                RectF(x, info.insertionMarkerTop, x + 1f, info.insertionMarkerBottom)
            } else {
                val start = info.composingTextStart
                val text = info.composingText ?: return null
                if (start < 0 || text.isEmpty()) return null
                info.getCharacterBounds(start + text.length - 1) ?: return null
            }
            val toScreen = Matrix(info.matrix)
            toScreen.mapRect(local)
            return local
        }
    }
}
