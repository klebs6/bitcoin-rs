use bitcoin_imports::*;
use bitcoin_support::*;
use bitcoin_locked_pool::*;
use bitcoin_locked_page_allocator::*;

/// Allocator that always “locks” memory – sufficient for unit testing.
struct DummyLockedPageAllocator;

impl LockedPageAllocator for DummyLockedPageAllocator {}

impl AllocateLocked for DummyLockedPageAllocator {
    fn allocate_locked(&mut self, len: usize, locking_success: *mut bool) -> *mut c_void {
        unsafe {
            *locking_success = true;
            libc::malloc(len) as *mut c_void
        }
    }
}

impl FreeLocked for DummyLockedPageAllocator {
    fn free_locked(&mut self, addr: *mut c_void, len: usize) {
        unsafe {
            if addr.is_null() { return; }
            memory_cleanse(addr, len);
            libc::free(addr as *mut libc::c_void);
        }
    }
}

impl GetLimit for DummyLockedPageAllocator {
    fn get_limit(&mut self) -> usize { usize::MAX }
}

#[traced_test]
fn locked_page_arena_basic_operations() {
    let mut alloc = DummyLockedPageAllocator;
    let arena_size = LOCKED_POOL_ARENA_SIZE;
    let mut locked = false;
    let base = alloc.allocate_locked(arena_size, &mut locked as *mut bool);
    assert!(locked, "memory must be reported as locked");
    assert!(!base.is_null());

    let mut arena = unsafe {
        LockedPageArena::new(
            &mut alloc as *mut _ as *mut dyn LockedPageAllocator,
            base,
            arena_size,
            LOCKED_POOL_ARENA_ALIGN,
        )
    };

    // ----------  allocate & free  ----------
    let p = arena.alloc(128);
    assert!(!p.is_null());
    assert!(arena.address_in_arena(p));
    arena.free(p);

    // ----------  stats must reflect zero‑usage  ----------
    let s = arena.stats();
    assert_eq!(*s.used(), 0);
    assert_eq!(*s.free(), arena_size);

    // dropping `arena` must release memory – left to ASan / Valgrind to verify
}
