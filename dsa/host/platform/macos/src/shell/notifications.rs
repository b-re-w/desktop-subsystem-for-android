//! Guest notifications in macOS's Notification Center.
//!
//! Each guest notification is one `UNNotificationRequest` whose identifier is Android's key,
//! so a notification posted again replaces its banner and one removed in the guest leaves
//! the Notification Center. Clicking a banner opens the notification in the guest; clearing
//! it dismisses it there.
//!
//! The same watch carries DSA's own news about updates: that an update is ready, was applied,
//! or failed. One kind of update has one identifier ([`thread_of`]), so the news that an
//! update was applied replaces the news that it was ready, and a click on it opens the
//! settings app's Updates page ([`UPDATES_URL`]); nothing about it goes to the guest.
//!
//! The notification's action buttons are the banner's actions, so a person acts on it from
//! the Notification Center, with the keyboard or VoiceOver too, and choosing one runs that
//! action in the guest. macOS takes a banner's actions from its category, which is
//! registered before the banner is posted, so each set of buttons gets a category of its own
//! ([`buttons_category`]), registered the first time a notification has it. A button that
//! takes a reply is a text input action (`UNTextInputNotificationAction`), whose response
//! carries what the user wrote (`userText`); one whose app takes only its own choices shows
//! each choice as a button instead, since a banner has no list to pick from. A button that
//! deletes is drawn as destructive.

use std::hash::{Hash, Hasher};

use dsa_host_core::notifications::{
    Notification, NotificationAction, NotificationButton, SEMANTIC_ACTION_DELETE,
};
use dsa_host_core::update::alerts::{buttons_of, thread_of};

/// The link that opens the settings app on its Updates page.
pub const UPDATES_URL: &str = "dsa-settings://updates";

/// The category every DSA notification has, which asks macOS to report dismissals.
pub const CATEGORY: &str = "org.thisisthepy.dsa.notification";

/// The category of DSA's question whether Android restarts into a downloaded image, whose
/// actions are [Restart now] ([`RESTART_NOW`](dsa_host_core::update::alerts::RESTART_NOW)) and
/// [Later].
pub const RESTART_CATEGORY: &str = "org.thisisthepy.dsa.update-restart";

/// What the category of a set of buttons is called: this, then a hash of the buttons.
pub const BUTTONS_CATEGORY: &str = "org.thisisthepy.dsa.buttons.";

/// What the action of a button is called: this, then the button's place.
pub const BUTTON_ACTION: &str = "org.thisisthepy.dsa.button.";

/// What the action of one choice of a button is called: this, then the button's place, a dot
/// and the choice's place.
pub const CHOICE_ACTION: &str = "org.thisisthepy.dsa.choice.";

/// The most actions a banner is given; the rest are left out.
pub const MAX_ACTIONS: usize = 10;

/// One action of a banner, as a `UNNotificationAction` is made.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionSpec {
    pub identifier: String,
    pub title: String,
    /// The placeholder of its text field, for a button that takes a reply; `None` for a
    /// plain button.
    pub text_input: Option<String>,
    /// Drawn as destructive: the button deletes.
    pub destructive: bool,
}

/// The actions of a banner with `buttons`: one per button with words, and for a button whose
/// app takes only its own choices, one per choice. Each names the button's place in the
/// notification.
pub fn button_actions(buttons: &[NotificationButton]) -> Vec<ActionSpec> {
    let mut actions = Vec::new();
    for (place, button) in buttons
        .iter()
        .enumerate()
        .take(dsa_host_core::notifications::MAX_BUTTONS)
    {
        if button.title.trim().is_empty() {
            continue;
        }
        let destructive = button.semantic_action == SEMANTIC_ACTION_DELETE;
        if button.reply && button.reply_choices_only && !button.reply_choices.is_empty() {
            for (choice_place, choice) in button.reply_choices.iter().enumerate() {
                actions.push(ActionSpec {
                    identifier: format!("{CHOICE_ACTION}{place}.{choice_place}"),
                    title: choice.clone(),
                    text_input: None,
                    destructive,
                });
            }
            continue;
        }
        let text_input = button.reply.then(|| {
            if button.reply_label.trim().is_empty() {
                button.title.clone()
            } else {
                button.reply_label.clone()
            }
        });
        actions.push(ActionSpec {
            identifier: format!("{BUTTON_ACTION}{place}"),
            title: button.title.clone(),
            text_input,
            destructive,
        });
    }
    actions.truncate(MAX_ACTIONS);
    actions
}

