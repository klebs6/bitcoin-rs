// ---------------- [ File: bitcoin-locked-pool/tests/config_constants.rs ]
use bitcoin_imports::*;
use bitcoin_support::*;
use bitcoin_locked_pool::*;

#[traced_test]
fn arena_configuration_consistency() {
    trace!(
        size  = LOCKED_POOL_ARENA_SIZE,
        align = LOCKED_POOL_ARENA_ALIGN,
        "checking locked‑pool configuration constants"
    );
    assert_eq!(LOCKED_POOL_ARENA_SIZE, 256 * 1024);
    assert_eq!(LOCKED_POOL_ARENA_ALIGN, 16);
    assert_eq!(
        LOCKED_POOL_ARENA_SIZE % LOCKED_POOL_ARENA_ALIGN,
        0,
        "arena size must be a multiple of alignment"
    );
}
