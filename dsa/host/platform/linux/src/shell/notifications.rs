//! The guest's notifications as the desktop's, through the freedesktop notification service.
//!
//! Each guest notification becomes one desktop notification, which a later post with the
//! same key replaces and a removal closes. Clicking it opens it in the guest; dismissing it
//! dismisses it there. The service numbers notifications itself, so the host keeps which
//! number stands for which key.
//!
//! The notification's action buttons are the desktop notification's actions
//! (`button:<place>`), so a person acts on it from the desktop's notification list, with the
//! keyboard or a screen reader too. Invoking one runs that action in the guest. A button that
//! takes a reply becomes the service's inline reply where the service offers one (the
//! `inline-reply` capability, as KDE Plasma's has): the service shows a text field, with the
//! app's label as its placeholder (`x-kde-reply-placeholder-text`), and sends what the user
//! wrote with `NotificationReplied`, which runs the action with that reply. A service without
//! it shows the button as any other, which runs the action with no reply. The freedesktop
//! specification has no field for a reply, so there is nothing better to fall back to; only
//! the first reply button is the inline reply, since the service shows one field.
//!
//! The same watch carries DSA's own news about updates: that an Android image is ready, was
//! applied, or failed. Those show under DSA's settings app (its launcher entry,
//! [`SETTINGS_ENTRY`]), one per kind of update, so the news that an update was applied
//! replaces the news that it was ready. A click on one opens the settings app's Updates
//! page; nothing about it goes to the guest. The question whether Android restarts into a
//! downloaded image carries the actions [Restart now], which asks the service to restart, and
//! [Later], which only closes it.

use std::collections::BTreeMap;

use dsa_host_core::notifications::{Notification, NotificationAction, NotificationButton};
use dsa_host_core::update::alerts::{buttons_of, is_update_key, thread_of, RESTART_NOW};

use super::desktop_entry::app_id;

/// The action a click on the notification's body invokes.
pub const DEFAULT_ACTION: &str = "default";

/// The service's reason for a notification the user dismissed (`NotificationClosed`).
pub const CLOSED_BY_USER: u32 = 2;

/// What the action of a notification's button is called: `button:` and the button's place.
pub const BUTTON_ACTION: &str = "button:";

/// The action that is the service's inline reply, and the capability that offers it.
pub const INLINE_REPLY: &str = "inline-reply";

/// The action of the button at `place`.
pub fn button_action(place: usize) -> String {
    format!("{BUTTON_ACTION}{place}")
}

/// The launcher entry of DSA's settings app, which DSA's own notifications show under.
pub const SETTINGS_ENTRY: &str = "io.github.thisisthepy.DSA.Settings";

/// The arguments that open DSA's settings on the Updates page.
pub const UPDATE_PAGE_ARGUMENTS: [&str; 2] = ["--page", "updates"];

/// Where a notification with `key` is shown: its own place, except that DSA's news about one
/// kind of update shares one, so a newer replaces the older.
fn slot(key: &str) -> String {
    thread_of(key).unwrap_or_else(|| key.to_string())
}

/// The urgency hint: 0 low, 1 normal, 2 critical.
pub fn urgency(notification: &Notification) -> u8 {
    match notification.importance {
        i32::MIN..=2 => 0,
        _ => 1,
    }
}

/// Text in the body, with the characters the service's markup gives meaning escaped.
pub fn body_text(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            c => escaped.push(c),
        }
    }
    escaped
}

/// The arguments of `org.freedesktop.Notifications.Notify` for one notification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notify {
    pub app_name: String,
    pub replaces_id: u32,
    pub summary: String,
    pub body: String,
    pub actions: Vec<String>,
    pub urgency: u8,
    /// The `desktop-entry` hint: the app's launcher entry, so the desktop shows its icon and
    /// groups its notifications.
    pub desktop_entry: String,
    /// Whether the notification stays until the user acts on it (one that may not be
    /// dismissed, as an ongoing call's).
    pub resident: bool,
    /// The placeholder of the inline reply field (`x-kde-reply-placeholder-text`); empty
    /// when the notification has no inline reply.
    pub reply_placeholder: String,
}

