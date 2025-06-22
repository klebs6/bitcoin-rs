// -----------------------------------------------------------------------------
//  bitcoin‑locked‑pool/tests/locked_pool_live.rs
// -----------------------------------------------------------------------------
use std::{ffi::c_void, panic};

use bitcoin_locked_pool::*;
use tracing::info;

/**
  | These tests used the live LockedPoolManager
  | object, this is also used by other tests so the
  | conditions are somewhat less controllable and
  | thus the tests are somewhat more error-prone.
  */
#[traced_test]
fn lockedpool_tests_live() {
    use std::ptr::{null_mut, write_volatile, read_volatile};

    let pool = LockedPoolManager::instance();
    let initial = pool.stats();

    // ----------------  Basic alloc / read‑write  ----------------
    let a0 = pool.alloc(16);
    assert!(!a0.is_null());
    unsafe {
        write_volatile(a0 as *mut u32, 0x1234);
        assert_eq!(read_volatile(a0 as *mut u32), 0x1234);
    }

    // ----------------  Free + double‑free panic  ----------------
    pool.free(a0);
    assert!(
        panic::catch_unwind(panic::AssertUnwindSafe(|| pool.free(a0))).is_err(),
        "double‑free must panic"
    );

    // ----------------  Totals and usage  ------------------------
    assert!(
        *pool.stats().total() <= *initial.total() + locked_pool::ARENA_SIZE,
        "no more than one new arena should have been allocated"
    );
    assert_eq!(
        *pool.stats().used(),
        *initial.used(),
        "usage must return to initial level"
    );
}

