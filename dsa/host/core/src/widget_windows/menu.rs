//! What the menu of a widget window offers, whichever host draws it: Widget settings, for a
//! widget whose app lets it be set up again, Resize, with the sizes the widget allows, Move
//! and Size, which move and resize it with the arrow keys as a window's system menu does
//! ([`super::keyboard`]), and Remove, as Android's home screen offers them.
//!
//! The user opens it by long-pressing the widget, by right-clicking it, or with the menu key,
//! Shift+F10, Enter or Space while the widget has the keyboard.
//! Choosing Widget settings asks the service to open the widget's setup again, which
//! shows as the app's window; once the user finishes it, the widget shows what they chose.
//! Choosing a size resizes the window, which tells the service and Android the new size as a
//! drag of its edges does. Choosing Remove asks the service to remove the widget; its window
//! closes when the service says the widget is gone, so the settings app and every other
//! watcher learn of it the same way.

use crate::ui_language::UiLanguage;
use crate::widgets::{clamp_size, Placement, WidgetProvider};

/// A size the Resize menu offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeName {
    /// The smallest the widget allows.
    Small,
    /// The size the widget is placed at.
    Default,
    /// One and a half times its default.
    Large,
    /// Twice its default.
    ExtraLarge,
}

/// One size of the Resize menu, in dp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SizeChoice {
    pub name: SizeName,
    pub width_dp: u32,
    pub height_dp: u32,
}

/// What the user chose in the menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuChoice {
    /// Resize the window to this size, in dp.
    Resize { width_dp: u32, height_dp: u32 },
    /// Remove the widget.
    Remove,
    /// Open the widget's setup again.
    Settings,
    /// Move the widget with the arrow keys until Enter or Esc.
    Move,
    /// Resize the widget with the arrow keys until Enter or Esc.
    Size,
}

/// The menu item of Remove. A size is [`FIRST_SIZE`] plus its place in the list.
pub const REMOVE: u32 = 1;
/// The menu item that opens the sizes.
pub const RESIZE: u32 = 2;
/// The menu item of Widget settings.
pub const SETTINGS: u32 = 3;
/// The menu item of Move, with the keyboard.
pub const MOVE: u32 = 4;
/// The menu item of Size, with the keyboard.
pub const SIZE: u32 = 5;
/// The menu item of the first size.
pub const FIRST_SIZE: u32 = 100;

/// The sizes the Resize menu offers for `provider`, each within what the widget allows. A
/// size equal to one before it is left out, so a widget that cannot be resized offers one.
pub fn size_choices(provider: &WidgetProvider) -> Vec<SizeChoice> {
    let (width, height) = clamp_size(
        provider,
        provider.default_width_dp,
        provider.default_height_dp,
    );
    let asked = [
        (SizeName::Small, 0, 0),
        (SizeName::Default, width, height),
        (SizeName::Large, width * 3 / 2, height * 3 / 2),
        (SizeName::ExtraLarge, width * 2, height * 2),
    ];
    let mut out: Vec<SizeChoice> = Vec::with_capacity(asked.len());
    for (name, w, h) in asked {
        let (width_dp, height_dp) = clamp_size(provider, w, h);
        if out
            .iter()
            .all(|c| (c.width_dp, c.height_dp) != (width_dp, height_dp))
        {
            out.push(SizeChoice {
                name,
                width_dp,
                height_dp,
            });
        }
    }
    out
}

/// What the menu item `command` chose among `sizes`; None for no choice (as when the menu was
/// dismissed) or an unknown one.
pub fn choice_of(command: u32, sizes: &[SizeChoice]) -> Option<MenuChoice> {
    if command == REMOVE {
        return Some(MenuChoice::Remove);
    }
    if command == SETTINGS {
        return Some(MenuChoice::Settings);
    }
    if command == MOVE {
        return Some(MenuChoice::Move);
    }
    if command == SIZE {
        return Some(MenuChoice::Size);
    }
    let size = sizes.get(command.checked_sub(FIRST_SIZE)? as usize)?;
    Some(MenuChoice::Resize {
        width_dp: size.width_dp,
        height_dp: size.height_dp,
    })
}

/// Whether `placement` is at the size of `choice`, which the menu marks.
pub fn is_current(choice: &SizeChoice, placement: &Placement) -> bool {
    (choice.width_dp, choice.height_dp) == (placement.width_dp, placement.height_dp)
}

/// The words of the menu in one language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuText {
    pub settings: &'static str,
    pub resize: &'static str,
    /// Move with the arrow keys, as a window's system menu says it.
    pub move_widget: &'static str,
    /// Resize with the arrow keys, as a window's system menu says it.
    pub size: &'static str,
    pub remove: &'static str,
    pub small: &'static str,
    pub default: &'static str,
    pub large: &'static str,
    pub extra_large: &'static str,
    /// What assistive technology calls a widget with no name of its own.
    pub widget: &'static str,
}

