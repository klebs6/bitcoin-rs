// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_is_one.rs ]
crate::ix!();

/// Check whether a scalar equals one.
/// 
#[cfg(feature="widemul-int128")]
#[inline]
pub fn scalar_is_one(a: *const Scalar) -> i32 {
    unsafe { ((((*a).d[0] ^ 1) | (*a).d[1] | (*a).d[2] | (*a).d[3]) == 0) as i32 }
}

#[cfg(feature="widemul-int64")]
#[inline]
pub fn scalar_is_one(a: *const Scalar) -> i32 {
    unsafe {
        ((((*a).d[0] ^ 1)
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
pub fn scalar_is_one(a: *const Scalar) -> i32 {
    unsafe { (*a == 1) as i32 }
}

#[cfg(test)]
mod scalar_is_one_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_is_one_matches_expected_for_selected_vectors() {
        info!("validating scalar_is_one on selected vectors");

        let a0 = scalar_from_u32(0);
        let a1 = scalar_from_u32(1);
        let a2 = scalar_from_u32(2);

        unsafe {
            debug!("checking 0");
            assert_eq!(scalar_is_one(&a0 as *const Scalar), 0);

            debug!("checking 1");
            assert_eq!(scalar_is_one(&a1 as *const Scalar), 1);

            debug!("checking 2");
            assert_eq!(scalar_is_one(&a2 as *const Scalar), 0);
        }
    }
}
