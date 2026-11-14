import DsaSettingsCore
import XCTest

/// The settings app shows the first of the user's preferred languages that DSA has, by the rules
/// every DSA interface follows, and English when it has none.
final class InterfaceLanguageTests: XCTestCase {
    private var package: URL {
        URL(fileURLWithPath: #filePath).deletingLastPathComponent().deletingLastPathComponent().deletingLastPathComponent()
    }

    func test_fr17_the_first_preferred_language_with_a_translation_is_shown() {
        let cases: [([String], String)] = [
            (["ko-KR"], "ko"),
            (["ja-JP"], "ja"),
            (["zh-Hant-TW"], "zh-Hant"),
            (["zh-Hant-HK"], "zh-Hant"),
            (["zh-Hans-CN"], "zh-Hans"),
            (["zh-Hans-SG"], "zh-Hans"),
            (["pt-PT"], "pt-BR"),
            (["pt-BR"], "pt-BR"),
            (["nl-NL", "de-DE"], "de"),
            (["nl-NL"], "en"),
            ([], "en"),
        ]
        for (preferred, expected) in cases {
            XCTAssertEqual(InterfaceLanguage.resolve(preferred), expected, "\(preferred)")
        }
    }

    func test_fr17_chinese_follows_its_script_and_region() {
        for tag in ["zh-Hant", "zh-TW", "zh-HK", "zh-MO", "zh_TW", "zh-Hant-MO", "ZH-hant"] {
            XCTAssertEqual(InterfaceLanguage.code(tag), "zh-Hant", tag)
        }
        for tag in ["zh", "zh-Hans", "zh-CN", "zh-SG", "zh-Hans-HK", "zh-Hans-MO"] {
            XCTAssertEqual(InterfaceLanguage.code(tag), "zh-Hans", tag)
        }
    }

    func test_fr17_every_region_of_a_language_finds_its_translation() {
        let cases = [
            ("en-GB", "en"), ("ko", "ko"), ("es-419", "es"), ("fr-CA", "fr"), ("de-CH", "de"),
            ("pt", "pt-BR"), ("pt-AO", "pt-BR"), ("ru-RU", "ru"), ("it-IT", "it"), ("vi-VN", "vi"),
            ("id-ID", "id"), ("in", "id"), ("tr-TR", "tr"), ("ja_JP.UTF-8", "ja"),
        ]
        for (tag, expected) in cases {
            XCTAssertEqual(InterfaceLanguage.code(tag), expected, tag)
        }
        for tag in ["nl", "pl-PL", "sv", "kok", "", " "] {
            XCTAssertNil(InterfaceLanguage.code(tag), tag)
        }
    }

    func test_fr17_every_language_is_its_own_code() {
        XCTAssertEqual(InterfaceLanguage.all.count, 14)
        for code in InterfaceLanguage.all {
            XCTAssertEqual(InterfaceLanguage.code(code), code)
        }
    }

    /// The codes `CFBundleLocalizations` lists in a packaging `Info.plist`.
    private func declared(_ plist: String) throws -> [String] {
        let url = package.deletingLastPathComponent().appendingPathComponent("packaging/\(plist)")
        let data = try Data(contentsOf: url)
        let root = try XCTUnwrap(try PropertyListSerialization.propertyList(from: data, format: nil) as? [String: Any])
        return try XCTUnwrap(root["CFBundleLocalizations"] as? [String], "\(plist) declares no languages")
    }

    func test_fr17_the_app_declares_every_interface_language() throws {
        XCTAssertEqual(try declared("Info.plist"), InterfaceLanguage.all)
        XCTAssertEqual(try declared("window-host-Info.plist"), InterfaceLanguage.all)
        // And the build checks the String Catalog gave a table for each.
        let script = try String(
            contentsOf: package.deletingLastPathComponent().appendingPathComponent("packaging/build-app.sh"), encoding: .utf8)
        XCTAssertTrue(script.contains("for language in \(InterfaceLanguage.all.joined(separator: " ")); do"),
                      "build-app.sh does not check a table for every language")
    }
}
