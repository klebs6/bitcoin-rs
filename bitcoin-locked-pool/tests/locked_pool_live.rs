// ---------------- [ File: bitcoin-locked-pool/tests/locked_pool_live.rs ]
// -----------------------------------------------------------------------------
//  bitcoin‑locked‑pool/tests/locked_pool_live.rs
// -----------------------------------------------------------------------------
use std::{ffi::c_void, panic};

use bitcoin_locked_pool::*;
use bitcoin_imports::*;

#[traced_test]
fn lockedpool_tests_live() {
    use std::ptr::{read_volatile, write_volatile};

    // Obtain a **mutable** pointer to the global singleton.
    let pool_ptr = LockedPoolManager::instance() as *const _ as *mut LockedPoolManager;

    unsafe {
        let initial = (*pool_ptr).stats();

        // ----------------  Basic alloc / read‑write  ----------------
        let a0 = (*pool_ptr).alloc(16);
        assert!(!a0.is_null());
        write_volatile(a0 as *mut u32, 0x1234);
        assert_eq!(read_volatile(a0 as *mut u32), 0x1234);

        // ----------------  Free + double‑free panic  ----------------
        (*pool_ptr).free(a0);
        assert!(
            panic::catch_unwind(panic::AssertUnwindSafe(|| {
                (*pool_ptr).free(a0);
            }))
            .is_err(),
            "double‑free must panic"
        );

        // ----------------  Totals and usage  ------------------------
        assert!(
            *(*pool_ptr).stats().total() <= *initial.total() + LOCKED_POOL_ARENA_SIZE,
            "no more than one new arena should have been allocated"
        );
        assert_eq!(
            *(*pool_ptr).stats().used(),
            *initial.used(),
            "usage must return to initial level"
        );

        trace!("live‑test completed successfully");
    }
}
