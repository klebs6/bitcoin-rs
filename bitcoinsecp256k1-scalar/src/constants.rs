// ---------------- [ File: bitcoinsecp256k1-scalar/src/constants.rs ]
crate::ix!();

pub const SCALAR_ONE:  Scalar = scalar_const!(0, 0, 0, 0, 0, 0, 0, 1);
pub const SCALAR_ZERO: Scalar = scalar_const!(0, 0, 0, 0, 0, 0, 0, 0);

/**
  | The curve has an endomorphism, where
  | lambda * (x, y) = (beta * x, y), where lambda
  | is:
  |
  */
#[cfg(not(feature="exhaustive-test-order"))]
lazy_static! {
    pub static ref const_lambda: Scalar = scalar_const!(
        0x5363AD4C,
        0xC05C30E0,
        0xA5261C02,
        0x8812645A,
        0x122E22EA,
        0x20816678,
        0xDF02967C,
        0x1B23BD72
    );
}

#[cfg(feature="widemul-int128")]
lazy_static! {
    pub static ref const_modinfo_scalar: ModInv64ModInfo = ModInv64ModInfo {
        modulus: ModInv64Signed62 {
            v: [
                0x3FD25E8CD0364141_i64,
                0x2ABB739ABD2280EE_i64,
                -0x15_i64,
                0_i64,
                256_i64
            ]
        },
        modulus_inv62: 0x34F20099AA774EC1_u64,
    };
}

#[cfg(feature="widemul-int64")]
lazy_static! {
    pub static ref const_modinfo_scalar: ModInv32ModInfo = ModInv32ModInfo {
        modulus: ModInv32Signed30 {
            v: [
                0x10364141_i32,
                0x3F497A33_i32,
                0x348A03BB_i32,
                0x2BB739AB_i32,
                -0x146_i32,
                0_i32,
                0_i32,
                0_i32,
                65536_i32
            ]
        },
        modulus_inv30: 0x2A774EC1_u32,
    };
}

#[cfg(test)]
mod scalar_constants_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_one_and_zero_constants_have_expected_encoding_and_predicates() {
        info!("checking SCALAR_ZERO/SCALAR_ONE canonical encodings");

        let z_be = scalar_to_be_bytes(&SCALAR_ZERO);
        let o_be = scalar_to_be_bytes(&SCALAR_ONE);

        debug!(?z_be, "SCALAR_ZERO bytes");
        debug!(?o_be, "SCALAR_ONE bytes");

        assert_eq!(z_be, SCALAR_ZERO_BE);
        assert_eq!(o_be, SCALAR_ONE_BE);

        unsafe {
            assert_eq!(scalar_is_zero(&SCALAR_ZERO as *const Scalar), 1);
            assert_eq!(scalar_is_zero(&SCALAR_ONE as *const Scalar), 0);
            assert_eq!(scalar_is_one(&SCALAR_ONE as *const Scalar), 1);
            assert_eq!(scalar_is_one(&SCALAR_ZERO as *const Scalar), 0);
        }
    }

    #[traced_test]
    #[cfg(not(feature = "exhaustive-test-order"))]
    fn endomorphism_lambda_constant_matches_known_vector() {
        info!("checking const_lambda endomorphism constant against known bytes");
        let lambda = &*const_lambda;
        let lambda_be = scalar_to_be_bytes(lambda);

        debug!(?lambda_be, "const_lambda bytes");
        assert_eq!(lambda_be, SECP256K1_LAMBDA_BE);
        assert!(scalar_is_normalized_bytes(&lambda_be));
    }

    #[traced_test]
    #[cfg(feature = "widemul-int128")]
    fn modinv64_modinfo_scalar_constants_have_expected_inverse_word() {
        info!("checking const_modinfo_scalar (widemul-int128) modulus_inv62");
        let mi = &*const_modinfo_scalar;
        assert_eq!(mi.modulus_inv62, 0x34F20099AA774EC1_u64);
        debug!(modulus_inv62 = mi.modulus_inv62, "modulus inverse");
    }

    #[traced_test]
    #[cfg(feature = "widemul-int64")]
    fn modinv32_modinfo_scalar_constants_have_expected_inverse_word() {
        info!("checking const_modinfo_scalar (widemul-int64) modulus_inv30");
        let mi = &*const_modinfo_scalar;
        assert_eq!(mi.modulus_inv30, 0x2A774EC1_u32);
        debug!(modulus_inv30 = mi.modulus_inv30, "modulus inverse");
    }

}
