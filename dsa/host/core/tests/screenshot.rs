//! How a screenshot is named, whichever host takes it.

use std::path::Path;

use dsa_host_core::screenshot::{file_name, free_path, LocalTime};

const TIME: LocalTime = LocalTime {
    year: 2026,
    month: 11,
    day: 8,
    hour: 9,
    minute: 5,
    second: 3,
};

#[test]
fn fr22_a_screenshot_is_named_as_the_snipping_tool_names_its_own() {
    assert_eq!(
        file_name("Screenshot", &TIME, 1),
        "Screenshot 2026-11-08 090503.png"
    );
    assert_eq!(
        file_name("\u{c2a4}\u{d06c}\u{b9b0}\u{c0f7}", &TIME, 2),
        "\u{c2a4}\u{d06c}\u{b9b0}\u{c0f7} 2026-11-08 090503 (2).png"
    );
}

#[test]
fn fr22_a_name_taken_goes_to_the_next_copy() {
    let folder = Path::new("shots");
    let taken = [
        folder.join("Screenshot 2026-11-08 090503.png"),
        folder.join("Screenshot 2026-11-08 090503 (2).png"),
    ];
    assert_eq!(
        free_path(folder, "Screenshot", &TIME, |path| taken
            .iter()
            .any(|t| t == path)),
        folder.join("Screenshot 2026-11-08 090503 (3).png")
    );
    assert_eq!(free_path(folder, "Screenshot", &TIME, |_| false), taken[0]);
}
