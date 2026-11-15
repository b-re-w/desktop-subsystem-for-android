//! The version numbers of each release channel: DSA's own (stable `X.Y.0`, pre-release
//! `X.Y.Z` with Z above 0, development `X.Y.Z-dev.N`) and the Android images' revisions
//! (stable `-n`, pre-release `-n.m`, development `-n.m-dev.k`), and how a computer on each
//! channel reads the image lists so that a stable computer never takes a pre-release image.

use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

use dsa_host_core::update::manifest::{self, FileEntry, ImageEntry, Manifest, FORMAT};
use dsa_host_core::update::policy::{decide_on_channel, Decision, InstalledImage};
use dsa_host_core::update::program::{self, ProgramManifest, ReleaseChannel, SetupFile, Version};
use dsa_host_core::update::service::check_on_channel;
use dsa_host_core::update::store::{Fetch, ImageStore};
use dsa_host_core::update::versions::{
    channel_takes_image, development_build_of, image_order, image_revision_of, is_stable_image,
    prerelease_image_list_url, Channel, ImageRevision,
};
use ed25519_dalek::{Signer, SigningKey};
use sha2::{Digest, Sha256};

/// The AOSP build the image versions below are made from; an image's build number is it times
/// 100 plus the stable revision `n`.
const AOSP: u64 = 16373615;

fn build(n: u64) -> u64 {
    AOSP * 100 + n
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn key() -> SigningKey {
    SigningKey::from_bytes(&[9; 32])
}

fn trusted() -> Vec<ed25519_dalek::VerifyingKey> {
    vec![manifest::parse_key(&hex(key().verifying_key().as_bytes())).unwrap()]
}

#[test]
fn fr15_image_versions_order_stable_prerelease_and_development() {
    assert_eq!(
        image_revision_of("17.0.0-16373615-15"),
        Some(ImageRevision {
            revision: 15,
            prerelease: 0,
            development: None
        })
    );
    assert_eq!(
        image_revision_of("17.0.0-16373615-15.2-arm64"),
        Some(ImageRevision {
            revision: 15,
            prerelease: 2,
            development: None
        })
    );
    assert_eq!(
        image_revision_of("17.0.0-16373615-15.1-dev.7"),
        Some(ImageRevision {
            revision: 15,
            prerelease: 1,
            development: Some(7)
        })
    );
    assert_eq!(development_build_of("17.0.0-16373615-15.1-dev.7"), Some(7));
    assert_eq!(
        development_build_of("17.0.0-16373615-15.1-dev.7-arm64"),
        Some(7)
    );
    assert_eq!(development_build_of("17.0.0-16373615-15.1"), None);
    // Not a revision: neither a stable nor a pre-release number.
    assert_eq!(image_revision_of("17.0.0-16373615-15.x"), None);
    assert_eq!(image_revision_of("17.0.0-16373615-15.1-dev."), None);

    assert!(is_stable_image("17.0.0-16373615-15"));
    assert!(is_stable_image("17.0.0-16373615-15-arm64"));
    assert!(!is_stable_image("17.0.0-16373615-15.1"));
    assert!(!is_stable_image("17.0.0-16373615-15.1-dev.3"));
    // Development builds from before the channels.
    assert!(!is_stable_image("17.0.0-16373615-16-dev.3"));

    // -15 < -15.1-dev.k < -15.1 < -15.2 < -16, as the image list writes them: the build
    // number carries n alone, so it is the same for every version on top of -15.
    let order = |version: &str, n| image_order(17, build(n), version);
    let ordered = [
        order("17.0.0-16373615-15", 15),
        order("17.0.0-16373615-15.1-dev.3", 15),
        order("17.0.0-16373615-15.1-dev.4", 15),
        order("17.0.0-16373615-15.1", 15),
        order("17.0.0-16373615-15.2-dev.5", 15),
        order("17.0.0-16373615-15.2", 15),
        order("17.0.0-16373615-16-dev.6", 16),
        order("17.0.0-16373615-16", 16),
        order("17.0.0-16373615-16.1", 16),
    ];
    for pair in ordered.windows(2) {
        assert!(
            pair[0] < pair[1],
            "{:?} is not before {:?}",
            pair[0],
            pair[1]
        );
    }
}

#[test]
fn fr15_program_versions_order_development_builds_below_their_release() {
    let v = |text: &str| Version::parse(text).unwrap();
    let ordered = [
        "1.0.0",
        "1.0.1-dev.4",
        "1.0.1-dev.5",
        "1.0.1",
        "1.0.2",
        "1.1.0",
    ];
    for pair in ordered.windows(2) {
        assert!(
            v(pair[0]) < v(pair[1]),
            "{} is not before {}",
            pair[0],
            pair[1]
        );
    }
    // A pre-release stays below the next stable release.
    assert!(v("1.0.99") < v("1.1.0"));
    assert!(v("1.1.0-dev.9") < v("1.1.0"));

    assert!(v("1.0.0").fits_stable_channel());
    assert!(v("1.1.0").fits_stable_channel());
    assert!(v("2.0.0").fits_stable_channel());
    assert!(!v("1.0.1").fits_stable_channel());
    assert!(!v("1.1.0-dev.3").fits_stable_channel());
    // Versions below 1.0 were all pre-releases, numbered before the rule.
    assert!(v("0.2.3").fits_stable_channel());
}

fn image(version: &str, n: u64) -> ImageEntry {
    let contents = format!("image {version}").into_bytes();
    ImageEntry {
        version: version.into(),
        android_major: 17,
        build: build(n),
        files: vec![FileEntry {
            name: "system.vhdx".into(),
            url: format!("mem://images/{version}/system.vhdx"),
            size: contents.len() as u64,
            sha256: hex(&Sha256::digest(&contents)),
        }],
    }
}

fn list(images: Vec<ImageEntry>) -> Manifest {
    Manifest {
        format: FORMAT,
        supported_majors: vec![16, 17],
        images,
    }
}

fn installed(version: &str, n: u64) -> InstalledImage {
    InstalledImage {
        version: version.into(),
        android_major: 17,
        build: build(n),
    }
}

/// A server of signed image lists and the image files they name, which records what was
/// fetched.
#[derive(Default)]
struct Server {
    files: HashMap<String, Vec<u8>>,
    fetched: Mutex<Vec<String>>,
}

impl Server {
    fn publish(&mut self, url: &str, manifest: &Manifest) {
        for entry in &manifest.images {
            self.files.insert(
                entry.files[0].url.clone(),
                format!("image {}", entry.version).into_bytes(),
            );
        }
        let bytes = serde_json::to_vec(manifest).unwrap();
        self.files.insert(
            format!("{url}.sig"),
            hex(&key().sign(&bytes).to_bytes()).into_bytes(),
        );
        self.files.insert(url.into(), bytes);
    }

    fn fetched(&self, url: &str) -> bool {
        self.fetched.lock().unwrap().iter().any(|u| u == url)
    }
}

impl Fetch for Server {
    fn fetch(&self, url: &str, out: &mut dyn Write) -> Result<u64, String> {
        self.fetched.lock().unwrap().push(url.to_string());
        let bytes = self
            .files
            .get(url)
            .ok_or_else(|| format!("{url}: HTTP 404"))?;
        out.write_all(bytes).map_err(|e| e.to_string())?;
        Ok(bytes.len() as u64)
    }
}

const STABLE_LIST: &str = "mem://download/images/images.json";
const PRERELEASE_LIST: &str = "mem://download/images/images-prerelease.json";

/// A store whose current image is `first`, with its files in place.
fn store_on(first: &InstalledImage) -> ImageStore {
    static NEXT: AtomicU32 = AtomicU32::new(0);
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "release-versions-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&root);
    let dir = root.join("images").join(&first.version);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("system.vhdx"), b"the installed image").unwrap();
    ImageStore::open(&root, first).unwrap()
}

