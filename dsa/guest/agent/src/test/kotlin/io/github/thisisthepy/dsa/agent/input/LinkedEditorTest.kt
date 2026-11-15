package io.github.thisisthepy.dsa.agent.input

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

/** An InputConnection that records each call an input method makes on it. */
class FakeCalls(private val name: String = "") : EditorCalls {
    val calls = mutableListOf<String>()
    var end: Int? = 10

    /** The editor's text before and after the cursor. */
    var before: String? = ""
    var after: String? = ""

    private fun call(what: String): Boolean {
        calls += if (name.isEmpty()) what else "$name $what"
        return true
    }

    override fun beginBatchEdit() = call("begin")
    override fun endBatchEdit() = call("end")
    override fun setComposingText(text: String, newCursorPosition: Int) = call("compose $text $newCursorPosition")
    override fun finishComposingText() = call("finish")
    override fun commitText(text: String, newCursorPosition: Int) = call("commit $text $newCursorPosition")
    override fun deleteSurroundingText(before: Int, after: Int) = call("delete $before $after")
    override fun setSelection(start: Int, end: Int) = call("select $start $end")
    override fun setComposingRegion(start: Int, end: Int) = call("region $start $end")
    override fun selectionEnd(): Int? = end
    override fun textBeforeCursor(length: Int): String? = before?.takeLast(length)
    override fun textAfterCursor(length: Int): String? = after?.take(length)
}

class LinkedEditorTest {
    private val started = FakeCalls("editor")
    private val fallback = FakeCalls("fallback")
    private var link: EditorLink? = EditorLink(started, fallback)
    private val editor = LinkedEditor { link }

    @Test
    fun fr6_each_composition_update_replaces_the_composing_text_in_one_batch() {
        assertTrue(editor.setComposing("ㅎ", 1))
        assertTrue(editor.setComposing("하", 1))
        assertEquals(
            listOf(
                "editor begin", "editor compose ㅎ 1", "editor end",
                "editor begin", "editor compose 하 1", "editor end",
            ),
            started.calls,
        )
        assertTrue(fallback.calls.isEmpty())
        assertTrue(editor.composesInline)
    }

    @Test
    fun fr6_a_commit_replaces_the_composition_and_an_empty_one_removes_it() {
        editor.setComposing("한", 1)
        editor.commit("한")
        editor.setComposing("ㄱ", 1)
        editor.setComposing("", 0)
        assertEquals(
            listOf("editor commit 한 1", "editor begin", "editor compose ㄱ 1", "editor end", "editor compose  1", "editor finish"),
            started.calls.drop(3),
        )
    }

    @Test
    fun fr6_a_cursor_inside_the_composition_is_set_after_it() {
        // The selection ends at 10 after "abc" is composed: the cursor after "a" is at 8.
        editor.setComposing("abc", 1)
        assertEquals(listOf("editor begin", "editor compose abc 1", "editor select 8 8", "editor end"), started.calls)
        started.calls.clear()
        editor.setComposing("abc", 0)
        assertEquals(listOf("editor begin", "editor compose abc 0", "editor end"), started.calls)
    }

    @Test
    fun fr6_an_empty_composing_region_ends_the_composition_and_keeps_its_text() {
        editor.setComposingRegion(4, 4)
        editor.setComposingRegion(2, 4)
        assertEquals(listOf("editor finish", "editor region 2 4"), started.calls)
    }

    @Test
    fun fr6_without_a_connection_of_its_own_the_editor_still_types_commits_but_shows_no_composition() {
        link = EditorLink(started = null, fallback = fallback)
        assertFalse(editor.composesInline)
        editor.commit("한")
        assertEquals(listOf("fallback commit 한 1"), fallback.calls)
        // No editor at all.
        link = null
        assertFalse(editor.setComposing("한", 1))
        assertFalse(editor.commit("한"))
    }

    @Test
    fun fr6_a_composition_goes_on_in_the_same_syllable_when_input_restarts_on_its_editor() {
        started.end = 1
        editor.setComposing("하", 1)
        // Input restarts: a new connection, on an editor that kept 하 before its cursor and
        // lost the composing span.
        val restarted = FakeCalls("restarted").apply {
            end = 1
            before = "하"
        }
        link = EditorLink(restarted, fallback)
        editor.setComposing("한", 1)
        assertEquals(
            listOf("restarted region 0 1", "restarted begin", "restarted compose 한 1", "restarted end"),
            restarted.calls,
        )
        // Later updates on the same connection go as ever.
        restarted.calls.clear()
        editor.setComposing("한", 1)
        assertEquals(listOf("restarted begin", "restarted compose 한 1", "restarted end"), restarted.calls)
    }

    @Test
    fun fr6_a_commit_after_input_restarted_replaces_the_syllable_rather_than_typing_it_twice() {
        editor.setComposing("한", 1)
        val restarted = FakeCalls("restarted").apply {
            end = 3
            before = "ab한"
        }
        link = EditorLink(restarted, fallback)
        editor.commit("한")
        assertEquals(listOf("restarted region 2 3", "restarted commit 한 1"), restarted.calls)
    }

    @Test
    fun fr6_an_editor_that_changed_as_input_restarted_composes_anew_at_its_cursor() {
        editor.setComposing("하", 1)
        // The app emptied the field as input restarted.
        val restarted = FakeCalls("restarted").apply {
            end = 0
            before = ""
        }
        link = EditorLink(restarted, fallback)
        editor.setComposing("ㄴ", 1)
        assertEquals(listOf("restarted begin", "restarted compose ㄴ 1", "restarted end"), restarted.calls)
        assertNull(editor.resumeComposition())
    }

    @Test
    fun fr6_resuming_a_composition_sets_its_region_again_and_says_where() {
        started.end = 5
        started.before = "abc하"
        editor.setComposing("하", 1)
        started.calls.clear()
        assertEquals(4 until 5, editor.resumeComposition())
        assertEquals(listOf("editor region 4 5"), started.calls)
        // Input started on another editor: what was composed is not this one's.
        editor.inputStarted(restarting = false)
        assertNull(editor.resumeComposition())
        // Nothing composed: nothing to resume.
        editor.setComposing("", 0)
        assertNull(editor.resumeComposition())
    }
}
