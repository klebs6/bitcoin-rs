// ---------------- [ File: bitcoinsecp256k1-ecmult/src/pippenger_bucket_window.rs ]
crate::ix!();

/// Returns optimal bucket_window (number of bits of a scalar represented by a set of buckets) for
/// a given number of points.
///
pub fn pippenger_bucket_window(n: usize) -> i32 {
    tracing::trace!(target: "secp256k1::ecmult", n = n, "pippenger_bucket_window");

    if n <= 1 {
        1
    } else if n <= 4 {
        2
    } else if n <= 20 {
        3
    } else if n <= 57 {
        4
    } else if n <= 136 {
        5
    } else if n <= 235 {
        6
    } else if n <= 1260 {
        7
    } else if n <= 4420 {
        9
    } else if n <= 7880 {
        10
    } else if n <= 16050 {
        11
    } else {
        PIPPENGER_MAX_BUCKET_WINDOW as i32
    }
}
