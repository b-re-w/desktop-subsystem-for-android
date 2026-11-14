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
//! own.
//!
//! Both sections only reserve their size (`SEC_RESERVE`). A section made committed charges its
//! whole size against the system's commit limit at once: 1 GiB of it added about 1000 MiB of
//! system commit on the owner's PC, against about 20 to 40 MiB for the same section reserved,
//! which HCS takes and maps for the VM without committing it. The service commits what both
//! sides read from the start when it makes the sections: the whole grants table, and the
//! release slots at the start of the data region. The renderer commits each grant's granules
//! before it writes the grant's key (`dsa/graphics/include/dsa_graphics_memory.h`), so the
//! guest maps only committed pages. A committed page stays committed until the section closes
//! with the VM (Windows decommits no page of a section view), so the commit is the high-water
//! mark of what grants used; the renderer zeroes and discards the pages of every grant it takes
//! back, which gives the memory back while the commit stays.
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

    use dsa_host_core::graphics_memory::{ack_bytes, write_grants_header, GRANTS_MAGIC};
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
        CreateFileMappingW, DiscardVirtualMemory, MapViewOfFile, UnmapViewOfFile, VirtualAlloc,
        VirtualQuery, FILE_MAP_READ, FILE_MAP_WRITE, MEMORY_BASIC_INFORMATION,
        MEMORY_MAPPED_VIEW_ADDRESS, MEM_COMMIT, PAGE_READWRITE, SEC_RESERVE,
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

    /// Makes the page-file backed section `name` of `length` bytes that `sddl` admits, which
    /// reserves its size and commits nothing (see the module's notes). Also says whether a
    /// section of that name was still open (a renderer kept its view of the boot before), which
    /// is then taken as it is.
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
                PAGE_READWRITE | SEC_RESERVE,
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

    /// This process's view of `len` bytes of a section, unmapped when dropped. Its pages may be
    /// only reserved, so nothing here makes a slice of it before the range is committed.
    struct View {
        base: *mut u8,
        len: usize,
    }

    impl View {
        /// Maps `length` bytes of `section`.
        fn map(section: &Handle, length: u64) -> io::Result<View> {
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
            Ok(View {
                base: address.Value.cast(),
                len: length as usize,
            })
        }

        /// Commits the pages of `len` bytes from `offset`; pages committed already cost nothing
        /// more. Fails when the system refuses (its commit limit is reached).
        fn commit(&self, offset: usize, len: usize) -> io::Result<()> {
            match offset.checked_add(len) {
                Some(end) if end <= self.len => {}
                _ => return Err(io::Error::other("a commit outside the section's view")),
            }
            // SAFETY: a range of this view; committing touches no page.
            let at = unsafe {
                VirtualAlloc(
                    self.base.add(offset).cast(),
                    len,
                    MEM_COMMIT,
                    PAGE_READWRITE,
                )
            };
            if at.is_null() {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        }

        /// Zeroes the committed pages of the view and gives them back; the pages that are only
        /// reserved are left alone, since touching one faults.
        fn scrub_committed(&self) {
            let mut offset = 0;
            while offset < self.len {
                // SAFETY: a plain structure, for which all zeros is a valid value.
                let mut info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
                // SAFETY: a query of a page of this view.
                let got = unsafe {
                    VirtualQuery(
                        self.base.add(offset).cast(),
                        &mut info,
                        std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
                    )
                };
                if got == 0 || info.RegionSize == 0 {
                    return;
                }
                let end = (offset + info.RegionSize).min(self.len);
                if info.State == MEM_COMMIT {
                    // SAFETY: committed pages of this view, which nothing reads now (the VM
                    // that used them is gone and the next one has not started).
                    unsafe {
                        std::ptr::write_bytes(self.base.add(offset), 0, end - offset);
                        DiscardVirtualMemory(self.base.add(offset).cast(), end - offset);
                    }
                }
                offset = end;
            }
        }
    }

    impl Drop for View {
        fn drop(&mut self) {
            // SAFETY: unmapped once; nothing refers to it after.
            unsafe {
                UnmapViewOfFile(MEMORY_MAPPED_VIEW_ADDRESS {
                    Value: self.base.cast(),
                })
            };
        }
    }

    /// `why` as an error that says what could not be committed.
    fn not_committed(what: &str, why: io::Error) -> io::Error {
        io::Error::new(why.kind(), format!("{what} could not be committed ({why})"))
    }

    /// The two sections the service makes for a VM, held for as long as the VM runs.
    pub struct GraphicsSections {
        _data: Handle,
        _grants: Handle,
        pub id: u64,
    }

    impl GraphicsSections {
        /// Makes both sections of `region`, which `sddl` admits, commits what the guest and the
        /// renderer read from the start (the whole grants table and the data region's release
        /// slots), and writes the table's header with a new id. Sections a renderer still holds
        /// from the boot before are taken as they are: the table is cleared, and the committed
        /// pages of the data region zeroed and given back, so no grant or byte of the last boot
        /// reaches this one. Fails, and the boot goes without the region, when the system will
        /// not commit those pages.
        pub fn create(region: &GraphicsRegion, sddl: &str) -> io::Result<GraphicsSections> {
            let (data, data_existed) = create(&region.data_section, sddl, region.data_length)?;
            let (grants, _) = create(&region.grants_section, sddl, region.grants_length)?;
            {
                let view = View::map(&data, region.data_length)?;
                view.commit(0, ack_bytes(region.data_length) as usize)
                    .map_err(|e| not_committed("the graphics memory's release slots", e))?;
                if data_existed {
                    view.scrub_committed();
                }
            }
            let id = new_region_id();
            let view = View::map(&grants, region.grants_length)?;
            view.commit(0, view.len)
                .map_err(|e| not_committed("the graphics grants table", e))?;
            // SAFETY: the whole view, committed just now; nothing else in this process uses it,
            // and the slice goes before the view does.
            let table = unsafe { std::slice::from_raw_parts_mut(view.base, view.len) };
            table.fill(0);
            let written = write_grants_header(table, id, region.data_length);
            drop(view);
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
        /// Only the grants table and the release slots are committed; the renderer commits each
        /// grant's pages in its view of the data region before it hands the grant out, and
        /// nothing here touches the rest.
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