/// The `Notify` call for one of DSA's own notices that the compositor raises itself, such as
/// the one that Android restarts because its graphics did not recover: under DSA's settings
/// app, as the news about updates, with nothing to click.
pub fn dsa_notice(notification: &Notification) -> Notify {
    Notify {
        app_name: notification.app_label.clone(),
        replaces_id: 0,
        summary: notification.title.clone(),
        body: body_text(&notification.text),
        actions: Vec::new(),
        urgency: urgency(notification),
        desktop_entry: SETTINGS_ENTRY.to_string(),
        resident: false,
        reply_placeholder: String::new(),
    }
}

/// What the host's notifications and the service's numbers stand for.
#[derive(Debug, Default)]
pub struct Shown {
    /// The number of what is shown in each place ([`slot`]).
    by_key: BTreeMap<String, u32>,
    /// The key of the notification each number shows.
    by_id: BTreeMap<u32, String>,
    /// The buttons of each notification shown, by key, so an invoked action names its button.
    buttons: BTreeMap<String, Vec<NotificationButton>>,
    /// Whether the service offers an inline reply (its `inline-reply` capability).
    pub inline_reply: bool,
}

/// The place of the button that is the notification's inline reply: its first that takes a
/// reply, when the service offers inline replies.
fn inline_reply_of(buttons: &[NotificationButton], offered: bool) -> Option<usize> {
    if !offered {
        return None;
    }
    buttons
        .iter()
        .position(|button| button.reply && !button.title.trim().is_empty())
}

impl Shown {
    pub fn new() -> Shown {
        Shown::default()
    }

    /// The `Notify` call that shows `notification`, replacing the one with its key. DSA's
    /// question whether Android restarts into a downloaded image gets [Restart now] and
    /// [Later], labelled in Korean when `korean`.
    pub fn notify_in(&self, notification: &Notification, korean: bool) -> Notify {
        let mut notify = self.notify(notification);
        let lang = if korean {
            dsa_host_core::ui_language::UiLanguage::Korean
        } else {
            dsa_host_core::ui_language::UiLanguage::English
        };
        for button in buttons_of(&notification.key, lang) {
            notify.actions.push(button.action.to_string());
            notify.actions.push(button.label);
        }
        notify
    }

    /// The `Notify` call that shows `notification`, replacing the one with its key.
    pub fn notify(&self, notification: &Notification) -> Notify {
        let app_name = if notification.app_label.is_empty() {
            notification.package_name.clone()
        } else {
            notification.app_label.clone()
        };
        let desktop_entry = if is_update_key(&notification.key) {
            SETTINGS_ENTRY.to_string()
        } else {
            app_id(&notification.package_name)
        };
        let mut actions = vec![DEFAULT_ACTION.to_string(), "Open".to_string()];
        let inline = inline_reply_of(&notification.buttons, self.inline_reply);
        let mut reply_placeholder = String::new();
        for (place, button) in notification
            .buttons
            .iter()
            .enumerate()
            .take(dsa_host_core::notifications::MAX_BUTTONS)
        {
            if button.title.trim().is_empty() {
                continue;
            }
            if Some(place) == inline {
                actions.push(INLINE_REPLY.to_string());
                reply_placeholder = if button.reply_label.trim().is_empty() {
                    button.title.clone()
                } else {
                    button.reply_label.clone()
                };
            } else {
                actions.push(button_action(place));
            }
            actions.push(button.title.clone());
        }
        Notify {
            app_name,
            replaces_id: self
                .by_key
                .get(&slot(&notification.key))
                .copied()
                .unwrap_or(0),
            summary: notification.title.clone(),
            body: body_text(&notification.text),
            actions,
            urgency: urgency(notification),
            desktop_entry,
            resident: !notification.clearable,
            reply_placeholder,
        }
    }