/// The menu's words in `lang`.
pub fn menu_text(lang: UiLanguage) -> MenuText {
    match lang {
        UiLanguage::English => MenuText {
            settings: "Widget settings",
            resize: "Resize",
            move_widget: "Move",
            size: "Size",
            remove: "Remove",
            small: "Small",
            default: "Default size",
            large: "Large",
            extra_large: "Extra large",
            widget: "Widget",
        },
        UiLanguage::Korean => MenuText {
            settings: "위젯 설정",
            resize: "크기 선택",
            move_widget: "이동",
            size: "크기 조정",
            remove: "제거",
            small: "작게",
            default: "기본 크기",
            large: "크게",
            extra_large: "아주 크게",
            widget: "위젯",
        },
        UiLanguage::Japanese => MenuText {
            settings: "ウィジェットの設定",
            resize: "サイズの選択",
            move_widget: "移動",
            size: "サイズ変更",
            remove: "削除",
            small: "小",
            default: "標準サイズ",
            large: "大",
            extra_large: "特大",
            widget: "ウィジェット",
        },
        UiLanguage::ChineseSimplified => MenuText {
            settings: "小组件设置",
            resize: "选择大小",
            move_widget: "移动",
            size: "大小",
            remove: "删除",
            small: "小",
            default: "默认大小",
            large: "大",
            extra_large: "特大",
            widget: "小组件",
        },
        UiLanguage::ChineseTraditional => MenuText {
            settings: "小工具設定",
            resize: "選擇大小",
            move_widget: "移動",
            size: "大小",
            remove: "移除",
            small: "小",
            default: "預設大小",
            large: "大",
            extra_large: "特大",
            widget: "小工具",
        },
        UiLanguage::Spanish => MenuText {
            settings: "Configuración del widget",
            resize: "Cambiar tamaño",
            move_widget: "Mover",
            size: "Tamaño",
            remove: "Quitar",
            small: "Pequeño",
            default: "Tamaño predeterminado",
            large: "Grande",
            extra_large: "Muy grande",
            widget: "Widget",
        },
        UiLanguage::French => MenuText {
            settings: "Paramètres du widget",
            resize: "Redimensionner",
            move_widget: "Déplacer",
            size: "Taille",
            remove: "Supprimer",
            small: "Petit",
            default: "Taille par défaut",
            large: "Grand",
            extra_large: "Très grand",
            widget: "Widget",
        },
        UiLanguage::German => MenuText {
            settings: "Widget-Einstellungen",
            resize: "Größe ändern",
            move_widget: "Verschieben",
            size: "Größe",
            remove: "Entfernen",
            small: "Klein",
            default: "Standardgröße",
            large: "Groß",
            extra_large: "Sehr groß",
            widget: "Widget",
        },
        UiLanguage::PortugueseBrazil => MenuText {
            settings: "Configurações do widget",
            resize: "Redimensionar",
            move_widget: "Mover",
            size: "Tamanho",
            remove: "Remover",
            small: "Pequeno",
            default: "Tamanho padrão",
            large: "Grande",
            extra_large: "Muito grande",
            widget: "Widget",
        },
        UiLanguage::Russian => MenuText {
            settings: "Настройки виджета",
            resize: "Изменить размер",
            move_widget: "Переместить",
            size: "Размер",
            remove: "Удалить",
            small: "Маленький",
            default: "Стандартный размер",
            large: "Большой",
            extra_large: "Очень большой",
            widget: "Виджет",
        },
        UiLanguage::Italian => MenuText {
            settings: "Impostazioni widget",
            resize: "Ridimensiona",
            move_widget: "Sposta",
            size: "Dimensione",
            remove: "Rimuovi",
            small: "Piccolo",
            default: "Dimensione predefinita",
            large: "Grande",
            extra_large: "Molto grande",
            widget: "Widget",
        },
        UiLanguage::Vietnamese => MenuText {
            settings: "Cài đặt tiện ích",
            resize: "Đổi kích thước",
            move_widget: "Di chuyển",
            size: "Kích thước",
            remove: "Gỡ bỏ",
            small: "Nhỏ",
            default: "Kích thước mặc định",
            large: "Lớn",
            extra_large: "Rất lớn",
            widget: "Tiện ích",
        },
        UiLanguage::Indonesian => MenuText {
            settings: "Pengaturan widget",
            resize: "Ubah ukuran",
            move_widget: "Pindahkan",
            size: "Ukuran",
            remove: "Hapus",
            small: "Kecil",
            default: "Ukuran default",
            large: "Besar",
            extra_large: "Sangat besar",
            widget: "Widget",
        },
        UiLanguage::Turkish => MenuText {
            settings: "Pencere öğesi ayarları",
            resize: "Yeniden boyutlandır",
            move_widget: "Taşı",
            size: "Boyut",
            remove: "Kaldır",
            small: "Küçük",
            default: "Varsayılan boyut",
            large: "Büyük",
            extra_large: "Çok büyük",
            widget: "Pencere öğesi",
        },
    }
}

