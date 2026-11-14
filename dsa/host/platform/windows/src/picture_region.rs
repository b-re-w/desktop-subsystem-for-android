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
//! The section only reserves its size (`SEC_RESERVE`), as the graphics sections do
//! (`crate::graphics_region`): a section made committed charges its whole size against the
//! system's commit limit at once, and HCS takes a reserved one and maps it for the VM without
//! committing it. The service commits the header's 64 KiB when it makes the section
//! ([`prepare_section`]), the only part anyone reads before a picture: the window manager
//! checks the header when it opens the section, and the guest kernel when it is told where the
//! region is. The window manager commits each slot's pages in its own view before it first
//! writes the slot (`dsa_host_core::video_slots`), and the guest reads a slot only once the
//! host has written it and named it in a message, so the guest never reaches a page that is
//! only reserved. A committed page stays committed until the section closes with the VM, so
//! the commit is the high-water mark of the slots the window manager wrote. A page takes
//! memory only once a picture is written to it; the window manager discards what it touched
//! once the last video closes, which gives the memory back while the commit stays, and lets
//! the view go. Whether the VM worker keeps the pages it mapped after the window manager
//! discards them is measured on a real PC.
//!
//! What this module decides (the name, the access list, the document's entry, reading the
//! address HCS reports, what is committed and cleared when the section is made) is tested on
//! every platform; making and mapping the section is Windows only.

use std::io;

use dsa_host_core::video_slots::{new_region_id, write_header, SLOTS_START};
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

/// The name the VM's document gives the section `name`. HCS opens the section by its kernel
/// object path: a document naming `Global\X`, the Win32 name the service creates it under, is
/// refused when the VM starts ("the specified path is invalid", 0x800700A1, from the Dynamic
/// Memory Controller), while `\BaseNamedObjects\X`, the same object, starts. Seen on Windows 11
/// build 26300.
pub fn hcs_section_name(name: &str) -> String {
    match name.get(..7) {
        Some(prefix) if prefix.eq_ignore_ascii_case(r"Global\") => {
            format!(r"\BaseNamedObjects\{}", &name[7..])
        }
        _ => name.to_string(),
    }
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
                "SectionName": hcs_section_name(&self.section_name),
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
        if !name.eq_ignore_ascii_case(section_name)
            && !name.eq_ignore_ascii_case(&hcs_section_name(section_name))
        {
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

/// The pages of the service's view of a section that may only reserve its size.
pub trait SectionPages {
    /// The view's length in bytes.
    fn len(&self) -> usize;
    /// Whether the view is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Commits the pages of `len` bytes from `offset`; pages committed already cost nothing
    /// more. Fails when the system refuses (its commit limit is reached).
    fn commit(&mut self, offset: usize, len: usize) -> io::Result<()>;
    /// The ranges of the view that are committed, as `(offset, len)`, in order.
    fn committed(&self) -> Vec<(usize, usize)>;
    /// Zeroes `len` committed bytes from `offset` and gives their pages back.
    fn scrub(&mut self, offset: usize, len: usize);
    /// The `len` committed bytes from `offset`, to write.
    fn bytes_mut(&mut self, offset: usize, len: usize) -> &mut [u8];
}

/// Readies the view `pages` of a section of `length` bytes for a new boot and returns the id
/// its header now names. A section that `existed` already (a window manager kept its view of
/// the boot before) has every committed page zeroed and given back first, so no picture of
/// the last boot reaches this one; its pages that are only reserved are left alone, since
/// writing one faults. Then the header's pages are committed and the header written. Fails,
/// and the boot goes without the region, when the section is smaller than `length` or the
/// system will not commit the header.
pub fn prepare_section(
    pages: &mut dyn SectionPages,
    existed: bool,
    length: u64,
) -> io::Result<u64> {
    if (pages.len() as u64) < length || pages.len() < SLOTS_START {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!(
                "the section is already open with {} bytes, not {length}",
                pages.len()
            ),
        ));
    }
    if existed {
        for (offset, len) in pages.committed() {
            pages.scrub(offset, len);
        }
    }
    pages.commit(0, SLOTS_START).map_err(|why| {
        io::Error::new(
            why.kind(),
            format!("the video pictures' region header could not be committed ({why})"),
        )
    })?;
    let id = new_region_id();
    write_header(pages.bytes_mut(0, 24), id, length);
    Ok(id)
}

#[cfg(windows)]
pub use system::{PictureSection, SectionView};

#[cfg(windows)]
mod system {
    use std::io;
    use std::sync::Arc;

