import Foundation

/// Which of DSA's fourteen interface languages the app shows, from the user's preferred
/// languages, by the rules every DSA interface follows.
///
/// Bundle's own matching is not left to pick it. It goes from a region to the language only when
/// the bundle has that language without a region, so it would send pt-PT, which DSA shows in
/// Brazilian Portuguese, to the next language in the list; and how it maps zh-HK, zh-MO and
/// zh-SG to a script depends on the macOS version. So the app resolves the language here and
/// reads that language's table itself.
public enum InterfaceLanguage {
    /// The String Catalog's language codes, English first, as `CFBundleLocalizations` lists them.
    public static let all = ["en", "ko", "ja", "zh-Hans", "zh-Hant", "es", "fr", "de", "pt-BR", "ru", "it", "vi", "id", "tr"]

    /// The catalog's code for one preferred language (`ko-KR`, `zh-Hant-HK`, `pt_PT`), or nil
    /// when DSA has no translation for it. Chinese follows its script: Traditional for `Hant` and
    /// for Taiwan, Hong Kong and Macao, Simplified otherwise. Portuguese is Brazilian Portuguese
    /// in every region. Indonesian answers to its old code `in` as well.
    public static func code(_ tag: String) -> String? {
        let parts = tag.trimmingCharacters(in: .whitespaces)
            .split(whereSeparator: { $0 == "." || $0 == "@" }).first.map(String.init) ?? ""
        let subtags = parts.split(whereSeparator: { $0 == "-" || $0 == "_" }).map { $0.lowercased() }
        guard let language = subtags.first else {
            return nil
        }
        let rest = subtags.dropFirst()
        switch language {
        case "en", "ko", "ja", "es", "fr", "de", "ru", "it", "vi", "tr":
            return language
        case "zh":
            let traditional = rest.contains { ["hant", "tw", "hk", "mo"].contains($0) } && !rest.contains("hans")
            return traditional ? "zh-Hant" : "zh-Hans"
        case "pt":
            return "pt-BR"
        case "id", "in":
            return "id"
        default:
            return nil
        }
    }

    /// The catalog's code for the first of `preferred` (most preferred first, as
    /// `Locale.preferredLanguages` lists them) that DSA has a translation for, else English.
    public static func resolve(_ preferred: [String]) -> String {
        preferred.lazy.compactMap(code).first ?? "en"
    }
}
