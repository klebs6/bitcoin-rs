// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/constants.rs ]
crate::ix!();

lazy_static!{
    pub static ref const_modinfo_fe: ModInv64ModInfo = unsafe {
        #[repr(C)]
        struct ModInv64ModInfoCompat {
            modulus:       [i64; 5],
            modulus_inv62: u64,
        }

        let mi = ModInv64ModInfoCompat {
            modulus: [
                -(0x1000003D1_i64),
                0_i64,
                0_i64,
                0_i64,
                256_i64,
            ],
            modulus_inv62: 0x27C7F6E22DDACACF_u64,
        };

        core::mem::transmute::<ModInv64ModInfoCompat, ModInv64ModInfo>(mi)
    };
}

#[cfg(test)]
mod constants_rs_exhaustive_tests {
    use super::*;

    const FIELD_P_B32: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x2F,
    ];

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

    fn be32_sub(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        let mut out = [0u8; 32];
        let mut borrow: i32 = 0;
        let mut i: i32 = 31;
        while i >= 0 {
            let ai = a[i as usize] as i32;
            let bi = b[i as usize] as i32;
            let mut v = ai - bi - borrow;
            if v < 0 {
                v += 256;
                borrow = 1;
            } else {
                borrow = 0;
            }
            out[i as usize] = v as u8;
            i -= 1;
        }
        assert_eq!(borrow, 0);
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

    unsafe fn fe_to_b32_normalized(fe: &mut Fe5x52) -> [u8; 32] {
        crate::fe_normalize(fe as *mut Fe5x52);
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), fe as *const Fe5x52);
        out
    }

    unsafe fn fe_mul_normalized(a: &Fe5x52, b: &Fe5x52) -> [u8; 32] {
        let mut r = Fe5x52::new();
        crate::fe_mul(&mut r as *mut Fe5x52, a as *const Fe5x52, b as *const Fe5x52);
        fe_to_b32_normalized(&mut r)
    }

    #[traced_test]
    fn constants_modinfo_drives_field_inversion_and_matches_var_inversion() {
        tracing::info!("exercising const_modinfo_fe via fe_inv/fe_inv_var across representative values");

        unsafe {
            let values: [u64; 22] = [
                0, 1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 63, 64, 127, 128, 255, 256, 1024, 65535, 65536,
            ];

            for &v in values.iter() {
                tracing::debug!(value_u64 = v, "testing inversion identity x*inv(x)");

                let x = fe_from_u64(v);

                let mut inv1 = Fe5x52::new();
                crate::fe_inv(&mut inv1 as *mut Fe5x52, &x as *const Fe5x52);

                let mut inv2 = Fe5x52::new();
                crate::fe_inv_var(&mut inv2 as *mut Fe5x52, &x as *const Fe5x52);

                let prod1 = fe_mul_normalized(&x, &inv1);
                let prod2 = fe_mul_normalized(&x, &inv2);

                if v == 0 {
                    assert_eq!(prod1, [0u8; 32]);
                    assert_eq!(prod2, [0u8; 32]);

                    let mut inv1_b = inv1;
                    let mut inv2_b = inv2;
                    assert_eq!(fe_to_b32_normalized(&mut inv1_b), [0u8; 32]);
                    assert_eq!(fe_to_b32_normalized(&mut inv2_b), [0u8; 32]);
                } else {
                    assert_eq!(prod1, u64_to_be32(1));
                    assert_eq!(prod2, u64_to_be32(1));

                    let mut inv1_b = inv1;
                    let mut inv2_b = inv2;
                    assert_eq!(fe_to_b32_normalized(&mut inv1_b), fe_to_b32_normalized(&mut inv2_b));
                }
            }

            tracing::debug!("testing inversion of p-1 equals p-1");
            let x = fe_from_b32_checked(&FIELD_P_MINUS_1_B32);

            let mut inv = Fe5x52::new();
            crate::fe_inv(&mut inv as *mut Fe5x52, &x as *const Fe5x52);

            let mut inv_b = inv;
            assert_eq!(fe_to_b32_normalized(&mut inv_b), FIELD_P_MINUS_1_B32);

            tracing::debug!("testing (-1)+(-1) normalization behavior to p-2");
            let mut r = fe_from_b32_checked(&FIELD_P_MINUS_1_B32);
            let a = fe_from_b32_checked(&FIELD_P_MINUS_1_B32);
            crate::fe_add(&mut r as *mut Fe5x52, &a as *const Fe5x52);
            let got = fe_to_b32_normalized(&mut r);

            let expected = be32_sub(&FIELD_P_B32, &u64_to_be32(2));
            assert_eq!(got, expected);
        }
    }

    #[traced_test]
    fn constants_modinfo_direct_modinv64_and_modinv64_var_agree_for_samples() {
        tracing::info!("exercising modinv64/modinv64_var directly with const_modinfo_fe");

        unsafe {
            let values: [u64; 12] = [1, 2, 3, 5, 7, 11, 13, 17, 19, 255, 256, 65536];

            for &v in values.iter() {
                tracing::debug!(value_u64 = v, "testing modinv64 vs modinv64_var agreement");

                let mut x = fe_from_u64(v);
                crate::fe_normalize(&mut x as *mut Fe5x52);

                let mut s1 = core::mem::MaybeUninit::<ModInv64Signed62>::uninit();
                let mut s2 = core::mem::MaybeUninit::<ModInv64Signed62>::uninit();

                crate::fe_to_signed62(s1.as_mut_ptr(), &x as *const Fe5x52);
                crate::fe_to_signed62(s2.as_mut_ptr(), &x as *const Fe5x52);

                modinv64(&mut *s1.as_mut_ptr(), &const_modinfo_fe);
                modinv64_var(&mut *s2.as_mut_ptr(), &const_modinfo_fe);

                let mut r1 = Fe5x52::new();
                let mut r2 = Fe5x52::new();

                crate::fe_from_signed62(&mut r1 as *mut Fe5x52, s1.as_ptr());
                crate::fe_from_signed62(&mut r2 as *mut Fe5x52, s2.as_ptr());

                let b1 = fe_to_b32_normalized(&mut r1);
                let b2 = fe_to_b32_normalized(&mut r2);

                assert_eq!(b1, b2);

                let check = fe_mul_normalized(&x, &fe_from_b32_checked(&b1));
                assert_eq!(check, u64_to_be32(1));
            }
        }
    }
}
