// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_set_int.rs ]
crate::ix!();

/// Set a scalar to an unsigned integer.
/// 
#[cfg(feature="widemul-int128")]
#[inline]
pub fn scalar_set_int(r: *mut Scalar, v: u32) {
    unsafe {
        (*r).d[0] = v as u64;
        (*r).d[1] = 0;
        (*r).d[2] = 0;
        (*r).d[3] = 0;
    }
}

#[cfg(feature="widemul-int64")]
#[inline]
pub fn scalar_set_int(r: *mut Scalar, v: u32) {
    unsafe {
        (*r).d[0] = v;
        (*r).d[1] = 0;
        (*r).d[2] = 0;
        (*r).d[3] = 0;
        (*r).d[4] = 0;
        (*r).d[5] = 0;
        (*r).d[6] = 0;
        (*r).d[7] = 0;
    }
}

#[cfg(feature="exhaustive-test-order")]
#[inline]
pub fn scalar_set_int(r: *mut Scalar, v: u32) {
    unsafe {
        *r = v;
    }
}

#[cfg(test)]
mod scalar_set_int_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_set_int_sets_expected_low_value_and_clears_upper_bits() {
        info!("validating scalar_set_int for representative small values");

        for v in [0u32, 1u32, 2u32, 3u32, 0xFFFF_FFFFu32] {
            let s = scalar_from_u32(v);
            let be = scalar_to_be_bytes(&s);

            let mut expected = [0u8; 32];
            expected[28..32].copy_from_slice(&v.to_be_bytes());

            debug!(v, ?be, ?expected, "scalar_set_int");
            assert_eq!(be, expected);
            assert!(scalar_is_normalized_bytes(&be));
        }
    }
}
