// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_cadd_bit.rs ]
crate::ix!();

/// Conditionally add a power of two to a scalar. 
///
/// The result is not allowed to overflow.
/// 
#[cfg(feature="widemul-int128")]
pub fn scalar_cadd_bit(r: *mut Scalar, bit: u32, flag: i32) {
    unsafe {
        let mut bit: u32 = bit;
        let mut t: u128;

        verify_check!(bit < 256);
        bit = bit.wrapping_add(((flag as u32).wrapping_sub(1)) & 0x100); /* forcing (bit >> 6) > 3 makes this a noop */

        t = ((*r).d[0] as u128)
            .wrapping_add((((((bit >> 6) == 0) as u64) as u128) << (bit & 0x3F)));
        (*r).d[0] = (t & 0xFFFF_FFFF_FFFF_FFFFu128) as u64;
        t >>= 64;

        t = t.wrapping_add((*r).d[1] as u128)
            .wrapping_add((((((bit >> 6) == 1) as u64) as u128) << (bit & 0x3F)));
        (*r).d[1] = (t & 0xFFFF_FFFF_FFFF_FFFFu128) as u64;
        t >>= 64;

        t = t.wrapping_add((*r).d[2] as u128)
            .wrapping_add((((((bit >> 6) == 2) as u64) as u128) << (bit & 0x3F)));
        (*r).d[2] = (t & 0xFFFF_FFFF_FFFF_FFFFu128) as u64;
        t >>= 64;

        t = t.wrapping_add((*r).d[3] as u128)
            .wrapping_add((((((bit >> 6) == 3) as u64) as u128) << (bit & 0x3F)));
        (*r).d[3] = (t & 0xFFFF_FFFF_FFFF_FFFFu128) as u64;

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!((t >> 64) == 0);
            verify_check!(scalar_check_overflow(r) == 0);
        }
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_cadd_bit(r: *mut Scalar, bit: u32, flag: i32) {
    unsafe {
        let mut bit: u32 = bit;
        let mut t: u64;

        verify_check!(bit < 256);
        bit = bit.wrapping_add(((flag as u32).wrapping_sub(1)) & 0x100); /* forcing (bit >> 5) > 7 makes this a noop */

        t = ((*r).d[0] as u64).wrapping_add((((bit >> 5) == 0) as u32 as u64) << (bit & 0x1F));
        (*r).d[0] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t.wrapping_add((*r).d[1] as u64)
            .wrapping_add((((bit >> 5) == 1) as u32 as u64) << (bit & 0x1F));
        (*r).d[1] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t.wrapping_add((*r).d[2] as u64)
            .wrapping_add((((bit >> 5) == 2) as u32 as u64) << (bit & 0x1F));
        (*r).d[2] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t.wrapping_add((*r).d[3] as u64)
            .wrapping_add((((bit >> 5) == 3) as u32 as u64) << (bit & 0x1F));
        (*r).d[3] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t.wrapping_add((*r).d[4] as u64)
            .wrapping_add((((bit >> 5) == 4) as u32 as u64) << (bit & 0x1F));
        (*r).d[4] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t.wrapping_add((*r).d[5] as u64)
            .wrapping_add((((bit >> 5) == 5) as u32 as u64) << (bit & 0x1F));
        (*r).d[5] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t.wrapping_add((*r).d[6] as u64)
            .wrapping_add((((bit >> 5) == 6) as u32 as u64) << (bit & 0x1F));
        (*r).d[6] = (t & 0xFFFF_FFFFu64) as u32;
        t >>= 32;

        t = t.wrapping_add((*r).d[7] as u64)
            .wrapping_add((((bit >> 5) == 7) as u32 as u64) << (bit & 0x1F));
        (*r).d[7] = (t & 0xFFFF_FFFFu64) as u32;

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!((t >> 32) == 0);
            verify_check!(scalar_check_overflow(r) == 0);
        }
    }
}

#[cfg(feature="exhaustive-test-order")]
pub fn scalar_cadd_bit(r: *mut Scalar, bit: u32, flag: i32) {
    unsafe {
        if flag != 0 && bit < 32 {
            *r = r.wrapping_add(1u32 << bit);
        }
        #[cfg(VERIFY)]
        {
            verify_check!(bit < 32);
            /* Verify that adding (1 << bit) will not overflow any in-range scalar *r by overflowing the underlying uint32_t. */
            verify_check!(((1u32 << bit).wrapping_sub(1)) <= (u32::MAX - EXHAUSTIVE_TEST_ORDER));
            verify_check!(scalar_check_overflow(r) == 0);
        }
    }
}

#[cfg(test)]
mod scalar_cadd_bit_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_cadd_bit_adds_power_of_two_when_flag_is_set() {
        info!("validating scalar_cadd_bit adds 2^bit when flag!=0");

        let mut s = scalar_zero_value();
        unsafe {
            scalar_cadd_bit(&mut s as *mut Scalar, 0, 1);
        }
        let s1 = scalar_to_be_bytes(&s);
        debug!(?s1, "after +2^0");
        assert_eq!(s1, SCALAR_ONE_BE);

        unsafe {
            scalar_cadd_bit(&mut s as *mut Scalar, 0, 1);
        }
        let s2 = scalar_to_be_bytes(&s);
        debug!(?s2, "after +2^0 again");
        assert_eq!(s2, SCALAR_TWO_BE);

        let mut hi = scalar_zero_value();
        unsafe {
            scalar_cadd_bit(&mut hi as *mut Scalar, 255, 1);
        }
        let hi_be = scalar_to_be_bytes(&hi);
        let expected_hi = be_set_bit_256(255);
        debug!(?hi_be, ?expected_hi, "after +2^255");
        assert_eq!(hi_be, expected_hi);
        assert!(scalar_is_normalized_bytes(&hi_be));

        let mut unchanged = scalar_from_u32(7);
        unsafe {
            scalar_cadd_bit(&mut unchanged as *mut Scalar, 5, 0);
        }
        let unchanged_be = scalar_to_be_bytes(&unchanged);
        debug!(?unchanged_be, "flag=0 no change");
        assert_eq!(
            unchanged_be,
            [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 7
            ]
        );
    }

    #[traced_test]
    #[cfg(feature = "secp256k1-verify")]
    fn scalar_cadd_bit_rejects_out_of_range_bit_and_overflowing_additions_under_verify() {
        info!("validating scalar_cadd_bit verify-time preconditions");

        let invalid_bit_panicked = std::panic::catch_unwind(|| {
            let mut s = scalar_zero_value();
            unsafe {
                scalar_cadd_bit(&mut s as *mut Scalar, 256, 1);
            }
        })
        .is_err();
        assert!(invalid_bit_panicked);

        let overflow_panicked = std::panic::catch_unwind(|| {
            let mut s = scalar_zero_value();
            unsafe {
                scalar_cadd_bit(&mut s as *mut Scalar, 255, 1);
                scalar_cadd_bit(&mut s as *mut Scalar, 255, 1);
            }
        })
        .is_err();
        assert!(overflow_panicked);
    }
}
