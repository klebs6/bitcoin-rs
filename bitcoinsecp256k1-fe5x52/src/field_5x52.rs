// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/field_5x52.rs ]
/*!
  | Implements arithmetic modulo FFFFFFFF FFFFFFFF
  |  FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE
  |  FFFFFC2F, represented as 5 uint64_t's in base
  |  2^52. 
  |
  |  The values are allowed to contain >52 each. In
  |  particular, each FieldElem has a 'magnitude'
  |  associated with it. 
  |
  |  Internally, a magnitude M means each element
  |  is at most M*(2^53-1), except the most
  |  significant one, which is limited to
  |  M*(2^49-1). 
  |
  |  All operations accept any input with magnitude
  |  at most M, and have different rules for
  |  propagating magnitude to their output.
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/field_5x52.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/field_5x52_impl.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/field_5x52_asm_impl.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/field_5x52_int128_impl.h]

#[derive(Copy,Clone,Debug)]
pub struct Fe5x52 {

    /**
      | X = sum(i=0..4, n[i]*2^(i*52)) mod
      | p where p = 2^256 - 0x1000003D1
      |
      */
    pub n:          [u64; 5],

    #[cfg(feature="secp256k1-verify")]
    pub magnitude:  i32,

    #[cfg(feature="secp256k1-verify")]
    pub normalized: i32,
}

impl Fe5x52 {

    pub const fn new() -> Self {
        Self {
            n: [0; 5],
            #[cfg(feature="secp256k1-verify")] magnitude:  0,
            #[cfg(feature="secp256k1-verify")] normalized: 0,
        }
    }
}

#[cfg(test)]
mod field_5x52_rs_exhaustive_tests {
    use super::*;

    #[traced_test]
    fn fe5x52_new_constructs_all_zero_limbs_and_verify_metadata_defaults() {
        tracing::info!("testing Fe5x52::new default construction");

        let fe = Fe5x52::new();
        assert_eq!(fe.n, [0u64; 5]);

        #[cfg(feature = "secp256k1-verify")]
        {
            assert_eq!(fe.magnitude, 0);
            assert_eq!(fe.normalized, 0);
        }
    }
}
