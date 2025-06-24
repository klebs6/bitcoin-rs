// ---------------- [ File: bitcoin-locked-page-allocator/src/posix.rs ]
crate::ix!();

use libc::{
    c_void, 
    getrlimit, 
    madvise, 
    mlock, 
    mmap, 
    munlock, 
    munmap, 
    sysconf, 
    RLIMIT_MEMLOCK, 
    RLIM_INFINITY,
    MAP_ANONYMOUS, 
    MAP_FAILED, 
    MAP_PRIVATE, 
    PROT_READ, 
    PROT_WRITE,
    _SC_PAGESIZE,
};

#[cfg(target_os = "linux")]
use libc::MADV_DONTDUMP;

#[cfg(target_os = "freebsd")]
use libc::MADV_NOCORE;

/* --- Implementation: PosixLockedPageAllocator  --- */

/**
  | LockedPageAllocator specialized
  | for OSes that don't try to be special
  | snowflakes.
  |
  */
#[cfg(unix)]
#[derive(Getters, Builder)]
#[getset(get = "pub")]
pub struct PosixLockedPageAllocator {
    page_size: usize,
}

#[cfg(unix)]
impl LockedPageAllocator for PosixLockedPageAllocator {}

#[cfg(unix)]
impl AllocateLocked for PosixLockedPageAllocator {
    fn allocate_locked(&mut self, len: usize, locking_success: *mut bool) -> *mut c_void {
        unsafe {
            let aligned = align_up(len, self.page_size);
            trace!(len, aligned, "mmap request (POSIX)");

            let addr = mmap(
                core::ptr::null_mut(),
                aligned,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            );

            if addr == MAP_FAILED {
                error!(len, "mmap failed");
                return core::ptr::null_mut();
            }

            let locked = mlock(addr, aligned) == 0;
            *locking_success = locked;
            if !locked {
                warn!(aligned, "mlock failed – page will not be resident");
            }

            #[cfg(target_os = "linux")]
            madvise(addr, aligned, MADV_DONTDUMP);

            #[cfg(target_os = "freebsd")]
            madvise(addr, aligned, MADV_NOCORE);

            trace!(?addr, locked, "allocate_locked complete");
            addr
        }
    }
}

#[cfg(unix)]
impl FreeLocked for PosixLockedPageAllocator {
    fn free_locked(&mut self, addr: *mut c_void, len: usize) {
        unsafe {
            if addr.is_null() || len == 0 {
                return;
            }
            let aligned = align_up(len, self.page_size);
            memory_cleanse(addr, aligned);
            munlock(addr, aligned);
            munmap(addr, aligned);
            trace!(?addr, aligned, "free_locked complete");
        }
    }
}

#[cfg(unix)]
impl GetLimit for PosixLockedPageAllocator {
    fn get_limit(&mut self) -> usize {
        unsafe {
            let mut rlim = core::mem::zeroed();
            if getrlimit(RLIMIT_MEMLOCK, &mut rlim) == 0 && rlim.rlim_cur != RLIM_INFINITY {
                return rlim.rlim_cur as usize;
            }
        }
        usize::MAX
    }
}

#[cfg(unix)]
impl Default for PosixLockedPageAllocator {
    /// Run‑time discovery of the system page size **with robust fall‑backs**.
    ///
    /// * Ensures the determined page size is a power‑of‑two – a hard
    ///   requirement for `align_up`’s internal invariants.
    /// * Emits helpful diagnostics when the platform call fails or returns an
    ///   unexpected value (e.g. non‑power‑of‑two on exotic platforms).
    ///
    /// This closes the crash pathway observed in `locked_pool_full_coverage`
    /// where `sysconf(_SC_PAGESIZE)` returned an error (‑1), propagating a
    /// non‑power‑of‑two into `align_up`, which then tripped its `debug_assert!`.
    fn default() -> Self {
        // SAFETY: `sysconf` is a simple, re‑entrant libc wrapper.
        let raw_pagesize = unsafe { sysconf(_SC_PAGESIZE) };

        // Convert and validate.  A negative or zero result signals failure.
        let mut ps = if raw_pagesize > 0 {
            raw_pagesize as usize
        } else {
            warn!(
                target: "locked_page_alloc",
                raw_pagesize,
                "sysconf(_SC_PAGESIZE) failed – falling back to 4 KiB"
            );
            4 * 1024 // universally valid, conservative default
        };

        // Enforce power‑of‑two to satisfy `align_up` invariants.
        if !ps.is_power_of_two() {
            let rounded = ps.next_power_of_two();
            warn!(
                target: "locked_page_alloc",
                original = ps,
                rounded,
                "page size not power‑of‑two – rounding up"
            );
            ps = rounded;
        }

        trace!(
            target: "locked_page_alloc",
            page_size = ps,
            "PosixLockedPageAllocator initialised"
        );
        Self { page_size: ps }
    }
}
