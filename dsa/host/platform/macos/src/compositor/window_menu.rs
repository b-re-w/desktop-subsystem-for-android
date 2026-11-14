//! The menu bar of an app's window host: the app menu and the Window menu.
//!
//! A Mac app's window commands live in its Window menu, so that is where the items of an
//! Android desktop window's window menu go. After macOS's own items (Minimize, Zoom) and a
//! separator come New Window (only for an app that supports several windows), Screenshot,
//! Aspect Ratio, Open by Default and App Info, then another separator, Bring All to Front,
//! and the list of the app's windows, which AppKit keeps. The same items open from a
//! secondary click on the band of a window whose app draws its own title bar, where the
//! window has no title of macOS's own to click.
//!
//! Each Android item acts on the app's front window. The window host takes the screenshot
//! itself, since it has the window's frames; every other item goes to the window manager as
//! `FromHost::MenuChosen`.
//!
//! The words are in the first of the user's preferred languages DSA has a translation for,
//! else English ([`crate::ui_language`]).

use dsa_host_core::compositor::window_hosts::MenuAction;
use dsa_host_core::ui_language::UiLanguage;

/// One entry of the Window menu, before the window list AppKit adds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowMenuItem {
    Separator,
    Minimize,
    Zoom,
    NewWindow,
    Screenshot,
    AspectRatio,
    OpenByDefault,
    AppInfo,
    BringAllToFront,
}

/// The Window menu's entries, in order. New Window is left out for an app that does not
/// support several windows, since its second task would only bring the first to the front.
pub fn window_menu(new_window: bool) -> Vec<WindowMenuItem> {
    let mut items = vec![
        WindowMenuItem::Minimize,
        WindowMenuItem::Zoom,
        WindowMenuItem::Separator,
    ];
    items.extend(android_items(new_window));
    items.extend([WindowMenuItem::Separator, WindowMenuItem::BringAllToFront]);
    items
}

/// The Android desktop window's items alone, as the band's secondary click shows them after
/// Minimize and Zoom.
pub fn android_items(new_window: bool) -> Vec<WindowMenuItem> {
    let mut items = Vec::new();
    if new_window {
        items.push(WindowMenuItem::NewWindow);
    }
    items.extend([
        WindowMenuItem::Screenshot,
        WindowMenuItem::AspectRatio,
        WindowMenuItem::OpenByDefault,
        WindowMenuItem::AppInfo,
    ]);
    items
}

/// The menu item tag that names an Android entry when it is chosen; 0 for macOS's own entries
/// and the separator, which AppKit carries out itself.
pub fn tag(item: WindowMenuItem) -> isize {
    match item {
        WindowMenuItem::NewWindow => 1,
        WindowMenuItem::Screenshot => 2,
        WindowMenuItem::AspectRatio => 3,
        WindowMenuItem::OpenByDefault => 4,
        WindowMenuItem::AppInfo => 5,
        _ => 0,
    }
}

/// The Android entry a chosen menu item's tag names.
pub fn from_tag(tag: isize) -> Option<WindowMenuItem> {
    [
        WindowMenuItem::NewWindow,
        WindowMenuItem::Screenshot,
        WindowMenuItem::AspectRatio,
        WindowMenuItem::OpenByDefault,
        WindowMenuItem::AppInfo,
    ]
    .into_iter()
    .find(|item| self::tag(*item) == tag)
}

/// What the window manager does for an entry; `None` for Screenshot, which the window host
/// takes, and for macOS's own entries.
pub fn action(item: WindowMenuItem) -> Option<MenuAction> {
    match item {
        WindowMenuItem::NewWindow => Some(MenuAction::NewWindow),
        WindowMenuItem::AspectRatio => Some(MenuAction::AspectRatio),
        WindowMenuItem::OpenByDefault => Some(MenuAction::OpenByDefault),
        WindowMenuItem::AppInfo => Some(MenuAction::AppInfo),
        _ => None,
    }
}

