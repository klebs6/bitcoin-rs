// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_is_zero.rs ]
crate::ix!();

/// Check whether a scalar equals zero.
/// 
#[cfg(feature="widemul-int128")]
#[inline]
pub fn scalar_is_zero(a: *const Scalar) -> i32 {
    unsafe { (((*a).d[0] | (*a).d[1] | (*a).d[2] | (*a).d[3]) == 0) as i32 }
}

#[cfg(feature="widemul-int64")]
#[inline]
pub fn scalar_is_zero(a: *const Scalar) -> i32 {
    unsafe {
        (((*a).d[0]
            | (*a).d[1]
            | (*a).d[2]
            | (*a).d[3]
            | (*a).d[4]
            | (*a).d[5]
            | (*a).d[6]
            | (*a).d[7])
            == 0) as i32
    }
}

#[cfg(feature="exhaustive-test-order")]
#[inline]
pub fn scalar_is_zero(a: *const Scalar) -> i32 {
    unsafe { (*a == 0) as i32 }
}

#[cfg(test)]
mod scalar_is_zero_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_is_zero_matches_expected_for_canonical_vectors() {
        info!("validating scalar_is_zero on canonical vectors");

        for (i, be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let s = scalar_from_be_bytes(be);
            let expected = if be_is_zero_32(be) { 1 } else { 0 };
            let got = unsafe { scalar_is_zero(&s as *const Scalar) };
            debug!(i, expected, got, ?be, "scalar_is_zero");
            assert_eq!(got, expected);
        }
    }
}
