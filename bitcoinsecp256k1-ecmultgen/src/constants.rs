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
