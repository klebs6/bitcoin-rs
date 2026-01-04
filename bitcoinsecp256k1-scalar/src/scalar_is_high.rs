// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_is_high.rs ]
crate::ix!();

/// Check whether a scalar is higher than
/// the group order divided by 2.
/// 
#[cfg(feature="widemul-int128")]
pub fn scalar_is_high(a: *const Scalar) -> i32 {
    unsafe {
        let mut yes: i32 = 0;
        let mut no: i32 = 0;
        no |= ((*a).d[3] < N_H_3) as i32;
        yes |= (((*a).d[3] > N_H_3) as i32) & !no;
        no |= (((*a).d[2] < N_H_2) as i32) & !yes; /* No need for a > check. */
        no |= (((*a).d[1] < N_H_1) as i32) & !yes;
        yes |= (((*a).d[1] > N_H_1) as i32) & !no;
        yes |= (((*a).d[0] > N_H_0) as i32) & !no;
        yes
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_is_high(a: *const Scalar) -> i32 {
    unsafe {
        let mut yes: i32 = 0;
        let mut no: i32 = 0;
        no |= ((*a).d[7] < N_H_7) as i32;
        yes |= (((*a).d[7] > N_H_7) as i32) & !no;
        no |= (((*a).d[6] < N_H_6) as i32) & !yes; /* No need for a > check. */
        no |= (((*a).d[5] < N_H_5) as i32) & !yes; /* No need for a > check. */
        no |= (((*a).d[4] < N_H_4) as i32) & !yes; /* No need for a > check. */
        no |= (((*a).d[3] < N_H_3) as i32) & !yes;
        yes |= (((*a).d[3] > N_H_3) as i32) & !no;
        no |= (((*a).d[2] < N_H_2) as i32) & !yes;
        yes |= (((*a).d[2] > N_H_2) as i32) & !no;
        no |= (((*a).d[1] < N_H_1) as i32) & !yes;
        yes |= (((*a).d[1] > N_H_1) as i32) & !no;
        yes |= (((*a).d[0] > N_H_0) as i32) & !no;
        yes
    }
}

#[cfg(feature="exhaustive-test-order")]
pub fn scalar_is_high(a: *const Scalar) -> i32 {
    unsafe { (*a > (EXHAUSTIVE_TEST_ORDER / 2)) as i32 }
}

#[cfg(test)]
mod scalar_highness_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_is_high_matches_half_order_boundary() {
        info!("validating scalar_is_high around n/2 boundary");

        let half = scalar_from_be_bytes(&SECP256K1_ORDER_HALF_BE);
        let one = scalar_from_u32(1);

        let mut half_plus_one = scalar_zero_value();
        unsafe {
            let _ov = scalar_add(
                &mut half_plus_one as *mut Scalar,
                &half as *const Scalar,
                &one as *const Scalar,
            );
        }

        let nm1 = scalar_from_be_bytes(&SECP256K1_ORDER_MINUS_1_BE);

        unsafe {
            let is_half_high = scalar_is_high(&half as *const Scalar);
            let is_half_plus_one_high = scalar_is_high(&half_plus_one as *const Scalar);
            let is_nm1_high = scalar_is_high(&nm1 as *const Scalar);

            debug!(is_half_high, "half");
            debug!(is_half_plus_one_high, "half+1");
            debug!(is_nm1_high, "n-1");

            assert_eq!(is_half_high, 0);
            assert_eq!(is_half_plus_one_high, 1);
            assert_eq!(is_nm1_high, 1);
        }
    }
}
