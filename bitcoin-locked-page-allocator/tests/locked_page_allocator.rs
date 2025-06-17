// ---------------- [ File: bitcoin-locked-page-allocator/tests/locked_page_allocator.rs ]
use bitcoin_support::*;
use bitcoin_imports::*;
use bitcoin_locked_page_allocator::*;


/**
  | Mock LockedPageAllocator for testing
  |
  */
#[derive(Getters, Builder)]
#[getset(get="pub")]
pub struct TestLockedPageAllocator {
    count: i32,
    lockedcount: i32,
}

impl TestLockedPageAllocator {
    pub fn new(count: i32, lockedcount: i32) -> Self {
        Self { count, lockedcount }
    }
}

impl LockedPageAllocator for TestLockedPageAllocator {}

impl AllocateLocked for TestLockedPageAllocator {
    fn allocate_locked(&mut self, _len: usize, locking_success: *mut bool) -> *mut std::ffi::c_void {
        unsafe { *locking_success = false };

        if self.count <= 0 {
            return core::ptr::null_mut();
        }

        self.count -= 1;

        if self.lockedcount > 0 {
            self.lockedcount -= 1;
            unsafe { *locking_success = true };
        }

        let fake = 0x0800_0000u64 + ((self.count as u64) << 24);
        trace!(fake = format_args!("{fake:#x}"), "Test allocator produced fake address");
        fake as *mut std::ffi::c_void
    }
}

impl FreeLocked for TestLockedPageAllocator {
    fn free_locked(&mut self, _addr: *mut std::ffi::c_void, _len: usize) {
        // Nothing to do – this is a mock.
    }
}

impl GetLimit for TestLockedPageAllocator {
    fn get_limit(&mut self) -> usize {
        usize::MAX
    }
}
