//! The Android items of a task window's system menu.
//!
//! The system menu (Alt+Space, a right click on the title bar, a click on the app's icon in
//! it) keeps Windows' own items: Restore, Move, Size, Minimize, Maximize and Close. Under them,
//! after a separator, come the items an Android desktop window offers in its window menu:
//! Full screen (on and off, as F11, whatever the user's setting for the app's full screen), New
//! window (only for an app that supports several windows), Screenshot, Aspect ratio, Open
//! by default and App info. They sit where a Windows user looks for what a window can do, and
//! they take no room in the title bar. The default title bar and one the app draws itself
//! share the same menu, since Windows opens it from the same window. The window shows it as a
//! Windows 11 menu ([`super::window_menu`]), whose Android part is these items.
//!
//! The window host takes the screenshot itself ([`super::screenshot`]), since it has the
//! window's frames, and turns full screen on and off itself, as it does for F11. Every other item goes to the window manager as `FromHost::MenuChosen`.
//!
//! The items speak the user's display language when DSA has it, else English
//! ([`crate::ui_language`]).

use dsa_host_core::compositor::window_hosts::MenuAction;
use dsa_host_core::ui_language::UiLanguage;

/// One entry of the Android part of the system menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemMenuItem {
    /// The line between Windows' items and these.
    Separator,
    FullScreen,
    NewWindow,
    Screenshot,
    AspectRatio,
    OpenByDefault,
    AppInfo,
}

/// The entries under Windows' own items, in order. New window is left out for an app that
/// does not support several windows, since its second task would only bring the first to the
/// front.
pub fn items(new_window: bool) -> Vec<SystemMenuItem> {
    let mut items = vec![SystemMenuItem::Separator, SystemMenuItem::FullScreen];
    if new_window {
        items.push(SystemMenuItem::NewWindow);
    }
    items.extend([
        SystemMenuItem::Screenshot,
        SystemMenuItem::AspectRatio,
        SystemMenuItem::OpenByDefault,
        SystemMenuItem::AppInfo,
    ]);
    items
}

/// Every entry this menu may have, for removing them all before the menu is made again.
pub const ALL: [SystemMenuItem; 7] = [
    SystemMenuItem::Separator,
    SystemMenuItem::FullScreen,
    SystemMenuItem::NewWindow,
    SystemMenuItem::Screenshot,
    SystemMenuItem::AspectRatio,
    SystemMenuItem::OpenByDefault,
    SystemMenuItem::AppInfo,
];

/// The command ID of an entry. `WM_SYSCOMMAND` carries an item's ID with its low four bits
/// used by Windows, and Windows' own commands are 0xF000 and above, so each ID has its low
/// four bits zero and stays below them.
pub fn command(item: SystemMenuItem) -> u32 {
    match item {
        SystemMenuItem::Separator => 0xE000,
        SystemMenuItem::NewWindow => 0xE010,
        SystemMenuItem::Screenshot => 0xE020,
        SystemMenuItem::AspectRatio => 0xE030,
        SystemMenuItem::OpenByDefault => 0xE040,
        SystemMenuItem::AppInfo => 0xE050,
        SystemMenuItem::FullScreen => 0xE060,
    }
}

/// The entry a `WM_SYSCOMMAND` chose, from its `wparam`; `None` for Windows' own commands and
/// for the separator, which cannot be chosen.
pub fn chosen(wparam: usize) -> Option<SystemMenuItem> {
    let id = (wparam & 0xFFF0) as u32;
    ALL.into_iter()
        .filter(|item| *item != SystemMenuItem::Separator)
        .find(|item| command(*item) == id)
}

/// What the window manager does for an entry; `None` for the separator, Screenshot and Full
/// screen, which the window host takes.
pub fn action(item: SystemMenuItem) -> Option<MenuAction> {
    match item {
        SystemMenuItem::NewWindow => Some(MenuAction::NewWindow),
        SystemMenuItem::AspectRatio => Some(MenuAction::AspectRatio),
        SystemMenuItem::OpenByDefault => Some(MenuAction::OpenByDefault),
        SystemMenuItem::AppInfo => Some(MenuAction::AppInfo),
        SystemMenuItem::Separator | SystemMenuItem::Screenshot | SystemMenuItem::FullScreen => None,
    }
}

