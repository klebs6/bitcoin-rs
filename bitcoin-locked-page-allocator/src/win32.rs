// ---------------- [ File: bitcoin-locked-page-allocator/src/win32.rs ]
crate::ix!();

/* --- Implementation: Win32LockedPageAllocator  --- */

#[cfg(windows)]
use windows_sys::Win32::{
    System::{
        Memory::{
            VirtualAlloc, VirtualFree, VirtualLock, VirtualUnlock, MEM_COMMIT, MEM_RELEASE,
            MEM_RESERVE, PAGE_READWRITE,
        },
        SystemInformation::{GetSystemInfo, SYSTEM_INFO},
    },
};

/**
  | LockedPageAllocator specialized
  | for Windows.
  |
  */
#[cfg(windows)]
#[derive(Getters, Builder)]
#[getset(get = "pub")]
pub struct Win32LockedPageAllocator {
    page_size: usize,
}

#[cfg(windows)]
impl Win32LockedPageAllocator {
    /// Construct a new allocator, determining the system page size at runtime.
    pub fn new() -> Self {
        unsafe {
            let mut info = SYSTEM_INFO::default();
            GetSystemInfo(&mut info);
            Self {
                page_size: info.dwPageSize as usize,
            }
        }
    }
}

#[cfg(windows)]
impl Default for Win32LockedPageAllocator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(windows)]
impl LockedPageAllocator for Win32LockedPageAllocator {}

#[cfg(windows)]
impl AllocateLocked for Win32LockedPageAllocator {
    fn allocate_locked(&mut self, len: usize, locking_success: *mut bool) -> *mut c_void {
        unsafe {
            let aligned = align_up(len, self.page_size);
            trace!(len, aligned, "VirtualAlloc request (Win32)");

            let addr = VirtualAlloc(
                core::ptr::null_mut(),
                aligned,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            );

            if addr.is_null() {
                error!(aligned, "VirtualAlloc failed");
                return core::ptr::null_mut();
            }

            // VirtualLock is used to attempt to keep keying material out of swap. Note
            // that it does not provide this as a guarantee, but, in practice, memory
            // that has been VirtualLock'd almost never gets written to the pagefile
            // except in rare circumstances where memory is extremely low.
            let locked = VirtualLock(addr, aligned) != 0;
            *locking_success = locked;

            if !locked {
                warn!(aligned, "VirtualLock failed – page may be paged out");
            }

            trace!(?addr, locked, "allocate_locked complete");
            addr as *mut c_void
        }
    }
}

#[cfg(windows)]
impl FreeLocked for Win32LockedPageAllocator {
    fn free_locked(&mut self, addr: *mut c_void, len: usize) {
        unsafe {
            if addr.is_null() || len == 0 {
                return;
            }
            let aligned = align_up(len, self.page_size);
            memory_cleanse(addr, aligned);

            // Best‑effort unlock; ignore failure.
            VirtualUnlock(addr, aligned);

            let released = VirtualFree(addr, 0, MEM_RELEASE) != 0;
            if !released {
                warn!(?addr, "VirtualFree failed");
            }

            trace!(?addr, aligned, released, "free_locked complete");
        }
    }
}

#[cfg(windows)]
impl GetLimit for Win32LockedPageAllocator {
    fn get_limit(&mut self) -> usize {
        // Windows does not expose a per‑process mlock limit; treat as “unlimited”.
        usize::MAX
    }
}
