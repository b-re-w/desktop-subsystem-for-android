//! Who asks for root, as the administrator prompt names them.
//!
//! A su call comes with the caller's Android uid and the packages that run under it. Most
//! callers are apps, which the prompt names by their label (in the language Android shows,
//! as in the app list and the title bar) and shows with their launcher icon, the package only
//! as a second, smaller line. Some callers are not apps a person would recognise by their
//! package: `adb shell` runs as Android's shell uid (2000), and Android's own services run
//! below the first app uid (10000). Those are named in words and shown with a generic icon, so
//! the person approving sees who is asking rather than `com.android.shell`.
//!
//! The words come in every language the host's own words come in
//! ([`UiLanguage`]). The generic icons are drawn here, so every host shows the same ones
//! without shipping image files.

use crate::png::{self, Rgba};
use crate::ui_language::UiLanguage;

/// Android's shell app ID (`AID_SHELL`), which `adb shell` runs as.
pub const SHELL_APP_ID: u32 = 2000;

/// Android's system app ID (`AID_SYSTEM`), which the system server runs as.
pub const SYSTEM_APP_ID: u32 = 1000;

/// The first app ID Android gives an installed app (`AID_APP_START`). Below it are Android's
/// own services.
pub const FIRST_APP_ID: u32 = 10_000;

/// How many uids each Android user has; a uid is the user's number times this plus the app ID.
const PER_USER_RANGE: u32 = 100_000;

/// The side of a generic icon, in pixels: the largest an icon image on Windows holds, so the
/// prompt never scales it up.
pub const GENERIC_ICON_SIDE: u32 = 256;

/// The app ID of `uid`: the same for an app in every Android user.
pub fn app_id(uid: u32) -> u32 {
    uid % PER_USER_RANGE
}

/// Who asks for root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Requester {
    /// One app, named by its label and shown with its launcher icon.
    App,
    /// `adb shell`, Android's shell uid.
    DeveloperShell,
    /// One of Android's own services, below the first app uid.
    System,
    /// Several apps that share one user ID (`android:sharedUserId`), named together.
    SharedApps,
}

impl Requester {
    /// The generic icon the prompt shows for a requester that is not an app.
    pub fn generic_icon(self) -> Option<GenericIcon> {
        match self {
            Requester::DeveloperShell => Some(GenericIcon::Shell),
            Requester::System => Some(GenericIcon::System),
            Requester::App | Requester::SharedApps => None,
        }
    }
}

/// The name of `adb shell` in the prompt.
pub fn developer_shell_name(language: UiLanguage) -> &'static str {
    requester_words(language).developer_shell
}

/// The name of Android's own service running as `uid`.
pub fn system_name(uid: u32, language: UiLanguage) -> &'static str {
    let words = requester_words(language);
    match app_id(uid) {
        0 | SYSTEM_APP_ID => words.system,
        // AID_RADIO
        1001 => words.phone,
        // AID_BLUETOOTH
        1002 => words.bluetooth,
        // AID_NFC
        1027 => words.nfc,
        _ => words.service,
    }
}

/// The names of the requesters that are not apps, in one language.
struct RequesterWords {
    developer_shell: &'static str,
    /// The system server and the other services of the system uid.
    system: &'static str,
    /// The phone services (`AID_RADIO`).
    phone: &'static str,
    bluetooth: &'static str,
    nfc: &'static str,
    /// Any other service below the first app uid.
    service: &'static str,
}

