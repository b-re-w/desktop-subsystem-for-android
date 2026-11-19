//! The parts of the host's configuration file that are the same on every host: the update
//! settings, the default values, the rules that decide whether a file is usable, and the
//! placeholders an installer fills in.
//!
//! The file itself, with the keys only one host has (its VM's sockets, its pipe, how it finds
//! a user's profile), is read by each platform crate, which takes its defaults and its rules
//! from here so every host reads the same file the same way.

use std::fmt;
use std::path::{Path, PathBuf};

use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

use crate::update::manifest;
use crate::update::policy::InstalledImage;
use crate::update::program::ReleaseChannel;
use crate::update::store::ImageStore;

/// The VM's memory when the configuration names none and the user set no limit, in MiB: what
/// it boots with, before it grows. 2 GiB rather than 3: the guest kernel keeps 64 bytes of page
/// records for every 4 KiB page it boots with, resident for as long as the VM runs (48 MiB for
/// 3 GiB, 32 MiB for 2 GiB), while memory added later carries its own records inside it. A guest
/// with Google Play grows once or twice after boot instead.
pub const DEFAULT_MEMORY_MB: u32 = 2048;

/// Whether "automatic" sizes the VM to the computer (its memory less what the host keeps, see
/// [`automatic_vm_memory_mb`]) rather than [`DEFAULT_MEMORY_MB`]. Off for now: the guest
/// kernel keeps its page records for the VM's whole maximum resident, about 1.6% of it (some
/// 460 MiB for a 29.5 GiB VM), so a large maximum raises idle memory until that cost is
/// measured and accepted.
///
/// [`automatic_vm_memory_mb`]: crate::memory::automatic_vm_memory_mb
pub const SIZE_TO_COMPUTER: bool = false;

/// The VM's memory in MiB for a user whose limit is `limit_mb` (0 for automatic), on a
/// computer with `total_mb` of memory when that is known: their limit, never more than the
/// computer has, else [`DEFAULT_MEMORY_MB`] (or the computer's size, with
/// [`SIZE_TO_COMPUTER`]).
pub fn sized_memory_mb(limit_mb: u32, total_mb: Option<u64>) -> u32 {
    if limit_mb == 0 && !SIZE_TO_COMPUTER {
        return DEFAULT_MEMORY_MB;
    }
    match total_mb {
        Some(total_mb) => {
            let sized = crate::memory::vm_memory_mb(total_mb, limit_mb);
            u32::try_from(sized).unwrap_or(u32::MAX)
        }
        None if limit_mb > 0 => limit_mb,
        None => DEFAULT_MEMORY_MB,
    }
}

pub fn default_vm_name() -> String {
    "dsa-android".into()
}
pub fn default_processors() -> u32 {
    4
}
pub fn default_control_port() -> u32 {
    5000
}
pub fn default_log_port() -> u32 {
    5001
}
pub fn default_connect_timeout_secs() -> u64 {
    120
}
pub fn default_stop_timeout_secs() -> u64 {
    30
}
pub fn default_data_disk_size_gb() -> u32 {
    64
}
pub fn default_ready_timeout_secs() -> u64 {
    crate::session::DEFAULT_READY_TIMEOUT.as_secs()
}
pub fn default_doze_delay_secs() -> u64 {
    crate::session::DEFAULT_DOZE_DELAY.as_secs()
}
pub fn default_check_interval_hours() -> u64 {
    24
}
pub fn default_boot_timeout_secs() -> u64 {
    300
}
pub fn default_true() -> bool {
    true
}

/// Where new images are published and who may sign them.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateSettings {
    /// The signed image list of the stable channel (`images.json`, or `images-arm64.json` on
    /// an ARM64 host), which names stable images only. Its signature is at the same URL plus
    /// `.sig`. A computer on the pre-release channel reads the pre-release list published
    /// beside it (`images-prerelease.json`, `images-prerelease-arm64.json`) instead, and this
    /// one while that cannot be fetched.
    pub list_url: String,
    /// Public keys, 32 bytes in hexadecimal, any of which may sign the list.
    pub trusted_keys: Vec<String>,
    #[serde(default = "default_check_interval_hours")]
    pub check_interval_hours: u64,
    /// How long a new image may take to finish its first boot before it is undone.
    #[serde(default = "default_boot_timeout_secs")]
    pub boot_timeout_secs: u64,
    /// The signed program list of the stable channel, for updating DSA itself. Its signature
    /// is at the same URL plus `.sig`, and the same keys may sign it. Without it (and without
    /// `prerelease_program_list_url`) DSA is not updated.
    #[serde(default)]
    pub program_list_url: Option<String>,
    /// The signed program list of the pre-release channel, which a computer that takes
    /// pre-releases reads beside the stable one. A configuration written before the channels
    /// existed has none, and its `program_list_url` is then the only list.
    #[serde(default)]
    pub prerelease_program_list_url: Option<String>,
    /// The channel the installer was published on: `stable` or `prerelease`. A computer
    /// whose settings record no channel yet starts on it. Without it the version decides:
    /// below 1.0 every release is a pre-release.
    #[serde(default)]
    pub channel: Option<ReleaseChannel>,
    /// False turns updating DSA itself off; the program list is then never fetched.
    #[serde(default = "default_true")]
    pub program: bool,
    /// The service's own folder for downloaded setup files.
    #[serde(default)]
    pub program_dir: Option<PathBuf>,
}