    /// The service numbered `notification` as `id`: as [`Shown::shown`], and its buttons are
    /// kept so an invoked action names its button.
    pub fn shown_with(&mut self, notification: &Notification, id: u32) {
        self.shown(&notification.key, id);
        if notification.buttons.is_empty() {
            self.buttons.remove(&notification.key);
        } else {
            self.buttons
                .insert(notification.key.clone(), notification.buttons.clone());
        }
    }

    /// `NotificationReplied`: the inline reply the user sent, which runs the notification's
    /// reply button with it. `None` for a number that is not one of ours, a notification
    /// with no reply button, or an empty reply.
    pub fn replied(&self, id: u32, text: &str) -> Option<NotificationAction> {
        let key = self.by_id.get(&id)?;
        let buttons = self.buttons.get(key)?;
        let place = inline_reply_of(buttons, true)?;
        if text.trim().is_empty() {
            return None;
        }
        Some(NotificationAction::Button {
            key: key.clone(),
            button: place as u32,
            title: buttons[place].title.clone(),
            reply: dsa_host_core::notifications::cut_reply(text.to_string()),
        })
    }

    /// The service numbered the notification with `key` as `id`.
    pub fn shown(&mut self, key: &str, id: u32) {
        if let Some(old) = self.by_key.insert(slot(key), id) {
            if old != id {
                self.by_id.remove(&old);
            }
        }
        self.by_id.insert(id, key.to_string());
    }

    /// The notification with `key` is gone from the guest: the number to close, if shown.
    pub fn removed(&mut self, key: &str) -> Option<u32> {
        self.buttons.remove(key);
        let id = self.by_key.remove(&slot(key))?;
        self.by_id.remove(&id);
        Some(id)
    }

    /// `ActionInvoked`: what the guest is told. A click opens the notification; a button runs
    /// its action, with no reply.
    pub fn action_invoked(&self, id: u32, action: &str) -> Option<NotificationAction> {
        let key = self.by_id.get(&id)?;
        if action == DEFAULT_ACTION {
            return Some(NotificationAction::Open { key: key.clone() });
        }
        let place: usize = action.strip_prefix(BUTTON_ACTION)?.parse().ok()?;
        let button = self.buttons.get(key)?.get(place)?;
        Some(NotificationAction::Button {
            key: key.clone(),
            button: place as u32,
            title: button.title.clone(),
            reply: String::new(),
        })
    }

    /// Whether `ActionInvoked` is [Restart now] on DSA's question whether Android restarts
    /// into a downloaded image: the host then asks the service to restart.
    pub fn restart_asked(&self, id: u32, action: &str) -> bool {
        action == RESTART_NOW && self.by_id.get(&id).is_some_and(|key| is_update_key(key))
    }

    /// `NotificationClosed`: what the guest is told. Only the user's dismissal reaches the
    /// guest; one that expired stays posted there, as on Windows.
    pub fn closed(&mut self, id: u32, reason: u32) -> Option<NotificationAction> {
        let key = self.by_id.remove(&id)?;
        self.by_key.remove(&slot(&key));
        self.buttons.remove(&key);
        (reason == CLOSED_BY_USER).then_some(NotificationAction::Dismiss { key })
    }

