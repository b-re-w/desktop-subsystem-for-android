//! The graphics memory the Android VM shares with the renderer (`dsa_host_core::graphics_memory`):
//! Vulkan host-visible memory and gralloc buffers' staging copies, which then cross no socket.
//!
//! The service makes two named sections backed by the page file before it creates the VM, and
//! names both in the VM's HCS document as shared memory regions: the data region, which the
//! guest may write, and the grants table, which it may only read. Once the VM runs, HCS says
//! where they are (`SharedMemoryRegion` properties), and the service tells the guest's control
//! daemon, whose kernel then lets each app map the grants the renderer gives it. The renderer,
//! in the user's session, opens the same sections by name, imports grants of the data region
//! into Vulkan and writes the table.
//!
//! The sections admit SYSTEM, the VM worker processes (S-1-5-83-0) and the user the VM belongs
//! to, whose renderer maps them; their names carry that user's SID, so each user's VM has its
//! own. A section commits its whole size against the system's commit limit when it is made
//! (page file, not memory); a page takes memory only once the guest or the GPU writes it, and
//! the renderer zeroes and discards the pages of every grant it takes back.
//!
//! What this module decides (the names, the document's entries) is tested on every platform;
//! making the sections is Windows only.

use crate::picture_region::hcs_section_name;
use dsa_host_core::graphics_memory;
use serde_json::{json, Value};

/// The data section's name for the VM of the user `user_sid`, in the global namespace: the
/// service runs in session 0 and the renderer in the user's session.
pub fn data_section_name(user_sid: &str) -> String {
    format!(r"Global\DSA-GraphicsMemory-{user_sid}")
}

/// The grants table's section name for the VM of the user `user_sid`.
pub fn grants_section_name(user_sid: &str) -> String {
    format!(r"Global\DSA-GraphicsGrants-{user_sid}")
}

/// The two regions a VM's document names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicsRegion {
    pub data_section: String,
    pub data_length: u64,
    pub grants_section: String,
    pub grants_length: u64,
}

/// The regions for a data region of `mb` MiB for the VM of `user_sid`, or `None` when the
/// user's settings turned it off (0), it is too small to be worth it, or the VM has no user to
/// name it after. `None` for `mb` gives the default size.
pub fn graphics_region(user_sid: Option<&str>, mb: Option<u64>) -> Option<GraphicsRegion> {
    let data_length = graphics_memory::data_bytes(mb)?;
    let user = user_sid?;
    Some(GraphicsRegion {
        data_section: data_section_name(user),
        data_length,
        grants_section: grants_section_name(user),
        grants_length: graphics_memory::grants_bytes(data_length),
    })
}

impl GraphicsRegion {
    /// The two entries of the document's `Devices.SharedMemory.Regions`: the data region,
    /// which the guest writes, and the grants table, which it must not. Each section goes by
    /// its kernel object path (`\BaseNamedObjects\...`): HCS refuses to start a VM whose
    /// document names a section by its Win32 `Global\` name.
    pub fn to_hcs_regions(&self) -> Vec<Value> {
        vec![
            json!({
                "SectionName": hcs_section_name(&self.data_section),
                "StartOffset": 0,
                "Length": self.data_length,
                "AllowGuestWrite": true,
                "HiddenFromGuest": false,
            }),
            json!({
                "SectionName": hcs_section_name(&self.grants_section),
                "StartOffset": 0,
                "Length": self.grants_length,
                "AllowGuestWrite": false,
                "HiddenFromGuest": false,
            }),
        ]
    }
}

/// The document's `Devices.SharedMemory` for the regions a VM has: the video pictures' and the
/// graphics memory's. `None` when it has neither.
pub fn shared_memory_device(
    pictures: Option<&crate::picture_region::PictureRegion>,
    graphics: Option<&GraphicsRegion>,
) -> Option<Value> {
    let mut regions = Vec::new();
    if let Some(pictures) = pictures {
        if let Some(entries) = pictures.to_hcs()["Regions"].as_array() {
            regions.extend(entries.iter().cloned());
        }
    }
    if let Some(graphics) = graphics {
        regions.extend(graphics.to_hcs_regions());
    }
    (!regions.is_empty()).then(|| json!({ "Regions": regions }))
}

#[cfg(windows)]
pub use system::{GraphicsSections, RendererGraphicsMemory};

#[cfg(windows)]
mod system {
    use std::io;
    use std::sync::Arc;

    use dsa_host_core::graphics_memory::{write_grants_header, GRANTS_MAGIC};
    use dsa_host_core::video_slots::{new_region_id, SharedRegion};
    use windows_sys::Win32::Foundation::{
        CloseHandle, GetLastError, LocalFree, ERROR_ALREADY_EXISTS, HANDLE, HLOCAL,
        INVALID_HANDLE_VALUE,
    };
    use windows_sys::Win32::Security::Authorization::{
        ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1,
    };
    use windows_sys::Win32::Security::{PSECURITY_DESCRIPTOR, SECURITY_ATTRIBUTES};
    use windows_sys::Win32::System::Memory::{
        CreateFileMappingW, DiscardVirtualMemory, MapViewOfFile, UnmapViewOfFile, FILE_MAP_READ,
        FILE_MAP_WRITE, MEMORY_MAPPED_VIEW_ADDRESS, PAGE_READWRITE, SEC_COMMIT,
    };

    use super::GraphicsRegion;
    use crate::picture_region::SectionView;

    fn wide(text: &str) -> Vec<u16> {
        text.encode_utf16().chain([0]).collect()
    }

    struct Handle(HANDLE);

