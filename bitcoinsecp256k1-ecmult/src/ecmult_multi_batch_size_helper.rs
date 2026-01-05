// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_multi_batch_size_helper.rs ]
crate::ix!();

/// Compute the number of batches and the batch size given the maximum batch size and the total
/// number of points
///
pub fn ecmult_multi_batch_size_helper(
        n_batches:          *mut usize,
        n_batch_points:     *mut usize,
        max_n_batch_points: usize,
        n:                  usize) -> i32 {
    
    todo!();
        /*
        if (max_n_batch_points == 0) {
            return 0;
        }
        if (max_n_batch_points > ECMULT_MAX_POINTS_PER_BATCH) {
            max_n_batch_points = ECMULT_MAX_POINTS_PER_BATCH;
        }
        if (n == 0) {
            *n_batches = 0;
            *n_batch_points = 0;
            return 1;
        }
        /* Compute ceil(n/max_n_batch_points) and ceil(n/n_batches) */
        *n_batches = 1 + (n - 1) / max_n_batch_points;
        *n_batch_points = 1 + (n - 1) / *n_batches;
        return 1;
        */
}