#[test]
fn fr15_a_stable_host_never_takes_a_prerelease_or_development_image() {
    let on_15 = installed("17.0.0-16373615-15", 15);
    // A list that names a pre-release and a development image by mistake.
    let mixed = list(vec![
        image("17.0.0-16373615-15", 15),
        image("17.0.0-16373615-15.1", 15),
        image("17.0.0-16373615-15.2-dev.4", 15),
    ]);
    assert!(!channel_takes_image(
        ReleaseChannel::Stable,
        "17.0.0-16373615-15.1"
    ));
    assert!(!channel_takes_image(
        ReleaseChannel::Stable,
        "17.0.0-16373615-15.2-dev.4"
    ));
    assert_eq!(
        decide_on_channel(
            &on_15,
            &mixed,
            &[],
            Channel::Latest,
            false,
            ReleaseChannel::Stable
        ),
        Decision::UpToDate
    );
    // A stable image newer than the pre-releases reaches it.
    let with_16 = list(vec![
        image("17.0.0-16373615-15.1", 15),
        image("17.0.0-16373615-16", 16),
    ]);
    match decide_on_channel(
        &on_15,
        &with_16,
        &[],
        Channel::Latest,
        false,
        ReleaseChannel::Stable,
    ) {
        Decision::Apply(image) => assert_eq!(image.version, "17.0.0-16373615-16"),
        other => panic!("{other:?}"),
    }

    // Through the service's check: it reads the stable list only, and even that list naming
    // -15.1 does not move it.
    let mut server = Server::default();
    server.publish(STABLE_LIST, &mixed);
    server.publish(
        PRERELEASE_LIST,
        &list(vec![image("17.0.0-16373615-15.1", 15)]),
    );
    let store = store_on(&on_15);
    let news = check_on_channel(
        &store,
        &server,
        STABLE_LIST,
        &trusted(),
        ReleaseChannel::Stable,
        &mut |_| {},
    )
    .unwrap();
    assert_eq!(news, None);
    assert_eq!(store.state().unwrap().current, on_15);
    assert!(server.fetched(STABLE_LIST));
    assert!(
        !server.fetched(PRERELEASE_LIST),
        "a stable host read the pre-release list"
    );
}