/// The label of a size in the menu, with the size itself, such as "Large (270 × 165)".
pub fn size_label(text: &MenuText, choice: &SizeChoice) -> String {
    let name = match choice.name {
        SizeName::Small => text.small,
        SizeName::Default => text.default,
        SizeName::Large => text.large,
        SizeName::ExtraLarge => text.extra_large,
    };
    format!("{name} ({} × {})", choice.width_dp, choice.height_dp)
}

/// What assistive technology calls the menu of a widget: the widget's own name, or "Widget".
pub fn menu_name(text: &MenuText, provider: &WidgetProvider) -> String {
    if provider.label.trim().is_empty() {
        text.widget.to_string()
    } else {
        provider.label.clone()
    }
}

/// The menu's keyboard items, with their commands: Move, and Size when the widget can be
/// resized at all.
pub fn keyboard_items(text: &MenuText, provider: &WidgetProvider) -> Vec<(u32, &'static str)> {
    let mut out = vec![(MOVE, text.move_widget)];
    if super::keyboard::KeyboardEdit::can_size(provider) {
        out.push((SIZE, text.size));
    }
    out
}

/// One entry of a widget's menu, as a host's own menu shows it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entry {
    /// An item the user chooses, `command` naming it for [`choice_of`]. A size is `checked`
    /// when the widget is at it now.
    Item {
        command: u32,
        label: String,
        checked: bool,
    },
    /// Resize, which opens the sizes.
    Sizes {
        label: String,
        sizes: Vec<Entry>,
    },
    Separator,
}

