
// -----------------------------------------------------------------------------
//  bitcoin‑locked‑pool/tests/locked_pool_mock.rs
// -----------------------------------------------------------------------------
use std::ffi::c_void;

use bitcoin_locked_pool::*;
use bitcoin_locked_page_allocator::{AllocateLocked, FreeLocked, GetLimit, LockedPageAllocator};
use bitcoin_support::{align_up, memory_cleanse};
use tracing::{info, trace};

/// **Deterministic stub**: allows a fixed number of arenas (`max_arenas`),
/// with only `max_locked` of them succeeding the lock operation.
struct TestLockedPageAllocator {
    max_arenas:  usize,
    max_locked:  usize,
    arena_count: usize,
    locked_count: usize,
    page_size:   usize,
}

impl TestLockedPageAllocator {
    fn new(max_arenas: usize, max_locked: usize) -> Self {
        Self {
            max_arenas,
            max_locked,
            arena_count: 0,
            locked_count: 0,
            page_size: 4096,
        }
    }
}

impl LockedPageAllocator for TestLockedPageAllocator {}

impl AllocateLocked for TestLockedPageAllocator {
    fn allocate_locked(&mut self, len: usize, locking_success: *mut bool) -> *mut c_void {
        if self.arena_count >= self.max_arenas {
            return std::ptr::null_mut();
        }
        self.arena_count += 1;

        // “Allocate” some raw memory (test only).
        unsafe {
            let aligned = align_up(len, self.page_size);
            let ptr = libc::malloc(aligned) as *mut c_void;
            if ptr.is_null() {
                return ptr;
            }
            let locked = if self.locked_count < self.max_locked {
                self.locked_count += 1;
                true
            } else {
                false
            };
            *locking_success = locked;
            trace!(aligned, locked, ?ptr, "Test allocator allocate_locked");
            ptr
        }
    }
}

impl FreeLocked for TestLockedPageAllocator {
    fn free_locked(&mut self, addr: *mut c_void, len: usize) {
        unsafe {
            if addr.is_null() { return; }
            let aligned = align_up(len, self.page_size);
            memory_cleanse(addr, aligned);
            libc::free(addr as *mut libc::c_void);
            trace!(?addr, aligned, "Test allocator free_locked");
        }
    }
}

impl GetLimit for TestLockedPageAllocator {
    fn get_limit(&mut self) -> usize { usize::MAX }
}

#[traced_test]
fn lockedpool_tests_mock() {
    // ------------------------------------------------------------
    //  Test over three virtual arenas, only one of which “locks”
    // ------------------------------------------------------------
    let alloc = Box::new(TestLockedPageAllocator::new(3, 1));
    let mut pool = LockedPool::new(alloc, None);

    assert_eq!(*pool.stats().total(), 0);
    assert_eq!(*pool.stats().locked(), 0);

    // --------  Reject unreasonable requests  --------
    assert!(pool.alloc(0).is_null());
    assert_eq!(*pool.stats().used(), 0);
    assert_eq!(*pool.stats().free(), 0);

    assert!(pool.alloc(locked_pool::ARENA_SIZE + 1).is_null());
    assert_eq!(*pool.stats().used(), 0);
    assert_eq!(*pool.stats().free(), 0);

    // --------  Six half‑arena allocations  ----------
    let a0 = pool.alloc(locked_pool::ARENA_SIZE / 2); assert!(!a0.is_null());
    assert_eq!(*pool.stats().locked(), locked_pool::ARENA_SIZE);

    let a1 = pool.alloc(locked_pool::ARENA_SIZE / 2); assert!(!a1.is_null());
    let a2 = pool.alloc(locked_pool::ARENA_SIZE / 2); assert!(!a2.is_null());
    let a3 = pool.alloc(locked_pool::ARENA_SIZE / 2); assert!(!a3.is_null());
    let a4 = pool.alloc(locked_pool::ARENA_SIZE / 2); assert!(!a4.is_null());
    let a5 = pool.alloc(locked_pool::ARENA_SIZE / 2); assert!(!a5.is_null());

    // 4ᵗʰ arena would exceed the test allocator’s limit – must fail
    assert!(pool.alloc(16).is_null());

    // --------  Free in mixed order  -----------------
    for p in [a0, a2, a4, a1, a3, a5] { pool.free(p); }

    assert_eq!(*pool.stats().total(), 3 * locked_pool::ARENA_SIZE);
    assert_eq!(*pool.stats().locked(), locked_pool::ARENA_SIZE);
    assert_eq!(*pool.stats().used(), 0);
}