fn requester_words(language: UiLanguage) -> RequesterWords {
    let (developer_shell, system, phone, bluetooth, nfc, service) = match language {
        UiLanguage::English => (
            "adb shell",
            "Android system",
            "Android phone services",
            "Android Bluetooth",
            "Android NFC",
            "Android system service",
        ),
        UiLanguage::Korean => (
            "adb 셸",
            "Android 시스템",
            "Android 전화 서비스",
            "Android 블루투스",
            "Android NFC",
            "Android 시스템 서비스",
        ),
        UiLanguage::Japanese => (
            "adb シェル",
            "Android システム",
            "Android 電話サービス",
            "Android Bluetooth",
            "Android NFC",
            "Android システム サービス",
        ),
        UiLanguage::ChineseSimplified => (
            "adb shell",
            "Android 系统",
            "Android 电话服务",
            "Android 蓝牙",
            "Android NFC",
            "Android 系统服务",
        ),
        UiLanguage::ChineseTraditional => (
            "adb shell",
            "Android 系統",
            "Android 電話服務",
            "Android 藍牙",
            "Android NFC",
            "Android 系統服務",
        ),
        UiLanguage::Spanish => (
            "adb shell",
            "Sistema Android",
            "Servicios de telefonía de Android",
            "Bluetooth de Android",
            "NFC de Android",
            "Servicio del sistema Android",
        ),
        UiLanguage::French => (
            "adb shell",
            "Système Android",
            "Services téléphoniques Android",
            "Bluetooth Android",
            "NFC Android",
            "Service système Android",
        ),
        UiLanguage::German => (
            "adb shell",
            "Android-System",
            "Android-Telefondienste",
            "Android-Bluetooth",
            "Android-NFC",
            "Android-Systemdienst",
        ),
        UiLanguage::PortugueseBrazil => (
            "adb shell",
            "Sistema Android",
            "Serviços de telefonia do Android",
            "Bluetooth do Android",
            "NFC do Android",
            "Serviço do sistema Android",
        ),
        UiLanguage::Russian => (
            "adb shell",
            "Система Android",
            "Телефонные службы Android",
            "Bluetooth Android",
            "NFC Android",
            "Системная служба Android",
        ),
        UiLanguage::Italian => (
            "adb shell",
            "Sistema Android",
            "Servizi telefonici Android",
            "Bluetooth Android",
            "NFC Android",
            "Servizio di sistema Android",
        ),
        UiLanguage::Vietnamese => (
            "adb shell",
            "Hệ thống Android",
            "Dịch vụ điện thoại Android",
            "Bluetooth Android",
            "NFC Android",
            "Dịch vụ hệ thống Android",
        ),
        UiLanguage::Indonesian => (
            "adb shell",
            "Sistem Android",
            "Layanan telepon Android",
            "Bluetooth Android",
            "NFC Android",
            "Layanan sistem Android",
        ),
        UiLanguage::Turkish => (
            "adb kabuğu",
            "Android sistemi",
            "Android telefon hizmetleri",
            "Android Bluetooth",
            "Android NFC",
            "Android sistem hizmeti",
        ),
    };
    RequesterWords {
        developer_shell,
        system,
        phone,
        bluetooth,
        nfc,
        service,
    }
}

/// An icon for a requester that is not an app.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericIcon {
    /// A terminal prompt, for the developer shell.
    Shell,
    /// A gear, for Android's own services.
    System,
}

impl GenericIcon {
    /// A short name for files the icon is written to.
    pub fn file_stem(self) -> &'static str {
        match self {
            GenericIcon::Shell => "dsa-root-requester-shell",
            GenericIcon::System => "dsa-root-requester-system",
        }
    }

    /// The icon as a PNG of [`GENERIC_ICON_SIDE`] pixels square.
    pub fn png(self) -> Vec<u8> {
        png::encode(&self.draw(), true)
    }

    /// The icon's pixels: a rounded square plate with the sign on it, each pixel sampled 4 x 4
    /// times so the edges are smooth.
    pub fn draw(self) -> Rgba {
        const GRID: u32 = 4;
        let side = GENERIC_ICON_SIDE;
        let mut pixels = vec![0u8; (side * side * 4) as usize];
        for y in 0..side {
            for x in 0..side {
                let (mut r, mut g, mut b, mut covered) = (0u32, 0u32, 0u32, 0u32);
                for sy in 0..GRID {
                    for sx in 0..GRID {
                        let px = x as f32 + (sx as f32 + 0.5) / GRID as f32;
                        let py = y as f32 + (sy as f32 + 0.5) / GRID as f32;
                        if let Some([cr, cg, cb]) = self.sample(px, py) {
                            r += u32::from(cr);
                            g += u32::from(cg);
                            b += u32::from(cb);
                            covered += 1;
                        }
                    }
                }
                if covered == 0 {
                    continue;
                }
                let at = ((y * side + x) * 4) as usize;
                pixels[at] = (r / covered) as u8;
                pixels[at + 1] = (g / covered) as u8;
                pixels[at + 2] = (b / covered) as u8;
                pixels[at + 3] = (covered * 255 / (GRID * GRID)) as u8;
            }
        }
        Rgba {
            width: side,
            height: side,
            pixels,
        }
    }

    /// The colour at (`x`, `y`) on a 256-pixel icon, `None` outside the plate.
    fn sample(self, x: f32, y: f32) -> Option<[u8; 3]> {
        if !in_rounded_square(x, y, 12.0, 244.0, 56.0) {
            return None;
        }
        match self {
            GenericIcon::Shell => {
                const PLATE: [u8; 3] = [0x26, 0x2b, 0x33];
                const GLYPH: [u8; 3] = [0xf2, 0xf4, 0xf7];
                const CURSOR: [u8; 3] = [0x3d, 0xdc, 0x84];
                let chevron = distance_to_segment(x, y, (76.0, 90.0), (128.0, 128.0)) <= 12.0
                    || distance_to_segment(x, y, (128.0, 128.0), (76.0, 166.0)) <= 12.0;
                let cursor = (140.0..=190.0).contains(&x) && (152.0..=174.0).contains(&y);
                Some(if chevron {
                    GLYPH
                } else if cursor {
                    CURSOR
                } else {
                    PLATE
                })
            }
            GenericIcon::System => {
                const PLATE: [u8; 3] = [0x3d, 0xdc, 0x84];
                const GEAR: [u8; 3] = [0xff, 0xff, 0xff];
                const TEETH: f32 = 8.0;
                let (dx, dy) = (x - 128.0, y - 128.0);
                let radius = (dx * dx + dy * dy).sqrt();
                let turn = dy.atan2(dx) / std::f32::consts::TAU * TEETH;
                let part = turn - turn.floor();
                let tooth = !(0.22..=0.78).contains(&part);
                let gear = radius >= 26.0 && (radius <= 60.0 || (tooth && radius <= 82.0));
                Some(if gear { GEAR } else { PLATE })
            }
        }
    }
}