    /// Every number the guest's notifications are shown under, to close them all when the
    /// guest stops. DSA's own news about updates stays: it is not the guest's.
    pub fn clear(&mut self) -> Vec<u32> {
        let guest: Vec<u32> = self
            .by_id
            .iter()
            .filter(|(_, key)| !is_update_key(key))
            .map(|(id, _)| *id)
            .collect();
        for id in &guest {
            if let Some(key) = self.by_id.remove(id) {
                self.by_key.remove(&slot(&key));
                self.buttons.remove(&key);
            }
        }
        guest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn notification(key: &str) -> Notification {
        Notification {
            key: key.into(),
            package_name: "com.example.chat".into(),
            app_label: "Chat".into(),
            title: "Ann".into(),
            text: "see <you> & me".into(),
            importance: 3,
            clearable: true,
            ..Notification::default()
        }
    }

    #[test]
    fn fr8_a_notification_shows_with_its_app_title_and_text() {
        let shown = Shown::new();
        let notify = shown.notify(&notification("k1"));
        assert_eq!(notify.app_name, "Chat");
        assert_eq!(notify.summary, "Ann");
        assert_eq!(notify.body, "see &lt;you&gt; &amp; me");
        assert_eq!(notify.replaces_id, 0);
        assert_eq!(notify.desktop_entry, "dsa-com.example.chat");
        assert_eq!(notify.urgency, 1);
        assert!(!notify.resident);
    }

    #[test]
    fn fr8_a_new_post_with_the_same_key_replaces_the_shown_one() {
        let mut shown = Shown::new();
        shown.shown("k1", 41);
        assert_eq!(shown.notify(&notification("k1")).replaces_id, 41);
        assert_eq!(shown.removed("k1"), Some(41));
        assert_eq!(shown.removed("k1"), None);
    }

    #[test]
    fn fr8_clicking_opens_it_in_the_guest_and_dismissing_dismisses_it() {
        let mut shown = Shown::new();
        shown.shown("k1", 7);
        assert_eq!(
            shown.action_invoked(7, DEFAULT_ACTION),
            Some(NotificationAction::Open { key: "k1".into() })
        );
        assert_eq!(
            shown.closed(7, CLOSED_BY_USER),
            Some(NotificationAction::Dismiss { key: "k1".into() })
        );
        assert_eq!(shown.closed(7, CLOSED_BY_USER), None);
    }

    #[test]
    fn fr8_an_expired_notification_stays_posted_in_the_guest() {
        let mut shown = Shown::new();
        shown.shown("k1", 9);
        assert_eq!(shown.closed(9, 1), None);
    }

    #[test]
    fn fr14_update_news_shows_under_the_settings_app_and_the_newer_replaces_the_older() {
        use dsa_host_core::update::alerts::{AlertKind, Subject, UpdateAlert};
        let mut shown = Shown::new();
        let ready = UpdateAlert::new(Subject::Android, AlertKind::Ready, "16.0.3").notification();
        let notify = shown.notify(&ready);
        assert_eq!(notify.app_name, "DSA");
        assert_eq!(notify.desktop_entry, SETTINGS_ENTRY);
        assert_eq!(notify.replaces_id, 0);
        shown.shown(&ready.key, 12);
        let applied =
            UpdateAlert::new(Subject::Android, AlertKind::Applied, "16.0.3").notification();
        assert_eq!(shown.notify(&applied).replaces_id, 12);
        shown.shown(&applied.key, 12);
        // A click is the host's to act on: it names the alert, which opens the settings.
        let click = shown.action_invoked(12, DEFAULT_ACTION).unwrap();
        assert!(is_update_key(click.key()));
        assert_eq!(UPDATE_PAGE_ARGUMENTS, ["--page", "updates"]);
        // The guest stopping closes its own notifications only.
        shown.shown("0|com.example.chat|1|null|10100", 13);
        assert_eq!(shown.clear(), vec![13]);
        assert_eq!(shown.notify(&applied).replaces_id, 12);
    }

    #[test]
    fn nfr5_the_notice_that_android_restarts_for_its_graphics_shows_under_dsa() {
        use dsa_host_core::compositor::frame_stall::Notice;
        let notice =
            Notice::Restarting.notification(dsa_host_core::ui_language::UiLanguage::Korean);
        let notify = dsa_notice(&notice);
        assert_eq!(notify.app_name, "DSA");
        assert_eq!(notify.desktop_entry, SETTINGS_ENTRY);
        assert_eq!(notify.summary, notice.title);
        assert!(notify.actions.is_empty(), "nothing to click");
        assert_eq!(notify.urgency, 1);
    }

    #[test]
    fn fr14_the_question_to_restart_has_restart_now_and_later_actions() {
        use dsa_host_core::update::alerts::{AlertKind, Subject, UpdateAlert, LATER};
        let mut shown = Shown::new();
        let asked = UpdateAlert::new(Subject::Android, AlertKind::Restart, "16.0.3").notification();
        let notify = shown.notify_in(&asked, false);
        assert_eq!(
            notify.actions,
            [
                DEFAULT_ACTION,
                "Open",
                RESTART_NOW,
                "Restart now",
                LATER,
                "Later"
            ]
        );
        assert_eq!(shown.notify_in(&asked, true).actions[3], "지금 다시 시작");
        shown.shown(&asked.key, 21);
        assert!(shown.restart_asked(21, RESTART_NOW));
        assert!(!shown.restart_asked(21, LATER));
        assert!(!shown.restart_asked(21, DEFAULT_ACTION));
        assert_eq!(shown.action_invoked(21, RESTART_NOW), None);
        // An Android notification never restarts anything.
        shown.shown("0|com.example.chat|1|null|10100", 22);
        assert!(!shown.restart_asked(22, RESTART_NOW));
        let ready = UpdateAlert::new(Subject::Android, AlertKind::Ready, "16.0.3").notification();
        assert_eq!(shown.notify_in(&ready, false).actions.len(), 2);
    }

    fn with_buttons(key: &str) -> Notification {
        Notification {
            buttons: vec![
                NotificationButton {
                    title: "Reply".into(),
                    reply: true,
                    reply_label: "Reply to Ann".into(),
                    semantic_action: 1,
                    ..NotificationButton::default()
                },
                NotificationButton::default(),
                NotificationButton {
                    title: "Mark as read".into(),
                    semantic_action: 2,
                    ..NotificationButton::default()
                },
            ],
            ..notification(key)
        }
    }

    #[test]
    fn fr18_a_notifications_buttons_are_its_actions_and_run_in_the_guest() {
        let mut shown = Shown::new();
        let posted = with_buttons("k1");
        let notify = shown.notify(&posted);
        // The button with no words is left out; the others keep their places.
        assert_eq!(
            notify.actions,
            [
                DEFAULT_ACTION,
                "Open",
                "button:0",
                "Reply",
                "button:2",
                "Mark as read"
            ]
        );
        assert!(notify.reply_placeholder.is_empty());
        shown.shown_with(&posted, 5);
        assert_eq!(
            shown.action_invoked(5, "button:2"),
            Some(NotificationAction::Button {
                key: "k1".into(),
                button: 2,
                title: "Mark as read".into(),
                reply: String::new(),
            })
        );
        assert_eq!(shown.action_invoked(5, "button:9"), None);
        assert_eq!(shown.action_invoked(5, "button:x"), None);
    }

    #[test]
    fn fr18_a_reply_button_is_the_inline_reply_where_the_service_offers_one() {
        let mut shown = Shown::new();
        shown.inline_reply = true;
        let posted = with_buttons("k1");
        let notify = shown.notify(&posted);
        assert_eq!(notify.actions[2..4], [INLINE_REPLY, "Reply"]);
        assert_eq!(notify.reply_placeholder, "Reply to Ann");
        shown.shown_with(&posted, 6);
        assert_eq!(
            shown.replied(6, "see you \u{c548}\u{b155}"),
            Some(NotificationAction::Button {
                key: "k1".into(),
                button: 0,
                title: "Reply".into(),
                reply: "see you \u{c548}\u{b155}".into(),
            })
        );
        assert_eq!(shown.replied(6, " "), None);
        assert_eq!(shown.replied(99, "hi"), None);
        // Once it is gone, a late reply goes nowhere.
        shown.removed("k1");
        assert_eq!(shown.replied(6, "hi"), None);
    }

    #[test]
    fn fr8_a_quiet_notification_is_low_urgency_and_an_ongoing_one_stays() {
        let shown = Shown::new();
        let mut quiet = notification("k2");
        quiet.importance = 2;
        quiet.clearable = false;
        let notify = shown.notify(&quiet);
        assert_eq!(notify.urgency, 0);
        assert!(notify.resident);
    }
}
