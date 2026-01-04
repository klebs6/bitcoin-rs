// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_is_zero.rs ]
crate::ix!();

#[inline] pub fn fe_is_zero(a: *const Fe5x52) -> i32 {

    unsafe {
        let t = (*a).n;

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check((*a).normalized != 0);
            fe_verify(a);
        }

        (((t[0] | t[1] | t[2] | t[3] | t[4]) == 0) as i32)
    }
}

#[cfg(test)]
mod fe_is_zero_rs_exhaustive_tests {
    use super::*;

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

    #[traced_test]
    fn fe_is_zero_reports_zero_for_canonical_zero_and_nonzero_otherwise() {
        tracing::info!("testing fe_is_zero for canonical zero and simple nonzero");

        unsafe {
            let mut z = fe_from_b32_checked(&u64_to_be32(0));
            crate::fe_normalize(&mut z as *mut Fe5x52);
            assert_eq!(crate::fe_is_zero(&z as *const Fe5x52), 1);

            let mut one = fe_from_b32_checked(&u64_to_be32(1));
            crate::fe_normalize(&mut one as *mut Fe5x52);
            assert_eq!(crate::fe_is_zero(&one as *const Fe5x52), 0);

            let mut a = fe_from_b32_checked(&u64_to_be32(65536));
            crate::fe_normalize(&mut a as *mut Fe5x52);
            assert_eq!(crate::fe_is_zero(&a as *const Fe5x52), 0);
        }
    }
}