/// The category of a banner with `buttons`, the same for the same buttons; `None` when they
/// make no action.
pub fn buttons_category(buttons: &[NotificationButton]) -> Option<String> {
    if button_actions(buttons).is_empty() {
        return None;
    }
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    buttons.hash(&mut hasher);
    Some(format!("{BUTTONS_CATEGORY}{:016x}", hasher.finish()))
}

/// What choosing the action `action` of the banner of the notification with `key`, whose
/// buttons are `buttons`, asks for, with what the user wrote in its text field (`text`).
/// `None` for an action that is not a button's, or a reply sent empty.
pub fn button_response(
    key: &str,
    buttons: &[NotificationButton],
    action: &str,
    text: Option<&str>,
) -> Option<NotificationAction> {
    let (place, reply) = if let Some(place) = action.strip_prefix(BUTTON_ACTION) {
        let place: usize = place.parse().ok()?;
        let reply = match (buttons.get(place)?.reply, text) {
            (true, Some(text)) if text.trim().is_empty() => return None,
            (true, Some(text)) => text.to_string(),
            _ => String::new(),
        };
        (place, reply)
    } else {
        let (place, choice) = action.strip_prefix(CHOICE_ACTION)?.split_once('.')?;
        let place: usize = place.parse().ok()?;
        let choice: usize = choice.parse().ok()?;
        (
            place,
            buttons.get(place)?.reply_choices.get(choice)?.clone(),
        )
    };
    Some(NotificationAction::Button {
        key: key.to_string(),
        button: place as u32,
        title: buttons.get(place)?.title.clone(),
        reply: dsa_host_core::notifications::cut_reply(reply),
    })
}

/// What the user did with a banner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    /// Clicked or cleared it.
    Notification(NotificationAction),
    /// Chose [Restart now] on DSA's question: the service restarts Android into the new image.
    RestartForUpdate,
    /// Answered an app's request to pin a shortcut: [Add to desktop] (`accepted`), or
    /// [Cancel] or a dismissal.
    Pin { request_id: u32, accepted: bool },
}

/// How loudly a notification arrives (`UNNotificationInterruptionLevel`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interruption {
    /// Into the Notification Center only, without a banner or sound.
    Passive,
    /// A banner and the notification sound.
    Active,
}

impl Interruption {
    /// The `UNNotificationInterruptionLevel` value.
    pub fn level(self) -> usize {
        match self {
            Interruption::Passive => 0,
            Interruption::Active => 1,
        }
    }
}

/// What a `UNMutableNotificationContent` is given for one guest notification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Content {
    pub identifier: String,
    pub title: String,
    pub subtitle: String,
    pub body: String,
    /// Groups an app's notifications together.
    pub thread: String,
    pub interruption: Interruption,
    /// [`CATEGORY`], [`RESTART_CATEGORY`] for the question with its buttons, or the category
    /// of the notification's own buttons ([`buttons_category`]).
    pub category: String,
}

/// The content of `notification`'s banner. The app's name heads it when the notification has
/// a title of its own, and is the title when it has none. Android's low and minimum
/// importances (1 and 2) never interrupt; the others do.
pub fn content(notification: &Notification) -> Content {
    let (title, subtitle) = if notification.title.trim().is_empty() {
        (notification.app_label.clone(), String::new())
    } else {
        (notification.title.clone(), notification.app_label.clone())
    };
    let interruption = if notification.importance >= 3 || notification.high_priority {
        Interruption::Active
    } else {
        Interruption::Passive
    };
    let category = if !buttons_of(&notification.key, false).is_empty() {
        RESTART_CATEGORY.to_string()
    } else {
        buttons_category(&notification.buttons).unwrap_or_else(|| CATEGORY.to_string())
    };
    match thread_of(&notification.key) {
        Some(thread) => Content {
            identifier: thread.clone(),
            title,
            subtitle,
            body: notification.text.clone(),
            thread,
            interruption,
            category,
        },
        None => Content {
            identifier: notification.key.clone(),
            title,
            subtitle,
            body: notification.text.clone(),
            thread: notification.package_name.clone(),
            interruption,
            category,
        },
    }
}

#[cfg(target_os = "macos")]
pub use center::NotificationCenter;

#[cfg(target_os = "macos")]
mod center {
    use std::path::PathBuf;
    use std::sync::mpsc::Sender;
    use std::sync::{Mutex, OnceLock};

    use block2::{Block, RcBlock};
    use dsa_host_core::notifications::{Notification, NotificationAction, NotificationButton};
    use dsa_host_core::update::alerts::RESTART_NOW;
    use dsa_host_core::update::alerts::{
        buttons_of, is_update_key, AlertKind, Subject, UpdateAlert,
    };
    use objc2::msg_send;
    use objc2::rc::Retained;
    use objc2::runtime::{AnyClass, AnyObject, Bool, ClassBuilder, Sel};
    use objc2::sel;

