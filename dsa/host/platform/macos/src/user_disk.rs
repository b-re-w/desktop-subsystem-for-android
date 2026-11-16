//! The user's Android disk on macOS: a raw disk image in their own folder.
//!
//! The rules (a new disk is a template copy, a newer image's Android partitions are copied in
//! while the user's data stays, a disk Android never wrote to is replaced) are the host core's
//! ([`dsa_host_core::disk`]); this runs them on raw files ([`RawDiskBackend`]), whose copies
//! APFS clones without copying their blocks.

use std::path::{Path, PathBuf};

use dsa_host_unix::raw_disk::RawDiskBackend;

/// Brings the user's raw `disk` to image `version`, whose disk template (raw) is `template`.
pub fn bring_disk_to(disk: &Path, template: &Path, version: &str) -> Result<(), String> {
    dsa_host_core::disk::bring_disk_to(&RawDiskBackend, disk, template, version)
}

/// The raw image of the disk `image_disk` (a VHDX of an image, or already raw), converting
/// it into `cache` the first time. The conversion goes through a temporary file, so a cut
/// short one is never taken for the disk.
pub fn raw_disk(image_disk: &Path, cache: &Path) -> Result<PathBuf, String> {
    let is_vhdx = crate::vhdx::is_vhdx(image_disk)
        .map_err(|e| format!("reading {}: {e}", image_disk.display()))?;
    if !is_vhdx {
        return Ok(image_disk.to_path_buf());
    }
    let stem = image_disk
        .file_stem()
        .ok_or_else(|| format!("{} is not a file", image_disk.display()))?;
    let raw = cache.join(stem).with_extension("img");
    if raw.exists() {
        return Ok(raw);
    }
    std::fs::create_dir_all(cache).map_err(|e| format!("creating {}: {e}", cache.display()))?;
    let partial = raw.with_extension("img.new");
    crate::vhdx::convert_to_raw(image_disk, &partial).map_err(|e| {
        let _ = std::fs::remove_file(&partial);
        format!("converting {} to a raw disk: {e}", image_disk.display())
    })?;
    std::fs::rename(&partial, &raw).map_err(|e| format!("placing {}: {e}", raw.display()))?;
    Ok(raw)
}

/// Makes sure the raw pstore disk at `path` exists. One already there is kept with its
/// records; anything else at `path` is replaced.
pub fn ensure_pstore_disk(path: &Path) -> Result<(), String> {
    if let Some(folder) = path.parent() {
        std::fs::create_dir_all(folder)
            .map_err(|e| format!("creating {}: {e}", folder.display()))?;
    }
    dsa_host_core::disk::pstore::ensure_raw(path)
}
