// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_add.rs ]
crate::ix!();

/// Add two scalars together (modulo the
/// group order). Returns whether it overflowed.
/// 
#[cfg(feature="widemul-int128")]
pub fn scalar_add(r: *mut Scalar, a: *const Scalar, b: *const Scalar) -> i32 {
    unsafe {
        let mut t: u128 = ((*a).d[0] as u128).wrapping_add((*b).d[0] as u128);
        (*r).d[0] = (t & 0xFFFF_FFFF_FFFF_FFFFu128) as u64;
        t >>= 64;

        t = t
            .wrapping_add((*a).d[1] as u128)
            .wrapping_add((*b).d[1] as u128);
        (*r).d[1] = (t & 0xFFFF_FFFF_FFFF_FFFFu128) as u64;
        t >>= 64;

        t = t
            .wrapping_add((*a).d[2] as u128)
            .wrapping_add((*b).d[2] as u128);
        (*r).d[2] = (t & 0xFFFF_FFFF_FFFF_FFFFu128) as u64;
        t >>= 64;

        t = t
            .wrapping_add((*a).d[3] as u128)
            .wrapping_add((*b).d[3] as u128);
        (*r).d[3] = (t & 0xFFFF_FFFF_FFFF_FFFFu128) as u64;
        t >>= 64;

        let overflow: u32 = (t as u32).wrapping_add(scalar_check_overflow(r) as u32);
        verify_check!((overflow == 0) || (overflow == 1));
        scalar_reduce(r, overflow);
        overflow as i32
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_add(r: *mut Scalar, a: *const Scalar, b: *const Scalar) -> i32 {
    unsafe {
        let mut t: u64 = ((*a).d[0] as u64).wrapping_add((*b).d[0] as u64);
        (*r).d[0] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t
            .wrapping_add((*a).d[1] as u64)
            .wrapping_add((*b).d[1] as u64);
        (*r).d[1] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t
            .wrapping_add((*a).d[2] as u64)
            .wrapping_add((*b).d[2] as u64);
        (*r).d[2] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t
            .wrapping_add((*a).d[3] as u64)
            .wrapping_add((*b).d[3] as u64);
        (*r).d[3] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t
            .wrapping_add((*a).d[4] as u64)
            .wrapping_add((*b).d[4] as u64);
        (*r).d[4] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t
            .wrapping_add((*a).d[5] as u64)
            .wrapping_add((*b).d[5] as u64);
        (*r).d[5] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t
            .wrapping_add((*a).d[6] as u64)
            .wrapping_add((*b).d[6] as u64);
        (*r).d[6] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t
            .wrapping_add((*a).d[7] as u64)
            .wrapping_add((*b).d[7] as u64);
        (*r).d[7] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        let overflow: u32 = (t as u32).wrapping_add(scalar_check_overflow(r) as u32);
        verify_check!((overflow == 0) || (overflow == 1));
        scalar_reduce(r, overflow);
        overflow as i32
    }
}

#[cfg(feature="exhaustive-test-order")]
pub fn scalar_add(r: *mut Scalar, a: *const Scalar, b: *const Scalar) -> i32 {
    unsafe {
        *r = (*a + *b) % EXHAUSTIVE_TEST_ORDER;
        (*r < *b) as i32
    }
}

#[cfg(test)]
mod scalar_addition_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn scalar_add_matches_reference_modular_addition_on_canonical_vectors() {
        info!("validating scalar_add against reference mod-n addition");

        for (i, a_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let a = scalar_from_be_bytes(a_be);
            for (j, b_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
                let b = scalar_from_be_bytes(b_be);

                let mut r = scalar_zero_value();
                let overflow = unsafe {
                    scalar_add(
                        &mut r as *mut Scalar,
                        &a as *const Scalar,
                        &b as *const Scalar,
                    )
                };

                let got = scalar_to_be_bytes(&r);
                let expected = be_add_mod_n(a_be, b_be);
                let expected_overflow = be_add_overflow_flag(a_be, b_be) as i32;

                trace!(i, j, overflow, expected_overflow, ?got, ?expected, "scalar_add case");
                assert_eq!(got, expected);
                assert_eq!(overflow, expected_overflow);
                assert!(scalar_is_normalized_bytes(&got));
            }
        }
    }

    #[traced_test]
    fn scalar_add_wraps_order_minus_one_plus_one_to_zero() {
        info!("validating (n-1)+1 == 0");

        let a = scalar_from_be_bytes(&SECP256K1_ORDER_MINUS_1_BE);
        let b = scalar_from_u32(1);

        let mut r = scalar_zero_value();
        let overflow = unsafe {
            scalar_add(
                &mut r as *mut Scalar,
                &a as *const Scalar,
                &b as *const Scalar,
            )
        };
        let got = scalar_to_be_bytes(&r);

        debug!(overflow, ?got, "n-1 + 1");
        assert_eq!(overflow, 1);
        assert_eq!(got, SCALAR_ZERO_BE);
    }
}
