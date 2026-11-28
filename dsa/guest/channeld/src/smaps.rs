//! What a process's file mappings hold: its share of each mapped file's resident pages, from
//! `/proc/<pid>/smaps`, and whether those pages are the page cache, shared memory, or the
//! system disk the guest maps in place (DAX).
//!
//! The guest maps its system disk in place: the files of a mount with `dax` or `dax=always`
//! are read straight from persistent memory the host backs with the disk image, and those
//! pages are the host's file cache, not memory the VM holds. A process's PSS counts them all
//! the same, so `system_server` read as 145 MiB of which about 80 was code on the system disk.
//! Each mapping's line in smaps names its file's device, which `/proc/self/mountinfo` ties to a
//! mount and its options.
//!
//! Summed over every process, the PSS of one file's mappings is the number of its pages that
//! are mapped anywhere (PSS splits each page among the processes that map it), so the same
//! lines say which files the mapped part of the page cache holds.
//!
//! A mapping's PSS also counts the pages a process copied on write (a library's relocated
//! data), which are anonymous memory; the file's share is the PSS less the mapping's
//! `Anonymous` line. That is exact for pages only this process holds and close otherwise.

use std::collections::HashMap;

/// What a mapped file's resident pages are.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backing {
    /// A file on a mount the guest reads in place (`dax`, `dax=always`): the host's pages.
    Dax,
    /// Shared memory (tmpfs, a memfd, ashmem): counted in `Shmem`, not the page cache proper.
    Shmem,
    /// The page cache: guest memory the kernel can drop when clean.
    PageCache,
    /// Anything else a mapping can name (a dma-buf, an anonymous inode): neither.
    Other,
}

/// The mounts of one `/proc/<pid>/mountinfo`, by device.
#[derive(Debug, Clone, Default)]
pub struct Mounts {
    devices: Vec<((u32, u32), Backing)>,
}

/// The mounts of a `/proc/<pid>/mountinfo`: `id parent major:minor root mountpoint options
/// [optional fields] - fstype source super-options`. A line that does not parse is skipped.
pub fn mounts(mountinfo: &str) -> Mounts {
    let mut devices = Vec::new();
    for line in mountinfo.lines() {
        let Some((head, tail)) = line.split_once(" - ") else {
            continue;
        };
        let head: Vec<&str> = head.split_whitespace().collect();
        let tail: Vec<&str> = tail.split_whitespace().collect();
        let (Some(dev), Some(options), Some(fstype)) = (head.get(2), head.get(5), tail.first())
        else {
            continue;
        };
        let Some((major, minor)) = dev.split_once(':') else {
            continue;
        };
        let (Ok(major), Ok(minor)) = (major.parse::<u32>(), minor.parse::<u32>()) else {
            continue;
        };
        let super_options = tail.get(2).copied().unwrap_or("");
        let dax = options
            .split(',')
            .chain(super_options.split(','))
            .any(|o| o == "dax" || o == "dax=always");
        let backing = if dax {
            Backing::Dax
        } else if matches!(*fstype, "tmpfs" | "ramfs") {
            Backing::Shmem
        } else {
            Backing::PageCache
        };
        devices.push(((major, minor), backing));
    }
    Mounts { devices }
}

impl Mounts {
    /// What the pages of a file on device `dev` at `path` are. A device no mount names is a
    /// memfd or ashmem (shared memory) when its path says so, and otherwise not a file of the
    /// page cache at all.
    pub fn backing(&self, dev: (u32, u32), path: &str) -> Backing {
        if let Some((_, backing)) = self.devices.iter().find(|(d, _)| *d == dev) {
            return *backing;
        }
        if path.starts_with("/memfd:") || path.starts_with("/dev/ashmem") {
            Backing::Shmem
        } else {
            Backing::Other
        }
    }
}

/// One file mapping of a process, as its smaps entry gives it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mapping {
    /// The file's device (major, minor) and inode.
    pub dev: (u32, u32),
    pub inode: u64,
    pub path: String,
    /// Its PSS, and the anonymous pages (copies made on write) among it, in KiB.
    pub pss_kib: u64,
    pub anonymous_kib: u64,
}

impl Mapping {
    /// The file's own share: the PSS less the pages copied on write.
    pub fn file_pss_kib(&self) -> u64 {
        self.pss_kib.saturating_sub(self.anonymous_kib)
    }
}

/// The text after the first `n` whitespace-separated fields of `line`, trimmed.
fn after_fields(line: &str, n: usize) -> &str {
    let mut rest = line;
    for _ in 0..n {
        rest = rest.trim_start();
        match rest.find(char::is_whitespace) {
            Some(end) => rest = &rest[end..],
            None => return "",
        }
    }
    rest.trim()
}

