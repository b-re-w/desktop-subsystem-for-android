//! Which image to move to.

use serde::{Deserialize, Serialize};

use super::manifest::{ImageEntry, Manifest};
use super::program::ReleaseChannel;
use super::versions::{channel_takes_image, image_order, target_major, Channel};

/// The image a user runs now.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstalledImage {
    pub version: String,
    pub android_major: u32,
    pub build: u64,
}

impl InstalledImage {
    pub fn of(entry: &ImageEntry) -> InstalledImage {
        InstalledImage {
            version: entry.version.clone(),
            android_major: entry.android_major,
            build: entry.build,
        }
    }

    /// Where the image stands among others, newest last ([`image_order`]).
    pub fn order(&self) -> (u32, u64, u64, u64) {
        image_order(self.android_major, self.build, &self.version)
    }
}

/// Where `entry` stands among other images, newest last ([`image_order`]).
fn order_of(entry: &ImageEntry) -> (u32, u64, u64, u64) {
    image_order(entry.android_major, entry.build, &entry.version)
}

/// What to do after reading the release list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// Nothing newer to move to.
    UpToDate,
    /// Move to this image.
    Apply(ImageEntry),
    /// Nothing to move to, and the installed image is no longer supported. The text says
    /// so in words a person can act on.
    Unsupported(String),
}

/// Checks that moving from `installed` to `image` is allowed. The reason names the rule.
pub fn check(
    installed: &InstalledImage,
    image: &ImageEntry,
    manifest: &Manifest,
    failed: &[String],
) -> Result<(), String> {
    check_for(installed, image, manifest, failed, false)
}

/// [`check`], with `older_allowed` set when the user agreed to erase their data to move to an
/// older major version: then an image of a lower major is allowed, whatever its build.
pub fn check_for(
    installed: &InstalledImage,
    image: &ImageEntry,
    manifest: &Manifest,
    failed: &[String],
    older_allowed: bool,
) -> Result<(), String> {
    let older = image.android_major < installed.android_major;
    if !manifest.supported_majors.contains(&image.android_major) {
        return Err(format!(
            "Android {} ({}) is not a supported version (supported: {})",
            image.android_major,
            image.version,
            majors(manifest)
        ));
    }
    if older && !older_allowed {
        return Err(format!(
            "{} is Android {}, older than the installed Android {}; Android cannot move user \
             data to an older major version",
            image.version, image.android_major, installed.android_major
        ));
    }
    if !older && order_of(image) <= installed.order() {
        return Err(format!(
            "{} is not newer than the installed {}",
            image.version, installed.version
        ));
    }
    if failed.contains(&image.version) {
        return Err(format!(
            "{} failed to boot here before, so it is not tried again",
            image.version
        ));
    }
    Ok(())
}

fn majors(manifest: &Manifest) -> String {
    manifest
        .supported_majors
        .iter()
        .map(|m| m.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Chooses the newest image `check` allows. A user on a version that lost support moves to
/// the newest supported one, the same as everyone else.
pub fn decide(installed: &InstalledImage, manifest: &Manifest, failed: &[String]) -> Decision {
    decide_for(installed, manifest, failed, Channel::Latest, false)
}

/// [`decide`] for a computer that follows `channel`. On a chosen major only images of that
/// major are taken (of the major it moves up to, once it lost support), so a newer major does
/// not replace the user's choice. `older_allowed` is set once the user agreed to erase their
/// data for an older major.
pub fn decide_for(
    installed: &InstalledImage,
    manifest: &Manifest,
    failed: &[String],
    channel: Channel,
    older_allowed: bool,
) -> Decision {
    decide_among(installed, manifest, failed, channel, older_allowed, None)
}

/// [`decide_for`] for a computer on the release channel `release`, which takes only the
/// images that channel may have ([`channel_takes_image`]): a computer on the stable channel
/// never moves to a pre-release (`-<n>.<m>`) or development (`-dev.<k>`) image, even when the
/// list it read names one. The service's periodic check decides with this.
pub fn decide_on_channel(
    installed: &InstalledImage,
    manifest: &Manifest,
    failed: &[String],
    channel: Channel,
    older_allowed: bool,
    release: ReleaseChannel,
) -> Decision {
    decide_among(
        installed,
        manifest,
        failed,
        channel,
        older_allowed,
        Some(release),
    )
}

fn decide_among(
    installed: &InstalledImage,
    manifest: &Manifest,
    failed: &[String],
    channel: Channel,
    older_allowed: bool,
    release: Option<ReleaseChannel>,
) -> Decision {
    let target = target_major(channel, &manifest.supported_majors);
    let newest = manifest
        .images
        .iter()
        .filter(|image| match release {
            Some(release) => channel_takes_image(release, &image.version),
            None => true,
        })
        .filter(|image| match channel {
            Channel::Latest => true,
            Channel::Major(_) => Some(image.android_major) == target,
        })
        .filter(|image| check_for(installed, image, manifest, failed, older_allowed).is_ok())
        .max_by_key(|image| order_of(image));
    match newest {
        Some(image) => Decision::Apply(image.clone()),
        None if !manifest.supported_majors.contains(&installed.android_major) => {
            Decision::Unsupported(format!(
                "Android {} ({}) is no longer supported (supported: {}) and no image to move \
                 to is available yet",
                installed.android_major,
                installed.version,
                majors(manifest)
            ))
        }
        None => Decision::UpToDate,
    }
}