/// The menu's words in one language. Each label marks its access key with `&`; the keys
/// differ from Windows' own items (R, M, S, N, X, C), so each key still picks one item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuText {
    pub full_screen: &'static str,
    pub new_window: &'static str,
    pub screenshot: &'static str,
    pub aspect_ratio: &'static str,
    pub open_by_default: &'static str,
    pub app_info: &'static str,
    /// What a screenshot's file name starts with, as Windows' Snipping Tool names its own.
    pub screenshot_file: &'static str,
}

/// The menu's words in `language`. The access keys differ from those of Windows' own items in
/// the same language ([`super::window_menu::window_text`]).
pub fn menu_text(language: UiLanguage) -> MenuText {
    let [full_screen, new_window, screenshot, aspect_ratio, open_by_default, app_info, screenshot_file] =
        match language {
            UiLanguage::English => [
                "&Full screen",
                "N&ew window",
                "Screens&hot",
                "&Aspect ratio",
                "&Open by default",
                "App &info",
                "Screenshot",
            ],
            UiLanguage::Korean => [
                "전체 화면(&F)",
                "새 창(&E)",
                "스크린샷(&H)",
                "화면 비율(&A)",
                "링크 열기 설정(&O)",
                "앱 정보(&I)",
                "스크린샷",
            ],
            UiLanguage::Japanese => [
                "全画面表示(&F)",
                "新しいウィンドウ(&E)",
                "スクリーンショット(&H)",
                "縦横比(&A)",
                "デフォルトで開く(&O)",
                "アプリ情報(&I)",
                "スクリーンショット",
            ],
            UiLanguage::ChineseSimplified => [
                "全屏(&F)",
                "新建窗口(&E)",
                "屏幕截图(&H)",
                "宽高比(&A)",
                "默认打开(&O)",
                "应用信息(&I)",
                "屏幕截图",
            ],
            UiLanguage::ChineseTraditional => [
                "全螢幕(&F)",
                "新增視窗(&E)",
                "螢幕擷取畫面(&H)",
                "長寬比(&A)",
                "預設開啟(&O)",
                "應用程式資訊(&I)",
                "螢幕擷取畫面",
            ],
            UiLanguage::Spanish => [
                "&Pantalla completa",
                "Nu&eva ventana",
                "Capt&ura de pantalla",
                "Rel&ación de aspecto",
                "Abrir &de forma predeterminada",
                "&Información de la aplicación",
                "Captura de pantalla",
            ],
            UiLanguage::French => [
                "&Plein écran",
                "&Nouvelle fenêtre",
                "&Capture d'écran",
                "R&apport hauteur/largeur",
                "&Ouvrir par défaut",
                "&Infos sur l'appli",
                "Capture d'écran",
            ],
            UiLanguage::German => [
                "Voll&bild",
                "Neues &Fenster",
                "Scr&eenshot",
                "Se&itenverhältnis",
                "S&tandardmäßig öffnen",
                "&App-Info",
                "Screenshot",
            ],
            UiLanguage::PortugueseBrazil => [
                "Tela &inteira",
                "Nova &janela",
                "Ca&ptura de tela",
                "Pr&oporção",
                "&Abrir por padrão",
                "Informações &do app",
                "Captura de tela",
            ],
            UiLanguage::Russian => [
                "Во весь &экран",
                "&Новое окно",
                "Сни&мок экрана",
                "Соо&тношение сторон",
                "&Открывать по умолчанию",
                "О при&ложении",
                "Снимок экрана",
            ],
            UiLanguage::Italian => [
                "Sc&hermo intero",
                "&Nuova finestra",
                "Screensh&ot",
                "&Proporzioni",
                "&Apri per impostazione predefinita",
                "In&formazioni app",
                "Screenshot",
            ],
            UiLanguage::Vietnamese => [
                "Toàn màn &hình",
                "&Cửa sổ mới",
                "Ảnh chụp &màn hình",
                "Tỷ &lệ khung hình",
                "Mở th&eo mặc định",
                "Thông t&in ứng dụng",
                "Ảnh chụp màn hình",
            ],
            UiLanguage::Indonesian => [
                "&Layar penuh",
                "Jendela &baru",
                "Tan&gkapan layar",
                "&Rasio aspek",
                "Buka se&cara default",
                "&Info aplikasi",
                "Tangkapan layar",
            ],
            UiLanguage::Turkish => [
                "Tam ekra&n",
                "Yeni &pencere",
                "Ekr&an görüntüsü",
                "En b&oy oranı",
                "&Varsayılan olarak aç",
                "&Uygulama bilgisi",
                "Ekran görüntüsü",
            ],
        };
    MenuText {
        full_screen,
        new_window,
        screenshot,
        aspect_ratio,
        open_by_default,
        app_info,
        screenshot_file,
    }
}

