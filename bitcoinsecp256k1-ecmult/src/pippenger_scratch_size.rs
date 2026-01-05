// ---------------- [ File: bitcoinsecp256k1-ecmult/src/pippenger_scratch_size.rs ]
crate::ix!();

/// Returns the scratch size required for a given number of points (excluding base point G) without
/// considering alignment.
///
pub fn pippenger_scratch_size(
        n_points:      usize,
        bucket_window: i32) -> usize {
    
    todo!();
        /*
            size_t entries = 2*n_points + 2;
        size_t entry_size = sizeof(ge) + sizeof(scalar) + sizeof(struct pippenger_point_state) + (WNAF_SIZE(bucket_window+1)+1)*sizeof(int);
        return (sizeof(gej) << bucket_window) + sizeof(struct pippenger_state) + entries * entry_size;
        */
}
