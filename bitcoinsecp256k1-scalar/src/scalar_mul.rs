// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_mul.rs ]
crate::ix!();

/**
  | Multiply two scalars (modulo the group
  | order).
  |
  */
#[cfg(feature="widemul-int128")]
pub fn scalar_mul(r: *mut Scalar, a: *const Scalar, b: *const Scalar) {
    unsafe {
        let mut l: [u64; 8] = [0u64; 8];
        scalar_mul_512(l.as_mut_ptr(), a, b);
        scalar_reduce_512(r, l.as_ptr());
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_mul(r: *mut Scalar, a: *const Scalar, b: *const Scalar) {
    unsafe {
        let mut l: [u32; 16] = [0u32; 16];
        scalar_mul_512(l.as_mut_ptr(), a, b);
        scalar_reduce_512(r, l.as_ptr());
    }
}

#[cfg(feature="exhaustive-test-order")]
pub fn scalar_mul(r: *mut Scalar, a: *const Scalar, b: *const Scalar) {
    unsafe {
        *r = (*a * *b) % EXHAUSTIVE_TEST_ORDER;
    }
}

#[cfg(test)]
mod scalar_multiplication_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn scalar_mul_matches_reference_modular_multiplication_on_canonical_vectors() {
        info!("validating scalar_mul against reference mod-n multiplication");

        for (i, a_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let a = scalar_from_be_bytes(a_be);
            for (j, b_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
                let b = scalar_from_be_bytes(b_be);

                let mut r = scalar_zero_value();
                unsafe {
                    scalar_mul(
                        &mut r as *mut Scalar,
                        &a as *const Scalar,
                        &b as *const Scalar,
                    );
                }
                let got = scalar_to_be_bytes(&r);
                let expected = be_mul_mod_n(a_be, b_be);

                trace!(i, j, ?got, ?expected, "mul case");
                assert_eq!(got, expected);
                assert!(scalar_is_normalized_bytes(&got));

                // Commutativity sanity check: a*b == b*a.
                let mut r2 = scalar_zero_value();
                unsafe {
                    scalar_mul(
                        &mut r2 as *mut Scalar,
                        &b as *const Scalar,
                        &a as *const Scalar,
                    );
                }
                assert_eq!(scalar_to_be_bytes(&r2), got);
            }
        }

        debug!("scalar_mul reference match completed");
    }

    #[traced_test]
    fn scalar_mul_has_expected_identities() {
        info!("validating scalar_mul identities: a*0=0, a*1=a");

        let zero = scalar_from_u32(0);
        let one = scalar_from_u32(1);

        for (i, a_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let a = scalar_from_be_bytes(a_be);

            let mut r0 = scalar_zero_value();
            unsafe {
                scalar_mul(
                    &mut r0 as *mut Scalar,
                    &a as *const Scalar,
                    &zero as *const Scalar,
                );
            }
            assert_eq!(scalar_to_be_bytes(&r0), SCALAR_ZERO_BE);

            let mut r1 = scalar_zero_value();
            unsafe {
                scalar_mul(
                    &mut r1 as *mut Scalar,
                    &a as *const Scalar,
                    &one as *const Scalar,
                );
            }
            let got = scalar_to_be_bytes(&r1);
            debug!(i, ?got, "a*1");
            assert_eq!(got, *a_be);
        }
    }
}