/// Whether (`x`, `y`) is inside the square from `low` to `high` on both axes with corners
/// rounded by `corner`.
fn in_rounded_square(x: f32, y: f32, low: f32, high: f32, corner: f32) -> bool {
    if x < low || x > high || y < low || y > high {
        return false;
    }
    let cx = x.clamp(low + corner, high - corner);
    let cy = y.clamp(low + corner, high - corner);
    let (dx, dy) = (x - cx, y - cy);
    dx * dx + dy * dy <= corner * corner
}

/// How far (`x`, `y`) is from the segment from `a` to `b`.
fn distance_to_segment(x: f32, y: f32, a: (f32, f32), b: (f32, f32)) -> f32 {
    let (vx, vy) = (b.0 - a.0, b.1 - a.1);
    let (wx, wy) = (x - a.0, y - a.1);
    let length = vx * vx + vy * vy;
    let t = if length == 0.0 {
        0.0
    } else {
        ((wx * vx + wy * vy) / length).clamp(0.0, 1.0)
    };
    let (px, py) = (a.0 + t * vx - x, a.1 + t * vy - y);
    (px * px + py * py).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fr35_android_uids_are_named_in_words() {
        assert_eq!(system_name(1000, UiLanguage::Korean), "Android 시스템");
        assert_eq!(system_name(1000, UiLanguage::English), "Android system");
        // The same service in a second Android user.
        assert_eq!(system_name(101_000, UiLanguage::English), "Android system");
        assert_eq!(
            system_name(1013, UiLanguage::Korean),
            "Android 시스템 서비스"
        );
        assert_eq!(developer_shell_name(UiLanguage::Korean), "adb 셸");
        assert_eq!(developer_shell_name(UiLanguage::English), "adb shell");
    }

    /// Every requester that is not an app has a name in every interface language, none empty,
    /// none with an em dash, and each naming Android or adb as the English does.
    #[test]
    fn fr17_requesters_are_named_in_every_language() {
        for lang in UiLanguage::ALL {
            let shell = developer_shell_name(lang);
            assert!(shell.starts_with("adb"), "{lang:?}: {shell}");
            for uid in [0, 1000, 1001, 1002, 1027, 1013] {
                let name = system_name(uid, lang);
                assert!(name.contains("Android"), "{lang:?}: {name}");
                assert!(!name.contains('\u{2014}'), "{lang:?}: {name}");
            }
            // The services stay told apart.
            assert_ne!(system_name(1000, lang), system_name(1013, lang), "{lang:?}");
        }
    }

    #[test]
    fn fr35_the_generic_icons_are_pngs_with_a_plate_and_a_clear_corner() {
        for icon in [GenericIcon::Shell, GenericIcon::System] {
            let png = icon.png();
            let image = png::decode(&png).expect("a PNG this module wrote");
            assert_eq!((image.width, image.height), (256, 256));
            let alpha = |x: u32, y: u32| image.pixels[((y * 256 + x) * 4 + 3) as usize];
            // The corner outside the rounded plate is clear, its middle is solid.
            assert_eq!(alpha(0, 0), 0, "{icon:?}");
            assert_eq!(alpha(14, 14), 0, "{icon:?}");
            assert_eq!(alpha(128, 30), 255, "{icon:?}");
            // The sign is drawn: the middle is not the plate's colour.
            let at = |x: u32, y: u32| {
                let i = ((y * 256 + x) * 4) as usize;
                [image.pixels[i], image.pixels[i + 1], image.pixels[i + 2]]
            };
            assert_ne!(at(128, 30), at(100, 110), "{icon:?}");
        }
        // Each icon is the same every time, so a program made with it is not made again.
        assert_eq!(GenericIcon::Shell.png(), GenericIcon::Shell.png());
        assert_ne!(GenericIcon::Shell.png(), GenericIcon::System.png());
    }
}
