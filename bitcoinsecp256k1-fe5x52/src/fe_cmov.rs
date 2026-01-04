// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_cmov.rs ]
crate::ix!();

#[inline] pub fn fe_cmov(
        r:    *mut Fe5x52,
        a:    *const Fe5x52,
        flag: i32)  {

    unsafe {
        let mask0: u64;
        let mask1: u64;

        //VG_CHECK_VERIFY(r as *const u64, core::mem::size_of::<[u64; 5]>());

        mask0 = (flag as u64).wrapping_add(!0u64);
        mask1 = !mask0;

        (*r).n[0] = ((*r).n[0] & mask0) | ((*a).n[0] & mask1);
        (*r).n[1] = ((*r).n[1] & mask0) | ((*a).n[1] & mask1);
        (*r).n[2] = ((*r).n[2] & mask0) | ((*a).n[2] & mask1);
        (*r).n[3] = ((*r).n[3] & mask0) | ((*a).n[3] & mask1);
        (*r).n[4] = ((*r).n[4] & mask0) | ((*a).n[4] & mask1);

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
mod fe_cmov_rs_exhaustive_tests {
    use super::*;

    fn u64_to_be32(v: u64) -> [u8; 32] {
        let mut out = [0u8; 32];
        out[24..32].copy_from_slice(&v.to_be_bytes());
        out
    }

    unsafe fn fe_from_u64(v: u64) -> Fe5x52 {
        let mut fe = Fe5x52::new();
        let bytes = u64_to_be32(v);
        let ret = crate::fe_set_b32(&mut fe as *mut Fe5x52, bytes.as_ptr());
        assert_eq!(ret, 1);
        fe
    }

    unsafe fn fe_get_b32_normalized(fe: &mut Fe5x52) -> [u8; 32] {
        crate::fe_normalize(fe as *mut Fe5x52);
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), fe as *const Fe5x52);
        out
    }

    #[traced_test]
    fn fe_cmov_respects_flag_and_preserves_source_and_aliasing_is_safe() {
        tracing::info!("testing fe_cmov flag behavior");

        unsafe {
            let mut r = fe_from_u64(7);
            let a = fe_from_u64(42);

            let r_before = fe_get_b32_normalized(&mut r);
            let a_before = {
                let mut tmp = a;
                fe_get_b32_normalized(&mut tmp)
            };

            tracing::debug!("flag=0 should keep r unchanged");
            crate::fe_cmov(&mut r as *mut Fe5x52, &a as *const Fe5x52, 0);
            assert_eq!(fe_get_b32_normalized(&mut r), r_before);

            tracing::debug!("flag=1 should overwrite r with a");
            crate::fe_cmov(&mut r as *mut Fe5x52, &a as *const Fe5x52, 1);
            assert_eq!(fe_get_b32_normalized(&mut r), a_before);

            tracing::debug!("source a should remain unchanged");
            let mut a_check = a;
            assert_eq!(fe_get_b32_normalized(&mut a_check), a_before);

            tracing::debug!("aliasing case r==a should be a no-op regardless of flag");
            let mut x = fe_from_u64(99);
            let x_before = fe_get_b32_normalized(&mut x);
            crate::fe_cmov(&mut x as *mut Fe5x52, &x as *const Fe5x52, 0);
            assert_eq!(fe_get_b32_normalized(&mut x), x_before);
            crate::fe_cmov(&mut x as *mut Fe5x52, &x as *const Fe5x52, 1);
            assert_eq!(fe_get_b32_normalized(&mut x), x_before);
        }
    }
}

#[cfg(test)]
mod fe_cmp_var_rs_exhaustive_tests {
    use super::*;

    const FIELD_P_MINUS_1_B32: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x2E,
    ];

    fn u64_to_be32(v: u64) -> [u8; 32] {
        let mut out = [0u8; 32];
        out[24..32].copy_from_slice(&v.to_be_bytes());
        out
    }

    unsafe fn fe_from_b32_checked(bytes: &[u8; 32]) -> Fe5x52 {
        let mut fe = Fe5x52::new();
        let ret = crate::fe_set_b32(&mut fe as *mut Fe5x52, bytes.as_ptr());
        assert_eq!(ret, 1);
        fe
    }

    unsafe fn fe_from_u64(v: u64) -> Fe5x52 {
        let bytes = u64_to_be32(v);
        fe_from_b32_checked(&bytes)
    }

    #[traced_test]
    fn fe_cmp_var_orders_normalized_values_correctly_including_field_top() {
        tracing::info!("testing fe_cmp_var ordering semantics for normalized inputs");

        unsafe {
            let mut vals: [Fe5x52; 6] = [
                fe_from_u64(0),
                fe_from_u64(1),
                fe_from_u64(2),
                fe_from_u64(3),
                fe_from_u64(65536),
                fe_from_b32_checked(&FIELD_P_MINUS_1_B32),
            ];

            let mut i: usize = 0;
            while i < vals.len() {
                crate::fe_normalize(&mut vals[i] as *mut Fe5x52);
                i += 1;
            }

            let pairs: &[(usize, usize, i32)] = &[
                (0, 0, 0),
                (1, 1, 0),
                (2, 2, 0),
                (5, 5, 0),
                (0, 1, -1),
                (1, 0, 1),
                (1, 2, -1),
                (2, 1, 1),
                (3, 4, -1),
                (4, 3, 1),
                (4, 5, -1),
                (5, 4, 1),
                (0, 5, -1),
                (5, 0, 1),
            ];

            for &(ai, bi, expected) in pairs.iter() {
                tracing::debug!(ai = ai, bi = bi, expected = expected, "comparing");
                let got = crate::fe_cmp_var(&vals[ai] as *const Fe5x52, &vals[bi] as *const Fe5x52);
                assert_eq!(got, expected);
            }
        }
    }
}
