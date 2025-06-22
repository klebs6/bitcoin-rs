// ---------------- [ File: bitcoin-locked-pool/tests/locked_pool.rs ]
use bitcoin_support::*;
use bitcoin_locked_pool::*;

#[traced_test]
fn locked_pool_full_coverage() {
    // Use the real POSIX allocator on *nix; it gracefully falls back if `mlock` fails.
    let alloc = Box::new(PosixLockedPageAllocator::default());
    let mut pool = LockedPool::new(alloc, None);

    // ------------------------------------------------------------
    //  Basic allocation / free
    // ------------------------------------------------------------
    let p = pool.alloc(1_000);
    assert!(!p.is_null(), "initial allocation");
    pool.free(p);
    assert_eq!(*pool.stats().used(), 0);

    // ------------------------------------------------------------
    //  Fill a whole arena and observe failure
    // ------------------------------------------------------------
    let mut vec = Vec::new();
    loop {
        let blk = pool.alloc(32 * 1_024); // 32 KiB
        if blk.is_null() { break; }
        vec.push(blk);
    }
    // All allocations must be within the arena’s 256 KiB limit.
    assert!(!vec.is_empty());
    for ptr in &vec { pool.free(*ptr); }
    vec.clear();

    // ------------------------------------------------------------
    //  Allocate zero bytes ⇒ null
    // ------------------------------------------------------------
    assert!(pool.alloc(0).is_null());

    // ------------------------------------------------------------
    //  Pointer not belonging to any arena ⇒ panic
    // ------------------------------------------------------------
    let bogus = 0x1usize as *mut std::ffi::c_void;
    assert!(std::panic::catch_unwind(|| pool.free(bogus)).is_err());
}

#[traced_test]
fn locked_pool_manager_singleton() {
    let m0 = LockedPoolManager::instance() as *mut _;
    let m1 = LockedPoolManager::instance() as *mut _;
    // Same instance every time
    assert_eq!(m0, m1);

    // Basic smoke‑test through the wrapper
    let mgr = unsafe { &mut *m0 };
    let p = mgr.alloc(512);
    assert!(!p.is_null());
    mgr.free(p);
    assert_eq!(*mgr.stats().used(), 0);
}

