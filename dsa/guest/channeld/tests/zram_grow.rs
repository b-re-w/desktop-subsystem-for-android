//! The second zram device for memory the guest gains after boot.

use dsa_guest_channeld::zram_grow::{
    extra_mib, step, stored_bytes, swap_header, swapped_on, Step, ZRAM_EXTRA_PROPERTY,
};

const MIB: u64 = 1024 * 1024;

#[test]
fn nfr1_memory_added_after_boot_gets_a_second_zram_device() {
    // Booted with 2 GiB (zram0 the same), grown to 4 GiB.
    assert_eq!(extra_mib(4096 * 1024, 2048 * MIB), 2048);
    // Rounded down to 64 MiB steps.
    assert_eq!(extra_mib((2048 + 1000) * 1024, 2048 * MIB), 960);
    assert_eq!(ZRAM_EXTRA_PROPERTY, "vendor.dsa.memory.zram_extra");
}

#[test]
fn nfr1_small_changes_in_memory_ask_for_no_device() {
    assert_eq!(extra_mib(2048 * 1024, 2048 * MIB), 0);
    assert_eq!(extra_mib((2048 + 255) * 1024, 2048 * MIB), 0);
    // A guest smaller than at boot asks for none.
    assert_eq!(extra_mib(1024 * 1024, 2048 * MIB), 0);
}

#[test]
fn nfr1_the_second_device_grows_and_goes_only_when_empty() {
    assert_eq!(step(0, 0, 0), Step::Leave);
    assert_eq!(step(512, 0, 0), Step::Make(512));
    // More memory again: remade larger, even with pages in it.
    assert_eq!(step(1024, 512, 100 * MIB), Step::Make(1024));
    // Memory gone: the device stays while it holds pages, goes once empty.
    assert_eq!(step(0, 512, 1), Step::Leave);
    assert_eq!(step(0, 512, 0), Step::Remove);
    assert_eq!(step(256, 512, 0), Step::Make(256));
    assert_eq!(step(256, 512, 1), Step::Leave);
}

#[test]
fn nfr1_the_second_device_has_a_version_1_swap_header() {
    let page = swap_header(4096, 512 * MIB);
    assert_eq!(page.len(), 4096);
    assert_eq!(&page[4086..], b"SWAPSPACE2");
    assert_eq!(u32::from_ne_bytes(page[1024..1028].try_into().unwrap()), 1);
    assert_eq!(
        u32::from_ne_bytes(page[1028..1032].try_into().unwrap()),
        (512 * MIB / 4096 - 1) as u32
    );
    assert_eq!(u32::from_ne_bytes(page[1032..1036].try_into().unwrap()), 0);
}

#[test]
fn nfr1_the_helper_reads_what_zram1_holds_and_whether_it_is_on() {
    assert_eq!(
        stored_bytes("  4096000  1000000  1200000 0 0 0 0 0 0\n"),
        4096000
    );
    assert_eq!(stored_bytes(""), 0);
    let swaps = "Filename\tType\tSize\tUsed\tPriority\n\
                 /dev/block/zram0 partition 2097148 0 -2\n\
                 /dev/block/zram1 partition 524284 0 -3\n";
    assert!(swapped_on(swaps, "/dev/block/zram1"));
    assert!(!swapped_on(swaps, "/dev/block/zram2"));
}
