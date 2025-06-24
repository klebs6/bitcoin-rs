// ---------------- [ File: bitcoin-locked-pool/tests/locked_pool_alloc.rs ]
use bitcoin_imports::*;
use bitcoin_support::*;
use bitcoin_locked_pool::*;
use bitcoin_locked_page_allocator::*;

/// Deterministic stub allocator reused across tests.
struct TestLockedPageAllocator {
    max_arenas:   usize,
    max_locked:   usize,
    arena_count:  usize,
    locked_count: usize,
    page_size:    usize,
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
        unsafe {
            let aligned = align_up(len, self.page_size);
            let ptr = libc::malloc(aligned) as *mut c_void;
            if ptr.is_null() { return ptr; }
            let locked = if self.locked_count < self.max_locked {
                self.locked_count += 1;
                true
            } else {
                false
            };
            *locking_success = locked;
            trace!(aligned, locked, ?ptr, "stub allocate_locked");
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
            trace!(?addr, aligned, "stub free_locked");
        }
    }
}

impl GetLimit for TestLockedPageAllocator {
    fn get_limit(&mut self) -> usize { usize::MAX }
}

#[traced_test]
fn allocation_limits_and_double_free() {
    let alloc = Box::new(TestLockedPageAllocator::new(2, 1));
    let mut pool = LockedPool::new(alloc, None);

    // oversize request must fail
    assert!(pool.alloc(LOCKED_POOL_ARENA_SIZE + 1).is_null());

    // small allocation succeeds
    let p = pool.alloc(1024);
    assert!(!p.is_null());

    // double‑free must panic
    pool.free(p);
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| pool.free(p)))
            .is_err(),
        "double‑free must panic"
    );
}

#[traced_test]
fn pool_grows_until_arena_cap() {
    let alloc = Box::new(TestLockedPageAllocator::new(3, 3));
    let mut pool = LockedPool::new(alloc, None);

    let mut v = Vec::new();
    loop {
        let blk = pool.alloc(LOCKED_POOL_ARENA_SIZE / 2); // two blocks per arena
        if blk.is_null() { break; }
        v.push(blk);
    }
    assert_eq!(v.len(), 6);                     // 3 arenas × 2 blocks
    assert_eq!(*pool.stats().total(), 3 * LOCKED_POOL_ARENA_SIZE);

    for p in v { pool.free(p); }
    assert_eq!(*pool.stats().used(), 0);
}
