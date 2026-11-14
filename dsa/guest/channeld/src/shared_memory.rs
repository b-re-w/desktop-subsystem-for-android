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
//!
//! The host also shares graphics memory, as two regions (a grants table and the memory apps map
//! their grants of): the daemon sets [`GRAPHICS_PROPERTY`] once it knows both
//! ([`GraphicsRegions`]), and init hands it to `/sys/module/dsa_graphics_memory/parameters/
//! graphics_memory`. Every app may open `/dev/dsa_graphics_memory`; each maps only the grants
//! whose keys the host's renderer gave it.

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

/// The names of the two regions of the graphics memory the host shares: the grants table,
/// which the guest only reads, and the data region apps map their grants of.
pub const GRAPHICS_GRANTS: &str = "graphics_grants";
pub const GRAPHICS_MEMORY: &str = "graphics_memory";

/// The property the daemon sets to where both graphics regions are, for init to hand the
/// kernel (`/sys/module/dsa_graphics_memory/parameters/graphics_memory`).
pub const GRAPHICS_PROPERTY: &str = "vendor.dsa.graphics_memory";

/// Collects the two graphics regions, which the host says one message each: the kernel takes
/// both in one value, and the first value of a boot is the one it keeps.
#[derive(Debug, Default)]
pub struct GraphicsRegions {
    grants: Option<(u64, u64)>,
    data: Option<(u64, u64)>,
}

impl GraphicsRegions {
    /// Whether `region` is one of the graphics regions.
    pub fn is_graphics(region: &GuestSharedMemory) -> bool {
        region.name == GRAPHICS_GRANTS || region.name == GRAPHICS_MEMORY
    }

    /// Takes `region`. Once both are known, the property's value:
    /// `0x<grants address>,<grants length>,0x<data address>,<data length>`.
    pub fn take(&mut self, region: &GuestSharedMemory) -> Option<String> {
        if region.length == 0 || region.guest_physical_address == 0 {
            return None;
        }
        let place = (region.guest_physical_address, region.length);
        match region.name.as_str() {
            GRAPHICS_GRANTS => self.grants = Some(place),
            GRAPHICS_MEMORY => self.data = Some(place),
            _ => return None,
        }
        let ((grants, grants_len), (data, data_len)) = (self.grants?, self.data?);
        Some(format!("{grants:#x},{grants_len},{data:#x},{data_len}"))
    }
}
