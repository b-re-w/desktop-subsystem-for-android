//! A user's window settings for each app: whether the app's windows may be resized, the size
//! they open at, how the app follows its window to a monitor of another scale, and where its
//! full screen shows.
//!
//! The settings app's Apps page and `dsa app-window` change them through the local API; the
//! service keeps them beside the user's other settings, and the user's window manager reads
//! them when an app's window opens. An app with the defaults (resizable, opening at the size
//! Android gives its task, scaling its own way) has no entry.

use dsa_protocol::host_api::types;
use serde::{Deserialize, Serialize};

use crate::compositor::window_sizing::MIN_CLIENT_DP;
use crate::tasks::FullScreenRequest;

/// The apps whose windows DSA scales the system way unless the user chose otherwise: apps that
/// break when their density changes while they run, so that only restarting their process
/// would repair them. [`DpiScaling::Auto`] is [`DpiScaling::System`] for these and
/// [`DpiScaling::App`] for every other app. This is the only copy of the list: the window
/// manager, `dsa app-window` and, through it, the settings apps all read it from here.
pub const SYSTEM_SCALED_BY_DEFAULT: &[&str] = &[
    // YouTube keeps values that depend on the density in its process. Moved to a monitor of
    // another scale, its bottom tabs are squashed and covered by the content, and its
    // miniplayer turns green, whether it handles the new density itself or its activity is
    // recreated; only a restart of its process draws it right again. Scaled the system way it
    // stays whole, and its text is sharp enough on a 125% monitor (measured on 2026-11-14).
    "com.google.android.youtube",
];

/// How an app follows its window to a monitor of another scale, as a user chooses it, the way
/// Windows lets a user choose who performs the high DPI scaling of a desktop program.
/// [`DpiScaling::System`] never recreates the app's activity; the default recreates it once
/// per move, when the move ends, except for the apps in [`SYSTEM_SCALED_BY_DEFAULT`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DpiScaling {
    /// DSA's choice for the app ([`DpiScaling::default_for`]): [`DpiScaling::System`] for an
    /// app in [`SYSTEM_SCALED_BY_DEFAULT`], [`DpiScaling::App`] for any other.
    #[default]
    Auto,
    /// The task takes the new monitor's density, and the app redraws at it: its text stays
    /// sharp. While the window is dragged it keeps its density and its window is scaled; when
    /// the move ends, it takes the new density and Android recreates its activity once, which
    /// restores its saved state (scroll position, typed text). That holds even for an app that
    /// says it handles a change of density itself, because some such apps lay out only part
    /// of their screen again. The app redraws briefly when it moves, as a Windows program does
    /// with "application".
    App,
    /// The task keeps its density, and the window manager scales the window's picture to the
    /// new monitor, so the app keeps its size on the screen. Scaled up, text can look a little
    /// soft.
    System,
    /// The app is given the new density without a restart, even though it does not say it
    /// handles one. That needs a change to Android's framework that is not made yet, so until
    /// then it works as [`DpiScaling::System`] does ([`DpiScaling::FORCED_AVAILABLE`]).
    SystemForced,
}

/// What a task's window does on a monitor of another scale, once the user's choice and the
/// app's own say are taken together.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalingMode {
    /// The task takes each monitor's density.
    App,
    /// The task keeps its density and the window is scaled.
    System,
}

impl DpiScaling {
    /// Whether [`DpiScaling::SystemForced`] does what it says yet. It needs a framework change
    /// that is not approved, so it does not.
    pub const FORCED_AVAILABLE: bool = false;

    /// Every choice, in the order the settings apps list them.
    pub const ALL: [DpiScaling; 4] = [
        DpiScaling::Auto,
        DpiScaling::App,
        DpiScaling::System,
        DpiScaling::SystemForced,
    ];

