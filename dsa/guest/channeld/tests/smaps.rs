//! What a process's file mappings hold, from `/proc/<pid>/smaps` and the mounts in
//! `/proc/self/mountinfo`, against text in the kernel's own formats.

use dsa_guest_channeld::smaps::{
    cached_files, mappings, mounts, process_shares, Backing, Mapping, ProcessShares,
};

/// The system disk mapped in place (erofs with DAX), `/data` on a dm device, an APEX on a loop
/// device, a tmpfs and a FUSE mount.
const MOUNTINFO: &str = "\
20 1 253:0 / / ro,relatime master:1 - erofs /dev/block/dm-0 ro,user_xattr,acl,dax=always\n\
21 20 253:1 / /vendor ro,relatime master:2 - erofs /dev/block/dm-1 ro,user_xattr,acl,dax=always\n\
22 20 253:5 / /data rw,nosuid,nodev,noatime master:30 - ext4 /dev/block/dm-5 rw,seclabel,resgid=1065\n\
23 20 7:8 / /apex/com.android.art@1 ro,nodev,noatime - ext4 /dev/block/loop8 ro,seclabel\n\
24 20 0:22 / /dev rw,nosuid,relatime master:3 - tmpfs tmpfs rw,seclabel,mode=755\n\
25 20 0:80 / /storage/emulated rw,nosuid,nodev - fuse /dev/fuse rw,user_id=0\n\
26 20 253:7 / /odm ro,relatime - erofs /dev/block/dm-7 ro,user_xattr,dax=never\n";

/// One process's smaps: a library on the system disk, its relocated (copied on write) pages,
/// an app's code under /data, a mapped font with a space in its path, an ART file of an APEX,
/// a memfd, a dma-buf, an anonymous heap and a FUSE file.
const SMAPS: &str = "\
7f0000000000-7f0000100000 r-xp 00000000 fd:00 1201                       /system/lib64/libhwui.so\n\
Size:               1024 kB\n\
Rss:                 800 kB\n\
Pss:                 300 kB\n\
Pss_Dirty:             0 kB\n\
Anonymous:             0 kB\n\
7f0000100000-7f0000110000 rw-p 00100000 fd:00 1201                       /system/lib64/libhwui.so\n\
Rss:                  64 kB\n\
Pss:                  64 kB\n\
Anonymous:            60 kB\n\
7f0000200000-7f0000400000 r--p 00000000 fd:05 99                         /data/app/~~x/com.example-1/base.apk\n\
Rss:                 500 kB\n\
Pss:                 250 kB\n\
Anonymous:             0 kB\n\
7f0000400000-7f0000410000 r--p 00000000 fd:05 77                         /data/fonts/My Font.ttf\n\
Pss:                  40 kB\n\
Anonymous:             0 kB\n\
7f0000500000-7f0000600000 r--p 00000000 07:08 12                         /apex/com.android.art/javalib/boot.art\n\
Pss:                 120 kB\n\
Anonymous:             0 kB\n\
7f0000600000-7f0000700000 rw-s 00000000 00:01 5                          /memfd:jit-cache (deleted)\n\
Pss:                  90 kB\n\
Anonymous:             0 kB\n\
7f0000700000-7f0000800000 rw-s 00000000 00:0d 3                          /dmabuf:\n\
Pss:                 400 kB\n\
Anonymous:             0 kB\n\
7f0000800000-7f0000900000 rw-p 00000000 00:00 0                          [anon:scudo:primary]\n\
Pss:                 700 kB\n\
Anonymous:           700 kB\n\
7f0000900000-7f0000910000 r--p 00000000 00:50 31                         /storage/emulated/0/a.mp4\n\
Pss:                  16 kB\n\
Anonymous:             0 kB\n\
7f0000a00000-7f0000a10000 r--p 00000000 fd:07 3                          /odm/lib64/libx.so\n\
Pss:                   8 kB\n\
Anonymous:             0 kB\n";

