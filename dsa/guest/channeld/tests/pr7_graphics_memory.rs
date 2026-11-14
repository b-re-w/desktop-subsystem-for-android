//! Where the graphics memory the host shares is, handed to the kernel once both of its regions
//! are known.

use dsa_guest_channeld::shared_memory::{
    property_value, GraphicsRegions, GRAPHICS_GRANTS, GRAPHICS_MEMORY, GRAPHICS_PROPERTY,
};
use dsa_protocol::control::GuestSharedMemory;

fn region(name: &str, address: u64, length: u64) -> GuestSharedMemory {
    GuestSharedMemory {
        name: name.into(),
        guest_physical_address: address,
        length,
    }
}

#[test]
fn pr7_the_kernel_gets_both_graphics_regions_in_one_value_once_both_are_known() {
    let mut regions = GraphicsRegions::default();
    let grants = region(GRAPHICS_GRANTS, 0x1_1000_0000, 2 << 20);
    let data = region(GRAPHICS_MEMORY, 0x1_1020_0000, 256 << 20);
    assert!(GraphicsRegions::is_graphics(&grants) && GraphicsRegions::is_graphics(&data));
    assert_eq!(regions.take(&data), None);
    assert_eq!(
        regions.take(&grants).as_deref(),
        Some("0x110000000,2097152,0x110200000,268435456")
    );
    // An empty region says nothing; the video pictures' are not graphics.
    let mut empty = GraphicsRegions::default();
    assert_eq!(empty.take(&region(GRAPHICS_GRANTS, 0, 2 << 20)), None);
    let pictures = region("video_pictures", 0x1_0800_0000, 64 << 20);
    assert!(!GraphicsRegions::is_graphics(&pictures));
    assert_eq!(property_value(&grants), None);
}

#[test]
fn pr7_init_hands_the_graphics_memory_to_the_kernel_and_opens_the_device_to_gpu_users() {
    assert_eq!(GRAPHICS_PROPERTY, "vendor.dsa.graphics_memory");
    let rc = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../image/dsa-memory.rc"),
    )
    .unwrap();
    assert!(rc.contains(&format!("on property:{GRAPHICS_PROPERTY}=*")));
    assert!(rc.contains(&format!(
        "write /sys/module/dsa_graphics_memory/parameters/graphics_memory ${{{GRAPHICS_PROPERTY}}}"
    )));
    assert!(rc.contains("chmod 0666 /dev/dsa_graphics_memory"));
}
