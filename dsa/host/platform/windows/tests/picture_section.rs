//! What the service commits and clears when it makes the video pictures' section, which only
//! reserves its size: the header's pages are committed before the header is written, a section
//! left from the boot before has only its committed pages zeroed and given back, and a header
//! that cannot be committed leaves the region out of the boot. The section is a fake page
//! layer in this process's memory.

use std::io;

use dsa_host_core::video_slots::{read_header, SLOTS_START, SLOT_ALIGNMENT};
use dsa_host_windows::picture_region::{prepare_section, SectionPages};

const PAGE: usize = 4096;
const LENGTH: usize = 8 * 1024 * 1024;

/// A section's view whose pages are only reserved until committed. Touching a reserved page
/// panics, as it faults on Windows.
struct FakePages {
    bytes: Vec<u8>,
    committed: Vec<bool>,
    commits: Vec<(usize, usize)>,
    scrubbed: Vec<(usize, usize)>,
    refuse: bool,
}

impl FakePages {
    fn new(len: usize) -> FakePages {
        FakePages {
            bytes: vec![0; len],
            committed: vec![false; len / PAGE],
            commits: Vec::new(),
            scrubbed: Vec::new(),
            refuse: false,
        }
    }

    fn pages(offset: usize, len: usize) -> std::ops::Range<usize> {
        offset / PAGE..(offset + len).div_ceil(PAGE)
    }

    fn assert_committed(&self, offset: usize, len: usize) {
        for page in Self::pages(offset, len) {
            assert!(self.committed[page], "page {page} is only reserved");
        }
    }

    /// What a boot before this one left: committed pages holding its bytes.
    fn left_from_a_boot_before(&mut self, offset: usize, len: usize, value: u8) {
        for page in Self::pages(offset, len) {
            self.committed[page] = true;
        }
        self.bytes[offset..offset + len].fill(value);
    }
}

impl SectionPages for FakePages {
    fn len(&self) -> usize {
        self.bytes.len()
    }

    fn commit(&mut self, offset: usize, len: usize) -> io::Result<()> {
        if self.refuse {
            return Err(io::Error::other(
                "the paging file is too small for this operation to complete",
            ));
        }
        for page in Self::pages(offset, len) {
            self.committed[page] = true;
        }
        self.commits.push((offset, len));
        Ok(())
    }

    fn committed(&self) -> Vec<(usize, usize)> {
        let mut ranges: Vec<(usize, usize)> = Vec::new();
        for (page, &on) in self.committed.iter().enumerate() {
            if !on {
                continue;
            }
            match ranges.last_mut() {
                Some((at, len)) if *at + *len == page * PAGE => *len += PAGE,
                _ => ranges.push((page * PAGE, PAGE)),
            }
        }
        ranges
    }

    fn scrub(&mut self, offset: usize, len: usize) {
        self.assert_committed(offset, len);
        self.bytes[offset..offset + len].fill(0);
        self.scrubbed.push((offset, len));
    }

    fn bytes_mut(&mut self, offset: usize, len: usize) -> &mut [u8] {
        self.assert_committed(offset, len);
        &mut self.bytes[offset..offset + len]
    }
}

#[test]
fn pr7_the_video_section_commits_its_header_when_it_is_made() {
    let mut pages = FakePages::new(LENGTH);
    let id = prepare_section(&mut pages, false, LENGTH as u64).expect("made");
    assert_eq!(
        pages.commits,
        vec![(0, SLOTS_START)],
        "the header and no more"
    );
    assert_eq!(read_header(&pages.bytes), Some((id, LENGTH as u64)));
    assert_ne!(id, 0);
    assert_eq!(pages.committed(), vec![(0, SLOTS_START)]);
    assert!(
        pages.scrubbed.is_empty(),
        "a new section has nothing to clear"
    );
}

#[test]
fn pr7_a_video_section_left_from_a_boot_before_clears_only_its_committed_pages() {
    let mut pages = FakePages::new(LENGTH);
    pages.left_from_a_boot_before(0, SLOTS_START, 0x11);
    let slot = SLOTS_START + 3 * SLOT_ALIGNMENT;
    pages.left_from_a_boot_before(slot, 2 * SLOT_ALIGNMENT, 0x22);
    let id = prepare_section(&mut pages, true, LENGTH as u64).expect("taken over");
    assert_eq!(
        pages.scrubbed,
        vec![(0, SLOTS_START), (slot, 2 * SLOT_ALIGNMENT)],
        "every committed page and nothing that is only reserved"
    );
    assert!(
        pages.bytes[SLOTS_START..].iter().all(|&b| b == 0),
        "no picture of the boot before is left"
    );
    assert_eq!(read_header(&pages.bytes), Some((id, LENGTH as u64)));
    // Its commit stays: the pages are given back, not decommitted.
    assert_eq!(
        pages.committed(),
        vec![(0, SLOTS_START), (slot, 2 * SLOT_ALIGNMENT)]
    );
}

#[test]
fn pr7_a_video_section_whose_header_cannot_be_committed_is_left_out() {
    let mut pages = FakePages::new(LENGTH);
    pages.refuse = true;
    let why = prepare_section(&mut pages, false, LENGTH as u64).expect_err("refused");
    assert!(
        why.to_string().contains("header could not be committed"),
        "{why}"
    );
    assert!(pages.bytes.iter().all(|&b| b == 0), "nothing was written");
    // One left from a boot before that is smaller than this boot's is not taken either.
    let mut small = FakePages::new(LENGTH / 2);
    assert!(prepare_section(&mut small, true, LENGTH as u64).is_err());
}
