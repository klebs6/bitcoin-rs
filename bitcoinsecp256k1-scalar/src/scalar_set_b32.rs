// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_set_b32.rs ]
crate::ix!();

/// Set a scalar from a big endian byte array. The
/// scalar will be reduced modulo group order `n`.
/// 
/// In:      bin:        pointer to a 32-byte array.
/// 
/// Out:     r:          scalar to be set.
/// 
///          overflow:   non-zero if the scalar was
///          bigger or equal to `n` before
///          reduction, zero otherwise (can be
///          NULL).
///
#[cfg(feature="widemul-int128")]
pub fn scalar_set_b32(r: *mut Scalar, b32: *const u8, overflow: *mut i32) {
    unsafe {
        let mut over: i32;

        (*r).d[0] = (*b32.add(31) as u64)
            | ((*b32.add(30) as u64) << 8)
            | ((*b32.add(29) as u64) << 16)
            | ((*b32.add(28) as u64) << 24)
            | ((*b32.add(27) as u64) << 32)
            | ((*b32.add(26) as u64) << 40)
            | ((*b32.add(25) as u64) << 48)
            | ((*b32.add(24) as u64) << 56);

        (*r).d[1] = (*b32.add(23) as u64)
            | ((*b32.add(22) as u64) << 8)
            | ((*b32.add(21) as u64) << 16)
            | ((*b32.add(20) as u64) << 24)
            | ((*b32.add(19) as u64) << 32)
            | ((*b32.add(18) as u64) << 40)
            | ((*b32.add(17) as u64) << 48)
            | ((*b32.add(16) as u64) << 56);

        (*r).d[2] = (*b32.add(15) as u64)
            | ((*b32.add(14) as u64) << 8)
            | ((*b32.add(13) as u64) << 16)
            | ((*b32.add(12) as u64) << 24)
            | ((*b32.add(11) as u64) << 32)
            | ((*b32.add(10) as u64) << 40)
            | ((*b32.add(9) as u64) << 48)
            | ((*b32.add(8) as u64) << 56);

        (*r).d[3] = (*b32.add(7) as u64)
            | ((*b32.add(6) as u64) << 8)
            | ((*b32.add(5) as u64) << 16)
            | ((*b32.add(4) as u64) << 24)
            | ((*b32.add(3) as u64) << 32)
            | ((*b32.add(2) as u64) << 40)
            | ((*b32.add(1) as u64) << 48)
            | ((*b32.add(0) as u64) << 56);

        over = scalar_reduce(r, scalar_check_overflow(r) as u32);
        if !overflow.is_null() {
            *overflow = over;
        }
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_set_b32(r: *mut Scalar, b32: *const u8, overflow: *mut i32) {
    unsafe {
        let mut over: i32;

        (*r).d[0] = (*b32.add(31) as u32)
            | ((*b32.add(30) as u32) << 8)
            | ((*b32.add(29) as u32) << 16)
            | ((*b32.add(28) as u32) << 24);

        (*r).d[1] = (*b32.add(27) as u32)
            | ((*b32.add(26) as u32) << 8)
            | ((*b32.add(25) as u32) << 16)
            | ((*b32.add(24) as u32) << 24);

        (*r).d[2] = (*b32.add(23) as u32)
            | ((*b32.add(22) as u32) << 8)
            | ((*b32.add(21) as u32) << 16)
            | ((*b32.add(20) as u32) << 24);

        (*r).d[3] = (*b32.add(19) as u32)
            | ((*b32.add(18) as u32) << 8)
            | ((*b32.add(17) as u32) << 16)
            | ((*b32.add(16) as u32) << 24);

        (*r).d[4] = (*b32.add(15) as u32)
            | ((*b32.add(14) as u32) << 8)
            | ((*b32.add(13) as u32) << 16)
            | ((*b32.add(12) as u32) << 24);

        (*r).d[5] = (*b32.add(11) as u32)
            | ((*b32.add(10) as u32) << 8)
            | ((*b32.add(9) as u32) << 16)
            | ((*b32.add(8) as u32) << 24);

        (*r).d[6] = (*b32.add(7) as u32)
            | ((*b32.add(6) as u32) << 8)
            | ((*b32.add(5) as u32) << 16)
            | ((*b32.add(4) as u32) << 24);

        (*r).d[7] = (*b32.add(3) as u32)
            | ((*b32.add(2) as u32) << 8)
            | ((*b32.add(1) as u32) << 16)
            | ((*b32.add(0) as u32) << 24);

        over = scalar_reduce(r, scalar_check_overflow(r) as u32);
        if !overflow.is_null() {
            *overflow = over;
        }
    }
}

#[cfg(feature="exhaustive-test-order")]
pub fn scalar_set_b32(r: *mut Scalar, b32: *const u8, overflow: *mut i32) {
    unsafe {
        let mut over: i32 = 0;
        *r = 0;
        for i in 0..32usize {
            *r = r.wrapping_mul(0x100).wrapping_add(*b32.add(i) as u32);
            if *r >= EXHAUSTIVE_TEST_ORDER {
                over = 1;
                *r %= EXHAUSTIVE_TEST_ORDER;
            }
        }
        if !overflow.is_null() {
            *overflow = over;
        }
    }
}

#[cfg(test)]
mod scalar_set_b32_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn scalar_set_b32_reduces_inputs_mod_order_and_sets_overflow_flag() {
        info!("validating scalar_set_b32 reduction and overflow reporting for representative inputs");

        for (i, input) in REDUCTION_TEST_INPUTS_BE.iter().enumerate() {
            let (s, overflow) = scalar_from_be_bytes_with_overflow(input);
            let got = scalar_to_be_bytes(&s);

            let (expected, expected_overflow) = be_reduce_256_mod_n(input);

            trace!(i, overflow, expected_overflow, ?input, ?got, ?expected, "set_b32 case");
            assert_eq!(overflow, expected_overflow as i32);
            assert_eq!(got, expected);
            assert!(scalar_is_normalized_bytes(&got));
        }

        debug!("explicitly checking input == n reduces to 0 with overflow=1");
        let (s, overflow) = scalar_from_be_bytes_with_overflow(&SECP256K1_ORDER_BE);
        assert_eq!(overflow, 1);
        assert_eq!(scalar_to_be_bytes(&s), SCALAR_ZERO_BE);
    }
}
