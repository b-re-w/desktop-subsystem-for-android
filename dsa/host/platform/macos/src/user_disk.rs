//! The user's Android disk on macOS: a raw disk image in their own folder.
//!
//! The rules (a new disk is a template copy, a newer image's Android partitions are copied in
//! while the user's data stays, a disk Android never wrote to is replaced) are the host core's
//! ([`dsa_host_core::disk`]); this runs them on raw files ([`RawDiskBackend`]), whose copies
//! APFS clones without copying their blocks.

use std::path::{Path, PathBuf};

use dsa_host_core::disk::DiskBackend;
use dsa_host_unix::raw_disk::RawDiskBackend;

/// The user's disks on this Mac, as the host core's disk algorithm makes them: raw files
/// ([`RawDiskBackend`]), with the image's VHDX disks converted into raw ones first
/// ([`raw_disk`]).
pub struct MacDisks;

impl DiskBackend for MacDisks {
    type Device = <RawDiskBackend as DiskBackend>::Device;

    fn open(&self, disk: &Path, write: bool) -> Result<Self::Device, String> {
        RawDiskBackend.open(disk, write)
    }

    fn create(&self, disk: &Path, template: Option<&Path>, size_gb: u32) -> Result<(), String> {
        RawDiskBackend.create(disk, template, size_gb)
    }

    fn write_beside(&self, path: &Path, bytes: &[u8]) -> Result<(), String> {
        RawDiskBackend.write_beside(path, bytes)
    }

    fn prepare_image_disk(&self, image_disk: &Path, cache: &Path) -> Result<PathBuf, String> {
        raw_disk(image_disk, cache)
    }
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
