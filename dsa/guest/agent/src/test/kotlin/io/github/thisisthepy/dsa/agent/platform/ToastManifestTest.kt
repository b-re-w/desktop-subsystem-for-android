package io.github.thisisthepy.dsa.agent.platform

import java.io.File
import javax.xml.parsers.DocumentBuilderFactory
import org.junit.Assert.assertTrue
import org.junit.Test
import org.w3c.dom.Element

/**
 * The agent draws the apps' text toasts, and each toast's accessibility event has to name the
 * app that showed it: the agent sends a screen reader's toast announcement to that app's task
 * by the event's package. The accessibility manager replaces a package the sender does not own
 * with the sender's, unless the sender holds ACT_AS_PACKAGE_FOR_ACCESSIBILITY. The agent's
 * manifest asks for it, and its privileged permission list names it with the rest.
 */
class ToastManifestTest {
    private fun names(path: String, tag: String): Set<String> {
        val document = DocumentBuilderFactory.newInstance()
            .apply { isNamespaceAware = true }
            .newDocumentBuilder()
            .parse(File(path))
        val nodes = document.getElementsByTagName(tag)
        return (0 until nodes.length).map { (nodes.item(it) as Element) }
            .map { element ->
                element.getAttributeNS("http://schemas.android.com/apk/res/android", "name")
                    .ifEmpty { element.getAttribute("name") }
            }
            .toSet()
    }

    @Test
    fun fr40_the_agent_may_report_a_toast_as_its_app() {
        val permission = "android.permission.ACT_AS_PACKAGE_FOR_ACCESSIBILITY"
        assertTrue(permission in names("src/main/AndroidManifest.xml", "uses-permission"))
        assertTrue(permission in names("privapp-permissions-dsa-agent.xml", "permission"))
        // The status bar the notification manager sends text toasts to is the agent.
        assertTrue("android.permission.STATUS_BAR_SERVICE" in names("src/main/AndroidManifest.xml", "uses-permission"))
    }
}
