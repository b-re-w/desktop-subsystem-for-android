//! DSA Settings for Linux works with a screen reader, as the Windows settings app does: every
//! control has a name, a control repeated in every row of a list says whose it is, the page and
//! group titles are headings, a banner or a toast is read out when it appears, a setting that
//! asks for an administrator says so to the reader, and a log line reads as a sentence. The
//! window needs a display, so its code is read as text; what it says comes from `view`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use dsa_settings_linux::i18n::fill;
use dsa_settings_linux::model::LogLine;
use dsa_settings_linux::view;
use serde_json::json;

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn ui(file: &str) -> String {
    std::fs::read_to_string(crate_dir().join("src/ui").join(file))
        .unwrap()
        .replace("\r\n", "\n")
}

/// Every file of the window, with its name.
fn ui_files() -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = std::fs::read_dir(crate_dir().join("src/ui"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .map(|p| {
            let name = p.file_name().unwrap().to_string_lossy().into_owned();
            let code = std::fs::read_to_string(&p).unwrap().replace("\r\n", "\n");
            (name, code)
        })
        .collect();
    files.sort();
    files
}

/// The translations of a `.po` file of single-line entries.
fn catalogue(path: &Path) -> BTreeMap<String, String> {
    let text = std::fs::read_to_string(path).unwrap();
    let unquote = |s: &str| serde_json::from_str::<String>(s.trim()).unwrap();
    let mut entries = BTreeMap::new();
    let mut id: Option<String> = None;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("msgid ") {
            id = Some(unquote(rest));
        } else if let Some(rest) = line.strip_prefix("msgstr ") {
            entries.insert(id.take().expect("msgstr without msgid"), unquote(rest));
        }
    }
    entries
}

fn korean() -> BTreeMap<String, String> {
    catalogue(&crate_dir().join("po/ko.po"))
}

/// The names of the variables bound to what `constructor` makes, in `code`:
/// `let progress = gtk::ProgressBar::builder()` gives `progress`.
fn bound(code: &str, constructor: &str) -> Vec<String> {
    code.lines()
        .filter(|line| line.contains(constructor))
        .filter_map(|line| {
            let after = line.split("let ").nth(1)?;
            let name = after
                .split(|c: char| c == ' ' || c == ':' || c == '=')
                .next()?;
            Some(name.trim().to_string())
        })
        .collect()
}

#[test]
fn fr18_log_line_reads_as_text() {
    let line = LogLine::parse(
        &json!({ "time": "2026-11-04 10:00:00.000", "priority": "W", "tag": "ActivityManager",
                 "pid": 12, "tid": 13, "message": "Start proc" })
        .to_string(),
    )
    .unwrap();
    assert_eq!(
        view::log_line_spoken(&line),
        "2026-11-04 10:00:00.000, Warning, ActivityManager: Start proc"
    );
    let dropped = LogLine::parse(&json!({ "priority": "W", "dropped": 3 }).to_string()).unwrap();
    assert_eq!(view::log_line_spoken(&dropped), "3 records dropped");

    // The same in Korean, from the translations the app installs.
    let ko = korean();
    let template = &ko["{0}, {1}, {2}: {3}"];
    assert_eq!(
        fill(
            template,
            &[
                "2026-11-04 10:00:00.000",
                &ko["Warning"],
                "ActivityManager",
                "Start proc"
            ]
        ),
        "2026-11-04 10:00:00.000, 경고, ActivityManager: Start proc"
    );
    assert_eq!(
        fill(&ko["{0} records dropped"], &["3"]),
        "기록 3개가 빠졌습니다"
    );

    // Each row of the log page is named with those words.
    let logs = ui("logs.rs");
    assert!(
        logs.contains("accessible_name(&item, &view::log_line_spoken(line));"),
        "the log page's rows read as their columns, not as a sentence"
    );
}

#[test]
fn fr18_repeated_item_buttons_name_their_item() {
    assert_eq!(view::item_name("Open", "Notes"), "Open, Notes");
    let ko = korean();
    assert_eq!(fill(&ko["{0}, {1}"], &[&ko["Open"], "Notes"]), "Notes 열기");

    let apps = ui("apps.rs");
    for wanted in [
        "item_button(&tr(\"Open\"), &app.label)",
        "item_button(&tr(\"Stop\"), &app.label)",
        "item_button(&tr(\"Remove\"), &widget.label)",
        "item_button(&tr(\"Add to desktop\"), &offer.label)",
    ] {
        assert!(apps.contains(wanted), "apps.rs has no {wanted}");
    }
    // The lists of apps and widgets make no button that says only what it does.
    for label in ["Open", "Stop", "Remove", "Add to desktop"] {
        let plain = format!("= button(&tr(\"{label}\"))");
        assert!(
            !apps.contains(&plain),
            "apps.rs makes a {label} button that does not say whose it is"
        );
    }
    // A button in a row of its own says the row's title too: several rows have Remove.
    let common = ui("common.rs");
    let row = &common[common.find("pub fn button_row(").unwrap()..];
    assert!(row[..row.find("\n}\n").unwrap()].contains("item_button(label, title)"));
}

