package io.github.thisisthepy.dsa.agent.platform

import java.io.File
import javax.xml.parsers.DocumentBuilderFactory
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import org.w3c.dom.Element

/**
 * The agent's texts that the user sees in Android ("Share on PC" and its messages) are in every
 * language DSA's interface has, with the same placeholders as English.
 */
class InterfaceLanguagesTest {
    /** The resource folder of each language besides English; Indonesian is `in` on Android. */
    private val folders = listOf(
        "values-ko", "values-ja", "values-zh-rCN", "values-zh-rTW", "values-es", "values-fr",
        "values-de", "values-pt-rBR", "values-ru", "values-it", "values-vi", "values-in", "values-tr",
    )

    private fun strings(folder: String): Map<String, String> {
        val file = File("src/main/res/$folder/strings.xml")
        assertTrue("$file is missing", file.exists())
        val nodes = DocumentBuilderFactory.newInstance().newDocumentBuilder().parse(file)
            .documentElement.getElementsByTagName("string")
        return (0 until nodes.length).map { nodes.item(it) as Element }
            .associate { it.getAttribute("name") to it.textContent }
    }

    private fun placeholders(text: String): List<String> =
        Regex("%(\\d+\\$)?[sd]").findAll(text).map { it.value }.sorted().toList()

    @Test
    fun fr17_every_translated_agent_text_is_in_every_interface_language() {
        val english = strings("values")
        val wanted = strings("values-ko").keys
        assertTrue(wanted.containsAll(listOf("share_target_name", "share_unavailable", "share_failed")))
        for (folder in folders) {
            val texts = strings(folder)
            assertEquals("$folder has other keys than Korean", wanted, texts.keys)
            for ((key, text) in texts) {
                assertTrue("$folder/$key is empty", text.isNotBlank())
                assertEquals("$folder/$key has other placeholders", placeholders(english.getValue(key)), placeholders(text))
                assertFalse("$folder/$key has an em dash", text.contains('—'))
                assertFalse("$folder/$key leaves an apostrophe unescaped", Regex("(?<!\\\\)'").containsMatchIn(
                    File("src/main/res/$folder/strings.xml").readText()))
            }
        }
    }
}
