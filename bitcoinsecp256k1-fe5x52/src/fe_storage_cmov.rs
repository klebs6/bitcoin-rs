// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_storage_cmov.rs ]
crate::ix!();

#[inline] pub fn fe_storage_cmov(
        r:    *mut FeStorage,
        a:    *const FeStorage,
        flag: i32)  {

    unsafe {
        let mask0: u64;
        let mask1: u64;

        //VG_CHECK_VERIFY(r as *const u64, core::mem::size_of::<[u64; 4]>());

        mask0 = (flag as u64).wrapping_add(!0u64);
        mask1 = !mask0;

        let rn: *mut u64 = r as *mut u64;
        let an: *const u64 = a as *const u64;

        *rn.add(0) = (*rn.add(0) & mask0) | (*an.add(0) & mask1);
        *rn.add(1) = (*rn.add(1) & mask0) | (*an.add(1) & mask1);
        *rn.add(2) = (*rn.add(2) & mask0) | (*an.add(2) & mask1);
        *rn.add(3) = (*rn.add(3) & mask0) | (*an.add(3) & mask1);
    }
}

#[cfg(test)]
mod fe_storage_cmov_rs_exhaustive_tests {
    use super::*;

    #[traced_test]
    fn fe_storage_cmov_respects_flag_and_preserves_source() {
        tracing::info!("testing fe_storage_cmov conditional move semantics");

        unsafe {
            let a = fe_storage_const!(0x11111111u32, 0x22222222u32, 0x33333333u32, 0x44444444u32, 0x55555555u32, 0x66666666u32, 0x77777777u32, 0x88888888u32);
            let b = fe_storage_const!(0xAAAAAAAAu32, 0xBBBBBBBBu32, 0xCCCCCCCCu32, 0xDDDDDDDDu32, 0xEEEEEEEEu32, 0xFFFFFFFFu32, 0x00000000u32, 0x12345678u32);

            let mut r = a;
            crate::fe_storage_cmov(&mut r as *mut FeStorage, &b as *const FeStorage, 0);
            let (r7, r6, r5, r4, r3, r2, r1, r0) = fe_storage_const_get!(r);
            let (a7, a6, a5, a4, a3, a2, a1, a0) = fe_storage_const_get!(a);
            assert_eq!((r7, r6, r5, r4, r3, r2, r1, r0), (a7, a6, a5, a4, a3, a2, a1, a0));

            crate::fe_storage_cmov(&mut r as *mut FeStorage, &b as *const FeStorage, 1);
            let (r7, r6, r5, r4, r3, r2, r1, r0) = fe_storage_const_get!(r);
            let (b7, b6, b5, b4, b3, b2, b1, b0) = fe_storage_const_get!(b);
            assert_eq!((r7, r6, r5, r4, r3, r2, r1, r0), (b7, b6, b5, b4, b3, b2, b1, b0));

            let (b7c, b6c, b5c, b4c, b3c, b2c, b1c, b0c) = fe_storage_const_get!(b);
            assert_eq!((b7, b6, b5, b4, b3, b2, b1, b0), (b7c, b6c, b5c, b4c, b3c, b2c, b1c, b0c));
        }
    }
}