/// The AppKit action macOS's own entries send down the responder chain; `None` for the
/// separator and the Android entries.
pub fn system_action(item: WindowMenuItem) -> Option<&'static str> {
    match item {
        WindowMenuItem::Minimize => Some("performMiniaturize:"),
        WindowMenuItem::Zoom => Some("performZoom:"),
        WindowMenuItem::BringAllToFront => Some("arrangeInFront:"),
        _ => None,
    }
}

/// The key that chooses an entry with Command, as every Mac app has it; empty for none.
pub fn key_equivalent(item: WindowMenuItem) -> &'static str {
    match item {
        WindowMenuItem::Minimize => "m",
        _ => "",
    }
}

/// The menu bar's words in one language. `{app}` in an entry stands for the app's name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuText {
    pub window: &'static str,
    pub minimize: &'static str,
    pub zoom: &'static str,
    pub new_window: &'static str,
    pub screenshot: &'static str,
    pub aspect_ratio: &'static str,
    pub open_by_default: &'static str,
    pub app_info: &'static str,
    pub bring_all_to_front: &'static str,
    pub hide_app: &'static str,
    pub hide_others: &'static str,
    pub show_all: &'static str,
    pub quit_app: &'static str,
}

/// The menu bar's words in `lang`, as macOS's own menus say them.
pub fn menu_text(lang: UiLanguage) -> MenuText {
    match lang {
        UiLanguage::English => MenuText {
            window: "Window",
            minimize: "Minimize",
            zoom: "Zoom",
            new_window: "New Window",
            screenshot: "Screenshot",
            aspect_ratio: "Aspect Ratio\u{2026}",
            open_by_default: "Open by Default\u{2026}",
            app_info: "App Info",
            bring_all_to_front: "Bring All to Front",
            hide_app: "Hide {app}",
            hide_others: "Hide Others",
            show_all: "Show All",
            quit_app: "Quit {app}",
        },
        UiLanguage::Korean => MenuText {
            window: "윈도우",
            minimize: "최소화",
            zoom: "확대/축소",
            new_window: "새 창",
            screenshot: "스크린샷",
            aspect_ratio: "화면 비율\u{2026}",
            open_by_default: "링크 열기 설정\u{2026}",
            app_info: "앱 정보",
            bring_all_to_front: "모두 앞으로 가져오기",
            hide_app: "{app} 가리기",
            hide_others: "기타 가리기",
            show_all: "모두 보기",
            quit_app: "{app} 종료",
        },
        UiLanguage::Japanese => MenuText {
            window: "ウインドウ",
            minimize: "しまう",
            zoom: "拡大/縮小",
            new_window: "新規ウインドウ",
            screenshot: "スクリーンショット",
            aspect_ratio: "縦横比\u{2026}",
            open_by_default: "デフォルトで開く\u{2026}",
            app_info: "アプリ情報",
            bring_all_to_front: "すべてを手前に移動",
            hide_app: "{app}を非表示",
            hide_others: "ほかを非表示",
            show_all: "すべてを表示",
            quit_app: "{app}を終了",
        },
        UiLanguage::ChineseSimplified => MenuText {
            window: "窗口",
            minimize: "最小化",
            zoom: "缩放",
            new_window: "新建窗口",
            screenshot: "截屏",
            aspect_ratio: "宽高比\u{2026}",
            open_by_default: "默认打开\u{2026}",
            app_info: "应用信息",
            bring_all_to_front: "前置全部窗口",
            hide_app: "隐藏\u{201c}{app}\u{201d}",
            hide_others: "隐藏其他",
            show_all: "全部显示",
            quit_app: "退出\u{201c}{app}\u{201d}",
        },
        UiLanguage::ChineseTraditional => MenuText {
            window: "視窗",
            minimize: "縮到最小",
            zoom: "縮放",
            new_window: "新增視窗",
            screenshot: "截圖",
            aspect_ratio: "長寬比\u{2026}",
            open_by_default: "預設開啟\u{2026}",
            app_info: "App 資訊",
            bring_all_to_front: "將此程式所有視窗移至最前",
            hide_app: "隱藏\u{300c}{app}\u{300d}",
            hide_others: "隱藏其他",
            show_all: "顯示全部",
            quit_app: "結束\u{300c}{app}\u{300d}",
        },
        UiLanguage::Spanish => MenuText {
            window: "Ventana",
            minimize: "Minimizar",
            zoom: "Zoom",
            new_window: "Nueva ventana",
            screenshot: "Captura de pantalla",
            aspect_ratio: "Relación de aspecto\u{2026}",
            open_by_default: "Abrir de forma predeterminada\u{2026}",
            app_info: "Información de la app",
            bring_all_to_front: "Traer todo al frente",
            hide_app: "Ocultar {app}",
            hide_others: "Ocultar otros",
            show_all: "Mostrar todo",
            quit_app: "Salir de {app}",
        },
        UiLanguage::French => MenuText {
            window: "Fenêtre",
            minimize: "Placer dans le Dock",
            zoom: "Réduire/agrandir",
            new_window: "Nouvelle fenêtre",
            screenshot: "Capture d\u{2019}écran",
            aspect_ratio: "Format d\u{2019}image\u{2026}",
            open_by_default: "Ouvrir par défaut\u{2026}",
            app_info: "Infos sur l\u{2019}app",
            bring_all_to_front: "Tout ramener au premier plan",
            hide_app: "Masquer {app}",
            hide_others: "Masquer les autres",
            show_all: "Tout afficher",
            quit_app: "Quitter {app}",
        },
        UiLanguage::German => MenuText {
            window: "Fenster",
            minimize: "Im Dock ablegen",
            zoom: "Zoomen",
            new_window: "Neues Fenster",
            screenshot: "Bildschirmfoto",
            aspect_ratio: "Seitenverhältnis\u{2026}",
            open_by_default: "Standardmäßig öffnen\u{2026}",
            app_info: "App-Info",
            bring_all_to_front: "Alle nach vorne bringen",
            hide_app: "{app} ausblenden",
            hide_others: "Andere ausblenden",
            show_all: "Alle einblenden",
            quit_app: "{app} beenden",
        },
        UiLanguage::PortugueseBrazil => MenuText {
            window: "Janela",
            minimize: "Minimizar",
            zoom: "Zoom",
            new_window: "Nova Janela",
            screenshot: "Captura de Tela",
            aspect_ratio: "Proporção\u{2026}",
            open_by_default: "Abrir por Padrão\u{2026}",
            app_info: "Informações do App",
            bring_all_to_front: "Trazer Todas para a Frente",
            hide_app: "Ocultar {app}",
            hide_others: "Ocultar Outros",
            show_all: "Mostrar Tudo",
            quit_app: "Encerrar {app}",
        },
        UiLanguage::Russian => MenuText {
            window: "Окно",
            minimize: "Свернуть",
            zoom: "Изменить масштаб",
            new_window: "Новое окно",
            screenshot: "Снимок экрана",
            aspect_ratio: "Соотношение сторон\u{2026}",
            open_by_default: "Открывать по умолчанию\u{2026}",
            app_info: "О приложении",
            bring_all_to_front: "Переместить все окна вперед",
            hide_app: "Скрыть {app}",
            hide_others: "Скрыть остальные",
            show_all: "Показать все",
            quit_app: "Завершить {app}",
        },
        UiLanguage::Italian => MenuText {
            window: "Finestra",
            minimize: "Contrai",
            zoom: "Zoom",
            new_window: "Nuova finestra",
            screenshot: "Istantanea schermo",
            aspect_ratio: "Proporzioni\u{2026}",
            open_by_default: "Apri per impostazione predefinita\u{2026}",
            app_info: "Info app",
            bring_all_to_front: "Porta tutto in primo piano",
            hide_app: "Nascondi {app}",
            hide_others: "Nascondi altre",
            show_all: "Mostra tutte",
            quit_app: "Esci da {app}",
        },
        UiLanguage::Vietnamese => MenuText {
            window: "Cửa sổ",
            minimize: "Thu nhỏ",
            zoom: "Thu/phóng",
            new_window: "Cửa sổ mới",
            screenshot: "Ảnh chụp màn hình",
            aspect_ratio: "Tỷ lệ khung hình\u{2026}",
            open_by_default: "Mở theo mặc định\u{2026}",
            app_info: "Thông tin ứng dụng",
            bring_all_to_front: "Đưa tất cả ra trước",
            hide_app: "Ẩn {app}",
            hide_others: "Ẩn ứng dụng khác",
            show_all: "Hiển thị tất cả",
            quit_app: "Thoát {app}",
        },
        UiLanguage::Indonesian => MenuText {
            window: "Jendela",
            minimize: "Minimalkan",
            zoom: "Perbesar/Perkecil",
            new_window: "Jendela Baru",
            screenshot: "Jepretan Layar",
            aspect_ratio: "Rasio Aspek\u{2026}",
            open_by_default: "Buka secara Default\u{2026}",
            app_info: "Info App",
            bring_all_to_front: "Bawa Semua ke Depan",
            hide_app: "Sembunyikan {app}",
            hide_others: "Sembunyikan Lainnya",
            show_all: "Tampilkan Semua",
            quit_app: "Keluar dari {app}",
        },
        UiLanguage::Turkish => MenuText {
            window: "Pencere",
            minimize: "Küçült",
            zoom: "Büyüt/Küçült",
            new_window: "Yeni Pencere",
            screenshot: "Ekran Görüntüsü",
            aspect_ratio: "En Boy Oranı\u{2026}",
            open_by_default: "Varsayılan Olarak Aç\u{2026}",
            app_info: "Uygulama Bilgileri",
            bring_all_to_front: "Tümünü Öne Getir",
            hide_app: "{app} Uygulamasını Gizle",
            hide_others: "Diğerlerini Gizle",
            show_all: "Tümünü Göster",
            quit_app: "{app} Uygulamasından Çık",
        },
    }
}

