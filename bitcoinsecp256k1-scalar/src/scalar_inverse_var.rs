// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_inverse_var.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar_low.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar_low_impl.h]
/// Compute the inverse of a scalar (modulo the
/// group order), without constant-time guarantee.
/// 
#[cfg(feature="widemul-int128")]
pub fn scalar_inverse_var(r: *mut Scalar, x: *const Scalar) {
    unsafe {
        let mut s: ModInv64Signed62 = core::mem::zeroed();

        #[cfg(feature="secp256k1-verify")]
        let zero_in: i32 = scalar_is_zero(x);

        scalar_to_signed62(&mut s, x);
        modinv64_var(&mut s, &*const_modinfo_scalar);
        scalar_from_signed62(r, &s);

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!(scalar_is_zero(r) == zero_in);
        }
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_inverse_var(r: *mut Scalar, x: *const Scalar) {
    unsafe {
        let mut s: ModInv32Signed30 = core::mem::zeroed();

        #[cfg(feature="secp256k1-verify")]
        let zero_in: i32 = scalar_is_zero(x);

        scalar_to_signed30(&mut s, x);
        modinv32_var(&mut s, &*const_modinfo_scalar);
        scalar_from_signed30(r, &s);

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!(scalar_is_zero(r) == zero_in);
        }
    }
}

#[cfg(feature="exhaustive-test-order")]
pub fn scalar_inverse_var(r: *mut Scalar, x: *const Scalar) {
    scalar_inverse(r, x);
}

#[cfg(test)]
mod scalar_inverse_var_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_inverse_var_agrees_with_scalar_inverse_on_representative_values() {
        info!("validating scalar_inverse_var agrees with scalar_inverse");

        let vectors: &[[u8; 32]] = &[
            SCALAR_ZERO_BE,
            SCALAR_ONE_BE,
            SCALAR_TWO_BE,
            SCALAR_THREE_BE,
            SCALAR_MAX_U32_BE,
            SECP256K1_ORDER_HALF_BE,
            SECP256K1_ORDER_MINUS_1_BE,
        ];

        for (i, a_be) in vectors.iter().enumerate() {
            let a = scalar_from_be_bytes(a_be);

            let mut inv_ct = scalar_zero_value();
            unsafe {
                scalar_inverse(&mut inv_ct as *mut Scalar, &a as *const Scalar);
            }

            let mut inv_var = scalar_zero_value();
            unsafe {
                scalar_inverse_var(&mut inv_var as *mut Scalar, &a as *const Scalar);
            }

            let ct_be = scalar_to_be_bytes(&inv_ct);
            let var_be = scalar_to_be_bytes(&inv_var);

            debug!(i, ?a_be, ?ct_be, ?var_be, "inverse vs inverse_var");
            assert_eq!(ct_be, var_be);
        }
    }
}
