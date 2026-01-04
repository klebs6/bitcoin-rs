// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_shr.rs ]
crate::ix!();

/// Shift a scalar right by some amount strictly
/// between 0 and 16, returning the low bits that
/// were shifted off
/// 
#[cfg(feature="widemul-int128")]
pub fn scalar_shr_int(r: *mut Scalar, n: i32) -> i32 {
    unsafe {
        verify_check!(n > 0);
        verify_check!(n < 16);

        let nn: u32 = n as u32;
        let ret: i32 = ((*r).d[0] & ((1u64 << nn) - 1)) as i32;

        (*r).d[0] = ((*r).d[0] >> nn).wrapping_add((*r).d[1] << (64 - nn));
        (*r).d[1] = ((*r).d[1] >> nn).wrapping_add((*r).d[2] << (64 - nn));
        (*r).d[2] = ((*r).d[2] >> nn).wrapping_add((*r).d[3] << (64 - nn));
        (*r).d[3] = (*r).d[3] >> nn;

        ret
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_shr_int(r: *mut Scalar, n: i32) -> i32 {
    unsafe {
        verify_check!(n > 0);
        verify_check!(n < 16);

        let nn: u32 = n as u32;
        let ret: i32 = ((*r).d[0] & ((1u32 << nn) - 1)) as i32;

        (*r).d[0] = ((*r).d[0] >> nn).wrapping_add((*r).d[1] << (32 - nn));
        (*r).d[1] = ((*r).d[1] >> nn).wrapping_add((*r).d[2] << (32 - nn));
        (*r).d[2] = ((*r).d[2] >> nn).wrapping_add((*r).d[3] << (32 - nn));
        (*r).d[3] = ((*r).d[3] >> nn).wrapping_add((*r).d[4] << (32 - nn));
        (*r).d[4] = ((*r).d[4] >> nn).wrapping_add((*r).d[5] << (32 - nn));
        (*r).d[5] = ((*r).d[5] >> nn).wrapping_add((*r).d[6] << (32 - nn));
        (*r).d[6] = ((*r).d[6] >> nn).wrapping_add((*r).d[7] << (32 - nn));
        (*r).d[7] = (*r).d[7] >> nn;

        ret
    }
}


#[cfg(feature="exhaustive-test-order")]
pub fn scalar_shr_int(r: *mut Scalar, n: i32) -> i32 {
    unsafe {
        verify_check!(n > 0);
        verify_check!(n < 16);
        let nn: u32 = n as u32;
        let ret: i32 = (*r & ((1u32 << nn) - 1)) as i32;
        *r >>= nn;
        ret
    }
}

#[cfg(test)]
mod scalar_shift_right_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_shr_int_shifts_and_returns_low_bits() {
        info!("validating scalar_shr_int semantics for 1..=15");

        let base = scalar_from_be_bytes(&SCALAR_MAX_U32_BE);
        let base_be = scalar_to_be_bytes(&base);

        for n in 1..16 {
            let mut s = scalar_clone_via_b32(&base);
            let ret = unsafe { scalar_shr_int(&mut s as *mut Scalar, n) };

            let got = scalar_to_be_bytes(&s);

            // Reference: interpret as 256-bit, shift right by n, return low bits.
            let mut expected = base_be;
            let mut low_bits: u32 = 0;
            for i in 0..n {
                let bit = (expected[31] >> i) & 1;
                low_bits |= (bit as u32) << i;
            }

            // Shift right by n.
            let mut carry: u8 = 0;
            for i in 0..32 {
                let new_carry = expected[i] & ((1u8 << (n as u8)) - 1);
                expected[i] = (expected[i] >> (n as u8)) | (carry << (8 - n as u8));
                carry = new_carry;
            }

            debug!(n, ret, low_bits, ?got, ?expected, "shr_int case");
            assert_eq!(ret as u32, low_bits);
            assert_eq!(got, expected);
            assert!(scalar_is_normalized_bytes(&got));
        }
    }
}
