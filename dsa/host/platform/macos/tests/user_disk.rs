//! The raw disks of the macOS host: an image's disk read as a raw disk, and the pstore disk.
//! The user's disk across image updates is checked in the host core and on `RawDiskBackend`.

use std::path::PathBuf;

use dsa_host_macos::user_disk::{ensure_pstore_disk, raw_disk};

fn folder(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("user-disk")
        .join(format!("{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn fr2_a_raw_disk_is_used_as_it_is() {
    let dir = folder("raw");
    let raw = dir.join("system.img");
    std::fs::write(&raw, [0u8; 4096]).unwrap();
    assert_eq!(raw_disk(&raw, &dir.join("cache")).unwrap(), raw);
}

#[test]
fn fr21_the_pstore_disk_is_made_once_and_kept() {
    let dir = folder("pstore");
    let disk = dir.join("pstore.img");
    ensure_pstore_disk(&disk).unwrap();
    let bytes = std::fs::read(&disk).unwrap();
    assert_eq!(bytes.len(), 4 << 20);
    assert_eq!(&bytes[512..520], b"EFI PART");
    // A record the kernel wrote stays.
    let mut kept = bytes.clone();
    kept[2 << 20] = 7;
    std::fs::write(&disk, &kept).unwrap();
    ensure_pstore_disk(&disk).unwrap();
    assert_eq!(std::fs::read(&disk).unwrap()[2 << 20], 7);
}
