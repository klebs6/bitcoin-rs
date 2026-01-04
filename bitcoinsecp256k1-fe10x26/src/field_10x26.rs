// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/field_10x26.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/field_10x26.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/field_10x26_impl.h]

pub struct Fe10x26 {

    /**
      | X = sum(i=0..9, n[i]*2^(i*26)) mod
      | p where p = 2^256 - 0x1000003D1
      |
      */
    pub n:          [u32; 10],

    #[cfg(feature="secp256k1-verify")]
    pub magnitude:  i32,

    #[cfg(feature="secp256k1-verify")]
    pub normalized: i32,
}

impl Fe10x26 {

    pub const fn new() -> Self {
        Self {
            n: [0; 10],
            #[cfg(feature="secp256k1-verify")] magnitude:  0,
            #[cfg(feature="secp256k1-verify")] normalized: 0,
        }
    }
}

#[cfg(test)]
mod field_10x26_type_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn fe10x26_new_is_all_zero_limbs() {
        info!("Fe10x26::new should initialize n to zeros");
        let a = Fe10x26::new();
        debug!(?a.n, "new() limbs");
        assert!(a.n.iter().all(|&x| x == 0));

        #[cfg(feature = "secp256k1-verify")]
        {
            assert_eq!(a.magnitude, 0);
            assert_eq!(a.normalized, 0);
        }
    }

    #[traced_test]
    fn fe10x26_works_with_basic_set_and_get_pipeline() {
        info!("Fe10x26 values should work with fe_set_int + fe_get_b32 after normalization");
        let mut a = Fe10x26::new();
        unsafe { fe_set_int(&mut a as *mut Fe10x26, 1) };

        let out = fe_to_be_bytes_normalized(&mut a);
        assert_eq!(out, BYTES_ONE);
    }
}
