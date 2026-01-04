// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_sqr.rs ]
crate::ix!();

pub fn fe_sqr(
        r: *mut Fe5x52,
        a: *const Fe5x52)  {

    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!((*a).magnitude <= 8);
            fe_verify(a);
        }

        let rn: *mut u64 = core::ptr::addr_of_mut!((*r).n) as *mut u64;
        let an: *const u64 = core::ptr::addr_of!((*a).n) as *const u64;

        secp_256k1_fe_sqr_inner(rn, an);

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = 1;
            (*r).normalized = 0;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_sqr_rs_exhaustive_tests {
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
    fn fe_sqr_agrees_with_fe_mul_self_for_representative_values() {
        tracing::info!("testing fe_sqr equals fe_mul(a,a) for representative inputs");

        unsafe {
            let vals: [u64; 10] = [0, 1, 2, 3, 5, 7, 11, 255, 1024, 65536];

            for &v in vals.iter() {
                tracing::debug!(value_u64 = v, "square comparison");
                let a = fe_from_u64(v);

                let mut sq = Fe5x52::new();
                crate::fe_sqr(&mut sq as *mut Fe5x52, &a as *const Fe5x52);

                let mut mul = Fe5x52::new();
                crate::fe_mul(&mut mul as *mut Fe5x52, &a as *const Fe5x52, &a as *const Fe5x52);

                let sq_b = fe_to_b32_normalized(&mut sq);
                let mul_b = fe_to_b32_normalized(&mut mul);

                assert_eq!(sq_b, mul_b);

                let expected = (v as u128) * (v as u128);
                assert!(expected <= u128::from(u64::MAX));
                assert_eq!(sq_b, u64_to_be32(expected as u64));
            }
        }
    }
}
