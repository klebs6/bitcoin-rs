// ---------------- [ File: bitcoinsecp256k1-ecmult/src/strauss_max_points.rs ]
crate::ix!();

pub fn strauss_max_points(
    error_callback: *const Callback,
    scratch:        *mut Scratch,
) -> usize {
    trace!(target: "secp256k1::ecmult", "strauss_max_points");

    unsafe {
        scratch_max_allocation(error_callback, scratch, STRAUSS_SCRATCH_OBJECTS) / strauss_scratch_size(1)
    }
}