#[test]
fn fr18_icon_only_buttons_have_names() {
    for (file, code) in ui_files() {
        if file == "common.rs" {
            continue;
        }
        assert!(
            !code.contains("gtk::Button::from_icon_name("),
            "{file} makes a button with only an icon; use icon_button, which names it"
        );
    }
    let common = ui("common.rs");
    let icon = &common[common.find("pub fn icon_button(").unwrap()..];
    assert!(icon[..icon.find("\n}\n").unwrap()].contains("accessible_name(&button, label);"));
    let copy = &common[common.find("pub fn copy_button(").unwrap()..];
    assert!(copy[..copy.find("\n}\n").unwrap()].contains("Property::Label(&tr(\"Copy\"))"));
}

#[test]
fn fr18_every_progress_indicator_has_a_name() {
    let mut unnamed = Vec::new();
    for (file, code) in ui_files() {
        for constructor in [
            "gtk::ProgressBar::builder()",
            "gtk::Spinner::new()",
            "gtk::ProgressBar::new()",
        ] {
            for name in bound(&code, constructor) {
                if !code.contains(&format!("accessible_name(&{name},")) {
                    unnamed.push(format!("{file}: {name}"));
                }
            }
        }
    }
    assert!(
        unnamed.is_empty(),
        "these progress indicators have no name: {unnamed:?}"
    );
}

#[test]
fn fr18_every_toggle_combo_and_search_box_has_a_name() {
    let mut unnamed = Vec::new();
    for (file, code) in ui_files() {
        for constructor in [
            "gtk::SearchEntry::builder()",
            "gtk::DropDown::",
            "gtk::Switch::",
            "gtk::CheckButton::",
        ] {
            for name in bound(&code, constructor) {
                if !code.contains(&format!("accessible_name(&{name},")) {
                    unnamed.push(format!("{file}: {name}"));
                }
            }
        }
        // Switch rows and combo rows are named by their titles, so none may be empty.
        for empty in ["Switch::new(\"\"", "Combo::new(\"\""] {
            assert!(
                !code.contains(empty),
                "{file} makes an untitled row: {empty}"
            );
        }
    }
    assert!(
        unnamed.is_empty(),
        "these controls have no name: {unnamed:?}"
    );
}

#[test]
fn fr18_page_titles_and_section_headers_have_heading_levels() {
    let common = ui("common.rs");
    assert!(common.contains("gtk::AccessibleRole::Heading"));
    let group = &common[common.find("pub fn group(").unwrap()..];
    assert!(group[..group.find("\n}\n").unwrap()].contains("mark_group_heading(&group, title);"));
    let mark = &common[common.find("fn mark_group_heading(").unwrap()..];
    assert!(mark[..mark.find("\n}\n").unwrap()].contains("Property::Level(2)"));
    let title = &common[common.find("pub fn page_title(").unwrap()..];
    assert!(title[..title.find("\n}\n").unwrap()].contains("heading_label(text, 1)"));
    assert!(ui("mod.rs").contains("header.set_title_widget(Some(&common::page_title(&title)));"));
}

#[test]
fn fr18_every_banner_and_toast_announces() {
    let common = ui("common.rs");
    assert!(common.contains("c\"gtk_accessible_announce\""));
    let toast = &common[common.find("pub fn toast(").unwrap()..];
    assert!(
        toast[..toast.find("\n    }\n").unwrap()].contains("announce(&self.window, text, false);")
    );
    let reveal = &common[common.find("pub fn reveal_banner(").unwrap()..];
    assert!(reveal[..reveal.find("\n}\n").unwrap()].contains("announce(banner, text, true);"));
    for (file, code) in ui_files() {
        if file == "common.rs" {
            continue;
        }
        assert!(
            !code.contains("set_revealed(true)"),
            "{file} opens a banner without telling a screen reader; use reveal_banner"
        );
    }
    // Results that appear in a row without moving the focus are told too.
    assert!(ui("status.rs").contains("announce(&ended_row, &text, !succeeded);"));
    assert!(ui("logs.rs").contains("announce(&status, &said, failed);"));
}

#[test]
fn fr18_shield_info_is_not_tooltip_only() {
    // Linux shows no shield: the group's description says a setting asks for an
    // administrator. A screen reader hears it with the control too.
    let system = ui("system.rs");
    let machine = &system[system.find("let machine_switch =").unwrap()..];
    assert!(machine[..machine.find("\n    };\n").unwrap()]
        .contains("needs_administrator(&switch.row, &subtitle);"));
    let status = ui("status.rs");
    assert!(
        status.matches("needs_administrator(").count() >= 4,
        "status.rs names too few settings that need an administrator"
    );
    let common = ui("common.rs");
    let needs = &common[common.find("pub fn needs_administrator(").unwrap()..];
    assert!(needs[..needs.find("\n}\n").unwrap()].contains("Property::Description("));
}

#[test]
fn fr18_new_strings_exist_in_korean_and_english() {
    let ko = korean();
    for text in [
        "{0}, {1}",
        "{0}, {1}, {2}: {3}",
        "{0} records dropped",
        "Lowest priority shown",
        "Copied",
        "This setting applies to every user of this computer, so changing it asks for an administrator's permission.",
    ] {
        let korean = ko.get(text).unwrap_or_else(|| panic!("po/ko.po has no Korean for {text:?}"));
        assert!(!korean.is_empty(), "{text:?} is empty in Korean");
        for i in 0..4 {
            let hole = format!("{{{i}}}");
            assert_eq!(text.contains(&hole), korean.contains(&hole), "{text:?} and {korean:?} differ in {hole}");
        }
    }
}
