//! The "Start below the title bar" switch of each app, on the Apps page of DSA Settings for
//! Linux, without a display: as read from and sent to `dsa`.

use std::sync::{Arc, Mutex};

use dsa_settings_linux::api::{Dsa, Failure, Problem};
use dsa_settings_linux::model::Client;
use dsa_settings_linux::view;
use serde_json::json;

/// A stand-in for the service: it records each command line and gives the answer the test
/// set for it.
struct Fake {
    calls: Mutex<Vec<String>>,
    answers: Vec<(String, String)>,
}

impl Fake {
    fn new(answers: &[(&str, serde_json::Value)]) -> Arc<Fake> {
        Arc::new(Fake {
            calls: Mutex::new(Vec::new()),
            answers: answers
                .iter()
                .map(|(line, json)| (line.to_string(), json.to_string()))
                .collect(),
        })
    }

    fn calls(&self) -> Vec<String> {
        self.calls.lock().unwrap().clone()
    }
}

impl Dsa for Fake {
    fn run(&self, args: &[String]) -> Result<String, Failure> {
        let line = args.join(" ");
        self.calls.lock().unwrap().push(line.clone());
        match self.answers.iter().find(|(l, _)| *l == line) {
            Some((_, out)) => Ok(out.clone()),
            None => Err(Failure::new(
                Problem::Refused,
                format!("no answer for {line}"),
            )),
        }
    }
}

fn client(fake: Arc<Fake>) -> Client {
    Client::new(fake.clone(), fake, false)
}

#[test]
fn fr30_the_below_title_bar_switch_is_read_and_changed() {
    let fake = Fake::new(&[
        (
            "app-window --json",
            json!({
                "apps": [
                    {"package": "com.example.reader", "fixed_size": false, "start_size": null,
                     "below_title_bar": true},
                    {"package": "com.example.player", "fixed_size": true, "start_size": null,
                     "below_title_bar": false},
                    {"package": "com.example.notes", "fixed_size": true, "start_size": null},
                ],
            }),
        ),
        (
            "app-window com.example.notes --below-title-bar on --json",
            json!({"apps": [{"package": "com.example.notes", "fixed_size": true,
                             "start_size": null, "below_title_bar": true}]}),
        ),
        (
            "app-window com.example.notes --below-title-bar off --json",
            json!({"apps": [{"package": "com.example.notes", "fixed_size": true,
                             "start_size": null, "below_title_bar": false}]}),
        ),
    ]);
    let c = client(fake.clone());
    let list = c.app_window_list().unwrap();
    let of = |package: &str| list.apps.iter().find(|w| w.package == package).unwrap();
    // On, off, and an entry from a dsa that does not name the switch, which is off.
    assert!(of("com.example.reader").below_title_bar);
    assert!(!of("com.example.player").below_title_bar);
    assert!(!of("com.example.notes").below_title_bar);
    assert!(view::starts_below_title_bar(
        Some(of("com.example.reader")),
        false
    ));
    assert!(!view::starts_below_title_bar(
        Some(of("com.example.notes")),
        false
    ));
    // An app with no settings is off, and a dsa that names no built-in list has none.
    assert!(!view::starts_below_title_bar(None, false));
    assert!(list.below_title_bar_by_default.is_empty());

    // The switch is sent on its own, so the app's other window settings stay as they are.
    c.set_app_below_title_bar("com.example.notes", true)
        .unwrap();
    assert_eq!(
        fake.calls().last().unwrap(),
        "app-window com.example.notes --below-title-bar on --json"
    );
    c.set_app_below_title_bar("com.example.notes", false)
        .unwrap();
    assert_eq!(
        fake.calls().last().unwrap(),
        "app-window com.example.notes --below-title-bar off --json"
    );
}

#[test]
fn fr30_an_app_on_the_built_in_list_starts_below_the_title_bar_until_the_user_says_otherwise() {
    let fake = Fake::new(&[
        (
            "app-window --json",
            json!({
                "apps": [
                    {"package": "com.example.player", "fixed_size": false, "start_size": null,
                     "below_title_bar": false},
                ],
                "below_title_bar_by_default": ["com.example.reader", "com.example.player", ""],
            }),
        ),
        (
            "app-window com.example.reader --fixed-size on --start-size auto --dpi-scaling auto --json",
            json!({"apps": []}),
        ),
    ]);
    let c = client(fake.clone());
    let list = c.app_window_list().unwrap();
    assert_eq!(
        list.below_title_bar_by_default,
        ["com.example.reader", "com.example.player"]
    );
    let listed = |package: &str| {
        list.below_title_bar_by_default
            .iter()
            .any(|listed| listed == package)
    };
    let window = |package: &str| list.apps.iter().find(|w| w.package == package);
    // An app on the list, with no choice of the user's, shows as on; any other app as off.
    assert!(view::starts_below_title_bar(
        window("com.example.reader"),
        listed("com.example.reader")
    ));
    assert!(!view::starts_below_title_bar(
        window("com.example.notes"),
        listed("com.example.notes")
    ));
    // The user's choice wins over the list.
    assert!(!view::starts_below_title_bar(
        window("com.example.player"),
        listed("com.example.player")
    ));

    // Changing another window setting never sends the switch, so the app keeps following the list.
    c.set_app_window(
        "com.example.reader",
        true,
        None,
        dsa_settings_linux::model::DpiScaling::Auto,
    )
    .unwrap();
    assert!(!fake.calls().last().unwrap().contains("--below-title-bar"));
}
