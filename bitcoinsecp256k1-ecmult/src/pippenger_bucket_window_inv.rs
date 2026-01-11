// ---------------- [ File: bitcoinsecp256k1-ecmult/src/pippenger_bucket_window_inv.rs ]
crate::ix!();

/// Returns the maximum optimal number of points for a bucket_window.
///
pub fn pippenger_bucket_window_inv(bucket_window: i32) -> usize {
    tracing::trace!(
        target: "secp256k1::ecmult",
        bucket_window = bucket_window,
        "pippenger_bucket_window_inv"
    );

    match bucket_window {
        1 => 1,
        2 => 4,
        3 => 20,
        4 => 57,
        5 => 136,
        6 => 235,
        7 => 1260,
        8 => 1260,
        9 => 4420,
        10 => 7880,
        11 => 16050,
        x if x == (PIPPENGER_MAX_BUCKET_WINDOW as i32) => usize::MAX,
        _ => 0,
    }
}
