//! An app's request to put one of its shortcuts on the desktop (`requestPinShortcut`), as
//! the desktop asks about it: a notification with [Add to desktop] and [Cancel]. On yes the
//! compositor writes the shortcut's desktop entry ([`super::desktop_entry::place_pinned_in`]);
//! a question nobody answers is declined after [`ASK_FOR`], so the app is not left waiting.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use dsa_host_core::tasks::PinShortcutRequest;
use dsa_host_core::ui_language::UiLanguage;

use super::desktop_entry::app_id;
use super::notifications::{body_text, Notify};

/// The action of [Add to desktop].
pub const ADD: &str = "dsa-pin-add";

/// The action of [Cancel].
pub const CANCEL: &str = "dsa-pin-cancel";

/// How long a question waits for the user before it is declined.
pub const ASK_FOR: Duration = Duration::from_secs(180);

/// The question's words in one language: its summary (with `{name}`, the shortcut's name),
/// its body (with `{app}`, the app's), and the two buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuestionText {
    pub summary: &'static str,
    pub body: &'static str,
    pub add: &'static str,
    pub cancel: &'static str,
}

/// The question's words in `language`.
pub fn question_text(language: UiLanguage) -> QuestionText {
    let words = |summary, body, add, cancel| QuestionText {
        summary,
        body,
        add,
        cancel,
    };
    match language {
        UiLanguage::English => words(
            "Add \u{201c}{name}\u{201d} to the desktop?",
            "{app} wants to put this shortcut on your desktop.",
            "Add to desktop",
            "Cancel",
        ),
        UiLanguage::Korean => words(
            "\u{201c}{name}\u{201d}을(를) 바탕 화면에 추가할까요?",
            "{app}에서 이 바로 가기를 바탕 화면에 두려고 합니다.",
            "바탕 화면에 추가",
            "취소",
        ),
        UiLanguage::Japanese => words(
            "「{name}」をデスクトップに追加しますか？",
            "{app} がこのショートカットをデスクトップに置こうとしています。",
            "デスクトップに追加",
            "キャンセル",
        ),
        UiLanguage::ChineseSimplified => words(
            "要将\u{201c}{name}\u{201d}添加到桌面吗？",
            "{app} 想将此快捷方式放到你的桌面上。",
            "添加到桌面",
            "取消",
        ),
        UiLanguage::ChineseTraditional => words(
            "要將「{name}」新增到桌面嗎？",
            "{app} 想將這個捷徑放到你的桌面上。",
            "新增到桌面",
            "取消",
        ),
        UiLanguage::Spanish => words(
            "¿Desea añadir \u{ab}{name}\u{bb} al escritorio?",
            "{app} quiere poner este acceso directo en su escritorio.",
            "Añadir al escritorio",
            "Cancelar",
        ),
        UiLanguage::French => words(
            "Ajouter \u{ab}\u{a0}{name}\u{a0}\u{bb} au bureau\u{a0}?",
            "{app} souhaite placer ce raccourci sur votre bureau.",
            "Ajouter au bureau",
            "Annuler",
        ),
        UiLanguage::German => words(
            "\u{201e}{name}\u{201c} zum Desktop hinzufügen?",
            "{app} möchte diese Verknüpfung auf Ihrem Desktop ablegen.",
            "Zum Desktop hinzufügen",
            "Abbrechen",
        ),
        UiLanguage::PortugueseBrazil => words(
            "Adicionar \u{201c}{name}\u{201d} à área de trabalho?",
            "{app} quer colocar este atalho na sua área de trabalho.",
            "Adicionar à área de trabalho",
            "Cancelar",
        ),
        UiLanguage::Russian => words(
            "Добавить \u{ab}{name}\u{bb} на рабочий стол?",
            "{app} хочет поместить этот ярлык на ваш рабочий стол.",
            "Добавить на рабочий стол",
            "Отмена",
        ),
        UiLanguage::Italian => words(
            "Aggiungere \u{201c}{name}\u{201d} al desktop?",
            "{app} vuole aggiungere questa scorciatoia al desktop.",
            "Aggiungi al desktop",
            "Annulla",
        ),
        UiLanguage::Vietnamese => words(
            "Thêm \u{201c}{name}\u{201d} vào màn hình nền?",
            "{app} muốn đặt lối tắt này trên màn hình nền của bạn.",
            "Thêm vào màn hình nền",
            "Hủy",
        ),
        UiLanguage::Indonesian => words(
            "Tambahkan \u{201c}{name}\u{201d} ke desktop?",
            "{app} ingin menaruh pintasan ini di desktop Anda.",
            "Tambahkan ke desktop",
            "Batal",
        ),
        UiLanguage::Turkish => words(
            "\u{201c}{name}\u{201d} masaüstüne eklensin mi?",
            "{app}, bu kısayolu masaüstünüze koymak istiyor.",
            "Masaüstüne ekle",
            "İptal",
        ),
    }
}

