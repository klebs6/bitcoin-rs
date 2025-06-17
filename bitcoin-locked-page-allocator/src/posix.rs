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
    fn default() -> Self {
        // *Every* POSIX‑y platform we care about supports `_SC_PAGESIZE`.
        let ps = unsafe { sysconf(_SC_PAGESIZE) as usize };
        Self { page_size: ps }
    }
}
