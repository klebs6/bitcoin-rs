// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_mul_int.rs ]
crate::ix!();

#[inline] pub fn fe_mul_int(
        r: *mut Fe5x52,
        a: i32)  {

    unsafe {
        let aa: u64 = a as u64;

        (*r).n[0] = (*r).n[0].wrapping_mul(aa);
        (*r).n[1] = (*r).n[1].wrapping_mul(aa);
        (*r).n[2] = (*r).n[2].wrapping_mul(aa);
        (*r).n[3] = (*r).n[3].wrapping_mul(aa);
        (*r).n[4] = (*r).n[4].wrapping_mul(aa);

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = (*r).magnitude.wrapping_mul(a);
            (*r).normalized = 0;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_mul_int_rs_exhaustive_tests {
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

    unsafe fn fe_to_b32_normalized(fe: &mut Fe5x52) -> [u8; 32] {
        crate::fe_normalize(fe as *mut Fe5x52);
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), fe as *const Fe5x52);
        out
    }

    #[traced_test]
    fn fe_mul_int_scales_by_small_nonnegative_integers_with_expected_results() {
        tracing::info!("testing fe_mul_int scaling behavior for small nonnegative factors");

        unsafe {
            let cases: &[(u64, i32, u64)] = &[
                (0, 0, 0),
                (0, 1, 0),
                (1, 0, 0),
                (1, 1, 1),
                (2, 3, 6),
                (7, 9, 63),
                (12345, 0, 0),
                (12345, 2, 24690),
                (65536, 10, 655360),
            ];

            for &(x, m, expected) in cases.iter() {
                tracing::debug!(x_u64 = x, mul_i32 = m, expected_u64 = expected, "scaling test");
                let mut r = fe_from_u64(x);
                crate::fe_mul_int(&mut r as *mut Fe5x52, m);
                let got = fe_to_b32_normalized(&mut r);
                assert_eq!(got, u64_to_be32(expected));
            }
        }
    }
}
