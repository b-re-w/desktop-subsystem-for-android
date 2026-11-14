//! The language the macOS host's own words are shown in: its menus, the back button, the
//! questions it asks in the Notification Center, the screenshots' file names.
//!
//! macOS keeps the user's languages in order in System Settings (General, Language & Region),
//! and `NSLocale.preferredLanguages` lists them most preferred first. The host shows the first
//! of them it has a translation for, and English when it has none, as a Mac app with those
//! localizations does: a user who reads Dutch first and German second gets German.
//!
//! The text handed to Android (the guest's own locale) is not this: it follows the full list of
//! the user's languages, whether DSA has words in them or not.

pub use dsa_host_core::ui_language::UiLanguage;

/// The language for the user's preferred languages `languages` (`NSLocale.preferredLanguages`,
/// most preferred first, such as `ko-KR` or `zh-Hant-TW`): the first DSA has a translation
/// for, English when there is none.
pub fn of_preferred<'a>(languages: impl IntoIterator<Item = &'a str>) -> UiLanguage {
    UiLanguage::first_supported(languages)
}

/// The language the user reads, from `NSLocale.preferredLanguages` as it is now, so a change in
/// System Settings shows in the next menu or question. English when macOS lists none.
#[cfg(target_os = "macos")]
pub fn current() -> UiLanguage {
    use objc2::msg_send;
    use objc2::runtime::AnyObject;

    use crate::objc::{class, string_of};

    // SAFETY: `NSLocale.preferredLanguages` may be asked from any thread; it returns an array
    // of strings, read here before the autorelease pool drains.
    objc2::rc::autoreleasepool(|_| unsafe {
        let Ok(locale) = class(c"NSLocale") else {
            return UiLanguage::English;
        };
        let languages: *mut AnyObject = msg_send![locale, preferredLanguages];
        if languages.is_null() {
            return UiLanguage::English;
        }
        let count: usize = msg_send![languages, count];
        let tags: Vec<String> = (0..count)
            .map(|i| string_of(msg_send![languages, objectAtIndex: i]))
            .collect();
        of_preferred(tags.iter().map(String::as_str))
    })
}

/// Whether `text` holds an em dash (U+2014), which DSA's words never use, in any language.
#[cfg(test)]
pub(crate) fn has_em_dash(text: &str) -> bool {
    text.contains('\u{2014}')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fr17_the_macos_host_shows_the_first_preferred_language_it_has() {
        let cases: [(&[&str], UiLanguage); 11] = [
            (&["ko-KR"], UiLanguage::Korean),
            (&["ja-JP"], UiLanguage::Japanese),
            (&["zh-Hant-TW"], UiLanguage::ChineseTraditional),
            (&["zh-Hant-HK"], UiLanguage::ChineseTraditional),
            (&["zh-Hans-CN"], UiLanguage::ChineseSimplified),
            (&["zh-Hans-SG"], UiLanguage::ChineseSimplified),
            (&["pt-PT"], UiLanguage::PortugueseBrazil),
            (&["pt-BR"], UiLanguage::PortugueseBrazil),
            (&["nl-NL", "de-DE"], UiLanguage::German),
            (&["nl-NL"], UiLanguage::English),
            (&[], UiLanguage::English),
        ];
        for (languages, expected) in cases {
            assert_eq!(
                of_preferred(languages.iter().copied()),
                expected,
                "{languages:?}"
            );
        }
    }

    #[test]
    fn fr17_every_macos_region_of_a_language_finds_its_translation() {
        let cases = [
            ("en-US", UiLanguage::English),
            ("en-GB", UiLanguage::English),
            ("zh-HK", UiLanguage::ChineseTraditional),
            ("zh-MO", UiLanguage::ChineseTraditional),
            ("zh-TW", UiLanguage::ChineseTraditional),
            ("zh-SG", UiLanguage::ChineseSimplified),
            ("zh-CN", UiLanguage::ChineseSimplified),
            ("es-419", UiLanguage::Spanish),
            ("fr-CA", UiLanguage::French),
            ("de-AT", UiLanguage::German),
            ("ru-RU", UiLanguage::Russian),
            ("it-IT", UiLanguage::Italian),
            ("vi-VN", UiLanguage::Vietnamese),
            ("id-ID", UiLanguage::Indonesian),
            ("tr-TR", UiLanguage::Turkish),
        ];
        for (tag, expected) in cases {
            assert_eq!(of_preferred([tag]), expected, "{tag}");
        }
    }
}
