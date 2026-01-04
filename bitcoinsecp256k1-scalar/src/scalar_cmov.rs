// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_cmov.rs ]
crate::ix!();

/// If flag is true, set *r equal to *a; otherwise leave it. 
///
/// Constant-time. Both *r and *a must be initialized.
/// 
#[cfg(feature="widemul-int128")]
#[inline]
pub fn scalar_cmov(r: *mut Scalar, a: *const Scalar, flag: i32) {
    unsafe {
        let mask0: u64 = (flag as u64).wrapping_add(!0u64);
        let mask1: u64 = !mask0;
        (*r).d[0] = ((*r).d[0] & mask0) | ((*a).d[0] & mask1);
        (*r).d[1] = ((*r).d[1] & mask0) | ((*a).d[1] & mask1);
        (*r).d[2] = ((*r).d[2] & mask0) | ((*a).d[2] & mask1);
        (*r).d[3] = ((*r).d[3] & mask0) | ((*a).d[3] & mask1);
    }
}

#[cfg(feature="widemul-int64")]
#[inline]
pub fn scalar_cmov(r: *mut Scalar, a: *const Scalar, flag: i32) {
    unsafe {
        let mask0: u32 = (flag as u32).wrapping_add(!0u32);
        let mask1: u32 = !mask0;
        (*r).d[0] = ((*r).d[0] & mask0) | ((*a).d[0] & mask1);
        (*r).d[1] = ((*r).d[1] & mask0) | ((*a).d[1] & mask1);
        (*r).d[2] = ((*r).d[2] & mask0) | ((*a).d[2] & mask1);
        (*r).d[3] = ((*r).d[3] & mask0) | ((*a).d[3] & mask1);
        (*r).d[4] = ((*r).d[4] & mask0) | ((*a).d[4] & mask1);
        (*r).d[5] = ((*r).d[5] & mask0) | ((*a).d[5] & mask1);
        (*r).d[6] = ((*r).d[6] & mask0) | ((*a).d[6] & mask1);
        (*r).d[7] = ((*r).d[7] & mask0) | ((*a).d[7] & mask1);
    }
}

#[cfg(feature="exhaustive-test-order")]
#[inline]
pub fn scalar_cmov(r: *mut Scalar, a: *const Scalar, flag: i32) {
    unsafe {
        let mask0: u32 = (flag as u32).wrapping_add(!0u32);
        let mask1: u32 = !mask0;
        *r = (*r & mask0) | (*a & mask1);
    }
}

#[cfg(test)]
mod scalar_cmov_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_cmov_flag_controls_assignment() {
        info!("validating scalar_cmov semantics");

        let a = scalar_from_be_bytes(&SCALAR_TWO_BE);
        let b = scalar_from_be_bytes(&SECP256K1_ORDER_MINUS_1_BE);

        let mut r = scalar_clone_via_b32(&a);
        unsafe {
            scalar_cmov(&mut r as *mut Scalar, &b as *const Scalar, 0);
        }
        let r0 = scalar_to_be_bytes(&r);
        debug!(?r0, "flag=0 preserves");
        assert_eq!(r0, SCALAR_TWO_BE);

        unsafe {
            scalar_cmov(&mut r as *mut Scalar, &b as *const Scalar, 1);
        }
        let r1 = scalar_to_be_bytes(&r);
        debug!(?r1, "flag=1 assigns");
        assert_eq!(r1, SECP256K1_ORDER_MINUS_1_BE);

        let mut alias = scalar_clone_via_b32(&b);
        unsafe {
            scalar_cmov(&mut alias as *mut Scalar, &alias as *const Scalar, 0);
            scalar_cmov(&mut alias as *mut Scalar, &alias as *const Scalar, 1);
        }
        let alias_be = scalar_to_be_bytes(&alias);
        debug!(?alias_be, "aliasing (r==a) is stable");
        assert_eq!(alias_be, SECP256K1_ORDER_MINUS_1_BE);
    }
}
