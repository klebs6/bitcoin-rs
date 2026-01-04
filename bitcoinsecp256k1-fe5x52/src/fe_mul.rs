// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_mul.rs ]
crate::ix!();

pub fn fe_mul(
        r: *mut Fe5x52,
        a: *const Fe5x52,
        b: *const Fe5x52)  {

    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!((*a).magnitude <= 8);
            verify_check!((*b).magnitude <= 8);
            fe_verify(a);
            fe_verify(b);
            verify_check!((r as *const Fe5x52) != b);
            verify_check!(a != b);
        }

        let rn: *mut u64 = core::ptr::addr_of_mut!((*r).n) as *mut u64;
        let an: *const u64 = core::ptr::addr_of!((*a).n) as *const u64;
        let bn: *const u64 = core::ptr::addr_of!((*b).n) as *const u64;

        secp_256k1_fe_mul_inner(rn, an, bn);

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = 1;
            (*r).normalized = 0;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_mul_rs_exhaustive_tests {
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
        fe_from_b32_checked(&u64_to_be32(v))
    }

    unsafe fn fe_to_b32_normalized(fe: &mut Fe5x52) -> [u8; 32] {
        crate::fe_normalize(fe as *mut Fe5x52);
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), fe as *const Fe5x52);
        out
    }

    unsafe fn fe_mul_and_get(a: &Fe5x52, b: &Fe5x52) -> [u8; 32] {
        let mut r = Fe5x52::new();
        crate::fe_mul(&mut r as *mut Fe5x52, a as *const Fe5x52, b as *const Fe5x52);
        fe_to_b32_normalized(&mut r)
    }

    #[traced_test]
    fn fe_mul_matches_known_products_for_small_values_and_known_field_extremes() {
        tracing::info!("testing fe_mul for small exact products and selected modulus-derived identities");

        unsafe {
            let small_cases: &[(u64, u64)] = &[
                (0, 0),
                (0, 1),
                (1, 0),
                (1, 1),
                (2, 2),
                (2, 3),
                (3, 2),
                (7, 11),
                (17, 19),
                (255, 256),
                (12345, 6789),
                (65536, 65536),
            ];

            for &(x, y) in small_cases.iter() {
                tracing::debug!(x_u64 = x, y_u64 = y, "testing exact u128 product");
                let a = fe_from_u64(x);
                let b = fe_from_u64(y);

                let got = fe_mul_and_get(&a, &b);

                let prod = (x as u128) * (y as u128);
                assert!(prod <= u128::from(u64::MAX));
                let expected = u64_to_be32(prod as u64);

                assert_eq!(got, expected);

                let got_swapped = fe_mul_and_get(&b, &a);
                assert_eq!(got, got_swapped);
            }

            tracing::debug!("testing (-1) * (-1) == 1");
            let minus_one = fe_from_b32_checked(&FIELD_P_MINUS_1_B32);
            let got = fe_mul_and_get(&minus_one, &minus_one);
            assert_eq!(got, u64_to_be32(1));

            tracing::debug!("testing (-1) * 2 == p-2");
            let two = fe_from_u64(2);
            let got = fe_mul_and_get(&minus_one, &two);
            let expected = be32_sub(&FIELD_P_B32, &u64_to_be32(2));
            assert_eq!(got, expected);

            tracing::debug!("testing r==a aliasing (in-place output) is permitted by interface");
            let b = fe_from_u64(9);
            let mut a_inplace = fe_from_u64(7);
            crate::fe_mul(&mut a_inplace as *mut Fe5x52, &a_inplace as *const Fe5x52, &b as *const Fe5x52);
            let got_inplace = fe_to_b32_normalized(&mut a_inplace);
            assert_eq!(got_inplace, u64_to_be32(63));
        }
    }
}