/// `template` with its `{key}` placeholders replaced by their values, in one pass, so a value
/// that itself holds braces is left as it is.
fn fill(template: &str, values: &[(&str, &str)]) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open..];
        match values
            .iter()
            .find(|(key, _)| after.starts_with(&format!("{{{key}}}")))
        {
            Some((key, value)) => {
                out.push_str(value);
                rest = &after[key.len() + 2..];
            }
            None => {
                out.push('{');
                rest = &after[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// The `Notify` call that asks the user about `request`, in `language`.
pub fn question(request: &PinShortcutRequest, language: UiLanguage) -> Notify {
    let app = if request.app_label.trim().is_empty() {
        request.package_name.as_str()
    } else {
        request.app_label.as_str()
    };
    let name = request.shortcut.label();
    let text = question_text(language);
    let summary = fill(text.summary, &[("name", name)]);
    let body = fill(text.body, &[("app", app)]);
    let (add, cancel) = (text.add, text.cancel);
    Notify {
        app_name: app.to_string(),
        replaces_id: 0,
        summary,
        body: body_text(&body),
        actions: vec![ADD.into(), add.into(), CANCEL.into(), cancel.into()],
        urgency: 1,
        desktop_entry: app_id(&request.package_name),
        resident: true,
        reply_placeholder: String::new(),
    }
}

/// The questions on screen, by the number the notification service gave each.
#[derive(Debug, Default)]
pub struct Questions {
    pending: BTreeMap<u32, (PinShortcutRequest, Instant)>,
}

impl Questions {
    pub fn new() -> Questions {
        Questions::default()
    }

    /// The question about `request` is shown as `id`, from `now`.
    pub fn asked(&mut self, id: u32, request: PinShortcutRequest, now: Instant) {
        self.pending.insert(id, (request, now));
    }

    /// `ActionInvoked` on `id`: the request and the user's answer, when `id` is one of ours.
    /// A click on the notification itself is no answer.
    pub fn action(&mut self, id: u32, action: &str) -> Option<(PinShortcutRequest, bool)> {
        let accepted = match action {
            ADD => true,
            CANCEL => false,
            _ => return None,
        };
        self.pending
            .remove(&id)
            .map(|(request, _)| (request, accepted))
    }

    /// `NotificationClosed` on `id`: a question dismissed unanswered is declined.
    pub fn closed(&mut self, id: u32) -> Option<PinShortcutRequest> {
        self.pending.remove(&id).map(|(request, _)| request)
    }

    /// The questions older than [`ASK_FOR`] at `now`, which are declined and closed.
    pub fn expired(&mut self, now: Instant) -> Vec<(u32, PinShortcutRequest)> {
        let old: Vec<u32> = self
            .pending
            .iter()
            .filter(|(_, (_, asked))| now.duration_since(*asked) >= ASK_FOR)
            .map(|(id, _)| *id)
            .collect();
        old.into_iter()
            .filter_map(|id| self.pending.remove(&id).map(|(request, _)| (id, request)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dsa_host_core::tasks::AppShortcut;

    fn request(request_id: u32) -> PinShortcutRequest {
        PinShortcutRequest {
            request_id,
            package_name: "com.example.chat".into(),
            app_label: "Chat".into(),
            shortcut: AppShortcut {
                id: "ana".into(),
                short_label: "Ana <3".into(),
                ..AppShortcut::default()
            },
        }
    }

    #[test]
    fn fr7_a_pin_request_asks_with_add_to_desktop_and_cancel() {
        let asked = question(&request(1), UiLanguage::English);
        assert_eq!(asked.actions, vec![ADD, "Add to desktop", CANCEL, "Cancel"]);
        assert!(asked.summary.contains("Ana <3"));
        assert!(asked.body.starts_with("Chat"));
        assert_eq!(asked.desktop_entry, "dsa-com.example.chat");
        assert!(asked.resident);
        let korean = question(&request(1), UiLanguage::Korean);
        assert_eq!(korean.actions[1], "바탕 화면에 추가");
        assert_eq!(korean.actions[3], "취소");
    }

    #[test]
    fn fr17_a_pin_request_asks_in_every_interface_language() {
        use dsa_host_core::ui_language::placeholders;
        let english = question_text(UiLanguage::English);
        for language in UiLanguage::ALL {
            let text = question_text(language);
            for words in [text.summary, text.body, text.add, text.cancel] {
                assert!(!words.trim().is_empty(), "{language:?}");
                assert!(!words.contains('\u{2014}'), "{language:?}: {words}");
            }
            assert_eq!(placeholders(text.summary), placeholders(english.summary));
            assert_eq!(placeholders(text.body), placeholders(english.body));
            assert!(placeholders(text.add).is_empty() && placeholders(text.cancel).is_empty());
            if language != UiLanguage::English {
                assert_ne!(text.add, english.add, "{language:?}");
            }
            let asked = question(&request(1), language);
            // The shortcut and the app are named; no placeholder is left.
            assert!(asked.summary.contains("Ana <3"), "{language:?}");
            assert!(asked.body.contains("Chat"), "{language:?}");
            assert!(!asked.summary.contains("{name}") && !asked.body.contains("{app}"));
            assert_eq!(asked.actions[0], ADD);
            assert_eq!(asked.actions[1], text.add);
            assert_eq!(asked.actions[2], CANCEL);
            assert_eq!(asked.actions[3], text.cancel);
        }
        assert_eq!(fill("{app} {name}", &[("app", "{name}")]), "{name} {name}");
        assert_eq!(fill("{other}", &[("app", "x")]), "{other}");
    }

    #[test]
    fn fr7_a_pin_question_is_answered_once_and_declined_when_left() {
        let now = Instant::now();
        let mut questions = Questions::new();
        questions.asked(10, request(1), now);
        questions.asked(11, request(2), now);
        questions.asked(12, request(3), now);
        // Another notification's action, or a click on the body, answers nothing.
        assert_eq!(questions.action(99, ADD), None);
        assert_eq!(questions.action(10, "default"), None);
        let (answered, yes) = questions.action(10, ADD).unwrap();
        assert_eq!((answered.request_id, yes), (1, true));
        assert_eq!(questions.action(10, ADD), None, "answered once");
        assert_eq!(questions.closed(11).map(|r| r.request_id), Some(2));
        assert!(questions.expired(now + ASK_FOR / 2).is_empty());
        let left = questions.expired(now + ASK_FOR);
        assert_eq!(left.len(), 1);
        assert_eq!((left[0].0, left[0].1.request_id), (12, 3));
    }
}