#[test]
fn fr15_the_prerelease_channel_reads_the_prerelease_image_list() {
    assert_eq!(
        prerelease_image_list_url("https://github.com/o/r/releases/download/images/images.json")
            .as_deref(),
        Some("https://github.com/o/r/releases/download/images/images-prerelease.json")
    );
    assert_eq!(
        prerelease_image_list_url(
            "https://github.com/o/r/releases/download/images/images-arm64.json"
        )
        .as_deref(),
        Some("https://github.com/o/r/releases/download/images/images-prerelease-arm64.json")
    );
    // A list of another name (a folder a development build left) has none beside it.
    assert_eq!(prerelease_image_list_url("D:\\mirror\\list.json"), None);
    assert_eq!(prerelease_image_list_url("mem://my-images.json"), None);

    let on_15 = installed("17.0.0-16373615-15", 15);
    let mut server = Server::default();
    server.publish(STABLE_LIST, &list(vec![image("17.0.0-16373615-15", 15)]));
    server.publish(
        PRERELEASE_LIST,
        &list(vec![
            image("17.0.0-16373615-15", 15),
            image("17.0.0-16373615-15.1", 15),
            // Never published; a list that names one is not followed.
            image("17.0.0-16373615-15.2-dev.4", 15),
        ]),
    );
    let store = store_on(&on_15);
    check_on_channel(
        &store,
        &server,
        STABLE_LIST,
        &trusted(),
        ReleaseChannel::Prerelease,
        &mut |_| {},
    )
    .unwrap();
    assert!(server.fetched(PRERELEASE_LIST));
    assert_eq!(
        store.state().unwrap().current.version,
        "17.0.0-16373615-15.1"
    );

    // Before any pre-release image list was published, the stable list is read.
    let mut stable_only = Server::default();
    stable_only.publish(STABLE_LIST, &list(vec![image("17.0.0-16373615-16", 16)]));
    let store = store_on(&on_15);
    check_on_channel(
        &store,
        &stable_only,
        STABLE_LIST,
        &trusted(),
        ReleaseChannel::Prerelease,
        &mut |_| {},
    )
    .unwrap();
    assert!(stable_only.fetched(PRERELEASE_LIST));
    assert_eq!(store.state().unwrap().current.version, "17.0.0-16373615-16");
}

fn program_list(version: &str) -> ProgramManifest {
    ProgramManifest {
        format: program::FORMAT,
        version: version.into(),
        setup: SetupFile {
            url: format!("https://example.invalid/DSA-v{version}-win-x64-setup.exe"),
            size: 1,
            sha256: hex(&Sha256::digest(version.as_bytes())),
        },
    }
}

#[test]
fn fr15_the_stable_program_list_names_stable_versions_only() {
    let version = |chosen: Option<(ReleaseChannel, &ProgramManifest)>| {
        chosen.map(|(channel, list)| (channel, list.version.clone()))
    };
    // A stable list naming a pre-release or a development build is not followed.
    for wrong in ["1.0.1", "1.1.0-dev.3"] {
        let stable = program_list(wrong);
        assert_eq!(
            version(program::choose(ReleaseChannel::Stable, Some(&stable), None)),
            None,
            "{wrong}"
        );
    }
    let stable = program_list("1.1.0");
    assert_eq!(
        version(program::choose(ReleaseChannel::Stable, Some(&stable), None)),
        Some((ReleaseChannel::Stable, "1.1.0".to_string()))
    );
    // The pre-release channel takes the higher of the two.
    let stable = program_list("1.0.0");
    let prerelease = program_list("1.0.2");
    assert_eq!(
        version(program::choose(
            ReleaseChannel::Prerelease,
            Some(&stable),
            Some(&prerelease)
        )),
        Some((ReleaseChannel::Prerelease, "1.0.2".to_string()))
    );
}
