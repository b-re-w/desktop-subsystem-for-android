//! The memory the host shares with the guest, handed to the kernel.
//!
//! The host gives the VM a region of its own memory that host and guest both see, so decoded
//! video pictures reach the codec HAL without being copied through the video port. Where the
//! region is in the VM's physical memory is known only once the VM runs, so the host says it on
//! the control channel (`GuestSharedMemory`) after the daemon's handshake. The daemon sets
//! [`PROPERTY`] to it, and init writes the value to the kernel's driver
//! (`/sys/module/dsa_shared_memory/parameters/video_pictures`, `dsa-memory.rc`), which then lets
//! the codec HAL map that range, and only that range, through `/dev/dsa_video_pictures`. Only the
//! daemon may set the property and only init may write the driver's parameter, so no app can
//! point the driver at other memory; the driver takes the first value of a boot and no other.

use dsa_protocol::control::GuestSharedMemory;

/// The name of the region that carries decoded video pictures.
pub const VIDEO_PICTURES: &str = "video_pictures";

/// The property the daemon sets to where the picture region is, for init to hand the kernel.
pub const PROPERTY: &str = "vendor.dsa.video_pictures";

/// The property's value for `region`, `0x<address>,<length>`, the form the kernel's parameter
/// takes; `None` for a region this guest has no use for (another name, or empty).
pub fn property_value(region: &GuestSharedMemory) -> Option<String> {
    if region.name != VIDEO_PICTURES || region.length == 0 || region.guest_physical_address == 0 {
        return None;
    }
    Some(format!(
        "{:#x},{}",
        region.guest_physical_address, region.length
    ))
}