    use super::{
        button_actions, button_response, content, Response, BUTTONS_CATEGORY, BUTTON_ACTION,
        CATEGORY, CHOICE_ACTION, RESTART_CATEGORY,
    };
    use crate::objc::{class, file_url, ns_string, string_of};
    use crate::shell::pin_shortcuts::{self, PIN_CATEGORY};
    use dsa_host_core::tasks::PinShortcutRequest;

    #[link(name = "UserNotifications", kind = "framework")]
    extern "C" {}

    const DEFAULT_ACTION: &str = "com.apple.UNNotificationDefaultActionIdentifier";
    const DISMISS_ACTION: &str = "com.apple.UNNotificationDismissActionIdentifier";
    /// `UNNotificationCategoryOptionCustomDismissAction`.
    const CUSTOM_DISMISS: usize = 1;
    /// `UNAuthorizationOptionBadge | Sound | Alert`.
    const AUTHORIZE: usize = 1 | 2 | 4;
    /// `UNNotificationPresentationOptionSound | List | Banner`: shown even while a DSA window
    /// is in front, as the guest's own notifications would be.
    const PRESENT: usize = 2 | 8 | 16;

    /// `UNNotificationActionOptionDestructive`.
    const DESTRUCTIVE: usize = 2;
    /// The most categories of buttons kept registered; the oldest goes first.
    const MAX_BUTTON_CATEGORIES: usize = 64;

    static ACTIONS: Mutex<Option<Sender<Response>>> = Mutex::new(None);

    /// The categories made for the buttons of the notifications posted, oldest first, with
    /// the buttons each stands for, so a chosen action names its button.
    static BUTTON_CATEGORIES: Mutex<Vec<(String, Vec<NotificationButton>)>> =
        Mutex::new(Vec::new());

    /// What the user chose on a banner of the category `category`: a button of a notification.
    fn chosen_button(
        category: &str,
        key: &str,
        action: &str,
        text: Option<&str>,
    ) -> Option<Response> {
        let known = BUTTON_CATEGORIES.lock().ok()?;
        let (_, buttons) = known.iter().find(|(id, _)| id == category)?;
        button_response(key, buttons, action, text).map(Response::Notification)
    }

    /// The `UNNotificationCategory` of the buttons `buttons`, called `id`.
    fn buttons_category_object(
        id: &str,
        buttons: &[NotificationButton],
    ) -> Result<*mut AnyObject, String> {
        // SAFETY: messages to classes and objects of the UserNotifications framework with
        // the arguments their methods take; every object is checked for nil before use.
        unsafe { buttons_category_object_unchecked(id, buttons) }
    }

    unsafe fn buttons_category_object_unchecked(
        id: &str,
        buttons: &[NotificationButton],
    ) -> Result<*mut AnyObject, String> {
        let action_class = class(c"UNNotificationAction")?;
        let text_class = class(c"UNTextInputNotificationAction")?;
        let category_class = class(c"UNNotificationCategory")?;
        let array_class = class(c"NSArray")?;
        let mutable_array_class = class(c"NSMutableArray")?;
        let actions: *mut AnyObject = msg_send![mutable_array_class, array];
        for spec in button_actions(buttons) {
            let options = if spec.destructive {
                DESTRUCTIVE
            } else {
                0usize
            };
            let identifier = ns_string(&spec.identifier);
            let title = ns_string(&spec.title);
            let action: *mut AnyObject = match &spec.text_input {
                Some(placeholder) => msg_send![
                    text_class,
                    actionWithIdentifier: &*identifier,
                    title: &*title,
                    options: options,
                    textInputButtonTitle: &*title,
                    textInputPlaceholder: &*ns_string(placeholder)
                ],
                None => msg_send![
                    action_class,
                    actionWithIdentifier: &*identifier,
                    title: &*title,
                    options: options
                ],
            };
            if !action.is_null() {
                let _: () = msg_send![actions, addObject: action];
            }
        }
        let empty: *mut AnyObject = msg_send![array_class, array];
        let category: *mut AnyObject = msg_send![
            category_class,
            categoryWithIdentifier: &*ns_string(id),
            actions: actions,
            intentIdentifiers: empty,
            options: CUSTOM_DISMISS
        ];
        Ok(category)
    }

