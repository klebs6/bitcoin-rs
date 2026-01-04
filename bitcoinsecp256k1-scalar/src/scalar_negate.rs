// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_negate.rs ]
crate::ix!();

/// Compute the complement of a scalar (modulo the group order).
/// 
#[cfg(feature="widemul-int128")]
pub fn scalar_negate(r: *mut Scalar, a: *const Scalar) {
    unsafe {
        let nonzero: u64 = 0xFFFF_FFFF_FFFF_FFFFu64.wrapping_mul((scalar_is_zero(a) == 0) as u64);

        let mut t: u128 = ((!(*a).d[0]) as u128).wrapping_add(N_0 as u128).wrapping_add(1u128);
        (*r).d[0] = (t as u64) & nonzero;
        t >>= 64;

        t = t.wrapping_add((!(*a).d[1]) as u128).wrapping_add(N_1 as u128);
        (*r).d[1] = (t as u64) & nonzero;
        t >>= 64;

        t = t.wrapping_add((!(*a).d[2]) as u128).wrapping_add(N_2 as u128);
        (*r).d[2] = (t as u64) & nonzero;
        t >>= 64;

        t = t.wrapping_add((!(*a).d[3]) as u128).wrapping_add(N_3 as u128);
        (*r).d[3] = (t as u64) & nonzero;
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_negate(r: *mut Scalar, a: *const Scalar) {
    unsafe {
        let nonzero: u32 = 0xFFFF_FFFFu32.wrapping_mul((scalar_is_zero(a) == 0) as u32);

        let mut t: u64 = ((!(*a).d[0]) as u64).wrapping_add(N_0 as u64).wrapping_add(1u64);
        (*r).d[0] = (t as u32) & nonzero;
        t >>= 32;

        t = t.wrapping_add((!(*a).d[1]) as u64).wrapping_add(N_1 as u64);
        (*r).d[1] = (t as u32) & nonzero;
        t >>= 32;

        t = t.wrapping_add((!(*a).d[2]) as u64).wrapping_add(N_2 as u64);
        (*r).d[2] = (t as u32) & nonzero;
        t >>= 32;

        t = t.wrapping_add((!(*a).d[3]) as u64).wrapping_add(N_3 as u64);
        (*r).d[3] = (t as u32) & nonzero;
        t >>= 32;

        t = t.wrapping_add((!(*a).d[4]) as u64).wrapping_add(N_4 as u64);
        (*r).d[4] = (t as u32) & nonzero;
        t >>= 32;

        t = t.wrapping_add((!(*a).d[5]) as u64).wrapping_add(N_5 as u64);
        (*r).d[5] = (t as u32) & nonzero;
        t >>= 32;

        t = t.wrapping_add((!(*a).d[6]) as u64).wrapping_add(N_6 as u64);
        (*r).d[6] = (t as u32) & nonzero;
        t >>= 32;

        t = t.wrapping_add((!(*a).d[7]) as u64).wrapping_add(N_7 as u64);
        (*r).d[7] = (t as u32) & nonzero;
    }
}

#[cfg(feature="exhaustive-test-order")]
pub fn scalar_negate(r: *mut Scalar, a: *const Scalar) {
    unsafe {
        if *a == 0 {
            *r = 0;
        } else {
            *r = EXHAUSTIVE_TEST_ORDER - *a;
        }
    }
}

#[cfg(test)]
mod scalar_negation_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn scalar_negate_matches_reference_modular_negation() {
        info!("validating scalar_negate against reference mod-n negation");

        for (i, a_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let a = scalar_from_be_bytes(a_be);

            let mut r = scalar_zero_value();
            unsafe {
                scalar_negate(&mut r as *mut Scalar, &a as *const Scalar);
            }
            let got = scalar_to_be_bytes(&r);
            let expected = be_neg_mod_n(a_be);

            trace!(i, ?got, ?expected, "negation case");
            assert_eq!(got, expected);
            assert!(scalar_is_normalized_bytes(&got));
        }
    }

    #[traced_test]
    fn scalar_plus_negation_is_zero_for_nonzero_inputs() {
        info!("validating a + (-a) == 0 for nonzero canonical vectors");

        for (i, a_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            if be_is_zero_32(a_be) {
                continue;
            }
            let a = scalar_from_be_bytes(a_be);

            let mut neg = scalar_zero_value();
            unsafe {
                scalar_negate(&mut neg as *mut Scalar, &a as *const Scalar);
            }

            let mut sum = scalar_zero_value();
            unsafe {
                let _ov = scalar_add(
                    &mut sum as *mut Scalar,
                    &a as *const Scalar,
                    &neg as *const Scalar,
                );
            }

            let sum_be = scalar_to_be_bytes(&sum);
            debug!(i, ?sum_be, "a + (-a)");
            assert_eq!(sum_be, SCALAR_ZERO_BE);
        }
    }
}
