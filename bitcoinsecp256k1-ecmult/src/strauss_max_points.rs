// ---------------- [ File: bitcoinsecp256k1-ecmult/src/strauss_max_points.rs ]
crate::ix!();

pub fn strauss_max_points(error_callback: *const Callback, scratch: *mut Scratch) -> usize {
    trace!(
        target: "secp256k1::ecmult",
        error_callback_is_null = error_callback.is_null(),
        scratch_is_null = scratch.is_null(),
        "strauss_max_points"
    );

    if error_callback.is_null() || scratch.is_null() {
        tracing::warn!(
            target: "secp256k1::ecmult",
            error_callback_is_null = error_callback.is_null(),
            scratch_is_null = scratch.is_null(),
            "strauss_max_points: null callback or scratch; returning 0"
        );
        return 0;
    }

    unsafe {
        let max_alloc: usize =
            scratch_max_allocation(error_callback, scratch, STRAUSS_SCRATCH_OBJECTS);
        let per_point: usize = strauss_scratch_size(1);
        let res: usize = max_alloc / per_point;

        tracing::debug!(
            target: "secp256k1::ecmult",
            max_alloc = max_alloc,
            per_point = per_point,
            res = res,
            "strauss_max_points: computed"
        );

        res
    }
}
#[cfg(test)]
mod strauss_max_points_contract_suite {
    use super::*;

    #[traced_test]
    fn strauss_max_points_returns_zero_for_zeroed_scratch_space() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "strauss_max_points_returns_zero_for_zeroed_scratch_space"
        );

        unsafe {
            let mut scratch: Scratch = core::mem::MaybeUninit::<Scratch>::zeroed().assume_init();
            let got = strauss_max_points(core::ptr::null(), core::ptr::addr_of_mut!(scratch));
            tracing::debug!(target: "secp256k1::ecmult::tests", got = got, "strauss_max_points");
            assert_eq!(got, 0);
        }
    }
}