    /// The name `dsa app-window --dpi-scaling` takes and prints.
    pub fn name(self) -> &'static str {
        match self {
            DpiScaling::Auto => "auto",
            DpiScaling::App => "app",
            DpiScaling::System => "system",
            DpiScaling::SystemForced => "system-forced",
        }
    }

    /// The choice `name` names, as [`DpiScaling::name`] gives it.
    pub fn from_name(name: &str) -> Option<DpiScaling> {
        DpiScaling::ALL
            .into_iter()
            .find(|scaling| scaling.name() == name)
    }

    /// What [`DpiScaling::Auto`] is for `package`: [`DpiScaling::System`] for an app in
    /// [`SYSTEM_SCALED_BY_DEFAULT`], [`DpiScaling::App`] for any other. Never `Auto`.
    pub fn default_for(package: &str) -> DpiScaling {
        if SYSTEM_SCALED_BY_DEFAULT.contains(&package) {
            DpiScaling::System
        } else {
            DpiScaling::App
        }
    }

    /// This choice for `package`, with [`DpiScaling::Auto`] as DSA's choice for that app.
    pub fn resolved_for(self, package: &str) -> DpiScaling {
        match self {
            DpiScaling::Auto => DpiScaling::default_for(package),
            chosen => chosen,
        }
    }

    /// What the window of a task of `package` does with this choice: what
    /// [`DpiScaling::mode`] says of [`DpiScaling::resolved_for`].
    pub fn mode_for(self, package: &str) -> ScalingMode {
        self.resolved_for(package).mode()
    }

    /// What a task's window does with this choice, for an app outside
    /// [`SYSTEM_SCALED_BY_DEFAULT`] ([`DpiScaling::mode_for`] takes the list into account).
    ///
    /// [`DpiScaling::Auto`] and [`DpiScaling::App`] give the task the new monitor's density
    /// once its move ends, and the guest recreates its activity then, whether or not the app
    /// says it handles a change of density: an app that says so can still draw a broken
    /// layout at the new density, and one that does not is recreated by Android anyway.
    /// [`DpiScaling::SystemForced`] works as [`DpiScaling::System`] does until the framework
    /// change it needs is made.
    pub fn mode(self) -> ScalingMode {
        match self {
            DpiScaling::Auto | DpiScaling::App => ScalingMode::App,
            DpiScaling::SystemForced if DpiScaling::FORCED_AVAILABLE => ScalingMode::App,
            DpiScaling::System | DpiScaling::SystemForced => ScalingMode::System,
        }
    }

    fn from_wire(value: i32) -> DpiScaling {
        match types::DpiScaling::try_from(value) {
            Ok(types::DpiScaling::App) => DpiScaling::App,
            Ok(types::DpiScaling::System) => DpiScaling::System,
            Ok(types::DpiScaling::SystemForced) => DpiScaling::SystemForced,
            Ok(types::DpiScaling::Auto) | Err(_) => DpiScaling::Auto,
        }
    }

    fn to_wire(self) -> i32 {
        match self {
            DpiScaling::Auto => types::DpiScaling::Auto,
            DpiScaling::App => types::DpiScaling::App,
            DpiScaling::System => types::DpiScaling::System,
            DpiScaling::SystemForced => types::DpiScaling::SystemForced,
        }
        .into()
    }
}

/// Where a window shows an app that asks to fill the screen, as a user chooses it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FullScreenPlace {
    /// As the default says: for one app, the user's choice for every app
    /// ([`FullScreenDefaults`]); for every app, what DSA does without a choice.
    #[default]
    Default,
    /// On the whole screen: the window fills its monitor, and its title bar shows while the
    /// pointer is at the top edge, so minimize and close stay in reach.
    Screen,
    /// Inside the window: it keeps its size and its title bar, and the app fills it with no
    /// title band of its own.
    Window,
}

impl FullScreenPlace {
    /// Every choice, in the order the settings apps list them.
    pub const ALL: [FullScreenPlace; 3] = [
        FullScreenPlace::Default,
        FullScreenPlace::Screen,
        FullScreenPlace::Window,
    ];