#[test]
fn nfr1_mounts_tell_dax_shared_memory_and_page_cache_apart() {
    let m = mounts(MOUNTINFO);
    assert_eq!(m.backing((253, 0), "/system/lib64/libc.so"), Backing::Dax);
    assert_eq!(m.backing((253, 1), "/vendor/lib64/x.so"), Backing::Dax);
    assert_eq!(m.backing((253, 5), "/data/app/a.apk"), Backing::PageCache);
    assert_eq!(m.backing((7, 8), "/apex/x"), Backing::PageCache);
    assert_eq!(m.backing((0, 22), "/dev/ashmem"), Backing::Shmem);
    assert_eq!(
        m.backing((0, 80), "/storage/emulated/0/a"),
        Backing::PageCache
    );
    // dax=never is the page cache.
    assert_eq!(
        m.backing((253, 7), "/odm/lib64/libx.so"),
        Backing::PageCache
    );
    // Files of no mount the report can see: a memfd is shared memory, a dma-buf is neither.
    assert_eq!(
        m.backing((0, 1), "/memfd:jit-cache (deleted)"),
        Backing::Shmem
    );
    assert_eq!(m.backing((0, 13), "/dmabuf:"), Backing::Other);
}

#[test]
fn nfr1_only_file_mappings_are_kept_with_their_device_and_inode() {
    let list = mappings(SMAPS);
    let paths: Vec<&str> = list.iter().map(|m| m.path.as_str()).collect();
    assert_eq!(
        paths,
        [
            "/system/lib64/libhwui.so",
            "/system/lib64/libhwui.so",
            "/data/app/~~x/com.example-1/base.apk",
            "/data/fonts/My Font.ttf",
            "/apex/com.android.art/javalib/boot.art",
            "/memfd:jit-cache (deleted)",
            "/dmabuf:",
            "/storage/emulated/0/a.mp4",
            "/odm/lib64/libx.so",
        ]
    );
    // Device numbers in smaps are hexadecimal: fd:05 is 253:5.
    assert_eq!(
        list[2],
        Mapping {
            dev: (253, 5),
            inode: 99,
            path: "/data/app/~~x/com.example-1/base.apk".into(),
            pss_kib: 250,
            anonymous_kib: 0,
        }
    );
    // The relocated pages of a library's data are anonymous, not the file's.
    assert_eq!(list[1].file_pss_kib(), 4);
}

#[test]
fn nfr1_a_process_share_is_split_into_dax_shared_memory_and_page_cache() {
    let shares = process_shares(&mappings(SMAPS), &mounts(MOUNTINFO));
    assert_eq!(
        shares,
        ProcessShares {
            // libhwui's 300 and its 4 that are not copies.
            dax_kib: 304,
            // memfd.
            shmem_kib: 90,
            // base.apk 250, the font 40, boot.art 120, the FUSE file 16, odm's library 8.
            page_cache_kib: 434,
        }
    );
}

#[test]
fn nfr1_cached_files_sum_every_process_share_largest_first() {
    let m = mounts(MOUNTINFO);
    let first = mappings(SMAPS);
    // A second process mapping the same APK (same device and inode) and the same library.
    let second = mappings(
        "7f1000000000-7f1000200000 r--p 00000000 fd:05 99    /data/app/~~x/com.example-1/base.apk\n\
         Pss:                 250 kB\n\
         Anonymous:             0 kB\n\
         7f1000200000-7f1000300000 r-xp 00000000 fd:00 1201  /system/lib64/libhwui.so\n\
         Pss:                 300 kB\n\
         Anonymous:             0 kB\n",
    );
    let (cached, dax) = cached_files([first.as_slice(), second.as_slice()], &m);
    assert_eq!(cached[0].path, "/data/app/~~x/com.example-1/base.apk");
    assert_eq!(cached[0].kib, 500);
    assert_eq!(cached[1].path, "/apex/com.android.art/javalib/boot.art");
    assert_eq!(cached[1].kib, 120);
    assert!(cached.iter().all(|f| !f.path.starts_with("/memfd")
        && !f.path.starts_with("/dmabuf")
        && !f.path.starts_with("/system")));
    assert_eq!(dax.len(), 1);
    assert_eq!(dax[0].path, "/system/lib64/libhwui.so");
    assert_eq!(dax[0].kib, 604);
}

#[test]
fn nfr1_text_that_is_not_smaps_gives_no_mappings() {
    assert!(mappings("").is_empty());
    assert!(mappings("garbage\nPss: 12 kB\n").is_empty());
    assert_eq!(process_shares(&[], &mounts("")), ProcessShares::default());
}