/// The label of a Window menu entry; `None` for the separator.
pub fn label(text: &MenuText, item: WindowMenuItem) -> Option<&'static str> {
    Some(match item {
        WindowMenuItem::Separator => return None,
        WindowMenuItem::Minimize => text.minimize,
        WindowMenuItem::Zoom => text.zoom,
        WindowMenuItem::NewWindow => text.new_window,
        WindowMenuItem::Screenshot => text.screenshot,
        WindowMenuItem::AspectRatio => text.aspect_ratio,
        WindowMenuItem::OpenByDefault => text.open_by_default,
        WindowMenuItem::AppInfo => text.app_info,
        WindowMenuItem::BringAllToFront => text.bring_all_to_front,
    })
}

/// One entry of the app menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppMenuItem {
    Separator,
    /// Its label, its AppKit action, its key with Command, and whether Option goes with it.
    Item(&'static str, &'static str, &'static str, bool),
}

/// The app menu: Hide, Hide Others, Show All and Quit, as every Mac app has them. Quitting
/// the window host closes the app's windows, which ends their tasks, as closing them does.
pub fn app_menu(text: &MenuText) -> Vec<AppMenuItem> {
    vec![
        AppMenuItem::Item(text.hide_app, "hide:", "h", false),
        AppMenuItem::Item(text.hide_others, "hideOtherApplications:", "h", true),
        AppMenuItem::Item(text.show_all, "unhideAllApplications:", "", false),
        AppMenuItem::Separator,
        AppMenuItem::Item(text.quit_app, "terminate:", "q", false),
    ]
}