/// The entries of a widget's menu, in order: Widget settings when the widget can be set up
/// again, Resize with its sizes (the current one checked), Move, Size when the widget can be
/// resized, a separator, and Remove.
pub fn entries(text: &MenuText, provider: &WidgetProvider, placement: &Placement) -> Vec<Entry> {
    let sizes: Vec<Entry> = size_choices(provider)
        .iter()
        .zip(FIRST_SIZE..)
        .map(|(size, command)| Entry::Item {
            command,
            label: size_label(text, size),
            checked: is_current(size, placement),
        })
        .collect();
    let mut out = Vec::with_capacity(6);
    if provider.reconfigurable {
        out.push(Entry::Item {
            command: SETTINGS,
            label: text.settings.to_string(),
            checked: false,
        });
    }
    if !sizes.is_empty() {
        out.push(Entry::Sizes {
            label: text.resize.to_string(),
            sizes,
        });
    }
    for (command, label) in keyboard_items(text, provider) {
        out.push(Entry::Item {
            command,
            label: label.to_string(),
            checked: false,
        });
    }
    out.push(Entry::Separator);
    out.push(Entry::Item {
        command: REMOVE,
        label: text.remove.to_string(),
        checked: false,
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clock() -> WidgetProvider {
        WidgetProvider {
            provider: "com.example.clock/.ClockWidget".into(),
            label: "Clock".into(),
            min_width_dp: 110,
            min_height_dp: 40,
            default_width_dp: 180,
            default_height_dp: 110,
            resize_mode: 3,
            ..WidgetProvider::default()
        }
    }

    fn at(width_dp: u32, height_dp: u32) -> Placement {
        Placement {
            x: 0,
            y: 0,
            width_dp,
            height_dp,
            resized: false,
        }
    }

    #[test]
    fn fr12_right_clicking_a_widget_offers_resize_and_remove() {
        let text = menu_text(UiLanguage::English);
        let menu = entries(&text, &clock(), &at(180, 110));
        assert!(matches!(&menu[0], Entry::Sizes { label, .. } if label == "Resize"));
        assert_eq!(menu[3], Entry::Separator);
        assert_eq!(
            menu[4],
            Entry::Item {
                command: REMOVE,
                label: "Remove".into(),
                checked: false
            }
        );
        let Entry::Sizes { sizes, .. } = &menu[0] else {
            unreachable!()
        };
        // The current size is checked, and each size chooses itself.
        let checked: Vec<&Entry> = sizes
            .iter()
            .filter(|entry| matches!(entry, Entry::Item { checked: true, .. }))
            .collect();
        assert_eq!(checked.len(), 1);
        let Entry::Item { command, label, .. } = checked[0] else {
            unreachable!()
        };
        assert_eq!(label, "Default size (180 × 110)");
        let choices = size_choices(&clock());
        assert_eq!(
            choice_of(*command, &choices),
            Some(MenuChoice::Resize {
                width_dp: 180,
                height_dp: 110
            })
        );
        assert_eq!(choice_of(REMOVE, &choices), Some(MenuChoice::Remove));
        assert_eq!(choice_of(9999, &choices), None);
    }

    #[test]
    fn fr12_the_widget_menu_offers_settings_only_for_a_widget_that_can_be_set_up_again() {
        let text = menu_text(UiLanguage::Korean);
        let plain = entries(&text, &clock(), &at(180, 110));
        assert!(!plain.iter().any(|entry| matches!(
            entry,
            Entry::Item {
                command: SETTINGS,
                ..
            }
        )));
        let settable = WidgetProvider {
            reconfigurable: true,
            ..clock()
        };
        let menu = entries(&text, &settable, &at(180, 110));
        assert_eq!(
            menu[0],
            Entry::Item {
                command: SETTINGS,
                label: "위젯 설정".into(),
                checked: false
            }
        );
        assert_eq!(
            choice_of(SETTINGS, &size_choices(&settable)),
            Some(MenuChoice::Settings)
        );
    }

    /// The menu has Move and Size, which move and resize the widget with the arrow keys, after
    /// Resize and before Remove, in English and in Korean. A widget that cannot be resized has
    /// Move only.
    #[test]
    fn fr18_widget_menu_lists_move_and_size_in_both_languages() {
        for (lang, words) in [
            (UiLanguage::English, ["Move", "Size"]),
            (UiLanguage::Korean, ["이동", "크기 조정"]),
        ] {
            let text = menu_text(lang);
            let menu = entries(&text, &clock(), &at(180, 110));
            let items: Vec<(u32, &str)> = menu
                .iter()
                .filter_map(|entry| match entry {
                    Entry::Item { command, label, .. } => Some((*command, label.as_str())),
                    _ => None,
                })
                .collect();
            assert_eq!(
                items,
                [(MOVE, words[0]), (SIZE, words[1]), (REMOVE, text.remove)]
            );
            assert!(matches!(menu[0], Entry::Sizes { .. }));
            // Move and Size are not the Resize submenu's own label.
            assert_ne!(text.size, text.resize);
            let fixed = WidgetProvider {
                resize_mode: 0,
                ..clock()
            };
            let menu = entries(&text, &fixed, &at(180, 110));
            assert!(menu
                .iter()
                .any(|e| matches!(e, Entry::Item { command: MOVE, .. })));
            assert!(!menu
                .iter()
                .any(|e| matches!(e, Entry::Item { command: SIZE, .. })));
        }
        let sizes = size_choices(&clock());
        assert_eq!(choice_of(MOVE, &sizes), Some(MenuChoice::Move));
        assert_eq!(choice_of(SIZE, &sizes), Some(MenuChoice::Size));
    }

    #[test]
    fn fr12_a_widget_that_cannot_be_resized_offers_one_size() {
        let fixed = WidgetProvider {
            resize_mode: 0,
            ..clock()
        };
        assert_eq!(size_choices(&fixed).len(), 1);
        assert_eq!(
            menu_name(&menu_text(UiLanguage::English), &WidgetProvider::default()),
            "Widget"
        );
        assert_eq!(
            menu_name(&menu_text(UiLanguage::English), &clock()),
            "Clock"
        );
    }

    fn all_words(text: &MenuText) -> [&'static str; 10] {
        [
            text.settings,
            text.resize,
            text.move_widget,
            text.size,
            text.remove,
            text.small,
            text.default,
            text.large,
            text.extra_large,
            text.widget,
        ]
    }

    /// Every interface language has every word of the widget menu, none empty and none with an
    /// em dash, and Size and Resize stay two different words.
    #[test]
    fn fr17_the_widget_menu_has_every_word_in_every_language() {
        for lang in UiLanguage::ALL {
            let text = menu_text(lang);
            for word in all_words(&text) {
                assert!(!word.trim().is_empty(), "{lang:?}");
                assert!(!word.contains('\u{2014}'), "{lang:?}: {word}");
            }
            assert_ne!(text.size, text.resize, "{lang:?}");
            let label = size_label(
                &text,
                &SizeChoice {
                    name: SizeName::Large,
                    width_dp: 270,
                    height_dp: 165,
                },
            );
            assert!(label.ends_with("(270 × 165)"), "{lang:?}: {label}");
        }
        assert_eq!(
            menu_text(UiLanguage::from_tag_or_english("nl-NL")),
            menu_text(UiLanguage::English)
        );
        assert_eq!(menu_text(UiLanguage::German).remove, "Entfernen");
    }
}
