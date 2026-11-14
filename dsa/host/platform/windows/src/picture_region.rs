//! The memory the Android VM shares with the window manager for decoded video pictures
//! (`dsa_host_core::video_slots`).
//!
//! The service makes a named section backed by the page file before it creates the VM, writes
//! the region's header into it, and names it in the VM's HCS document as a shared memory region
//! (`VirtualMachine.Devices.SharedMemory`), which HCS maps into the VM's physical address space
//! where the guest cannot write it. Once the VM runs, HCS says where (`SharedMemoryRegion`
//! properties), and the service tells the guest's control daemon, whose kernel then maps it for
//! the codec HAL. The window manager, in the user's session, opens the same section by its name
//! when a video plays and writes each decoded picture into a slot of it, so the picture reaches
//! the guest without a copy through the video port.
//!
//! The section admits SYSTEM, the VM worker processes (every VM's account is in
//! `NT VIRTUAL MACHINE\Virtual Machines`, S-1-5-83-0) and the user the VM belongs to, whose
//! window manager writes it. Its name carries that user's SID, so each user's VM has its own.
//!
//! A section commits its whole size against the system's commit limit when it is made (page
//! file, not memory); a page takes memory only once a picture is written to it. The window
//! manager discards what it touched once the last video closes, and lets the view go; the
//! section goes with the VM. Whether the VM worker keeps the pages it mapped after the window
//! manager discards them is measured on a real PC.
//!
//! What this module decides (the name, the access list, the document's entry, reading the
//! address HCS reports) is tested on every platform; making and mapping the section is
//! Windows only.

use serde_json::{json, Value};

/// Who may open the section: SYSTEM, the VM worker processes, and `user_sid`, who may map it
/// for reading and writing.
pub fn section_sddl(user_sid: &str) -> String {
    format!("D:P(A;;GA;;;SY)(A;;GA;;;S-1-5-83-0)(A;;GRGW;;;{user_sid})")
}

/// The section's name for the VM of the user `user_sid`, in the global namespace: the service
/// runs in session 0 and the window manager in the user's session.
pub fn section_name(user_sid: &str) -> String {
    format!(r"Global\DSA-VideoPictures-{user_sid}")
}

/// The region a VM's document names: the section and its size.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PictureRegion {
    pub section_name: String,
    pub length: u64,
}

/// The region of `mb` MiB for the VM of `user_sid`, or `None` when the user's settings turned it
/// off (0) or the VM has no user to name it after. `None` for `mb` gives the default size.
pub fn picture_region(user_sid: Option<&str>, mb: Option<u64>) -> Option<PictureRegion> {
    let length = match mb {
        Some(0) => return None,
        Some(mb) => mb.checked_mul(1024 * 1024)?,
        None => dsa_host_core::video_slots::DEFAULT_REGION_BYTES,
    };
    if length < dsa_host_core::video_slots::MIN_REGION_BYTES {
        return None;
    }
    // Whole 2 MiB, the granularity HCS maps memory in.
    let length = length.div_ceil(2 << 20) * (2 << 20);
    Some(PictureRegion {
        section_name: section_name(user_sid?),
        length,
    })
}

impl PictureRegion {
    /// The document's `Devices.SharedMemory`: one region, the whole section, which the guest
    /// may read but not write.
    pub fn to_hcs(&self) -> Value {
        json!({
            "Regions": [{
                "SectionName": self.section_name,
                "StartOffset": 0,
                "Length": self.length,
                "AllowGuestWrite": false,
                "HiddenFromGuest": false,
            }],
        })
    }
}

/// The properties query that asks a running VM where its shared memory regions are.
pub fn properties_query() -> String {
    json!({ "PropertyTypes": ["SharedMemoryRegion"] }).to_string()
}

/// Where the region of `section_name` is in the VM, from an HCS properties document
/// (`SharedMemoryRegionInfo`), as HCS gives it.
///
/// Builds have been seen to report a value that looks like a page number rather than a byte
/// address, so the value is passed on as it is: the guest kernel takes it as a byte address or
/// as the page number of one, whichever holds the region's header.
pub fn guest_address_from_properties(properties: &str, section_name: &str) -> Option<u64> {
    let value: Value = serde_json::from_str(properties).ok()?;
    let regions = value
        .as_object()?
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case("SharedMemoryRegionInfo"))?
        .1
        .as_array()?;
    regions.iter().find_map(|region| {
        let region = region.as_object()?;
        let field = |name: &str| {
            region
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(name))
                .map(|(_, v)| v)
        };
        let name = field("SectionName")?.as_str()?;
        if !name.eq_ignore_ascii_case(section_name) {
            return None;
        }
        match field("GuestPhysicalAddress")? {
            Value::Number(n) => n.as_u64(),
            Value::String(s) => s
                .strip_prefix("0x")
                .map_or_else(|| s.parse().ok(), |hex| u64::from_str_radix(hex, 16).ok()),
            _ => None,
        }
    })
}

#[cfg(windows)]
pub use system::{PictureSection, SectionView};

#[cfg(windows)]
mod system {
    use std::io;
    use std::sync::Arc;

