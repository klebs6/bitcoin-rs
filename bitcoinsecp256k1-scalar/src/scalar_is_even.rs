// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_is_even.rs ]
crate::ix!();

/// Check whether a scalar, considered as an nonnegative integer, is even.
/// 
#[cfg(feature="widemul-int128")]
#[inline]
pub fn scalar_is_even(a: *const Scalar) -> i32 {
    unsafe { (((*a).d[0] & 1) == 0) as i32 }
}

#[cfg(feature="widemul-int64")]
#[inline]
pub fn scalar_is_even(a: *const Scalar) -> i32 {
    unsafe { (((*a).d[0] & 1) == 0) as i32 }
}

#[cfg(feature="exhaustive-test-order")]
#[inline]
pub fn scalar_is_even(a: *const Scalar) -> i32 {
    unsafe { ((*a & 1) == 0) as i32 }
}

#[cfg(test)]
mod scalar_evenness_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_is_even_matches_expected_for_small_values() {
        info!("validating scalar_is_even on small values");

        let z = scalar_from_u32(0);
        let o = scalar_from_u32(1);
        let t = scalar_from_u32(2);
        let three = scalar_from_u32(3);

        unsafe {
            debug!("0");
            assert_eq!(scalar_is_even(&z as *const Scalar), 1);

            debug!("1");
            assert_eq!(scalar_is_even(&o as *const Scalar), 0);

            debug!("2");
            assert_eq!(scalar_is_even(&t as *const Scalar), 1);

            debug!("3");
            assert_eq!(scalar_is_even(&three as *const Scalar), 0);
        }
    }
}