    unsafe extern "C-unwind" fn did_receive(
        _this: &AnyObject,
        _: Sel,
        _center: *mut AnyObject,
        response: *mut AnyObject,
        done: *mut Block<dyn Fn()>,
    ) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let action: *mut AnyObject = msg_send![response, actionIdentifier];
            let notification: *mut AnyObject = msg_send![response, notification];
            let request: *mut AnyObject = msg_send![notification, request];
            let key = string_of(msg_send![request, identifier]);
            let action_id = string_of(action);
            let request_content: *mut AnyObject = msg_send![request, content];
            let category = string_of(msg_send![request_content, categoryIdentifier]);
            // A text input action's response carries what the user wrote.
            let has_text: bool = msg_send![response, respondsToSelector: sel!(userText)];
            let text = has_text.then(|| string_of(msg_send![response, userText]));
            let pin = pin_shortcuts::request_of(&key).and_then(|request_id| {
                pin_shortcuts::answer(&action_id, action_id == DISMISS_ACTION).map(|accepted| {
                    Response::Pin {
                        request_id,
                        accepted,
                    }
                })
            });
            let action = match action_id.as_str() {
                _ if pin_shortcuts::request_of(&key).is_some() => pin,
                DEFAULT_ACTION => Some(Response::Notification(NotificationAction::Open { key })),
                DISMISS_ACTION => Some(Response::Notification(NotificationAction::Dismiss { key })),
                RESTART_NOW if is_update_key(&key) => Some(Response::RestartForUpdate),
                _ if category.starts_with(BUTTONS_CATEGORY)
                    && (action_id.starts_with(BUTTON_ACTION)
                        || action_id.starts_with(CHOICE_ACTION)) =>
                {
                    chosen_button(&category, &key, &action_id, text.as_deref())
                }
                // [Later] only closes the banner.
                _ => None,
            };
            if let (Some(action), Ok(actions)) = (action, ACTIONS.lock()) {
                if let Some(actions) = actions.as_ref() {
                    let _ = actions.send(action);
                }
            }
        }));
        if let Some(done) = done.as_ref() {
            done.call(());
        }
    }

    unsafe extern "C-unwind" fn will_present(
        _this: &AnyObject,
        _: Sel,
        _center: *mut AnyObject,
        _notification: *mut AnyObject,
        done: *mut Block<dyn Fn(usize)>,
    ) {
        if let Some(done) = done.as_ref() {
            done.call((PRESENT,));
        }
    }

    fn delegate_class() -> Result<&'static AnyClass, String> {
        static CLASS: OnceLock<Result<&'static AnyClass, String>> = OnceLock::new();
        CLASS
            .get_or_init(|| unsafe {
                let mut b = ClassBuilder::new(c"DSANotificationDelegate", class(c"NSObject")?)
                    .ok_or("the notification delegate class is already defined")?;
                b.add_method(
                    sel!(userNotificationCenter:didReceiveNotificationResponse:withCompletionHandler:),
                    did_receive as unsafe extern "C-unwind" fn(_, _, _, _, _),
                );
                b.add_method(
                    sel!(userNotificationCenter:willPresentNotification:withCompletionHandler:),
                    will_present as unsafe extern "C-unwind" fn(_, _, _, _, _),
                );
                if let Some(protocol) =
                    objc2::runtime::AnyProtocol::get(c"UNUserNotificationCenterDelegate")
                {
                    b.add_protocol(protocol);
                }
                Ok(b.register())
            })
            .clone()
    }

    /// The Notification Center, as DSA posts to it.
    pub struct NotificationCenter {
        center: Retained<AnyObject>,
        _delegate: Retained<AnyObject>,
        /// Where icons are written for macOS to read as attachments.
        icons: PathBuf,
        /// DSA's own categories: every banner's, the question to restart and the request to
        /// pin a shortcut.
        base: Retained<AnyObject>,
    }

    impl NotificationCenter {
        /// Connects to the Notification Center and asks the user, the first time, to allow
        /// DSA's notifications. What the user does with them goes to `actions`. `icons` is a
        /// folder for the icons attached to notifications. The buttons of DSA's question are
        /// labelled in Korean when `korean`. Fails when this process is not an application
        /// bundle, which the Notification Center serves only.
        pub fn new(
            actions: Sender<Response>,
            icons: PathBuf,
            korean: bool,
        ) -> Result<NotificationCenter, String> {
            unsafe {
                let bundle_class = class(c"NSBundle")?;
                let bundle: *mut AnyObject = msg_send![bundle_class, mainBundle];
                let identifier: *mut AnyObject = msg_send![bundle, bundleIdentifier];
                if identifier.is_null() {
                    return Err("notifications need DSA to run from its application bundle".into());
                }
                let center_class = class(c"UNUserNotificationCenter")?;
                let center: Option<Retained<AnyObject>> =
                    msg_send![center_class, currentNotificationCenter];
                let center = center.ok_or("the Notification Center is not available")?;
                let delegate: Option<Retained<AnyObject>> = msg_send![delegate_class()?, new];
                let delegate = delegate.ok_or("no notification delegate could be made")?;
                let _: () = msg_send![&*center, setDelegate: &*delegate];
                *ACTIONS
                    .lock()
                    .map_err(|_| "the notification actions lock failed")? = Some(actions);

                let category_class = class(c"UNNotificationCategory")?;
                let array_class = class(c"NSArray")?;
                let empty: *mut AnyObject = msg_send![array_class, array];
                let id = ns_string(CATEGORY);
                let category: *mut AnyObject = msg_send![
                    category_class,
                    categoryWithIdentifier: &*id,
                    actions: empty,
                    intentIdentifiers: empty,
                    options: CUSTOM_DISMISS
                ];
                // DSA's question has [Restart now] and [Later], which run in the background:
                // neither brings an app to the front.
                let action_class = class(c"UNNotificationAction")?;
                let asked = UpdateAlert::new(Subject::Android, AlertKind::Restart, "0").key();
                let mutable_array_class = class(c"NSMutableArray")?;
                let buttons: *mut AnyObject = msg_send![mutable_array_class, array];
                for button in buttons_of(&asked, korean) {
                    let id = ns_string(button.action);
                    let title = ns_string(&button.label);
                    let action: *mut AnyObject = msg_send![
                        action_class,
                        actionWithIdentifier: &*id,
                        title: &*title,
                        options: 0usize
                    ];
                    if !action.is_null() {
                        let _: () = msg_send![buttons, addObject: action];
                    }
                }
                let restart_id = ns_string(RESTART_CATEGORY);
                let restart_category: *mut AnyObject = msg_send![
                    category_class,
                    categoryWithIdentifier: &*restart_id,
                    actions: buttons,
                    intentIdentifiers: empty,
                    options: CUSTOM_DISMISS
                ];
                // An app's request to pin a shortcut: [Add to desktop] and [Cancel], which
                // also run in the background.
                let (add_label, cancel_label) = pin_shortcuts::buttons(korean);
                let pin_buttons: *mut AnyObject = msg_send![mutable_array_class, array];
                for (id, title) in [
                    (pin_shortcuts::ADD, add_label),
                    (pin_shortcuts::CANCEL, cancel_label),
                ] {
                    let action: *mut AnyObject = msg_send![
                        action_class,
                        actionWithIdentifier: &*ns_string(id),
                        title: &*ns_string(title),
                        options: 0usize
                    ];
                    if !action.is_null() {
                        let _: () = msg_send![pin_buttons, addObject: action];
                    }
                }
                let pin_id = ns_string(PIN_CATEGORY);
                let pin_category: *mut AnyObject = msg_send![
                    category_class,
                    categoryWithIdentifier: &*pin_id,
                    actions: pin_buttons,
                    intentIdentifiers: empty,
                    options: CUSTOM_DISMISS
                ];
                let set_class = class(c"NSSet")?;
                // Kept, since registering a category of buttons later sets every category
                // again.
                let both: Option<Retained<AnyObject>> = msg_send![mutable_array_class, new];
                let both = both.ok_or("no list of notification categories could be made")?;
                let _: () = msg_send![&*both, addObject: category];
                if !restart_category.is_null() {
                    let _: () = msg_send![&*both, addObject: restart_category];
                }
                if !pin_category.is_null() {
                    let _: () = msg_send![&*both, addObject: pin_category];
                }
                let categories: *mut AnyObject = msg_send![set_class, setWithArray: &*both];
                let _: () = msg_send![&*center, setNotificationCategories: categories];

                let answered = RcBlock::new(|_granted: Bool, _error: *mut AnyObject| {});
                let _: () = msg_send![
                    &*center,
                    requestAuthorizationWithOptions: AUTHORIZE,
                    completionHandler: &*answered
                ];
                Ok(NotificationCenter {
                    center,
                    _delegate: delegate,
                    icons,
                    base: both,
                })
            }
        }

        /// Registers the category `id` of the buttons `buttons`, unless it is already, so a
        /// banner of that category shows them. macOS takes the whole set of categories at
        /// once, so DSA's own and every category of buttons kept are set again.
        fn register(&self, id: &str, buttons: &[NotificationButton]) -> Result<(), String> {
            let mut known = BUTTON_CATEGORIES
                .lock()
                .map_err(|_| "the notification categories lock failed")?;
            if known.iter().any(|(known_id, _)| known_id == id) {
                return Ok(());
            }
            known.push((id.to_string(), buttons.to_vec()));
            if known.len() > MAX_BUTTON_CATEGORIES {
                known.remove(0);
            }
            unsafe {
                let mutable_array_class = class(c"NSMutableArray")?;
                let all: *mut AnyObject =
                    msg_send![mutable_array_class, arrayWithArray: &*self.base];
                for (id, buttons) in known.iter() {
                    let category = buttons_category_object(id, buttons)?;
                    if !category.is_null() {
                        let _: () = msg_send![all, addObject: category];
                    }
                }
                let set_class = class(c"NSSet")?;
                let categories: *mut AnyObject = msg_send![set_class, setWithArray: all];
                let _: () = msg_send![&*self.center, setNotificationCategories: categories];
            }
            Ok(())
        }

        /// The icon's attachment, written to the icons folder under a name made from the
        /// notification's key.
        fn attachment(&self, notification: &Notification) -> Option<Retained<AnyObject>> {
            self.attachment_of(&notification.key, &notification.icon_png)
        }

        /// The attachment of the icon `png`, written under a name made from `key`.
        fn attachment_of(&self, key: &str, png: &[u8]) -> Option<Retained<AnyObject>> {
            if png.is_empty() {
                return None;
            }
            std::fs::create_dir_all(&self.icons).ok()?;
            let name: String = key.bytes().map(|b| format!("{b:02x}")).take(64).collect();
            let path = self.icons.join(format!("{name}.png"));
            std::fs::write(&path, png).ok()?;
            unsafe {
                let class = class(c"UNNotificationAttachment").ok()?;
                let url = file_url(&path).ok()?;
                let id = ns_string("icon");
                let mut error: *mut AnyObject = std::ptr::null_mut();
                let attachment: Option<Retained<AnyObject>> = msg_send![
                    class,
                    attachmentWithIdentifier: &*id,
                    URL: &*url,
                    options: std::ptr::null::<AnyObject>(),
                    error: &mut error
                ];
                attachment
            }
        }

        /// Shows `notification`, in place of the one with its key if there is one.
        pub fn post(&self, notification: &Notification) -> Result<(), String> {
            let content = content(notification);
            unsafe {
                let content_class = class(c"UNMutableNotificationContent")?;
                let banner: Option<Retained<AnyObject>> = msg_send![content_class, new];
                let banner = banner.ok_or("no notification content could be made")?;
                let _: () = msg_send![&*banner, setTitle: &*ns_string(&content.title)];
                let _: () = msg_send![&*banner, setSubtitle: &*ns_string(&content.subtitle)];
                let _: () = msg_send![&*banner, setBody: &*ns_string(&content.body)];
                let _: () = msg_send![&*banner, setThreadIdentifier: &*ns_string(&content.thread)];
                if content.category.starts_with(super::BUTTONS_CATEGORY) {
                    self.register(&content.category, &notification.buttons)?;
                }
                let _: () =
                    msg_send![&*banner, setCategoryIdentifier: &*ns_string(&content.category)];
                let level_selector = sel!(setInterruptionLevel:);
                let can_level: bool = msg_send![&*banner, respondsToSelector: level_selector];
                if can_level {
                    let _: () =
                        msg_send![&*banner, setInterruptionLevel: content.interruption.level()];
                }
                if content.interruption == super::Interruption::Active {
                    let sound_class = class(c"UNNotificationSound")?;
                    let sound: *mut AnyObject = msg_send![sound_class, defaultSound];
                    let _: () = msg_send![&*banner, setSound: sound];
                }
                if let Some(attachment) = self.attachment(notification) {
                    let array_class = class(c"NSArray")?;
                    let attachments: *mut AnyObject =
                        msg_send![array_class, arrayWithObject: &*attachment];
                    let _: () = msg_send![&*banner, setAttachments: attachments];
                }
                let request_class = class(c"UNNotificationRequest")?;
                let request: *mut AnyObject = msg_send![
                    request_class,
                    requestWithIdentifier: &*ns_string(&content.identifier),
                    content: &*banner,
                    trigger: std::ptr::null::<AnyObject>()
                ];
                if request.is_null() {
                    return Err("the notification request could not be made".into());
                }
                let _: () = msg_send![
                    &*self.center,
                    addNotificationRequest: request,
                    withCompletionHandler: std::ptr::null::<Block<dyn Fn(*mut AnyObject)>>()
                ];
            }
            Ok(())
        }

        /// Asks the user about an app's request to pin a shortcut: a banner with
        /// [Add to desktop] and [Cancel], in Korean when `korean`.
        pub fn ask_pin(&self, request: &PinShortcutRequest, korean: bool) -> Result<(), String> {
            let (title, body) = pin_shortcuts::texts(request, korean);
            let identifier = pin_shortcuts::identifier(request.request_id);
            unsafe {
                let content_class = class(c"UNMutableNotificationContent")?;
                let banner: Option<Retained<AnyObject>> = msg_send![content_class, new];
                let banner = banner.ok_or("no notification content could be made")?;
                let _: () = msg_send![&*banner, setTitle: &*ns_string(&title)];
                let _: () = msg_send![&*banner, setBody: &*ns_string(&body)];
                let _: () =
                    msg_send![&*banner, setThreadIdentifier: &*ns_string(&request.package_name)];
                let _: () = msg_send![&*banner, setCategoryIdentifier: &*ns_string(PIN_CATEGORY)];
                if let Some(attachment) =
                    self.attachment_of(&identifier, &request.shortcut.icon_png)
                {
                    let array_class = class(c"NSArray")?;
                    let attachments: *mut AnyObject =
                        msg_send![array_class, arrayWithObject: &*attachment];
                    let _: () = msg_send![&*banner, setAttachments: attachments];
                }
                let request_class = class(c"UNNotificationRequest")?;
                let asked: *mut AnyObject = msg_send![
                    request_class,
                    requestWithIdentifier: &*ns_string(&identifier),
                    content: &*banner,
                    trigger: std::ptr::null::<AnyObject>()
                ];
                if asked.is_null() {
                    return Err("the question could not be made".into());
                }
                let _: () = msg_send![
                    &*self.center,
                    addNotificationRequest: asked,
                    withCompletionHandler: std::ptr::null::<Block<dyn Fn(*mut AnyObject)>>()
                ];
            }
            Ok(())
        }

        /// Takes the notification with `key` out of the Notification Center.
        pub fn remove(&self, key: &str) {
            unsafe {
                let Ok(array_class) = class(c"NSArray") else {
                    return;
                };
                let ids: *mut AnyObject = msg_send![array_class, arrayWithObject: &*ns_string(key)];
                let _: () =
                    msg_send![&*self.center, removeDeliveredNotificationsWithIdentifiers: ids];
                let _: () =
                    msg_send![&*self.center, removePendingNotificationRequestsWithIdentifiers: ids];
            }
        }

        /// Takes every DSA notification out of the Notification Center, as when the guest
        /// stops.
        pub fn clear(&self) {
            unsafe {
                let _: () = msg_send![&*self.center, removeAllDeliveredNotifications];
                let _: () = msg_send![&*self.center, removeAllPendingNotificationRequests];
            }
        }
    }

    impl Drop for NotificationCenter {
        fn drop(&mut self) {
            if let Ok(mut actions) = ACTIONS.lock() {
                *actions = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(title: &str, importance: i32) -> Notification {
        Notification {
            key: "0|com.example|1|null|10123".into(),
            package_name: "com.example".into(),
            app_label: "Example".into(),
            title: title.into(),
            text: "Hello".into(),
            icon_png: Vec::new(),
            importance,
            high_priority: false,
            clearable: true,
            when_millis: 0,
            buttons: Vec::new(),
        }
    }

    fn chat_buttons() -> Vec<NotificationButton> {
        vec![
            NotificationButton {
                title: "Reply".into(),
                reply: true,
                reply_label: "Reply to Ann".into(),
                semantic_action: 1,
                ..NotificationButton::default()
            },
            NotificationButton::default(),
            NotificationButton {
                title: "Delete".into(),
                semantic_action: SEMANTIC_ACTION_DELETE,
                ..NotificationButton::default()
            },
            NotificationButton {
                title: "Answer".into(),
                reply: true,
                reply_choices: vec!["Yes".into(), "No".into()],
                reply_choices_only: true,
                ..NotificationButton::default()
            },
        ]
    }

    #[test]
    fn fr18_a_notifications_buttons_are_its_banners_actions() {
        let actions = button_actions(&chat_buttons());
        let ids: Vec<&str> = actions.iter().map(|a| a.identifier.as_str()).collect();
        assert_eq!(
            ids,
            [
                "org.thisisthepy.dsa.button.0",
                "org.thisisthepy.dsa.button.2",
                "org.thisisthepy.dsa.choice.3.0",
                "org.thisisthepy.dsa.choice.3.1"
            ]
        );
        assert_eq!(actions[0].text_input.as_deref(), Some("Reply to Ann"));
        assert_eq!(actions[1].text_input, None);
        assert!(actions[1].destructive && !actions[0].destructive);
        assert_eq!(actions[2].title, "Yes");
        // The same buttons have one category; other buttons another; none, DSA's own.
        let with = Notification {
            buttons: chat_buttons(),
            ..note("t", 3)
        };
        let category = content(&with).category;
        assert!(category.starts_with(BUTTONS_CATEGORY), "{category}");
        assert_eq!(buttons_category(&chat_buttons()), Some(category.clone()));
        assert_ne!(buttons_category(&chat_buttons()[2..]), Some(category));
        assert_eq!(buttons_category(&[NotificationButton::default()]), None);
        assert_eq!(content(&note("t", 3)).category, CATEGORY);
    }

    #[test]
    fn fr18_choosing_an_action_or_sending_a_reply_runs_the_button() {
        let key = "0|com.example|1|null|10123";
        let buttons = chat_buttons();
        assert_eq!(
            button_response(
                key,
                &buttons,
                "org.thisisthepy.dsa.button.0",
                Some("on my way")
            ),
            Some(NotificationAction::Button {
                key: key.into(),
                button: 0,
                title: "Reply".into(),
                reply: "on my way".into(),
            })
        );
        assert_eq!(
            button_response(key, &buttons, "org.thisisthepy.dsa.button.2", None),
            Some(NotificationAction::Button {
                key: key.into(),
                button: 2,
                title: "Delete".into(),
                reply: String::new(),
            })
        );
        assert_eq!(
            button_response(key, &buttons, "org.thisisthepy.dsa.choice.3.1", None),
            Some(NotificationAction::Button {
                key: key.into(),
                button: 3,
                title: "Answer".into(),
                reply: "No".into(),
            })
        );
        // An empty reply, and actions that name no button, do nothing.
        assert_eq!(
            button_response(key, &buttons, "org.thisisthepy.dsa.button.0", Some(" ")),
            None
        );
        for action in [
            "org.thisisthepy.dsa.button.9",
            "org.thisisthepy.dsa.button.x",
            "org.thisisthepy.dsa.choice.3.7",
            "org.thisisthepy.dsa.choice.3",
            "com.apple.UNNotificationDefaultActionIdentifier",
        ] {
            assert_eq!(
                button_response(key, &buttons, action, None),
                None,
                "{action}"
            );
        }
    }

    #[test]
    fn fr8_a_notification_is_replaced_by_one_with_its_key() {
        let content = content(&note("Title", 3));
        assert_eq!(content.identifier, "0|com.example|1|null|10123");
        assert_eq!(content.thread, "com.example");
    }

    #[test]
    fn fr8_the_app_heads_a_titled_notification_and_titles_an_untitled_one() {
        let titled = content(&note("Meeting", 3));
        assert_eq!(
            (titled.title.as_str(), titled.subtitle.as_str()),
            ("Meeting", "Example")
        );
        let untitled = content(&note("  ", 3));
        assert_eq!(
            (untitled.title.as_str(), untitled.subtitle.as_str()),
            ("Example", "")
        );
        assert_eq!(untitled.body, "Hello");
    }

    #[test]
    fn fr14_update_news_has_one_identifier_per_kind_of_update() {
        use dsa_host_core::update::alerts::{is_update_key, AlertKind, Subject, UpdateAlert};
        let ready = UpdateAlert::new(Subject::Android, AlertKind::Ready, "16.0.3").notification();
        let applied =
            UpdateAlert::new(Subject::Android, AlertKind::Applied, "16.0.3").notification();
        let (ready, applied) = (content(&ready), content(&applied));
        assert_eq!(ready.identifier, "dsa:update:android");
        assert_eq!(applied.identifier, ready.identifier);
        assert_eq!(ready.subtitle, "DSA");
        assert_eq!(ready.interruption, Interruption::Active);
        // A click names the identifier, which is the host's to act on.
        assert!(is_update_key(&applied.identifier));
        assert_eq!(UPDATES_URL, "dsa-settings://updates");
        assert_eq!(ready.category, CATEGORY);
    }

    #[test]
    fn fr14_the_question_to_restart_has_the_category_with_its_buttons() {
        use dsa_host_core::update::alerts::{AlertKind, Subject, UpdateAlert};
        let asked = content(
            &UpdateAlert::new(Subject::Android, AlertKind::Restart, "16.0.3").notification(),
        );
        assert_eq!(asked.category, RESTART_CATEGORY);
        assert_eq!(
            asked.identifier, "dsa:update:android",
            "the news that it was applied replaces it"
        );
        assert_eq!(content(&note("t", 3)).category, CATEGORY);
    }

    #[test]
    fn fr8_low_importance_never_interrupts() {
        assert_eq!(content(&note("t", 2)).interruption, Interruption::Passive);
        assert_eq!(content(&note("t", 3)).interruption, Interruption::Active);
        let mut urgent = note("t", 1);
        urgent.high_priority = true;
        assert_eq!(content(&urgent).interruption, Interruption::Active);
    }
}