/// Whether `line` starts a mapping (`start-end perms offset dev inode [path]`), and if so its
/// device and inode.
fn mapping_header(line: &str) -> Option<((u32, u32), u64)> {
    let mut fields = line.split_whitespace();
    let range = fields.next()?;
    let (start, end) = range.split_once('-')?;
    u64::from_str_radix(start, 16).ok()?;
    u64::from_str_radix(end, 16).ok()?;
    let perms = fields.next()?;
    if perms.len() != 4 {
        return None;
    }
    fields.next()?;
    let (major, minor) = fields.next()?.split_once(':')?;
    let dev = (
        u32::from_str_radix(major, 16).ok()?,
        u32::from_str_radix(minor, 16).ok()?,
    );
    let inode = fields.next()?.parse().ok()?;
    Some((dev, inode))
}

/// The value in KiB of a `Key:   123 kB` line whose key is exactly `key`.
fn kib_field(line: &str, key: &str) -> Option<u64> {
    line.strip_prefix(key)?
        .strip_prefix(':')?
        .split_whitespace()
        .next()?
        .parse()
        .ok()
}

/// The file mappings of one `/proc/<pid>/smaps` (inode not 0, a path that starts with `/`), in
/// its order. Anonymous memory and the kernel's named areas (`[heap]`, `[anon:...]`) are left
/// out.
pub fn mappings(smaps: &str) -> Vec<Mapping> {
    let mut list: Vec<Mapping> = Vec::new();
    // Whether the lines that follow belong to a file mapping being kept.
    let mut keeping = false;
    for line in smaps.lines() {
        if let Some((dev, inode)) = mapping_header(line) {
            let path = after_fields(line, 5);
            keeping = inode != 0 && path.starts_with('/');
            if keeping {
                list.push(Mapping {
                    dev,
                    inode,
                    path: path.to_string(),
                    pss_kib: 0,
                    anonymous_kib: 0,
                });
            }
            continue;
        }
        if !keeping {
            continue;
        }
        let Some(current) = list.last_mut() else {
            continue;
        };
        if let Some(kib) = kib_field(line, "Pss") {
            current.pss_kib = kib;
        } else if let Some(kib) = kib_field(line, "Anonymous") {
            current.anonymous_kib = kib;
        }
    }
    list
}

/// One process's file mappings, by what their pages are, in KiB of PSS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ProcessShares {
    /// Files on the disks the guest maps in place: the host's pages.
    pub dax_kib: u64,
    pub shmem_kib: u64,
    pub page_cache_kib: u64,
}

/// What one process's file mappings hold.
pub fn process_shares(mappings: &[Mapping], mounts: &Mounts) -> ProcessShares {
    let mut shares = ProcessShares::default();
    for m in mappings {
        let kib = m.file_pss_kib();
        match mounts.backing(m.dev, &m.path) {
            Backing::Dax => shares.dax_kib += kib,
            Backing::Shmem => shares.shmem_kib += kib,
            Backing::PageCache => shares.page_cache_kib += kib,
            Backing::Other => {}
        }
    }
    shares
}

/// One file and the KiB of its pages the processes map, summed over every process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedFile {
    pub path: String,
    pub kib: u64,
}

/// The files the processes map, largest first: those whose pages are the page cache, then
/// those on a disk the guest maps in place. Each file counts once by device and inode, named
/// by the first path seen for it.
pub fn cached_files<'a>(
    processes: impl IntoIterator<Item = &'a [Mapping]>,
    mounts: &Mounts,
) -> (Vec<CachedFile>, Vec<CachedFile>) {
    type ByFile = HashMap<((u32, u32), u64), CachedFile>;
    let mut cached: ByFile = HashMap::new();
    let mut dax: ByFile = HashMap::new();
    for mappings in processes {
        for m in mappings {
            let files = match mounts.backing(m.dev, &m.path) {
                Backing::PageCache => &mut cached,
                Backing::Dax => &mut dax,
                Backing::Shmem | Backing::Other => continue,
            };
            files
                .entry((m.dev, m.inode))
                .or_insert_with(|| CachedFile {
                    path: m.path.clone(),
                    kib: 0,
                })
                .kib += m.file_pss_kib();
        }
    }
    let sorted = |files: ByFile| {
        let mut files: Vec<CachedFile> = files.into_values().collect();
        files.sort_by(|a, b| b.kib.cmp(&a.kib).then_with(|| a.path.cmp(&b.path)));
        files
    };
    (sorted(cached), sorted(dax))
}

/// Reads one process's file mappings; `None` when its smaps cannot be read (it ended, or the
/// caller may not read it).
#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn read_mappings(pid: u32) -> Option<Vec<Mapping>> {
    std::fs::read_to_string(format!("/proc/{pid}/smaps"))
        .ok()
        .map(|smaps| mappings(&smaps))
}

/// Reads this process's view of the mounts.
#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn read_mounts() -> Mounts {
    std::fs::read_to_string("/proc/self/mountinfo")
        .map(|text| mounts(&text))
        .unwrap_or_default()
}
