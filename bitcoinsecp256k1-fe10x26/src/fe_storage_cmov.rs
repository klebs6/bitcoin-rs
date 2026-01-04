// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_storage_cmov.rs ]
crate::ix!();

#[inline] pub fn fe_storage_cmov(
    r:    *mut Fe10x26Storage,
    a:    *const Fe10x26Storage,
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
    }
}

#[cfg(test)]
mod fe_storage_cmov_interface_contract_suite {
    use super::*;
    use tracing::{debug, info};

    #[traced_test]
    fn fe_storage_cmov_flag_zero_keeps_r() {
        info!("fe_storage_cmov flag=0 should keep r unchanged");
        let mut r = Fe10x26Storage { n: [1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32] };
        let a = Fe10x26Storage { n: [9u32, 10u32, 11u32, 12u32, 13u32, 14u32, 15u32, 16u32] };
        let before = r.n;

        unsafe { fe_storage_cmov(&mut r as *mut Fe10x26Storage, &a as *const Fe10x26Storage, 0) };

        debug!(?r.n, "cmov flag=0 result");
        assert_eq!(r.n, before);
    }

    #[traced_test]
    fn fe_storage_cmov_flag_one_copies_a() {
        info!("fe_storage_cmov flag=1 should copy a into r");
        let mut r = Fe10x26Storage { n: [1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32] };
        let a = Fe10x26Storage { n: [9u32, 10u32, 11u32, 12u32, 13u32, 14u32, 15u32, 16u32] };

        unsafe { fe_storage_cmov(&mut r as *mut Fe10x26Storage, &a as *const Fe10x26Storage, 1) };

        debug!(?r.n, "cmov flag=1 result");
        assert_eq!(r.n, a.n);
    }
}