impl UpdateSettings {
    pub fn keys(&self) -> Result<Vec<VerifyingKey>, HostConfigError> {
        self.trusted_keys
            .iter()
            .map(|key| manifest::parse_key(key).map_err(|e| HostConfigError(e.to_string())))
            .collect()
    }

    /// The channel of the installer that put version `running` here: the one the
    /// configuration names, else the one the version was published on.
    pub fn installer_channel(&self, running: &str) -> ReleaseChannel {
        self.channel
            .unwrap_or_else(|| ReleaseChannel::of_version(running))
    }

    /// Whether this configuration names any program list.
    pub fn has_program_list(&self) -> bool {
        self.program_list_url.is_some() || self.prerelease_program_list_url.is_some()
    }
}

/// Why a configuration file could not be used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostConfigError(pub String);

impl fmt::Display for HostConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the host configuration is not usable: {}", self.0)
    }
}

impl std::error::Error for HostConfigError {}

/// Checks the keys that decide where the Android image comes from and how it is updated: an
/// image or an update store, the first image in that store, and a folder for setup files
/// when DSA updates itself.
pub fn check_image_sources(
    system_image: Option<&Path>,
    images_dir: Option<&Path>,
    first_image: Option<&InstalledImage>,
    update: Option<&UpdateSettings>,
) -> Result<(), HostConfigError> {
    match (system_image, images_dir, first_image) {
        (None, None, _) => {
            return Err(HostConfigError(
                "name either system_image or images_dir (with updates)".into(),
            ))
        }
        (_, Some(_), None) => {
            return Err(HostConfigError(
                "images_dir needs first_image, the image the installer put there".into(),
            ))
        }
        _ => {}
    }
    if let Some(update) = update {
        if images_dir.is_none() {
            return Err(HostConfigError("update needs images_dir".into()));
        }
        update.keys()?;
        if update.has_program_list() && update.program_dir.is_none() {
            return Err(HostConfigError(
                "program_list_url needs program_dir, the folder setup files are \
                 downloaded to"
                    .into(),
            ));
        }
    }
    Ok(())
}

/// Replaces `{data}` and `{program}` in the configuration file's text with the folders the
/// installer put the data and the program in. The user chooses both while installing, so
/// the file cannot name them when it is built.
pub fn expand_locations(text: &str, data: &Path, program: &Path) -> String {
    let json = |path: &Path| {
        let path = path.to_string_lossy();
        serde_json::to_string(path.trim_end_matches(['\\', '/']))
            .expect("a string serialises")
            .trim_matches('"')
            .to_string()
    };
    text.replace("{data}", &json(data))
        .replace("{program}", &json(program))
}

/// The file a preset installer leaves beside the program (in the app bundle's resources on
/// macOS) to fix the computer to one Android major. A default installer leaves none.
pub const PRESET_FILE: &str = "installer-preset.json";

/// What a preset installer (such as `a16`) carries: the Android major it installs and keeps.
/// The installer is the same build as the default one; only this differs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstallerPreset {
    pub android_major: u32,
}

impl InstallerPreset {
    pub fn parse(json: &str) -> Result<InstallerPreset, HostConfigError> {
        let preset: InstallerPreset =
            serde_json::from_str(json).map_err(|e| HostConfigError(e.to_string()))?;
        if preset.android_major == 0 {
            return Err(HostConfigError(
                "an installer preset names Android major 0, which is no version".into(),
            ));
        }
        Ok(preset)
    }

    /// Reads [`PRESET_FILE`] in `dir`: `Ok(None)` when there is none (a default
    /// installation).
    pub fn read_in(dir: &Path) -> Result<Option<InstallerPreset>, HostConfigError> {
        let path = dir.join(PRESET_FILE);
        match std::fs::read_to_string(&path) {
            Ok(text) => InstallerPreset::parse(&text)
                .map(Some)
                .map_err(|e| HostConfigError(format!("{}: {}", path.display(), e.0))),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(HostConfigError(format!("reading {}: {e}", path.display()))),
        }
    }

    /// Reads the preset in `dir`, if there is one, and applies it to `store`: the line to
    /// log, or `None` for a default installation.
    pub fn apply_from(dir: &Path, store: &ImageStore) -> Option<String> {
        match InstallerPreset::read_in(dir) {
            Ok(Some(preset)) => preset.apply(store),
            Ok(None) => None,
            Err(why) => Some(why.to_string()),
        }
    }

    /// Fixes `store` to this preset's major ([`ImageStore::apply_preset`]), once. Returns the
    /// line to log: what was done, or why the version was left as it was.
    pub fn apply(&self, store: &ImageStore) -> Option<String> {
        match store.apply_preset(self.android_major) {
            Ok(None) => store
                .state()
                .ok()
                .filter(|state| state.pinned_major() == Some(self.android_major))
                .map(|_| {
                    format!(
                        "this installation is fixed to Android {}",
                        self.android_major
                    )
                }),
            Ok(Some(why)) => Some(why),
            Err(why) => Some(format!(
                "fixing this installation to Android {}: {why}",
                self.android_major
            )),
        }
    }
}
