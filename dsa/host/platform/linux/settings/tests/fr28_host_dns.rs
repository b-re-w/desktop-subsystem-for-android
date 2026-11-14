//! The DNS row of the network page in DSA Settings for Linux, without a display: who answers
//! Android's names, as the service reports it.

use dsa_settings_linux::model::{DnsResolver, Settings};
use dsa_settings_linux::view;
use serde_json::json;

fn settings_with(network: serde_json::Value) -> Settings {
    Settings::read(
        &json!({ "render_gpu": "display", "advanced_networking": false, "network": network }),
    )
}

#[test]
fn fr28_the_network_page_says_this_computers_resolver_answers_androids_names() {
    let settings = settings_with(json!({
        "mode": "nat", "addresses": ["10.0.2.15"], "dns_servers": ["10.0.2.3"],
        "gateway": "10.0.2.2", "problem": null, "dns_resolver": "host"
    }));
    let network = settings.network.as_ref().unwrap();
    assert_eq!(network.dns_resolver, DnsResolver::Host);
    assert_eq!(
        view::network_dns_text(Some(network)),
        "Answered by this computer's resolver, so its hosts file, VPN and DNS filters apply to Android apps."
    );
}

#[test]
fn fr28_the_network_page_says_when_android_asks_the_networks_dns_servers_instead() {
    let settings = settings_with(json!({
        "mode": "nat", "addresses": ["10.0.2.15"], "dns_servers": ["10.0.2.3", "fd00::3"],
        "gateway": "10.0.2.2", "problem": null, "dns_resolver": "network"
    }));
    let network = settings.network.as_ref().unwrap();
    assert_eq!(network.dns_resolver, DnsResolver::Network);
    assert_eq!(
        view::network_dns_text(Some(network)),
        "This computer's resolver does not answer, so Android asks the network's DNS servers: \
         10.0.2.3, fd00::3"
    );
}

#[test]
fn fr28_a_service_that_does_not_say_who_answers_names_shows_the_dns_servers() {
    let settings = settings_with(json!({
        "mode": "nat", "addresses": ["10.0.2.15"], "dns_servers": ["10.0.2.3"],
        "gateway": "10.0.2.2", "problem": null
    }));
    let network = settings.network.as_ref().unwrap();
    assert_eq!(network.dns_resolver, DnsResolver::Unknown);
    assert_eq!(view::network_dns_text(Some(network)), "10.0.2.3");
    assert_eq!(view::network_dns_text(None), "None");
}

#[test]
fn fr28_the_dns_texts_are_translated() {
    for language in [
        "ko", "ja", "zh_CN", "zh_TW", "es", "fr", "de", "pt_BR", "ru", "it", "vi", "id", "tr",
    ] {
        let path = format!("{}/po/{language}.po", env!("CARGO_MANIFEST_DIR"));
        let po = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{path}: {e}"))
            .replace("\r\n", "\n");
        for text in [
            "DNS",
            "Answered by this computer's resolver, so its hosts file, VPN and DNS filters apply to Android apps.",
            "This computer's resolver does not answer, so Android asks the network's DNS servers: {0}",
        ] {
            let id = format!("msgid \"{text}\"\nmsgstr \"");
            let at = po
                .find(&id)
                .unwrap_or_else(|| panic!("{language}.po lacks {text}"));
            let rest = &po[at + id.len()..];
            let translated = &rest[..rest.find("\"\n").unwrap_or(rest.len())];
            assert!(!translated.is_empty(), "{language}.po leaves {text} untranslated");
            assert_eq!(
                text.contains("{0}"),
                translated.contains("{0}"),
                "{language}.po: {translated}"
            );
        }
    }
}