    // SAFETY: a section handle may be used and closed from any thread.
    unsafe impl Send for Handle {}
    // SAFETY: as above; nothing changes it after it is made.
    unsafe impl Sync for Handle {}

    impl Drop for Handle {
        fn drop(&mut self) {
            // SAFETY: closed once.
            unsafe { CloseHandle(self.0) };
        }
    }

    /// Makes the page-file backed section `name` of `length` bytes that `sddl` admits. Also
    /// says whether a section of that name was still open (a renderer kept its view of the boot
    /// before), which is then taken as it is.
    fn create(name: &str, sddl: &str, length: u64) -> io::Result<(Handle, bool)> {
        let sddl_w = wide(sddl);
        let mut descriptor: PSECURITY_DESCRIPTOR = std::ptr::null_mut();
        // SAFETY: a NUL-terminated SDDL string; the descriptor is freed below.
        let ok = unsafe {
            ConvertStringSecurityDescriptorToSecurityDescriptorW(
                sddl_w.as_ptr(),
                SDDL_REVISION_1,
                &mut descriptor,
                std::ptr::null_mut(),
            )
        };
        if ok == 0 {
            return Err(io::Error::last_os_error());
        }
        let attributes = SECURITY_ATTRIBUTES {
            nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: descriptor,
            bInheritHandle: 0,
        };
        let name_w = wide(name);
        // SAFETY: valid attributes and a NUL-terminated name; a page file backed section.
        let handle = unsafe {
            CreateFileMappingW(
                INVALID_HANDLE_VALUE,
                &attributes,
                PAGE_READWRITE | SEC_COMMIT,
                (length >> 32) as u32,
                length as u32,
                name_w.as_ptr(),
            )
        };
        let error = io::Error::last_os_error();
        // SAFETY: read straight after the call it describes.
        let existed = unsafe { GetLastError() } == ERROR_ALREADY_EXISTS;
        // SAFETY: allocated by ConvertStringSecurityDescriptorToSecurityDescriptorW.
        unsafe { LocalFree(descriptor as HLOCAL) };
        if handle.is_null() {
            return Err(error);
        }
        Ok((Handle(handle), existed))
    }

    /// Maps `length` bytes of `section`, lets `write` have them, and unmaps them.
    fn with_view(section: &Handle, length: u64, write: impl FnOnce(&mut [u8])) -> io::Result<()> {
        // SAFETY: a live section handle at least `length` bytes long.
        let address = unsafe {
            MapViewOfFile(
                section.0,
                FILE_MAP_READ | FILE_MAP_WRITE,
                0,
                0,
                length as usize,
            )
        };
        if address.Value.is_null() {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: this process's own view of `length` bytes, unmapped below.
        let bytes =
            unsafe { std::slice::from_raw_parts_mut(address.Value.cast::<u8>(), length as usize) };
        write(bytes);
        // SAFETY: unmapped once; nothing refers to it after.
        unsafe {
            UnmapViewOfFile(MEMORY_MAPPED_VIEW_ADDRESS {
                Value: address.Value,
            })
        };
        Ok(())
    }

    /// The two sections the service makes for a VM, held for as long as the VM runs.
    pub struct GraphicsSections {
        _data: Handle,
        _grants: Handle,
        pub id: u64,
    }

    impl GraphicsSections {
        /// Makes both sections of `region`, which `sddl` admits, and writes the grants table's
        /// header with a new id. Sections a renderer still holds from the boot before are taken
        /// as they are: the table is cleared, and the data region zeroed and given back, so no
        /// grant or byte of the last boot reaches this one.
        pub fn create(region: &GraphicsRegion, sddl: &str) -> io::Result<GraphicsSections> {
            let (data, data_existed) = create(&region.data_section, sddl, region.data_length)?;
            let (grants, _) = create(&region.grants_section, sddl, region.grants_length)?;
            if data_existed {
                with_view(&data, region.data_length, |bytes| {
                    bytes.fill(0);
                    // SAFETY: this process's view, which nothing else uses through it.
                    unsafe { DiscardVirtualMemory(bytes.as_mut_ptr().cast(), bytes.len()) };
                })?;
            }
            let id = new_region_id();
            let mut written = false;
            with_view(&grants, region.grants_length, |table| {
                table.fill(0);
                written = write_grants_header(table, id, region.data_length);
            })?;
            if !written {
                return Err(io::Error::other(
                    "the graphics grants section has no room for its header",
                ));
            }
            Ok(GraphicsSections {
                _data: data,
                _grants: grants,
                id,
            })
        }
    }

    /// The renderer's views of the two sections, held while it uses them.
    pub struct RendererGraphicsMemory {
        pub data: Arc<SectionView>,
        pub grants: Arc<SectionView>,
    }

    impl RendererGraphicsMemory {
        /// Opens the sections the service made for the VM of `user_sid` and maps both whole.
        pub fn open(user_sid: &str) -> io::Result<RendererGraphicsMemory> {
            let data = SectionView::open(&super::data_section_name(user_sid))?;
            let grants = SectionView::open(&super::grants_section_name(user_sid))?;
            Ok(RendererGraphicsMemory { data, grants })
        }

        /// The id in the grants table's header, which changes with each boot; `None` when the
        /// table has no header.
        pub fn region_id(&self) -> Option<u64> {
            if self.grants.len() < 16 {
                return None;
            }
            // SAFETY: the view is at least 16 bytes long; the service wrote the header before
            // the VM started.
            let header = unsafe { std::slice::from_raw_parts(self.grants.as_ptr(), 16) };
            (header[..8] == GRANTS_MAGIC)
                .then(|| u64::from_le_bytes(header[8..16].try_into().unwrap_or_default()))
        }
    }
}
