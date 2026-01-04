// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_inverse.rs ]
crate::ix!();

/// Compute the inverse of a scalar (modulo the
/// group order).
/// 
#[cfg(feature="widemul-int128")]
pub fn scalar_inverse(r: *mut Scalar, x: *const Scalar) {
    unsafe {
        let mut s: ModInv64Signed62 = core::mem::zeroed();

        #[cfg(feature="secp256k1-verify")]
        let zero_in: i32 = scalar_is_zero(x);

        scalar_to_signed62(&mut s, x);
        modinv64(&mut s, &*const_modinfo_scalar);
        scalar_from_signed62(r, &s);

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!(scalar_is_zero(r) == zero_in);
        }
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_inverse(r: *mut Scalar, x: *const Scalar) {
    unsafe {
        let mut s: ModInv32Signed30 = core::mem::zeroed();

        #[cfg(feature="secp256k1-verify")]
        let zero_in: i32 = scalar_is_zero(x);

        scalar_to_signed30(&mut s, x);
        modinv32(&mut s, &*const_modinfo_scalar);
        scalar_from_signed30(r, &s);

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!(scalar_is_zero(r) == zero_in);
        }
    }
}

#[cfg(feature="exhaustive-test-order")]
pub fn scalar_inverse(r: *mut Scalar, x: *const Scalar) {
    unsafe {
        *r = 0;
        for i in 0..(EXHAUSTIVE_TEST_ORDER as u32) {
            if (i.wrapping_mul(*x)) % EXHAUSTIVE_TEST_ORDER == 1 {
                *r = i;
            }
        }
        /* If this verify_check triggers we were given a noninvertible scalar (and thus
         * have a composite group order; fix it in exhaustive_tests.c). */
        verify_check!(*r != 0);
    }
}

#[cfg(test)]
mod scalar_inverse_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_inverse_zero_maps_to_zero_and_nonzero_inverts_to_one() {
        info!("validating scalar_inverse for 0 and representative nonzero values");

        let zero = scalar_from_u32(0);

        let mut inv0 = scalar_zero_value();
        unsafe {
            scalar_inverse(&mut inv0 as *mut Scalar, &zero as *const Scalar);
        }
        assert_eq!(scalar_to_be_bytes(&inv0), SCALAR_ZERO_BE);

        let nonzero_vectors: &[[u8; 32]] = &[
            SCALAR_ONE_BE,
            SCALAR_TWO_BE,
            SCALAR_THREE_BE,
            SCALAR_MAX_U32_BE,
            SECP256K1_ORDER_HALF_BE,
            SECP256K1_ORDER_MINUS_1_BE,
        ];

        for (i, a_be) in nonzero_vectors.iter().enumerate() {
            let a = scalar_from_be_bytes(a_be);

            let mut inv = scalar_zero_value();
            unsafe {
                scalar_inverse(&mut inv as *mut Scalar, &a as *const Scalar);
            }

            let mut prod = scalar_zero_value();
            unsafe {
                scalar_mul(
                    &mut prod as *mut Scalar,
                    &a as *const Scalar,
                    &inv as *const Scalar,
                );
            }

            let prod_be = scalar_to_be_bytes(&prod);
            debug!(i, ?a_be, inv_be = ?scalar_to_be_bytes(&inv), ?prod_be, "a * inv(a)");
            assert_eq!(prod_be, SCALAR_ONE_BE);
        }
    }
}