/// The label of an entry; `None` for the separator.
pub fn label(text: &MenuText, item: SystemMenuItem) -> Option<&'static str> {
    Some(match item {
        SystemMenuItem::Separator => return None,
        SystemMenuItem::FullScreen => text.full_screen,
        SystemMenuItem::NewWindow => text.new_window,
        SystemMenuItem::Screenshot => text.screenshot,
        SystemMenuItem::AspectRatio => text.aspect_ratio,
        SystemMenuItem::OpenByDefault => text.open_by_default,
        SystemMenuItem::AppInfo => text.app_info,
    })
}

/// The menu's words in the user's display language.
#[cfg(windows)]
pub fn display_text() -> MenuText {
    menu_text(crate::ui_language::user_language())
}

/// Makes the Android part of `hwnd`'s system menu again: the entries [`items`] gives for
/// `new_window`, after Windows' own. Entries made before are removed first, so the menu
/// follows the app when it starts or stops supporting several windows.
///
/// # Safety
///
/// `hwnd` is a live window of the calling thread.
#[cfg(windows)]
pub unsafe fn apply(hwnd: windows_sys::Win32::Foundation::HWND, new_window: bool) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        DeleteMenu, GetMenuItemCount, GetSystemMenu, InsertMenuItemW, MENUITEMINFOW, MFT_SEPARATOR,
        MFT_STRING, MF_BYCOMMAND, MIIM_FTYPE, MIIM_ID, MIIM_STRING,
    };
    let menu = GetSystemMenu(hwnd, 0);
    if menu.is_null() {
        return;
    }
    for item in ALL {
        // An entry the menu does not have is simply not found.
        DeleteMenu(menu, command(item), MF_BYCOMMAND);
    }
    let text = display_text();
    for item in items(new_window) {
        let mut label: Vec<u16> = label(&text, item)
            .unwrap_or("")
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let mut info: MENUITEMINFOW = std::mem::zeroed();
        info.cbSize = std::mem::size_of::<MENUITEMINFOW>() as u32;
        info.wID = command(item);
        if item == SystemMenuItem::Separator {
            info.fMask = MIIM_FTYPE | MIIM_ID;
            info.fType = MFT_SEPARATOR;
        } else {
            info.fMask = MIIM_FTYPE | MIIM_ID | MIIM_STRING;
            info.fType = MFT_STRING;
            info.dwTypeData = label.as_mut_ptr();
            info.cch = (label.len() - 1) as u32;
        }
        let at = GetMenuItemCount(menu).max(0) as u32;
        // The menu copies the label; `label` only has to live through the call.
        InsertMenuItemW(menu, at, 1, &info);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fr22_the_system_menu_carries_the_android_window_menu() {
        use SystemMenuItem::*;
        // After Windows' own items: a separator, then the Android window menu in its order.
        assert_eq!(
            items(true),
            vec![
                Separator,
                FullScreen,
                NewWindow,
                Screenshot,
                AspectRatio,
                OpenByDefault,
                AppInfo
            ]
        );
        // New window only for an app that supports several windows.
        assert_eq!(
            items(false),
            vec![
                Separator,
                FullScreen,
                Screenshot,
                AspectRatio,
                OpenByDefault,
                AppInfo
            ]
        );
        // Each ID is one WM_SYSCOMMAND carries back whole, and none is one of Windows' own.
        for item in ALL {
            let id = command(item);
            assert_eq!(id & 0xF, 0, "{item:?}");
            assert!(id < 0xF000, "{item:?}");
            assert_eq!(ALL.iter().filter(|other| command(**other) == id).count(), 1);
        }
        // Windows sets the low four bits of a system command; the item is still found.
        assert_eq!(chosen(command(Screenshot) as usize | 0x2), Some(Screenshot));
        assert_eq!(chosen(command(Separator) as usize), None);
        // SC_CLOSE stays Windows'.
        assert_eq!(chosen(0xF060), None);
        // The window manager carries out every item but the screenshot.
        assert_eq!(action(NewWindow), Some(MenuAction::NewWindow));
        assert_eq!(action(AspectRatio), Some(MenuAction::AspectRatio));
        assert_eq!(action(OpenByDefault), Some(MenuAction::OpenByDefault));
        assert_eq!(action(AppInfo), Some(MenuAction::AppInfo));
        assert_eq!(action(Screenshot), None);
        // Full screen goes on and off in the window itself, as F11 does.
        assert_eq!(action(FullScreen), None);
        assert_eq!(chosen(command(FullScreen) as usize), Some(FullScreen));
    }

    #[test]
    fn fr22_the_system_menu_speaks_korean_and_english() {
        for language in [UiLanguage::English, UiLanguage::Korean] {
            let text = menu_text(language);
            let labels: Vec<&str> = items(true)
                .into_iter()
                .filter_map(|item| label(&text, item))
                .collect();
            assert_eq!(labels.len(), 6);
            // One access key each, none of Windows' own (R, M, S, N, X, C).
            let mut keys = Vec::new();
            for label in labels {
                let at = label.find('&').expect(label);
                let key = label[at + 1..].chars().next().unwrap().to_ascii_uppercase();
                assert!(!"RMSNXC".contains(key), "{label}");
                assert!(!keys.contains(&key), "{label}");
                keys.push(key);
            }
        }
        assert_eq!(menu_text(UiLanguage::English).new_window, "N&ew window");
        assert_eq!(menu_text(UiLanguage::Korean).app_info, "앱 정보(&I)");
        assert_eq!(menu_text(UiLanguage::Korean).screenshot_file, "스크린샷");
    }

    #[test]
    fn fr17_the_system_menu_speaks_every_language() {
        use crate::ui_language::assert_translation;
        let english = menu_text(UiLanguage::English);
        for language in UiLanguage::ALL {
            let text = menu_text(language);
            let mut keys = Vec::new();
            for item in items(true) {
                let (Some(en), Some(marked)) = (label(&english, item), label(&text, item)) else {
                    continue;
                };
                assert_translation(language, &format!("{item:?}"), en, marked);
                // Exactly one access key, and no two items share one.
                assert_eq!(marked.matches('&').count(), 1, "{marked}");
                let at = marked.find('&').unwrap();
                let key = marked[at + 1..].chars().next().unwrap();
                let key: String = key.to_uppercase().collect();
                assert!(!keys.contains(&key), "{language:?}: {marked}");
                keys.push(key);
            }
            assert_translation(
                language,
                "the screenshot file name",
                english.screenshot_file,
                text.screenshot_file,
            );
            // A file name Windows accepts.
            assert!(!text
                .screenshot_file
                .contains(['\\', '/', ':', '*', '?', '"', '<', '>', '|']));
        }
    }
}
