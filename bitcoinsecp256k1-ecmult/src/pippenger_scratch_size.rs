// ---------------- [ File: bitcoinsecp256k1-ecmult/src/pippenger_scratch_size.rs ]
crate::ix!();

/// Returns the scratch size required for a given number of points (excluding base point G) without
/// considering alignment.
///
pub fn pippenger_scratch_size(
    n_points:      usize,
    bucket_window: i32,
) -> usize {

    use core::mem::size_of;

    trace!(
        target: "secp256k1::ecmult",
        n_points = n_points,
        bucket_window = bucket_window,
        "pippenger_scratch_size"
    );

    let entries: usize = 2usize * n_points + 2usize;
    let entry_size: usize = size_of::<Ge>()
        + size_of::<Scalar>()
        + size_of::<PippengerPointState>()
        + (wnaf_size!(bucket_window + 1) + 1usize) * size_of::<i32>();
    (size_of::<Gej>() << (bucket_window as usize))
        + size_of::<PippengerState>()
        + entries * entry_size

        /*
            size_t entries = 2*n_points + 2;
        size_t entry_size = sizeof(ge) + sizeof(scalar) + sizeof(struct pippenger_point_state) + (WNAF_SIZE(bucket_window+1)+1)*sizeof(int);
        return (sizeof(gej) << bucket_window) + sizeof(struct pippenger_state) + entries * entry_size;
        */

}