    use dsa_host_core::video_slots::SharedRegion;
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
        VirtualAlloc, VirtualQuery, FILE_MAP_READ, FILE_MAP_WRITE, MEMORY_BASIC_INFORMATION,
        MEMORY_MAPPED_VIEW_ADDRESS, MEM_COMMIT, PAGE_READWRITE, SEC_RESERVE,
    };

    use super::SectionPages;

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
        /// Makes the section `name` of `length` bytes, which `sddl` admits and which only
        /// reserves its size, commits the header's pages and writes the region's header with a
        /// new id ([`super::prepare_section`]). A section of that name that is still open (a
        /// window manager kept its view of the boot before) is taken as it is: its committed
        /// pages are zeroed and given back and its header written again, so the new boot's
        /// guest names the new id. Fails, and the boot goes without the region, when the
        /// system will not commit the header.
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
                    PAGE_READWRITE | SEC_RESERVE,
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
            let mut view = View::map(handle.0)?;
            let id = super::prepare_section(&mut view, existed, length)
                .map_err(|why| io::Error::new(why.kind(), format!("{name}: {why}")))?;
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
            // The view's length: every range of the one allocation the view is. The first range
            // alone is not it for a section that only reserves its size (the video pictures'
            // and the graphics memory), whose committed first pages end that range.
            let base: *mut u8 = address.Value.cast();
            let mut len = 0usize;
            loop {
                // SAFETY: a plain structure, for which all zeros is a valid value.
                let mut info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
                // SAFETY: a query of an address; it reads nothing there.
                let got = unsafe {
                    VirtualQuery(
                        base.wrapping_add(len).cast(),
                        &mut info,
                        std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
                    )
                };
                if got == 0 || info.RegionSize == 0 || info.AllocationBase != address.Value {
                    break;
                }
                len += info.RegionSize;
            }
            let view = View { base, len };
            if view.len < 24 {
                return Err(io::Error::other(
                    "the section's view has no room for a header",
                ));
            }
            Ok(view)
        }

        /// Commits the pages of `len` bytes from `offset` of the view; pages committed already
        /// cost nothing more. Fails when the system refuses (its commit limit is reached).
        fn commit_range(&self, offset: usize, len: usize) -> io::Result<()> {
            match offset.checked_add(len) {
                Some(end) if end <= self.len => {}
                _ => return Err(io::Error::other("a commit outside the section's view")),
            }
            if len == 0 {
                return Ok(());
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
    }

    impl SectionPages for View {
        fn len(&self) -> usize {
            self.len
        }

        fn commit(&mut self, offset: usize, len: usize) -> io::Result<()> {
            self.commit_range(offset, len)
        }

        fn committed(&self) -> Vec<(usize, usize)> {
            let mut ranges = Vec::new();
            let mut offset = 0;
            while offset < self.len {
                // SAFETY: a plain structure, for which all zeros is a valid value.
                let mut info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
                // SAFETY: a query of a page of this view; it reads nothing there.
                let got = unsafe {
                    VirtualQuery(
                        self.base.add(offset).cast(),
                        &mut info,
                        std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
                    )
                };
                if got == 0 || info.RegionSize == 0 {
                    break;
                }
                let end = (offset + info.RegionSize).min(self.len);
                if info.State == MEM_COMMIT {
                    ranges.push((offset, end - offset));
                }
                offset = end;
            }
            ranges
        }

        fn scrub(&mut self, offset: usize, len: usize) {
            let end = offset.saturating_add(len).min(self.len);
            if offset >= end {
                return;
            }
            // SAFETY: committed pages of this view (`committed` said so), which nothing reads
            // now: the VM that used them is gone and the next one has not started.
            unsafe {
                std::ptr::write_bytes(self.base.add(offset), 0, end - offset);
                DiscardVirtualMemory(self.base.add(offset).cast(), end - offset);
            }
        }

        fn bytes_mut(&mut self, offset: usize, len: usize) -> &mut [u8] {
            let end = offset.saturating_add(len).min(self.len);
            let offset = offset.min(end);
            // SAFETY: inside the view, committed by the caller first (`prepare_section`
            // commits the header before it writes it), and borrowed from `self` mutably.
            unsafe { std::slice::from_raw_parts_mut(self.base.add(offset), end - offset) }
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
        /// Opens the section `name` the service made and maps all of it. Only the header is
        /// committed; each slot's pages are committed before its first picture
        /// ([`SharedRegion::commit`]), and nothing here touches the rest.
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

        /// Commits the pages in this process's view (`VirtualAlloc`, `MEM_COMMIT`): the section
        /// only reserves its size, and the commit holds for every view of it, the VM's too,
        /// until the section closes.
        fn commit(&self, offset: usize, len: usize) -> Result<(), String> {
            self.view
                .commit_range(offset, len)
                .map_err(|e| e.to_string())
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