/// `label` with the app's name in the place of `{app}`.
pub fn with_app(label: &str, app: &str) -> String {
    label.replace("{app}", app)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fr22_the_window_menu_carries_the_android_window_menu() {
        use WindowMenuItem::*;
        assert_eq!(
            window_menu(true),
            vec![
                Minimize,
                Zoom,
                Separator,
                NewWindow,
                Screenshot,
                AspectRatio,
                OpenByDefault,
                AppInfo,
                Separator,
                BringAllToFront,
            ]
        );
    }

    #[test]
    fn fr22_new_window_is_left_out_for_an_app_with_one_window() {
        assert!(!window_menu(false).contains(&WindowMenuItem::NewWindow));
        assert!(!android_items(false).contains(&WindowMenuItem::NewWindow));
        assert_eq!(
            android_items(false).first(),
            Some(&WindowMenuItem::Screenshot)
        );
    }

    #[test]
    fn fr22_each_android_item_is_carried_out_where_it_belongs() {
        for item in android_items(true) {
            assert_eq!(from_tag(tag(item)), Some(item));
        }
        assert_eq!(from_tag(0), None);
        assert_eq!(
            action(WindowMenuItem::NewWindow),
            Some(MenuAction::NewWindow)
        );
        assert_eq!(
            action(WindowMenuItem::AspectRatio),
            Some(MenuAction::AspectRatio)
        );
        assert_eq!(
            action(WindowMenuItem::OpenByDefault),
            Some(MenuAction::OpenByDefault)
        );
        assert_eq!(action(WindowMenuItem::AppInfo), Some(MenuAction::AppInfo));
        // The window host takes the screenshot itself.
        assert_eq!(action(WindowMenuItem::Screenshot), None);
        // macOS carries out its own entries.
        assert_eq!(
            system_action(WindowMenuItem::Minimize),
            Some("performMiniaturize:")
        );
        assert_eq!(tag(WindowMenuItem::Minimize), 0);
    }

    #[test]
    fn fr22_the_menus_speak_the_users_language() {
        let english = menu_text(UiLanguage::English);
        let korean = menu_text(UiLanguage::Korean);
        for item in window_menu(true) {
            if item == WindowMenuItem::Separator {
                assert_eq!(label(&english, item), None);
                continue;
            }
            let (en, ko) = (
                label(&english, item).unwrap(),
                label(&korean, item).unwrap(),
            );
            assert!(!en.is_empty() && !ko.is_empty() && en != ko, "{item:?}");
        }
        assert_eq!(label(&korean, WindowMenuItem::AppInfo), Some("앱 정보"));
        assert_eq!(with_app(english.quit_app, "Notes"), "Quit Notes");
        assert_eq!(with_app(korean.hide_app, "메모"), "메모 가리기");
        let german = menu_text(UiLanguage::German);
        assert_eq!(with_app(german.quit_app, "Notizen"), "Notizen beenden");
    }

    #[test]
    fn fr17_every_menu_entry_is_in_every_language() {
        for lang in UiLanguage::ALL {
            let text = menu_text(lang);
            let mut words: Vec<&str> = window_menu(true)
                .into_iter()
                .filter_map(|item| label(&text, item))
                .collect();
            words.push(text.window);
            for entry in app_menu(&text) {
                if let AppMenuItem::Item(title, ..) = entry {
                    words.push(title);
                }
            }
            for word in &words {
                assert!(!word.trim().is_empty(), "{lang:?} has an empty entry");
                assert!(!crate::ui_language::has_em_dash(word), "{lang:?}: {word}");
            }
            // Hide and Quit name the app, wherever the language puts its name.
            for named in [text.hide_app, text.quit_app] {
                assert_eq!(
                    dsa_host_core::ui_language::placeholders(named),
                    ["app"],
                    "{lang:?}: {named}"
                );
            }
            for unnamed in [text.hide_others, text.show_all, text.window] {
                assert!(
                    dsa_host_core::ui_language::placeholders(unnamed).is_empty(),
                    "{lang:?}: {unnamed}"
                );
            }
            // The entries that open a dialog end in an ellipsis, as on every Mac.
            for dialog in [text.aspect_ratio, text.open_by_default] {
                assert!(dialog.ends_with('\u{2026}'), "{lang:?}: {dialog}");
            }
            if lang != UiLanguage::English {
                assert_ne!(text.window, english_window(), "{lang:?} is not translated");
            }
        }
    }

    fn english_window() -> &'static str {
        menu_text(UiLanguage::English).window
    }
}