    /// The name `dsa app-window --fullscreen` and `dsa settings set app-fullscreen` take and
    /// print.
    pub fn name(self) -> &'static str {
        match self {
            FullScreenPlace::Default => "default",
            FullScreenPlace::Screen => "screen",
            FullScreenPlace::Window => "window",
        }
    }

    /// The choice `name` names, as [`FullScreenPlace::name`] gives it.
    pub fn from_name(name: &str) -> Option<FullScreenPlace> {
        FullScreenPlace::ALL
            .into_iter()
            .find(|place| place.name() == name)
    }

    /// This choice, or `default` when it leaves the choice to the default.
    pub fn or(self, default: FullScreenPlace) -> FullScreenPlace {
        match self {
            FullScreenPlace::Default => default,
            chosen => chosen,
        }
    }

    pub fn from_wire(value: i32) -> FullScreenPlace {
        match types::FullScreenPlace::try_from(value) {
            Ok(types::FullScreenPlace::Screen) => FullScreenPlace::Screen,
            Ok(types::FullScreenPlace::Window) => FullScreenPlace::Window,
            Ok(types::FullScreenPlace::Default) | Err(_) => FullScreenPlace::Default,
        }
    }

    pub fn to_wire(self) -> i32 {
        match self {
            FullScreenPlace::Default => types::FullScreenPlace::Default,
            FullScreenPlace::Screen => types::FullScreenPlace::Screen,
            FullScreenPlace::Window => types::FullScreenPlace::Window,
        }
        .into()
    }
}

/// The user's full screen choices for every app, one for each kind of request, which an
/// app's own choice overrides.
///
/// A clear request (a video player's full screen button, a game that starts full screen)
/// fills the whole screen unless the user chose otherwise. An app that only hides its bars
/// (a feed of short videos) stays inside its window: the user opened it as a window, and the
/// app did not ask to leave it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullScreenDefaults {
    /// Where a clear full screen request is shown.
    pub fullscreen: FullScreenPlace,
    /// Where an app that hides both its bars alone is shown.
    pub immersive: FullScreenPlace,
}

impl Default for FullScreenDefaults {
    fn default() -> FullScreenDefaults {
        FullScreenDefaults {
            fullscreen: FullScreenPlace::Screen,
            immersive: FullScreenPlace::Window,
        }
    }
}

impl FullScreenDefaults {
    /// These choices with each part left to the default ([`FullScreenPlace::Default`]) as
    /// DSA's own default for it, so neither part is ever `Default`.
    pub fn resolved(self) -> FullScreenDefaults {
        let builtin = FullScreenDefaults::default();
        FullScreenDefaults {
            fullscreen: self.fullscreen.or(builtin.fullscreen),
            immersive: self.immersive.or(builtin.immersive),
        }
    }

    /// The choices a local API message carries.
    pub fn from_wire(defaults: Option<&types::FullScreenDefaults>) -> FullScreenDefaults {
        let Some(defaults) = defaults else {
            return FullScreenDefaults::default();
        };
        FullScreenDefaults {
            fullscreen: FullScreenPlace::from_wire(defaults.fullscreen),
            immersive: FullScreenPlace::from_wire(defaults.immersive),
        }
        .resolved()
    }

    /// The local API message that carries these choices.
    pub fn to_wire(self) -> types::FullScreenDefaults {
        let resolved = self.resolved();
        types::FullScreenDefaults {
            fullscreen: resolved.fullscreen.to_wire(),
            immersive: resolved.immersive.to_wire(),
        }
    }
}

/// Where a window shows a task whose app asks for `request`: the app's own choice for that
/// kind of request (`app`), else the user's choice for every app (`defaults`). `None` while
/// the app asks for no full screen. Never [`FullScreenPlace::Default`].
pub fn full_screen_place(
    request: FullScreenRequest,
    app: Option<&AppWindowSettings>,
    defaults: FullScreenDefaults,
) -> Option<FullScreenPlace> {
    let defaults = defaults.resolved();
    let (own, default) = match request {
        FullScreenRequest::None => return None,
        FullScreenRequest::Explicit => (app.map(|app| app.fullscreen), defaults.fullscreen),
        FullScreenRequest::BarsHidden => (app.map(|app| app.immersive), defaults.immersive),
    };
    Some(own.unwrap_or_default().or(default))
}

/// The user's window settings for every app that has any, with the full screen choices every
/// app follows unless it has its own: what a window manager reads.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AppWindows {
    pub apps: Vec<AppWindowSettings>,
    pub defaults: FullScreenDefaults,
}

/// The largest start size accepted, in density-independent pixels in each direction: more
/// than any monitor shows at 100%.
pub const MAX_START_DP: u32 = 8192;

/// The most apps with settings kept for one user.
pub const MAX_APPS: usize = 4096;

