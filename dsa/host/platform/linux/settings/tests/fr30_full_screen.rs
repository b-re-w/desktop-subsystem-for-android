//! Where an app's full screen shows, on the Apps page of DSA Settings for Linux, without a
//! display: the choice for every app and each app's own, as read from and sent to `dsa`.

use std::sync::{Arc, Mutex};

use dsa_settings_linux::api::{Dsa, Failure, Problem};
use dsa_settings_linux::model::{Client, FullScreenPlace};
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
fn fr30_an_apps_full_screen_choice_wins_over_the_default() {
    let fake = Fake::new(&[
        (
            "app-window --json",
            json!({
                "apps": [
                    {"package": "com.example.video", "fixed_size": false, "start_size": null,
                     "fullscreen": "window", "immersive": "default"},
                    {"package": "com.example.notes", "fixed_size": true, "start_size": null},
                ],
                "defaults": {"fullscreen": "screen", "immersive": "window"},
            }),
        ),
        (
            "app-window com.example.video --fullscreen window --immersive screen --json",
            json!({"apps": []}),
        ),
    ]);
    let c = client(fake.clone());
    let list = c.app_window_list().unwrap();
    let video = list
        .apps
        .iter()
        .find(|w| w.package == "com.example.video")
        .unwrap();
    assert_eq!(video.fullscreen, FullScreenPlace::Window);
    assert_eq!(video.immersive, FullScreenPlace::Default);
    // An app that names nothing follows the default for every app.
    let notes = list
        .apps
        .iter()
        .find(|w| w.package == "com.example.notes")
        .unwrap();
    assert_eq!(notes.fullscreen, FullScreenPlace::Default);
    assert_eq!(view::full_screen_indexes(None), (0, 0));
    assert_eq!(view::full_screen_indexes(Some(video)), (2, 0));
    let places: Vec<FullScreenPlace> = view::full_screen_choices()
        .into_iter()
        .map(|(_, place)| place)
        .collect();
    assert_eq!(places, FullScreenPlace::ALL);

    c.set_app_full_screen(
        "com.example.video",
        FullScreenPlace::Window,
        FullScreenPlace::Screen,
    )
    .unwrap();
    assert_eq!(
        fake.calls().last().unwrap(),
        "app-window com.example.video --fullscreen window --immersive screen --json"
    );
}

#[test]
fn fr30_each_kind_of_full_screen_request_follows_its_setting() {
    let fake = Fake::new(&[(
        "settings set app-immersive screen --json",
        json!({"app_fullscreen": "screen", "app_immersive": "screen"}),
    )]);
    let settings = client(fake.clone())
        .set_app_immersive(FullScreenPlace::Screen)
        .unwrap();
    assert_eq!(settings.app_fullscreen, FullScreenPlace::Screen);
    assert_eq!(settings.app_immersive, FullScreenPlace::Screen);
    // The choice for every app offers the two places, and a service that names neither reads as
    // DSA's defaults.
    assert_eq!(
        view::DEFAULT_FULL_SCREEN_PLACES,
        [FullScreenPlace::Screen, FullScreenPlace::Window]
    );
    assert_eq!(view::default_full_screen_index(FullScreenPlace::Window), 1);
    let codes: Vec<&str> = FullScreenPlace::ALL.iter().map(|p| p.code()).collect();
    assert_eq!(codes, ["default", "screen", "window"]);
    assert_eq!(
        FullScreenPlace::read_or("", FullScreenPlace::Window),
        FullScreenPlace::Window
    );
}
