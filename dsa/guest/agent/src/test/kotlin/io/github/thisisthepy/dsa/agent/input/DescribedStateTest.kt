package io.github.thisisthepy.dsa.agent.input

import io.github.thisisthepy.dsa.control.v1.ImeInputKind
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class DescribedStateTest {
    private val kind = ImeInputKind.IME_INPUT_KIND_SEARCH
    private val owner = "com.android.vending"

    /** One state of an editor: its whole text, the cursor, and the composition (-1 for none). */
    private data class Shown(val text: String, val cursor: Int, val composeStart: Int, val composeEnd: Int) {
        fun snapshot() = EditorSnapshot(text, 0, cursor, cursor)

        fun state() = EditorState(
            kind = ImeInputKind.IME_INPUT_KIND_SEARCH,
            text = text,
            textStart = 0,
            selectionStart = cursor,
            selectionEnd = cursor,
            compositionStart = if (composeStart >= 0) composeStart else 0,
            compositionEnd = if (composeStart >= 0) composeEnd else 0,
            packageName = "com.android.vending",
        )
    }

    /**
     * The states a Compose text field passes through as the host types [steps] (what the input
     * method commits, then what it composes next), each commit and each composition its own
     * edit, as the agent's input method sends them.
     */
    private fun statesOf(steps: List<Pair<String?, String?>>): List<Shown> {
        val states = mutableListOf<Shown>()
        var committed = ""
        for ((result, composition) in steps) {
            if (result != null) {
                committed += result
                states += Shown(committed, committed.length, -1, -1)
            }
            if (composition != null) {
                states += Shown(committed + composition, committed.length + composition.length, committed.length, committed.length + composition.length)
            }
        }
        return states
    }

    private val annyeong = listOf(
        null to "ㅇ", null to "아", null to "안", "안" to "ㄴ", null to "녀", null to "녕",
        "녕" to "ㅎ", null to "하", null to "핫", "하" to "세", null to "셍", "세" to "요", "요" to null,
    )

    private val hangul = listOf(
        null to "ㅎ", null to "하", null to "한", "한" to "ㄱ", null to "그", null to "글", "글" to null,
    )

    private fun report(shown: Shown, snapshot: EditorSnapshot, fromUpdate: Boolean = true) =
        describedState(kind, owner, shown.cursor, shown.cursor, shown.composeStart, shown.composeEnd, snapshot, fromUpdate)

    @Test
    fun fr6_a_compose_field_read_after_it_moved_on_is_never_described_as_a_state_it_never_had() {
        for ((typed, steps) in listOf("안녕하세요" to annyeong, "한글" to hangul)) {
            val states = statesOf(steps)
            // The editor's text is read 0 to several edits after the state it reported.
            for (late in 0..4) {
                val described = states.indices.mapNotNull { at ->
                    report(states[at], states[minOf(at + late, states.lastIndex)].snapshot())
                }
                val had = states.map(Shown::state)
                for (state in described) {
                    assertTrue("$typed, read $late edits late: $state is not a state the editor had", state in had)
                }
                // The last report is read at once, and describes where the editor ends.
                assertEquals("$typed, read $late edits late", states.last().state(), described.last())
                assertEquals(typed, described.last().text)
            }
        }
    }

    @Test
    fun fr6_the_commit_before_the_next_syllable_read_with_that_syllable_in_is_not_described() {
        // 안 committed, cursor after it, as the editor reported; ㄴ is already in when it is read.
        val committed = Shown("안", 1, -1, -1)
        val next = Shown("안ㄴ", 2, 1, 2)
        assertNull(report(committed, next.snapshot()))
        // Read before ㄴ came, it is described as it was.
        assertEquals(committed.state(), report(committed, committed.snapshot()))
        // The next syllable keeps its composition.
        assertEquals(next.state(), report(next, next.snapshot()))
    }

    @Test
    fun fr6_input_restarting_on_a_compose_field_takes_the_selection_it_read() {
        // The editor's start reports the selection it had; by the read, 하 composed in.
        val read = Shown("하", 1, 0, 1).snapshot()
        val state = describedState(kind, owner, 0, 0, -1, -1, read, fromUpdate = false)
        assertEquals(EditorState(kind, "하", 0, 1, 1, 0, 0, owner), state)
    }

    @Test
    fun fr6_a_long_field_is_described_around_its_selection_without_splitting_a_character() {
        val reach = EditorState.REACH
        val emoji = "😀"
        // An emoji straddles where the reach before the cursor ends, and one where it ends after.
        val before = "a".repeat(10) + emoji + "b".repeat(reach - 1)
        val after = "c".repeat(reach - 1) + emoji + "d".repeat(10)
        val text = before + after
        val cursor = before.length
        val state = describedState(kind, owner, cursor, cursor, -1, -1, EditorSnapshot(text, 0, cursor, cursor), fromUpdate = true)!!
        assertEquals(emoji + "b".repeat(reach - 1) + "c".repeat(reach - 1) + emoji, state.text)
        assertEquals(10, state.textStart)
        assertEquals(cursor, state.selectionStart)
        // A field read whole from a later offset keeps its place in the editor's text.
        val shifted = describedState(kind, owner, 105, 105, 100, 105, EditorSnapshot("hello", 100, 105, 105), fromUpdate = true)!!
        assertEquals(EditorState(kind, "hello", 100, 105, 105, 100, 105, owner), shifted)
    }
}
