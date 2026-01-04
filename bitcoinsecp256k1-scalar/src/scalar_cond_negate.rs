// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_cond_negate.rs ]
crate::ix!();

/// Conditionally negate a number, in constant time.
/// 
/// Returns -1 if the number was negated,
/// 1 otherwise
/// 
#[cfg(feature="widemul-int128")]
pub fn scalar_cond_negate(r: *mut Scalar, flag: i32) -> i32 {
    unsafe {
        /* If we are flag = 0, mask = 00...00 and this is a no-op;
         * if we are flag = 1, mask = 11...11 and this is identical to scalar_negate */
        let mask: u64 = ((flag == 0) as u64).wrapping_sub(1);
        let nonzero: u64 = ((scalar_is_zero(r) != 0) as u64).wrapping_sub(1);

        let mut t: u128 = (((*r).d[0] ^ mask) as u128).wrapping_add(((N_0.wrapping_add(1)) & mask) as u128);
        (*r).d[0] = (t as u64) & nonzero;
        t >>= 64;

        t = t
            .wrapping_add(((*r).d[1] ^ mask) as u128)
            .wrapping_add((N_1 & mask) as u128);
        (*r).d[1] = (t as u64) & nonzero;
        t >>= 64;

        t = t
            .wrapping_add(((*r).d[2] ^ mask) as u128)
            .wrapping_add((N_2 & mask) as u128);
        (*r).d[2] = (t as u64) & nonzero;
        t >>= 64;

        t = t
            .wrapping_add(((*r).d[3] ^ mask) as u128)
            .wrapping_add((N_3 & mask) as u128);
        (*r).d[3] = (t as u64) & nonzero;

        2 * ((mask == 0) as i32) - 1
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_cond_negate(r: *mut Scalar, flag: i32) -> i32 {
    unsafe {
        /* If we are flag = 0, mask = 00...00 and this is a no-op;
         * if we are flag = 1, mask = 11...11 and this is identical to scalar_negate */
        let mask: u32 = ((flag == 0) as u32).wrapping_sub(1);
        let nonzero: u32 = 0xFFFF_FFFFu32.wrapping_mul((scalar_is_zero(r) == 0) as u32);

        let mut t: u64 = (((*r).d[0] ^ mask) as u64).wrapping_add(((N_0.wrapping_add(1)) & mask) as u64);
        (*r).d[0] = (t as u32) & nonzero;
        t >>= 32;

        t = t
            .wrapping_add((((*r).d[1] ^ mask) as u64))
            .wrapping_add((N_1 & mask) as u64);
        (*r).d[1] = (t as u32) & nonzero;
        t >>= 32;

        t = t
            .wrapping_add((((*r).d[2] ^ mask) as u64))
            .wrapping_add((N_2 & mask) as u64);
        (*r).d[2] = (t as u32) & nonzero;
        t >>= 32;

        t = t
            .wrapping_add((((*r).d[3] ^ mask) as u64))
            .wrapping_add((N_3 & mask) as u64);
        (*r).d[3] = (t as u32) & nonzero;
        t >>= 32;

        t = t
            .wrapping_add((((*r).d[4] ^ mask) as u64))
            .wrapping_add((N_4 & mask) as u64);
        (*r).d[4] = (t as u32) & nonzero;
        t >>= 32;

        t = t
            .wrapping_add((((*r).d[5] ^ mask) as u64))
            .wrapping_add((N_5 & mask) as u64);
        (*r).d[5] = (t as u32) & nonzero;
        t >>= 32;

        t = t
            .wrapping_add((((*r).d[6] ^ mask) as u64))
            .wrapping_add((N_6 & mask) as u64);
        (*r).d[6] = (t as u32) & nonzero;
        t >>= 32;

        t = t
            .wrapping_add((((*r).d[7] ^ mask) as u64))
            .wrapping_add((N_7 & mask) as u64);
        (*r).d[7] = (t as u32) & nonzero;

        2 * ((mask == 0) as i32) - 1
    }
}

#[cfg(feature="exhaustive-test-order")]
pub fn scalar_cond_negate(r: *mut Scalar, flag: i32) -> i32 {
    unsafe {
        if flag != 0 {
            scalar_negate(r, r);
        }
        if flag != 0 { -1 } else { 1 }
    }
}

#[cfg(test)]
mod scalar_conditional_negation_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_cond_negate_flag_controls_negation_and_return_value() {
        info!("validating scalar_cond_negate flag semantics");

        for (i, a_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let a = scalar_from_be_bytes(a_be);

            let mut r0 = scalar_clone_via_b32(&a);
            let ret0 = unsafe { scalar_cond_negate(&mut r0 as *mut Scalar, 0) };
            let r0_be = scalar_to_be_bytes(&r0);

            debug!(i, ret0, ?r0_be, "flag=0 case");
            assert_eq!(ret0, 1);
            assert_eq!(r0_be, *a_be);

            let mut r1 = scalar_clone_via_b32(&a);
            let ret1 = unsafe { scalar_cond_negate(&mut r1 as *mut Scalar, 1) };
            let r1_be = scalar_to_be_bytes(&r1);
            let expected = be_neg_mod_n(a_be);

            debug!(i, ret1, ?r1_be, ?expected, "flag=1 case");
            assert_eq!(ret1, -1);
            assert_eq!(r1_be, expected);
        }
    }
}
