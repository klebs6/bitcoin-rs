// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_negate.rs ]
crate::ix!();

#[inline] pub fn fe_negate(
        r: *mut Fe5x52,
        a: *const Fe5x52,
        m: i32)  {

    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            verify_check((*a).magnitude <= m);
            fe_verify(a);
        }

        let mm = (m as u64).wrapping_add(1u64);
        let f2 = mm.wrapping_mul(2u64);

        (*r).n[0] = 0xFFFFEFFFFFC2F_u64.wrapping_mul(f2).wrapping_sub((*a).n[0]);
        (*r).n[1] = 0xFFFFFFFFFFFFF_u64.wrapping_mul(f2).wrapping_sub((*a).n[1]);
        (*r).n[2] = 0xFFFFFFFFFFFFF_u64.wrapping_mul(f2).wrapping_sub((*a).n[2]);
        (*r).n[3] = 0xFFFFFFFFFFFFF_u64.wrapping_mul(f2).wrapping_sub((*a).n[3]);
        (*r).n[4] = 0x0FFFFFFFFFFFF_u64.wrapping_mul(f2).wrapping_sub((*a).n[4]);

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = m.wrapping_add(1);
            (*r).normalized = 0;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_negate_rs_exhaustive_tests {
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

    const SAMPLE_B32: [u8; 32] = [
        0x01, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70,
        0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0x01,
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

    unsafe fn fe_to_b32_normalized(fe: &mut Fe5x52) -> [u8; 32] {
        crate::fe_normalize(fe as *mut Fe5x52);
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), fe as *const Fe5x52);
        out
    }

    #[traced_test]
    fn fe_negate_yields_additive_inverse_after_normalization_and_double_negation_roundtrips() {
        tracing::info!("testing fe_negate produces field additive inverse after normalization");

        unsafe {
            let samples: [&[u8; 32]; 5] = [
                &u64_to_be32(0),
                &u64_to_be32(1),
                &u64_to_be32(2),
                &SAMPLE_B32,
                &FIELD_P_MINUS_1_B32,
            ];

            for (idx, s) in samples.iter().enumerate() {
                tracing::debug!(sample_index = idx, "negation test");
                let a = fe_from_b32_checked(s);

                let mut neg = Fe5x52::new();
                crate::fe_negate(&mut neg as *mut Fe5x52, &a as *const Fe5x52, 1);

                let mut sum = a;
                crate::fe_add(&mut sum as *mut Fe5x52, &neg as *const Fe5x52);

                let mut sum_norm = sum;
                crate::fe_normalize(&mut sum_norm as *mut Fe5x52);
                let mut out = [0u8; 32];
                crate::fe_get_b32(out.as_mut_ptr(), &sum_norm as *const Fe5x52);
                assert_eq!(out, [0u8; 32]);

                let expected = if **s == [0u8; 32] {
                    [0u8; 32]
                } else {
                    be32_sub(&FIELD_P_B32, s)
                };

                let mut neg_norm = neg;
                let got = fe_to_b32_normalized(&mut neg_norm);
                assert_eq!(got, expected);

                let mut dbl = Fe5x52::new();
                crate::fe_negate(&mut dbl as *mut Fe5x52, &neg as *const Fe5x52, 2);
                let mut dbl_norm = dbl;
                let got_dbl = fe_to_b32_normalized(&mut dbl_norm);

                let mut a_norm = a;
                let expected_a = fe_to_b32_normalized(&mut a_norm);
                assert_eq!(got_dbl, expected_a);
            }
        }
    }
}
