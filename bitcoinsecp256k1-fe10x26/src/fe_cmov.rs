// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_cmov.rs ]
crate::ix!();

#[inline] pub fn fe_cmov(
    r:    *mut Fe10x26,
    a:    *const Fe10x26,
    flag: i32)  {

    unsafe {
        let mask0: u32;
        let mask1: u32;

        //VG_CHECK_VERIFY!((*r).n.as_ptr() as *const u8, core::mem::size_of_val(&(*r).n));

        mask0 = (flag as u32).wrapping_add(!0u32);
        mask1 = !mask0;

        (*r).n[0] = ((*r).n[0] & mask0) | ((*a).n[0] & mask1);
        (*r).n[1] = ((*r).n[1] & mask0) | ((*a).n[1] & mask1);
        (*r).n[2] = ((*r).n[2] & mask0) | ((*a).n[2] & mask1);
        (*r).n[3] = ((*r).n[3] & mask0) | ((*a).n[3] & mask1);
        (*r).n[4] = ((*r).n[4] & mask0) | ((*a).n[4] & mask1);
        (*r).n[5] = ((*r).n[5] & mask0) | ((*a).n[5] & mask1);
        (*r).n[6] = ((*r).n[6] & mask0) | ((*a).n[6] & mask1);
        (*r).n[7] = ((*r).n[7] & mask0) | ((*a).n[7] & mask1);
        (*r).n[8] = ((*r).n[8] & mask0) | ((*a).n[8] & mask1);
        (*r).n[9] = ((*r).n[9] & mask0) | ((*a).n[9] & mask1);

        #[cfg(feature="secp256k1-verify")]
        {
            if flag != 0 {
                (*r).magnitude = (*a).magnitude;
                (*r).normalized = (*a).normalized;
            }
        }
    }
}

#[cfg(test)]
mod fe_cmov_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    #[traced_test]
    fn fe_cmov_flag_zero_keeps_r_unchanged() {
        info!("fe_cmov flag=0 should keep r unchanged");
        let mut r = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_B);

        let before = fe_to_be_bytes_normalized(&mut fe_clone_value(&r));
        unsafe { fe_cmov(&mut r as *mut Fe10x26, &a as *const Fe10x26, 0) };
        let after = fe_to_be_bytes_normalized(&mut r);

        debug!(?before, ?after, "cmov flag=0");
        assert_eq!(after, before);
    }

    #[traced_test]
    fn fe_cmov_flag_one_copies_a_into_r() {
        info!("fe_cmov flag=1 should copy a into r");
        let mut r = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_B);

        unsafe { fe_cmov(&mut r as *mut Fe10x26, &a as *const Fe10x26, 1) };

        let out = fe_to_be_bytes_normalized(&mut r);
        trace!(?out, "cmov result");
        assert_eq!(out, BYTES_PATTERN_B);
    }

    #[traced_test]
    fn fe_cmov_allows_r_and_a_alias() {
        info!("fe_cmov should behave sensibly when r==a");
        let mut r = fe_from_be_bytes_checked(&BYTES_LOW_32_ONES);

        let rptr = &mut r as *mut Fe10x26;
        unsafe { fe_cmov(rptr, rptr as *const Fe10x26, 1) };

        let out = fe_to_be_bytes_normalized(&mut r);
        debug!(?out, "alias cmov output");
        assert_eq!(out, BYTES_LOW_32_ONES);
    }

    #[traced_test]
    fn fe_cmov_updates_verify_metadata_only_when_flag_set() {
        info!("under secp256k1-verify, cmov flag=1 updates metadata; flag=0 leaves it");
        let mut r = fe_from_be_bytes_checked(&BYTES_ONE);
        let a = fe_from_be_bytes_checked(&BYTES_TWO);

        #[cfg(feature = "secp256k1-verify")]
        {
            r.magnitude = 7;
            r.normalized = 0;
        }

        unsafe { fe_cmov(&mut r as *mut Fe10x26, &a as *const Fe10x26, 0) };
        #[cfg(feature = "secp256k1-verify")]
        {
            assert_eq!(r.magnitude, 7);
            assert_eq!(r.normalized, 0);
        }

        unsafe { fe_cmov(&mut r as *mut Fe10x26, &a as *const Fe10x26, 1) };
        #[cfg(feature = "secp256k1-verify")]
        {
            assert_eq!(r.magnitude, a.magnitude);
            assert_eq!(r.normalized, a.normalized);
        }
    }
}
