// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_reduce.rs ]
crate::ix!();

#[cfg(feature="widemul-int128")]
#[inline]
pub fn scalar_reduce(r: *mut Scalar, overflow: u32) -> i32 {
    unsafe {
        let mut t: u128;

        verify_check!(overflow <= 1);

        t = ((*r).d[0] as u128).wrapping_add((overflow as u128).wrapping_mul(N_C_0 as u128));
        (*r).d[0] = (t & 0xFFFF_FFFF_FFFF_FFFFu128) as u64;
        t >>= 64;

        t = t
            .wrapping_add((*r).d[1] as u128)
            .wrapping_add((overflow as u128).wrapping_mul(N_C_1 as u128));
        (*r).d[1] = (t & 0xFFFF_FFFF_FFFF_FFFFu128) as u64;
        t >>= 64;

        t = t
            .wrapping_add((*r).d[2] as u128)
            .wrapping_add((overflow as u128).wrapping_mul(N_C_2 as u128));
        (*r).d[2] = (t & 0xFFFF_FFFF_FFFF_FFFFu128) as u64;
        t >>= 64;

        t = t.wrapping_add((*r).d[3] as u128);
        (*r).d[3] = (t & 0xFFFF_FFFF_FFFF_FFFFu128) as u64;

        overflow as i32
    }
}

#[cfg(feature="widemul-int64")]
#[inline]
pub fn scalar_reduce(r: *mut Scalar, overflow: u32) -> i32 {
    unsafe {
        let mut t: u64;

        verify_check!(overflow <= 1);

        t = ((*r).d[0] as u64).wrapping_add((overflow as u64).wrapping_mul(N_C_0 as u64));
        (*r).d[0] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t
            .wrapping_add((*r).d[1] as u64)
            .wrapping_add((overflow as u64).wrapping_mul(N_C_1 as u64));
        (*r).d[1] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t
            .wrapping_add((*r).d[2] as u64)
            .wrapping_add((overflow as u64).wrapping_mul(N_C_2 as u64));
        (*r).d[2] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t
            .wrapping_add((*r).d[3] as u64)
            .wrapping_add((overflow as u64).wrapping_mul(N_C_3 as u64));
        (*r).d[3] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t
            .wrapping_add((*r).d[4] as u64)
            .wrapping_add((overflow as u64).wrapping_mul(N_C_4 as u64));
        (*r).d[4] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t.wrapping_add((*r).d[5] as u64);
        (*r).d[5] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t.wrapping_add((*r).d[6] as u64);
        (*r).d[6] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t.wrapping_add((*r).d[7] as u64);
        (*r).d[7] = (t & 0xFFFF_FFFFu64) as u32;

        overflow as i32
    }
}

#[cfg(test)]
mod scalar_reduce_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_reduce_subtracts_group_order_when_overflow_is_one() {
        info!("validating scalar_reduce subtracts n when overflow==1");

        let mut r: Scalar = scalar_const!(
            0xFFFFFFFF,
            0xFFFFFFFF,
            0xFFFFFFFF,
            0xFFFFFFFE,
            0xBAAEDCE6,
            0xAF48A03B,
            0xBFD25E8C,
            0xD0364141
        );

        let ret = unsafe { scalar_reduce(&mut r as *mut Scalar, 1) };
        let r_be = scalar_to_be_bytes(&r);

        debug!(ret, ?r_be, "scalar_reduce result");
        assert_eq!(ret, 1);
        assert_eq!(r_be, SCALAR_ZERO_BE);
    }

    #[traced_test]
    fn scalar_reduce_is_identity_when_overflow_is_zero() {
        info!("validating scalar_reduce is identity when overflow==0");

        for (idx, a_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let mut a = scalar_from_be_bytes(a_be);
            let before = scalar_to_be_bytes(&a);
            let ret = unsafe { scalar_reduce(&mut a as *mut Scalar, 0) };
            let after = scalar_to_be_bytes(&a);

            debug!(idx, ret, ?before, ?after, "scalar_reduce(identity)");
            assert_eq!(ret, 0);
            assert_eq!(before, after);
            assert!(scalar_is_normalized_bytes(&after));
        }
    }
}
