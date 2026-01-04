// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_add.rs ]
crate::ix!();

#[inline] pub fn fe_add(
    r: *mut Fe5x52,
    a: *const Fe5x52)  {

    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            fe_verify(a);
        }
        (*r).n[0] = (*r).n[0].wrapping_add((*a).n[0]);
        (*r).n[1] = (*r).n[1].wrapping_add((*a).n[1]);
        (*r).n[2] = (*r).n[2].wrapping_add((*a).n[2]);
        (*r).n[3] = (*r).n[3].wrapping_add((*a).n[3]);
        (*r).n[4] = (*r).n[4].wrapping_add((*a).n[4]);
        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = (*r).magnitude.wrapping_add((*a).magnitude);
            (*r).normalized = 0;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_add_rs_exhaustive_tests {
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

    #[traced_test]
    fn fe_add_matches_expected_sums_including_cross_limb_and_modulus_wrap_cases() {
        tracing::info!("testing fe_add behavior with representative and boundary-crossing additions");

        unsafe {
            let cross = (1u64 << 52) - 1u64;

            let cases: &[(u64, u64)] = &[
                (0, 0),
                (0, 1),
                (1, 0),
                (1, 1),
                (2, 3),
                (3, 2),
                (5, 7),
                (17, 19),
                (255, 256),
                (1024, 65535),
                (cross, 1),
                (1, cross),
                (cross, cross),
                (123456789, 987654321),
            ];

            for &(x, y) in cases.iter() {
                tracing::debug!(x_u64 = x, y_u64 = y, "fe_add then normalize should match u64 sum");
                let mut r = fe_from_u64(x);
                let a = fe_from_u64(y);

                crate::fe_add(&mut r as *mut Fe5x52, &a as *const Fe5x52);

                let got = fe_to_b32_normalized(&mut r);
                let expected = u64_to_be32(x.wrapping_add(y));
                assert_eq!(got, expected);
            }

            tracing::debug!("testing fe_add aliasing case r == a (doubling)");
            let mut r = fe_from_u64(7);
            crate::fe_add(&mut r as *mut Fe5x52, &r as *const Fe5x52);
            assert_eq!(fe_to_b32_normalized(&mut r), u64_to_be32(14));

            tracing::debug!("testing (p-1) + 1 normalizes to 0");
            let mut r = fe_from_b32_checked(&FIELD_P_MINUS_1_B32);
            let a = fe_from_u64(1);
            crate::fe_add(&mut r as *mut Fe5x52, &a as *const Fe5x52);
            assert_eq!(fe_to_b32_normalized(&mut r), [0u8; 32]);

            tracing::debug!("testing (p-1) + (p-1) normalizes to p-2");
            let mut r = fe_from_b32_checked(&FIELD_P_MINUS_1_B32);
            let a = fe_from_b32_checked(&FIELD_P_MINUS_1_B32);
            crate::fe_add(&mut r as *mut Fe5x52, &a as *const Fe5x52);
            let expected = be32_sub(&FIELD_P_B32, &u64_to_be32(2));
            assert_eq!(fe_to_b32_normalized(&mut r), expected);
        }
    }
}