/// One app's window settings.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppWindowSettings {
    pub package_name: String,
    /// Whether the app's windows keep the size they open at.
    #[serde(default)]
    pub fixed_size: bool,
    /// The size the app's windows open at, in density-independent pixels; `None` opens them
    /// at the size Android gives the task.
    #[serde(default)]
    pub start_size_dp: Option<(u32, u32)>,
    /// How the app follows its window to a monitor of another scale.
    #[serde(default)]
    pub dpi_scaling: DpiScaling,
    /// Where the app's windows show a clear full screen request.
    #[serde(default)]
    pub fullscreen: FullScreenPlace,
    /// Where the app's windows show the app hiding both its bars alone.
    #[serde(default)]
    pub immersive: FullScreenPlace,
}

impl AppWindowSettings {
    /// Whether these are the defaults, which need no entry.
    pub fn is_default(&self) -> bool {
        !self.fixed_size
            && self.start_size_dp.is_none()
            && self.dpi_scaling == DpiScaling::Auto
            && self.fullscreen == FullScreenPlace::Default
            && self.immersive == FullScreenPlace::Default
    }

    /// Checks the settings, as a client sent them: the package must be an Android package
    /// name, and a start size at least the smallest window and at most [`MAX_START_DP`].
    pub fn checked(self) -> Result<AppWindowSettings, String> {
        if !crate::compositor::window_hosts::is_valid_package(&self.package_name) {
            return Err(format!(
                "{:?} is not an Android package name",
                self.package_name
            ));
        }
        if let Some((width, height)) = self.start_size_dp {
            let fits = |length: u32| (MIN_CLIENT_DP..=MAX_START_DP).contains(&length);
            if !fits(width) || !fits(height) {
                return Err(format!(
                    "a start size of {width} x {height} is not between {MIN_CLIENT_DP} and \
                     {MAX_START_DP} in each direction"
                ));
            }
        }
        Ok(self)
    }

    /// The settings a local API message carries; a start size of 0 by 0 is none.
    pub fn from_wire(app: &types::AppWindowSettings) -> AppWindowSettings {
        AppWindowSettings {
            package_name: app.package_name.clone(),
            fixed_size: app.fixed_size,
            start_size_dp: (app.start_width_dp > 0 || app.start_height_dp > 0)
                .then_some((app.start_width_dp, app.start_height_dp)),
            dpi_scaling: DpiScaling::from_wire(app.dpi_scaling),
            fullscreen: FullScreenPlace::from_wire(app.fullscreen),
            immersive: FullScreenPlace::from_wire(app.immersive),
        }
    }

    /// The local API message that carries these settings.
    pub fn to_wire(&self) -> types::AppWindowSettings {
        let (start_width_dp, start_height_dp) = self.start_size_dp.unwrap_or((0, 0));
        types::AppWindowSettings {
            package_name: self.package_name.clone(),
            fixed_size: self.fixed_size,
            start_width_dp,
            start_height_dp,
            dpi_scaling: self.dpi_scaling.to_wire(),
            fullscreen: self.fullscreen.to_wire(),
            immersive: self.immersive.to_wire(),
        }
    }
}

