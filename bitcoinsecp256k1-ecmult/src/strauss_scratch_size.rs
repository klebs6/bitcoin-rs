// ---------------- [ File: bitcoinsecp256k1-ecmult/src/strauss_scratch_size.rs ]
crate::ix!();

pub fn strauss_scratch_size(n_points: usize) -> usize {
    trace!(target: "secp256k1::ecmult", n_points = n_points, "strauss_scratch_size");

    let point_size: usize = (2usize * size_of::<Ge>()
        + size_of::<Gej>()
        + size_of::<Fe>())
        * ecmult_table_size!(WINDOW_A)
        + size_of::<StraussPointState>()
        + size_of::<Gej>()
        + size_of::<Scalar>();
    n_points * point_size
}

#[cfg(test)]
mod strauss_scratch_size_contract_suite {
    use super::*;

    #[traced_test]
    fn strauss_scratch_size_is_zero_for_zero_points_and_scales_linearly() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "strauss_scratch_size_is_zero_for_zero_points_and_scales_linearly"
        );

        let z = strauss_scratch_size(0);
        let one = strauss_scratch_size(1);
        let two = strauss_scratch_size(2);

        tracing::debug!(
            target: "secp256k1::ecmult::tests",
            z = z,
            one = one,
            two = two,
            "strauss_scratch_size samples"
        );

        assert_eq!(z, 0);
        assert!(one > 0);
        assert_eq!(two, 2 * one);
    }
}
