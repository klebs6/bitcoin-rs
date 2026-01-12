// ---------------- [ File: bitcoinsecp256k1-ecmult/src/config.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/basic-config.h]
#[cfg(feature="secp256k1-use-basic-config")] 
pub const ECMULT_WINDOW_SIZE:   usize = 15;

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult_impl.h]

/// The number of entries a table with precomputed multiples needs to have.
///
#[macro_export]
macro_rules! ecmult_table_size {
    ($w:expr) => {
        (1usize << (($w as usize) - 2usize))
    };
}

/// The number of objects allocated on the scratch space for ecmult_multi algorithms
///
pub const PIPPENGER_SCRATCH_OBJECTS:   usize = 6;
pub const STRAUSS_SCRATCH_OBJECTS:     usize = 6;
pub const PIPPENGER_MAX_BUCKET_WINDOW: usize = 12;

/// Minimum number of points for which pippenger_wnaf is faster than strauss wnaf
///
pub const ECMULT_PIPPENGER_THRESHOLD:  usize = 88;
pub const ECMULT_MAX_POINTS_PER_BATCH: usize = 5000000;

lazy_static!{
    pub static ref ECMULT_CONTEXT_PREALLOCATED_SIZE: usize = {
        round_to_align!(size_of::<GeStorage>() * ecmult_table_size!(WINDOW_G))
            + round_to_align!(size_of::<GeStorage>() * ecmult_table_size!(WINDOW_G))
    };
}

#[cfg(test)]
mod config_contract_suite {
    use super::*;

    #[traced_test]
    fn ecmult_table_size_macro_matches_power_of_two_definition() {
        tracing::info!(target: "secp256k1::ecmult::tests", "ecmult_table_size_macro_matches_power_of_two_definition");

        for w in 2usize..=24usize {
            let got = ecmult_table_size!(w);
            let expected = 1usize << (w - 2usize);

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                w = w,
                got = got,
                expected = expected,
                "ecmult_table_size!(w)"
            );

            assert_eq!(got, expected);
        }
    }

    #[traced_test]
    fn ecmult_context_preallocated_size_matches_rounding_formula() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "ecmult_context_preallocated_size_matches_rounding_formula"
        );

        let expected: usize = round_to_align!(core::mem::size_of::<GeStorage>() * ecmult_table_size!(WINDOW_G))
            + round_to_align!(core::mem::size_of::<GeStorage>() * ecmult_table_size!(WINDOW_G));

        let got: usize = *ECMULT_CONTEXT_PREALLOCATED_SIZE;

        tracing::debug!(
            target: "secp256k1::ecmult::tests",
            expected = expected,
            got = got,
            window_g = WINDOW_G,
            "ECMULT_CONTEXT_PREALLOCATED_SIZE"
        );

        assert_eq!(got, expected);
        assert!(got > 0);
    }

    #[traced_test]
    fn algorithm_threshold_constants_are_reasonable() {
        tracing::info!(target: "secp256k1::ecmult::tests", "algorithm_threshold_constants_are_reasonable");

        tracing::debug!(
            target: "secp256k1::ecmult::tests",
            pippenger_threshold = ECMULT_PIPPENGER_THRESHOLD,
            max_points_per_batch = ECMULT_MAX_POINTS_PER_BATCH,
            pippenger_max_bucket_window = PIPPENGER_MAX_BUCKET_WINDOW,
            "threshold constants"
        );

        assert!(ECMULT_PIPPENGER_THRESHOLD > 0);
        assert!(ECMULT_MAX_POINTS_PER_BATCH > 0);
        assert!(PIPPENGER_MAX_BUCKET_WINDOW > 0);
    }
}
