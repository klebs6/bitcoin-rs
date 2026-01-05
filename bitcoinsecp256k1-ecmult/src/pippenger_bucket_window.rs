// ---------------- [ File: bitcoinsecp256k1-ecmult/src/pippenger_bucket_window.rs ]
crate::ix!();

/// Returns optimal bucket_window (number of bits of a scalar represented by a set of buckets) for
/// a given number of points.
///
pub fn pippenger_bucket_window(n: usize) -> i32 {
    
    todo!();
        /*
        if (n <= 1) {
            return 1;
        } else if (n <= 4) {
            return 2;
        } else if (n <= 20) {
            return 3;
        } else if (n <= 57) {
            return 4;
        } else if (n <= 136) {
            return 5;
        } else if (n <= 235) {
            return 6;
        } else if (n <= 1260) {
            return 7;
        } else if (n <= 4420) {
            return 9;
        } else if (n <= 7880) {
            return 10;
        } else if (n <= 16050) {
            return 11;
        } else {
            return PIPPENGER_MAX_BUCKET_WINDOW;
        }
        */
}
