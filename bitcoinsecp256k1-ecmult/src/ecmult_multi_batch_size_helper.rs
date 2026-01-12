// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_multi_batch_size_helper.rs ]
crate::ix!();

/// Compute the number of batches and the batch size given the maximum batch size and the total
/// number of points
///
pub fn ecmult_multi_batch_size_helper(
    n_batches:          *mut usize,
    n_batch_points:     *mut usize,
    max_n_batch_points: usize,
    n:                  usize,
) -> i32 {
    tracing::trace!(
        target: "secp256k1::ecmult",
        max_n_batch_points = max_n_batch_points,
        n = n,
        "ecmult_multi_batch_size_helper"
    );

    unsafe {
        let mut max_n_batch_points = max_n_batch_points;

        if max_n_batch_points == 0 {
            return 0;
        }
        if max_n_batch_points > ECMULT_MAX_POINTS_PER_BATCH {
            max_n_batch_points = ECMULT_MAX_POINTS_PER_BATCH;
        }
        if n == 0 {
            *n_batches = 0;
            *n_batch_points = 0;
            return 1;
        }
        /* Compute ceil(n/max_n_batch_points) and ceil(n/n_batches) */
        *n_batches = 1 + (n - 1) / max_n_batch_points;
        *n_batch_points = 1 + (n - 1) / *n_batches;
        1
    }
}

#[cfg(test)]
mod ecmult_multi_batching_contract_suite {
    use super::*;

    #[traced_test]
    fn batch_size_helper_rejects_zero_max_batch_points_without_mutating_outputs() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "batch_size_helper_rejects_zero_max_batch_points_without_mutating_outputs"
        );

        unsafe {
            let mut n_batches: usize = 0xDEAD_BEEF_DEAD_BEEFusize;
            let mut n_batch_points: usize = 0xCAFE_BABE_CAFE_BABEusize;

            let ok = ecmult_multi_batch_size_helper(
                core::ptr::addr_of_mut!(n_batches),
                core::ptr::addr_of_mut!(n_batch_points),
                0,
                123,
            );

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                ok = ok,
                n_batches = n_batches,
                n_batch_points = n_batch_points,
                "helper output"
            );

            assert_eq!(ok, 0);
            assert_eq!(n_batches, 0xDEAD_BEEF_DEAD_BEEFusize);
            assert_eq!(n_batch_points, 0xCAFE_BABE_CAFE_BABEusize);
        }
    }

    #[traced_test]
    fn batch_size_helper_handles_zero_points() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "batch_size_helper_handles_zero_points"
        );

        unsafe {
            let mut n_batches: usize = 1;
            let mut n_batch_points: usize = 1;

            let ok = ecmult_multi_batch_size_helper(
                core::ptr::addr_of_mut!(n_batches),
                core::ptr::addr_of_mut!(n_batch_points),
                1024,
                0,
            );

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                ok = ok,
                n_batches = n_batches,
                n_batch_points = n_batch_points,
                "helper output for n=0"
            );

            assert_eq!(ok, 1);
            assert_eq!(n_batches, 0);
            assert_eq!(n_batch_points, 0);
        }
    }

    #[traced_test]
    fn batch_size_helper_caps_max_batch_points_and_computes_ceils_correctly() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "batch_size_helper_caps_max_batch_points_and_computes_ceils_correctly"
        );

        unsafe {
            let mut n_batches: usize = 0;
            let mut n_batch_points: usize = 0;

            let n = ECMULT_MAX_POINTS_PER_BATCH + 1;
            let ok = ecmult_multi_batch_size_helper(
                core::ptr::addr_of_mut!(n_batches),
                core::ptr::addr_of_mut!(n_batch_points),
                ECMULT_MAX_POINTS_PER_BATCH + 1000,
                n,
            );

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                ok = ok,
                n_batches = n_batches,
                n_batch_points = n_batch_points,
                n = n,
                "helper output for capped max batch points"
            );

            assert_eq!(ok, 1);

            assert!(n_batches >= 2);
            assert!(n_batch_points <= ECMULT_MAX_POINTS_PER_BATCH);
            assert!(n_batches * n_batch_points >= n);
        }
    }

    #[traced_test]
    fn batch_size_helper_small_examples_match_expected_values() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "batch_size_helper_small_examples_match_expected_values"
        );

        unsafe {
            let mut n_batches: usize = 0;
            let mut n_batch_points: usize = 0;

            let ok = ecmult_multi_batch_size_helper(
                core::ptr::addr_of_mut!(n_batches),
                core::ptr::addr_of_mut!(n_batch_points),
                10,
                25,
            );

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                ok = ok,
                n_batches = n_batches,
                n_batch_points = n_batch_points,
                "helper output for max=10, n=25"
            );

            assert_eq!(ok, 1);
            assert_eq!(n_batches, 3);
            assert_eq!(n_batch_points, 9);
            assert!(n_batches * n_batch_points >= 25);
        }
    }
}
