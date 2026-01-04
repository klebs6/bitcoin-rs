// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_check_overflow.rs ]
crate::ix!();

#[cfg(feature="widemul-int128")]
#[inline]
pub fn scalar_check_overflow(a: *const Scalar) -> i32 {
    unsafe {
        let mut yes: i32 = 0;
        let mut no: i32 = 0;
        no |= ((*a).d[3] < N_3) as i32; /* No need for a > check. */
        no |= ((*a).d[2] < N_2) as i32;
        yes |= (((*a).d[2] > N_2) as i32) & !no;
        no |= ((*a).d[1] < N_1) as i32;
        yes |= (((*a).d[1] > N_1) as i32) & !no;
        yes |= (((*a).d[0] >= N_0) as i32) & !no;
        yes
    }
}

#[cfg(feature="widemul-int64")]
#[inline]
pub fn scalar_check_overflow(a: *const Scalar) -> i32 {
    unsafe {
        let mut yes: i32 = 0;
        let mut no: i32 = 0;
        no |= ((*a).d[7] < N_7) as i32; /* No need for a > check. */
        no |= ((*a).d[6] < N_6) as i32; /* No need for a > check. */
        no |= ((*a).d[5] < N_5) as i32; /* No need for a > check. */
        no |= ((*a).d[4] < N_4) as i32;
        yes |= (((*a).d[4] > N_4) as i32) & !no;
        no |= (((*a).d[3] < N_3) as i32) & !yes;
        yes |= (((*a).d[3] > N_3) as i32) & !no;
        no |= (((*a).d[2] < N_2) as i32) & !yes;
        yes |= (((*a).d[2] > N_2) as i32) & !no;
        no |= (((*a).d[1] < N_1) as i32) & !yes;
        yes |= (((*a).d[1] > N_1) as i32) & !no;
        yes |= (((*a).d[0] >= N_0) as i32) & !no;
        yes
    }
}

#[cfg(feature="exhaustive-test-order")]
#[inline]
pub fn scalar_check_overflow(a: *const Scalar) -> i32 {
    unsafe { (*a >= EXHAUSTIVE_TEST_ORDER) as i32 }
}

#[cfg(test)]
mod scalar_overflow_detection_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_check_overflow_detects_n_and_above() {
        info!("validating scalar_check_overflow boundary behavior");

        let n: Scalar = scalar_const!(
            0xFFFFFFFF,
            0xFFFFFFFF,
            0xFFFFFFFF,
            0xFFFFFFFE,
            0xBAAEDCE6,
            0xAF48A03B,
            0xBFD25E8C,
            0xD0364141
        );
        let n_minus_1: Scalar = scalar_const!(
            0xFFFFFFFF,
            0xFFFFFFFF,
            0xFFFFFFFF,
            0xFFFFFFFE,
            0xBAAEDCE6,
            0xAF48A03B,
            0xBFD25E8C,
            0xD0364140
        );
        let n_plus_1: Scalar = scalar_const!(
            0xFFFFFFFF,
            0xFFFFFFFF,
            0xFFFFFFFF,
            0xFFFFFFFE,
            0xBAAEDCE6,
            0xAF48A03B,
            0xBFD25E8C,
            0xD0364142
        );

        unsafe {
            let ov_n = scalar_check_overflow(&n as *const Scalar);
            let ov_nm1 = scalar_check_overflow(&n_minus_1 as *const Scalar);
            let ov_np1 = scalar_check_overflow(&n_plus_1 as *const Scalar);

            debug!(ov_n, ov_nm1, ov_np1, "overflow flags");
            assert_eq!(ov_n, 1);
            assert_eq!(ov_nm1, 0);
            assert_eq!(ov_np1, 1);
        }

        let zero = scalar_from_u32(0);
        let one = scalar_from_u32(1);
        unsafe {
            assert_eq!(scalar_check_overflow(&zero as *const Scalar), 0);
            assert_eq!(scalar_check_overflow(&one as *const Scalar), 0);
        }
    }
}
