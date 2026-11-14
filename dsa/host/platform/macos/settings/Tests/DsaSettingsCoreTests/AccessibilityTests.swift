import DsaSettingsCore
import XCTest

/// The settings app works with VoiceOver, as the Windows settings app works with a screen
/// reader: every control has a name, a control repeated in every row says whose it is, page and
/// section titles are headings, a banner is read out when it appears, the administrator shield
/// is not only a tool tip, and a log line reads as a sentence. The words come from the core and
/// the String Catalog; the views are read as text, every file of the app, so a page added later
/// is held to the same rules.
final class AccessibilityTests: XCTestCase {
    private var package: URL {
        URL(fileURLWithPath: #filePath).deletingLastPathComponent().deletingLastPathComponent().deletingLastPathComponent()
    }

    /// The texts of one language of the String Catalog, as the app reads them.
    private final class CatalogText: TextSource {
        let values: [String: String]

        init(_ values: [String: String]) {
            self.values = values
        }

        func get(_ key: String) -> String { values[key] ?? key }
    }

    private func catalog(_ language: String) throws -> CatalogText {
        let data = try Data(contentsOf: package.appendingPathComponent("Resources/Localizable.xcstrings"))
        let root = try XCTUnwrap(try JSONSerialization.jsonObject(with: data) as? [String: Any])
        let strings = try XCTUnwrap(root["strings"] as? [String: Any])
        var values: [String: String] = [:]
        for (key, entry) in strings {
            let unit = ((entry as? [String: Any])?["localizations"] as? [String: Any])?[language] as? [String: Any]
            values[key] = (unit?["stringUnit"] as? [String: Any])?["value"] as? String
        }
        return CatalogText(values)
    }

    /// Every view file of the app, by name.
    private func views() throws -> [(String, String)] {
        let folder = package.appendingPathComponent("Sources/DsaSettings")
        return try FileManager.default.contentsOfDirectory(at: folder, includingPropertiesForKeys: nil)
            .filter { $0.pathExtension == "swift" }
            .sorted { $0.lastPathComponent < $1.lastPathComponent }
            .map { ($0.lastPathComponent, try String(contentsOf: $0, encoding: .utf8).replacingOccurrences(of: "\r\n", with: "\n")) }
    }

    private func view(_ name: String) throws -> String {
        try XCTUnwrap(try views().first { $0.0 == name }?.1)
    }

    /// Whether `marker` follows line `index` of `lines` within `within` lines, the line itself included.
    private func follows(_ lines: [String], _ index: Int, _ marker: String, within: Int = 4) -> Bool {
        lines[index..<min(lines.count, index + within + 1)].contains { $0.contains(marker) }
    }

    func test_fr18_log_line_reads_as_text() throws {
        let line = try XCTUnwrap(LogLine.parse(
            #"{"time":"2026-11-04 10:00:00.000","priority":"W","pid":12,"tid":13,"tag":"ActivityManager","message":"Start proc"}"#))
        let dropped = try XCTUnwrap(LogLine.parse(#"{"priority":"W","dropped":3}"#))
        let english = try catalog("en")
        XCTAssertEqual(AccessibleText.logLine(english, line), "2026-11-04 10:00:00.000, Warning, ActivityManager: Start proc")
        XCTAssertEqual(AccessibleText.logLine(english, dropped), "3 records dropped")
        let korean = try catalog("ko")
        XCTAssertEqual(AccessibleText.logLine(korean, line), "2026-11-04 10:00:00.000, 경고, ActivityManager: Start proc")
        XCTAssertEqual(AccessibleText.logLine(korean, dropped), "기록 3개가 빠졌습니다")

        // The log page's rows read those words, not their columns.
        let pages = try view("AndroidPages.swift")
        XCTAssertTrue(pages.contains(".accessibilityLabel(AccessibleText.logLine(appText, line))"))
    }

    func test_fr18_repeated_item_buttons_name_their_item() throws {
        XCTAssertEqual(AccessibleText.itemName(try catalog("en"), "AppsOpenButton.Content", "Notes"), "Open, Notes")
        XCTAssertEqual(AccessibleText.itemName(try catalog("ko"), "AppsOpenButton.Content", "Notes"), "Notes 열기")
        // A control whose row is itself keeps its own name.
        XCTAssertEqual(AccessibleText.itemName(try catalog("en"), label: "Open", item: ""), "Open")

        let pages = try view("AndroidPages.swift")
        for wanted in [
            #".accessibilityLabel(name("AppsOpenButton.Content"))"#,
            #".accessibilityLabel(name("AppsInfoButton.Content"))"#,
            #".accessibilityLabel(name("AppsDesktopShortcutButton.Content"))"#,
            #".accessibilityLabel(name("AppsUninstallButton.Content"))"#,
            #".accessibilityLabel(name("AppsResizable.Content"))"#,
            #".accessibilityLabel(name("AppsRunAsArm.Content"))"#,
            #".accessibilityLabel(name("AppsStartSize.ToolTip"))"#,
            #".accessibilityLabel(name("AppsRoot.Name"))"#,
            #"AccessibleText.itemName(appText, "BackgroundOpenButton.Content", app.label)"#,
            #"AccessibleText.itemName(appText, "BackgroundStopButton.Content", app.label)"#,
            #"AccessibleText.itemName(appText, "WidgetsRemoveButton.Content", widget.label)"#,
            #"AccessibleText.itemName(appText, "WidgetsAddButton.Content", offer.label)"#,
        ] {
            XCTAssertTrue(pages.contains(wanted), "AndroidPages.swift has no \(wanted)")
        }
        // A button in a row of its own says the row's title, and each copy button what it copies.
        let components = try view("Components.swift")
        XCTAssertTrue(components.contains("AccessibleText.itemName(appText, label: action, item: title)"))
        XCTAssertTrue(components.contains("CopyButton(text: command, item: command)"))
        XCTAssertTrue(try view("SystemPages.swift").contains(#"AccessibleText.itemName(appText, "ChangeAsAdmin.Content", title)"#))
    }

    func test_fr18_every_toggle_combo_and_search_box_has_a_name() throws {
        var unnamed: [String] = []
        for (file, source) in try views() {
            let lines = source.components(separatedBy: "\n")
            for (index, line) in lines.enumerated() {
                // A control with no label of its own on screen.
                let bare = line.contains(#"Toggle("""#) || line.contains(#"Picker("""#) || line.contains(#"TextField("""#)
                    || line.contains(".labelsHidden()")
                if bare && !follows(lines, index, ".accessibilityLabel(", within: 6) {
                    unnamed.append("\(file):\(index + 1): \(line.trimmingCharacters(in: .whitespaces))")
                }
            }
        }
        XCTAssertEqual(unnamed, [], "these controls have no name for VoiceOver")
    }

    func test_fr18_every_progress_indicator_has_a_name() throws {
        var unnamed: [String] = []
        for (file, source) in try views() {
            let lines = source.components(separatedBy: "\n")
            for (index, line) in lines.enumerated() where line.contains("ProgressView(") {
                if !follows(lines, index, ".accessibilityLabel(", within: 1) {
                    unnamed.append("\(file):\(index + 1)")
                }
            }
        }
        XCTAssertEqual(unnamed, [], "these progress indicators have no name")
    }

    func test_fr18_icon_only_buttons_have_names() throws {
        // The app's buttons that show only a symbol: the copy button and the install bar's close.
        let components = try view("Components.swift")
        let copy = try XCTUnwrap(components.range(of: "struct CopyButton"))
        XCTAssertTrue(components[copy.lowerBound...].contains(".accessibilityLabel(AccessibleText.itemName(appText, label: T(\"CopyButton.ToolTip\"), item: item))"))
        let window = try view("SettingsWindow.swift")
        XCTAssertTrue(window.contains(#".accessibilityLabel(T("A11y_Close"))"#))
        // A link row is a button whose words are its title, not the chevron after it.
        XCTAssertTrue(components.contains(".accessibilityHidden(true)"))
    }

    func test_fr18_page_titles_and_section_headers_have_heading_levels() throws {
        let components = try view("Components.swift")
        XCTAssertTrue(components.contains(".accessibilityHeading(.h1)"))
        XCTAssertTrue(components.contains(".accessibilityHeading(.h2)"))
        XCTAssertTrue(components.contains("init(heading: String"))
        XCTAssertTrue(try view("SettingsWindow.swift").contains("PageHeading(text: T((page ?? .general).titleKey))"))
        var plain: [String] = []
        for (file, source) in try views() {
            let lines = source.components(separatedBy: "\n")
            for (index, line) in lines.enumerated() {
                // A section titled with a plain text, which VoiceOver does not list as a heading.
                if line.contains("Section(T(") {
                    plain.append("\(file):\(index + 1)")
                }
                if line.contains("} header: {") && !follows(lines, index, "SectionHeading(", within: 3) {
                    plain.append("\(file):\(index + 1)")
                }
            }
        }
        XCTAssertEqual(plain, [], "these section titles are not headings")
    }

    func test_fr18_every_banner_announces() throws {
        let components = try view("Components.swift")
        let banner = try XCTUnwrap(components.range(of: "struct Banner: View"))
        let body = components[banner.lowerBound...]
        XCTAssertTrue(body.contains(".onAppear { tell() }"))
        XCTAssertTrue(body.contains(".onChange(of: words) { _ in tell() }"))
        XCTAssertTrue(components.contains(".announcementRequested"))
        // Only the banners that are always on their page are read with it instead.
        var quiet: [String] = []
        for (file, source) in try views() {
            for line in source.components(separatedBy: "\n") where line.contains("announces: false") {
                quiet.append(file)
            }
        }
        XCTAssertEqual(quiet.sorted(), ["AndroidPages.swift", "SystemPages.swift", "SystemPages.swift"])

        let english = try catalog("en")
        XCTAssertEqual(AccessibleText.notice(english, title: "Problem", message: ""), "Problem")
        XCTAssertEqual(AccessibleText.notice(english, title: "", message: "Gone"), "Gone")
        XCTAssertEqual(AccessibleText.notice(english, title: "Problem", message: "Gone"), "Problem. Gone")
    }

    func test_fr18_shield_info_is_not_tooltip_only() throws {
        var tooltipOnly: [String] = []
        var shields = 0
        for (file, source) in try views() {
            let lines = source.components(separatedBy: "\n")
            for (index, line) in lines.enumerated() where line.contains(#".help(T("AdminShield.ToolTip"))"#) {
                shields += 1
                if !follows(lines, index, #".accessibilityLabel(T("AdminShield.ToolTip"))"#, within: 1) {
                    tooltipOnly.append("\(file):\(index + 1)")
                }
            }
        }
        XCTAssertGreaterThan(shields, 0)
        XCTAssertEqual(tooltipOnly, [], "these shields say an administrator is needed only in a tool tip")
        // The switch an administrator changes says it too, since the shield takes no focus.
        let system = try view("SystemPages.swift")
        let row = try XCTUnwrap(system.range(of: "struct AdministratorSwitchRow"))
        XCTAssertTrue(system[row.lowerBound...].contains(#".accessibilityHint(detail.isEmpty ? T("AdminShield.ToolTip")"#))
        XCTAssertGreaterThanOrEqual(system.components(separatedBy: #".accessibilityHint(T("AdminShield.ToolTip"))"#).count - 1, 3)
    }

    func test_fr18_new_strings_exist_in_korean_and_english() throws {
        let english = try catalog("en")
        let korean = try catalog("ko")
        func holes(_ text: String) -> [String] {
            (0..<4).map { "{\($0)}" }.filter { text.contains($0) }
        }
        for key in ["A11y_ItemName", "A11y_Notice", "A11y_LogLine", "A11y_LogDropped", "A11y_LogPriority",
                    "A11y_Loading", "A11y_Copied", "A11y_Close"] {
            let en = try XCTUnwrap(english.values[key], "\(key) has no English text")
            let ko = try XCTUnwrap(korean.values[key], "\(key) has no Korean text")
            XCTAssertFalse(en.isEmpty || ko.isEmpty, key)
            XCTAssertEqual(holes(en), holes(ko), "\(key) says other things in English and Korean")
        }
    }
}
