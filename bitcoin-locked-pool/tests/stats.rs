// ---------------- [ File: bitcoin-locked-pool/tests/stats.rs ]
use bitcoin_imports::*;
use bitcoin_support::*;
use bitcoin_locked_pool::*;
use bitcoin_locked_page_allocator::*;

#[traced_test]
fn stats_aggregate_correctly() {
    let alloc = Box::new(PosixLockedPageAllocator::default());
    let mut pool = LockedPool::new(alloc, None);

    let a = pool.alloc(512);
    let b = pool.alloc(1_024);
    assert!(!a.is_null() && !b.is_null());

    let s = pool.stats();
    assert_eq!(*s.used(), 1_536);
    assert_eq!(*s.chunks_used(), 2);

    pool.free(a);
    pool.free(b);
    assert_eq!(*pool.stats().used(), 0);
}