/// `apps` with `app` in place of its package's entry: removed when `app` has the defaults,
/// otherwise added or replaced. The list stays sorted by package name.
pub fn replace(
    apps: &[AppWindowSettings],
    app: AppWindowSettings,
) -> Result<Vec<AppWindowSettings>, String> {
    let mut list: Vec<AppWindowSettings> = apps
        .iter()
        .filter(|known| known.package_name != app.package_name)
        .cloned()
        .collect();
    if !app.is_default() {
        if list.len() >= MAX_APPS {
            return Err(format!(
                "window settings are kept for {MAX_APPS} apps at most; reset another app's first"
            ));
        }
        list.push(app);
    }
    list.sort_by(|a, b| a.package_name.cmp(&b.package_name));
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn notes(fixed_size: bool, start: Option<(u32, u32)>) -> AppWindowSettings {
        AppWindowSettings {
            package_name: "com.example.notes".into(),
            fixed_size,
            start_size_dp: start,
            ..AppWindowSettings::default()
        }
    }

    #[test]
    fn fr4_the_default_and_app_take_the_density_system_keeps_it_and_forced_works_as_system_for_now()
    {
        // Both end a move at the new density, with the activity recreated once.
        assert_eq!(DpiScaling::Auto.mode(), ScalingMode::App);
        assert_eq!(DpiScaling::App.mode(), ScalingMode::App);
        assert_eq!(DpiScaling::System.mode(), ScalingMode::System);
        // It needs a framework change that is not made yet.
        const { assert!(!DpiScaling::FORCED_AVAILABLE) };
        assert_eq!(DpiScaling::SystemForced.mode(), ScalingMode::System);
    }

    #[test]
    fn fr4_youtube_scales_the_system_way_by_default() {
        let youtube = "com.google.android.youtube";
        assert!(SYSTEM_SCALED_BY_DEFAULT.contains(&youtube));
        assert_eq!(DpiScaling::default_for(youtube), DpiScaling::System);
        assert_eq!(DpiScaling::Auto.resolved_for(youtube), DpiScaling::System);
        assert_eq!(DpiScaling::Auto.mode_for(youtube), ScalingMode::System);
        // An app not on the list takes each monitor's density.
        assert_eq!(
            DpiScaling::default_for("com.example.notes"),
            DpiScaling::App
        );
        assert_eq!(
            DpiScaling::Auto.mode_for("com.example.notes"),
            ScalingMode::App
        );
        // Every entry is a package name, and none is there twice.
        for (i, package) in SYSTEM_SCALED_BY_DEFAULT.iter().enumerate() {
            assert!(crate::compositor::window_hosts::is_valid_package(package));
            assert!(!SYSTEM_SCALED_BY_DEFAULT[..i].contains(package));
        }
    }

    #[test]
    fn fr4_the_users_choice_wins_over_the_built_in_list() {
        let youtube = "com.google.android.youtube";
        assert_eq!(DpiScaling::App.resolved_for(youtube), DpiScaling::App);
        assert_eq!(DpiScaling::App.mode_for(youtube), ScalingMode::App);
        assert_eq!(DpiScaling::System.mode_for(youtube), ScalingMode::System);
        assert_eq!(
            DpiScaling::SystemForced.mode_for(youtube),
            DpiScaling::SystemForced.mode()
        );
        // The list changes only what auto means: a choice for another app stays as it is.
        for scaling in [
            DpiScaling::App,
            DpiScaling::System,
            DpiScaling::SystemForced,
        ] {
            assert_eq!(scaling.resolved_for("com.example.notes"), scaling);
        }
    }

    #[test]
    fn fr4_the_dpi_scaling_choice_is_stored_with_the_other_window_settings() {
        for scaling in DpiScaling::ALL {
            assert_eq!(DpiScaling::from_name(scaling.name()), Some(scaling));
            let app = AppWindowSettings {
                dpi_scaling: scaling,
                ..notes(false, None)
            };
            assert_eq!(AppWindowSettings::from_wire(&app.to_wire()), app);
            let stored = serde_json::to_string(&app).unwrap();
            assert_eq!(
                serde_json::from_str::<AppWindowSettings>(&stored).unwrap(),
                app
            );
            // Any choice but the default is an entry of its own.
            assert_eq!(app.is_default(), scaling == DpiScaling::Auto);
        }
        assert_eq!(DpiScaling::from_name("enhanced"), None);
        // Settings stored before the choice existed read as auto.
        let old: AppWindowSettings =
            serde_json::from_str(r#"{"package_name":"com.example.notes","fixed_size":true}"#)
                .unwrap();
        assert_eq!(old.dpi_scaling, DpiScaling::Auto);
        assert_eq!(
            serde_json::to_value(DpiScaling::SystemForced).unwrap(),
            "system-forced"
        );
    }

    #[test]
    fn fr30_each_kind_of_full_screen_request_follows_its_setting() {
        use FullScreenPlace::{Screen, Window};
        let builtin = FullScreenDefaults::default();
        // A clear request fills the screen, and bars hidden alone stays in the window.
        assert_eq!(
            full_screen_place(FullScreenRequest::Explicit, None, builtin),
            Some(Screen)
        );
        assert_eq!(
            full_screen_place(FullScreenRequest::BarsHidden, None, builtin),
            Some(Window)
        );
        assert_eq!(
            full_screen_place(FullScreenRequest::None, None, builtin),
            None
        );
        // Each part of the setting moves its own kind only.
        let swapped = FullScreenDefaults {
            fullscreen: Window,
            immersive: Screen,
        };
        assert_eq!(
            full_screen_place(FullScreenRequest::Explicit, None, swapped),
            Some(Window)
        );
        assert_eq!(
            full_screen_place(FullScreenRequest::BarsHidden, None, swapped),
            Some(Screen)
        );
        // A part left to the default is DSA's own default, never "default".
        let unset = FullScreenDefaults {
            fullscreen: FullScreenPlace::Default,
            immersive: FullScreenPlace::Default,
        };
        assert_eq!(unset.resolved(), builtin);
        assert_eq!(
            FullScreenDefaults::from_wire(Some(&unset.to_wire())),
            builtin
        );
        assert_eq!(FullScreenDefaults::from_wire(None), builtin);
        assert_eq!(
            FullScreenDefaults::from_wire(Some(&swapped.to_wire())),
            swapped
        );
    }

    #[test]
    fn fr30_an_apps_full_screen_choice_wins_over_the_default() {
        use FullScreenPlace::{Screen, Window};
        let video = AppWindowSettings {
            fullscreen: Window,
            immersive: Screen,
            ..notes(false, None)
        };
        let builtin = FullScreenDefaults::default();
        assert_eq!(
            full_screen_place(FullScreenRequest::Explicit, Some(&video), builtin),
            Some(Window)
        );
        assert_eq!(
            full_screen_place(FullScreenRequest::BarsHidden, Some(&video), builtin),
            Some(Screen)
        );
        // An app that follows the default for one part follows the default there.
        let half = AppWindowSettings {
            immersive: FullScreenPlace::Default,
            ..video.clone()
        };
        assert_eq!(
            full_screen_place(FullScreenRequest::BarsHidden, Some(&half), builtin),
            Some(Window)
        );
        // The choices are kept like the other window settings, and only the defaults need no
        // entry.
        for place in FullScreenPlace::ALL {
            assert_eq!(FullScreenPlace::from_name(place.name()), Some(place));
            let app = AppWindowSettings {
                fullscreen: place,
                immersive: place,
                ..notes(false, None)
            };
            assert_eq!(AppWindowSettings::from_wire(&app.to_wire()), app);
            let stored = serde_json::to_string(&app).unwrap();
            assert_eq!(
                serde_json::from_str::<AppWindowSettings>(&stored).unwrap(),
                app
            );
            assert_eq!(app.is_default(), place == FullScreenPlace::Default);
        }
        assert_eq!(FullScreenPlace::from_name("monitor"), None);
        // Settings stored before the choice existed follow the default.
        let old: AppWindowSettings =
            serde_json::from_str(r#"{"package_name":"com.example.notes","fixed_size":true}"#)
                .unwrap();
        assert_eq!(old.fullscreen, FullScreenPlace::Default);
        assert_eq!(old.immersive, FullScreenPlace::Default);
    }

    #[test]
    fn fr30_an_apps_settings_replace_its_entry_and_the_defaults_remove_it() {
        let list = replace(&[], notes(true, None)).unwrap();
        assert_eq!(list, vec![notes(true, None)]);
        let list = replace(&list, notes(false, Some((400, 700)))).unwrap();
        assert_eq!(list, vec![notes(false, Some((400, 700)))]);
        assert!(replace(&list, notes(false, None)).unwrap().is_empty());
    }

    #[test]
    fn fr30_a_start_size_must_be_one_a_window_can_have() {
        assert!(notes(false, Some((400, 700))).checked().is_ok());
        assert!(notes(false, Some((100, 700))).checked().is_err());
        assert!(notes(false, Some((400, 9000))).checked().is_err());
        let bad = AppWindowSettings {
            package_name: "..\\evil".into(),
            ..notes(true, None)
        };
        assert!(bad.checked().is_err());
    }

    #[test]
    fn fr30_a_start_size_of_zero_on_the_wire_is_none() {
        let wire = notes(true, None).to_wire();
        assert_eq!((wire.start_width_dp, wire.start_height_dp), (0, 0));
        assert_eq!(AppWindowSettings::from_wire(&wire), notes(true, None));
        let wire = notes(false, Some((500, 800))).to_wire();
        assert_eq!(
            AppWindowSettings::from_wire(&wire),
            notes(false, Some((500, 800)))
        );
    }
}
