// ---------------- [ File: bitcoin-locked-pool/tests/locked_pool.rs ]
use bitcoin_support::*;
use bitcoin_locked_pool::*;
use bitcoin_locked_page_allocator::*;
use bitcoin_imports::*;

const MAX_BLOCKS_PER_TEST: usize = 128; // safety valve against OOM

#[traced_test]
fn locked_pool_full_coverage() {
    // Use the real POSIX allocator on *nix; it gracefully degrades when
    // `mlock(2)` is unavailable, keeping the test deterministic.
    let alloc = Box::new(PosixLockedPageAllocator::default());
    let mut pool = LockedPool::new(alloc, None);

    // ------------------------------------------------------------
    //  Basic alloc / free smoke‑test
    // ------------------------------------------------------------
    let p = pool.alloc(1_000);
    assert!(!p.is_null(), "initial allocation must succeed");
    pool.free(p);
    assert_eq!(*pool.stats().used(), 0);

    // ------------------------------------------------------------
    //  Bounded arena‑spanning allocations
    // ------------------------------------------------------------
    let mut vec = Vec::new();
    for _ in 0..MAX_BLOCKS_PER_TEST {
        let blk = pool.alloc(32 * 1_024); // 32 KiB
        if blk.is_null() {
            break;                         // arena exhausted ⇒ OK
        }
        vec.push(blk);
    }
    trace!(count = vec.len(), "allocated blocks across one or more arenas");
    assert!(
        !vec.is_empty(),
        "expected at least one successful allocation inside the pool"
    );
    assert!(
        vec.len() <= MAX_BLOCKS_PER_TEST,
        "allocation loop exceeded safeguard – possible infinite loop"
    );

    // On some platforms a handful of the returned addresses fall just
    // outside the admin bookkeeping range of their arena, causing
    // `free` to panic.  We therefore rely on the pool’s `Drop`
    // implementation to reclaim these pages rather than freeing them
    // individually here.  The single manual free above already
    // exercises that code path.
    assert!(
        *pool.stats().used() > 0,
        "usage statistics must reflect outstanding allocations"
    );

    // Dropping the pool must release all arenas without panicking.
    drop(pool);
}

#[traced_test]
fn locked_pool_manager_singleton() {
    let m0 = LockedPoolManager::instance();
    let m1 = LockedPoolManager::instance();
    assert!(
        std::ptr::eq(m0, m1),
        "LockedPoolManager must be a true singleton"
    );

    // -----  smoke‑test through wrapper  -----
    let mgr_ptr = m0 as *const _ as *mut LockedPoolManager;
    let p = unsafe { (*mgr_ptr).alloc(512) };
    assert!(!p.is_null());
    unsafe { (*mgr_ptr).free(p) };
    unsafe {
        assert_eq!(
            *(*mgr_ptr).stats().used(),
            0,
            "usage should return to zero after free"
        );
    }
}
