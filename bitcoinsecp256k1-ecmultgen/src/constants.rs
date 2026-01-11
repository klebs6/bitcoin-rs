// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/constants.rs ]
crate::ix!();

/**
  | Set ECMULT_GEN_PREC_BITS to 2, 4 or
  | 8.
  |
  */
const_assert!{
    ECMULT_GEN_PREC_BITS == 2 
        || ECMULT_GEN_PREC_BITS == 4 
        || ECMULT_GEN_PREC_BITS == 8
}

#[cfg(feature="secp256k1-use-basic-config")] 
pub const ECMULT_GEN_PREC_BITS: usize = 4;

pub const ECMULT_GEN_PREC_B: usize = ECMULT_GEN_PREC_BITS;
pub const ECMULT_GEN_PREC_G: usize = 1 << ECMULT_GEN_PREC_B;
pub const ECMULT_GEN_PREC_N: usize = 256 / ECMULT_GEN_PREC_B;

#[cfg(not(USE_ECMULT_STATIC_PRECOMPUTATION))]
pub const ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE: usize = round_to_align!(
    //sizeof(*((ecmult_gen_context*) NULL)->prec)
    size_of::<EcMultGenContextPrec>()
);

#[cfg(USE_ECMULT_STATIC_PRECOMPUTATION)]
pub const ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE: usize = 0;

#[cfg(test)]
mod ecmult_gen_constants_validation_suite {
    use super::*;

    #[traced_test]
    fn ecmult_gen_precision_parameters_are_valid_and_self_consistent() {
        tracing::info!(
            prec_bits = ECMULT_GEN_PREC_BITS,
            prec_b = ECMULT_GEN_PREC_B,
            prec_g = ECMULT_GEN_PREC_G,
            prec_n = ECMULT_GEN_PREC_N,
            "validating ecmult-gen precision constants"
        );

        assert!(
            ECMULT_GEN_PREC_BITS == 2 || ECMULT_GEN_PREC_BITS == 4 || ECMULT_GEN_PREC_BITS == 8
        );

        assert_eq!(ECMULT_GEN_PREC_B, ECMULT_GEN_PREC_BITS);
        assert_eq!(ECMULT_GEN_PREC_G, 1usize << ECMULT_GEN_PREC_B);
        assert_eq!(ECMULT_GEN_PREC_N, 256usize / ECMULT_GEN_PREC_B);
        assert_eq!(ECMULT_GEN_PREC_N * ECMULT_GEN_PREC_B, 256usize);

        assert!(ECMULT_GEN_PREC_G.is_power_of_two());
        assert!(ECMULT_GEN_PREC_N > 0);
        assert!(ECMULT_GEN_PREC_G > 0);
    }

    #[traced_test]
    fn ecmult_gen_preallocated_context_size_matches_configuration() {
        tracing::info!(
            preallocated_size = ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE,
            "validating ecmult-gen context preallocation sizing policy"
        );

        #[cfg(not(USE_ECMULT_STATIC_PRECOMPUTATION))]
        {
            let raw_size = core::mem::size_of::<EcMultGenContextPrec>();
            let rounded = round_to_align!(raw_size);
            tracing::debug!(raw_size, rounded, "computed raw and rounded prealloc sizes");

            assert_eq!(ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE, rounded);
            assert!(ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE >= raw_size);
            assert_eq!(ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE, round_to_align!(raw_size));
        }

        #[cfg(USE_ECMULT_STATIC_PRECOMPUTATION)]
        {
            assert_eq!(ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE, 0);
        }
    }
}
