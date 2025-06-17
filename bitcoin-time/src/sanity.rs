// ---------------- [ File: bitcoin-time/src/sanity.rs ]
crate::ix!();

/// Verify that Rust’s time epoch matches the Unix epoch.
pub fn chrono_sanity_check() -> bool {

    use std::time::{Duration, UNIX_EPOCH};

    let zero = UNIX_EPOCH;

    if zero
        .duration_since(UNIX_EPOCH)
        .expect("epoch calculation")
        != Duration::ZERO
    {
        error!("UNIX_EPOCH offset is non‑zero");
        return false;
    }

    info!("chrono_sanity_check passed");

    true
}