    use dsa_host_core::video_slots::{new_region_id, write_header, SharedRegion};
    use windows_sys::Win32::Foundation::{
        CloseHandle, GetLastError, LocalFree, ERROR_ALREADY_EXISTS, HANDLE, HLOCAL,
        INVALID_HANDLE_VALUE,
    };
    use windows_sys::Win32::Security::Authorization::{
        ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1,
    };
    use windows_sys::Win32::Security::{PSECURITY_DESCRIPTOR, SECURITY_ATTRIBUTES};
    use windows_sys::Win32::System::Memory::{
        CreateFileMappingW, DiscardVirtualMemory, MapViewOfFile, OpenFileMappingW, UnmapViewOfFile,
        VirtualQuery, FILE_MAP_READ, FILE_MAP_WRITE, MEMORY_BASIC_INFORMATION,
        MEMORY_MAPPED_VIEW_ADDRESS, PAGE_READWRITE, SEC_COMMIT,
    };

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

    /// The section the service makes for a VM, held for as long as the VM runs.
    pub struct PictureSection {
        _handle: Handle,
        pub id: u64,
        pub length: u64,
    }

    impl PictureSection {
        /// Makes the section `name` of `length` bytes, which `sddl` admits, and writes the
        /// region's header with a new id. A section of that name that is still open (a window
        /// manager kept its view of the boot before) is taken as it is, its header written
        /// again: the new boot's guest then names the new id.
        pub fn create(name: &str, sddl: &str, length: u64) -> io::Result<PictureSection> {
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
            // SAFETY: allocated by ConvertStringSecurityDescriptorToSecurityDescriptorW; the
            // section keeps its own copy.
            unsafe { LocalFree(descriptor as HLOCAL) };
            if handle.is_null() {
                return Err(error);
            }
            let handle = Handle(handle);
            let view = View::map(handle.0)?;
            if existed && (view.len as u64) < length {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!(
                        "{name} is already open with {} bytes, not {length}",
                        view.len
                    ),
                ));
            }
            let id = new_region_id();
            // SAFETY: the view is at least 24 bytes long and this process's own mapping.
            let header = unsafe { std::slice::from_raw_parts_mut(view.base, 24) };
            write_header(header, id, length);
            drop(view);
            Ok(PictureSection {
                _handle: handle,
                id,
                length,
            })
        }
    }

    /// A view of a section in this process.
    struct View {
        base: *mut u8,
        len: usize,
    }

    impl View {
        fn map(section: HANDLE) -> io::Result<View> {
            // SAFETY: a live section handle; the whole section is mapped.
            let address =
                unsafe { MapViewOfFile(section, FILE_MAP_READ | FILE_MAP_WRITE, 0, 0, 0) };
            if address.Value.is_null() {
                return Err(io::Error::last_os_error());
            }
            // SAFETY: a plain structure, for which all zeros is a valid value.
            let mut info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
            // SAFETY: a query of the view just mapped.
            let got = unsafe {
                VirtualQuery(
                    address.Value,
                    &mut info,
                    std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
                )
            };
            let view = View {
                base: address.Value.cast(),
                len: if got == 0 { 0 } else { info.RegionSize },
            };
            if view.len < 24 {
                return Err(io::Error::other(
                    "the section's view has no room for a header",
                ));
            }
            Ok(view)
        }
    }

    impl Drop for View {
        fn drop(&mut self) {
            // SAFETY: unmapped once; nothing refers to it after (the region holds the view).
            unsafe {
                UnmapViewOfFile(MEMORY_MAPPED_VIEW_ADDRESS {
                    Value: self.base.cast(),
                })
            };
        }
    }

    /// The window manager's view of the section, as the region its pictures go to.
    pub struct SectionView {
        _handle: Handle,
        view: View,
    }

    // SAFETY: the view is plain memory; who writes which bytes is the slots' business
    // (`dsa_host_core::video_slots`).
    unsafe impl Send for SectionView {}
    // SAFETY: as above.
    unsafe impl Sync for SectionView {}

    impl SectionView {
        /// Opens the section `name` the service made and maps all of it.
        pub fn open(name: &str) -> io::Result<Arc<SectionView>> {
            let name_w = wide(name);
            // SAFETY: a NUL-terminated name.
            let handle =
                unsafe { OpenFileMappingW(FILE_MAP_READ | FILE_MAP_WRITE, 0, name_w.as_ptr()) };
            if handle.is_null() {
                return Err(io::Error::last_os_error());
            }
            let handle = Handle(handle);
            let view = View::map(handle.0)?;
            Ok(Arc::new(SectionView {
                _handle: handle,
                view,
            }))
        }
    }

    impl SharedRegion for SectionView {
        fn as_ptr(&self) -> *mut u8 {
            self.view.base
        }

        fn len(&self) -> usize {
            self.view.len
        }

        /// Gives the pages back to Windows: their contents are dropped and they are taken out
        /// of this process's working set, while the section keeps its commit.
        fn discard(&self, offset: usize, len: usize) {
            let end = (offset + len).min(self.view.len);
            if offset >= end {
                return;
            }
            // SAFETY: a range of this view; nothing reads it now (no stream uses the slots).
            let code =
                unsafe { DiscardVirtualMemory(self.view.base.add(offset).cast(), end - offset) };
            if code != 0 {
                eprintln!(
                    "video: the shared picture memory was not given back ({code}); it goes with \
                     the view"
                );
            }
        }
    }
}
