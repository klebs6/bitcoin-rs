// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_clear.rs ]
crate::ix!();

#[inline] pub fn fe_clear(a: *mut Fe5x52)  {

    unsafe {
        let mut i: i32;

        #[cfg(feature="secp256k1-verify")]
        {
            (*a).magnitude = 0;
            (*a).normalized = 1;
        }

        i = 0;
        while i < 5 {
            (*a).n[i as usize] = 0;
            i += 1;
        }
    }
}

#[cfg(test)]
mod fe_clear_rs_exhaustive_tests {
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
    fn fe_clear_produces_canonical_zero_and_is_zero_recognizes_it() {
        tracing::info!("testing fe_clear zeroization semantics");

        unsafe {
            let mut a = fe_from_u64(123456789);
            let before = fe_get_b32_normalized(&mut a);
            assert_ne!(before, [0u8; 32]);

            crate::fe_clear(&mut a as *mut Fe5x52);

            let is_zero = crate::fe_is_zero(&a as *const Fe5x52);
            assert_eq!(is_zero, 1);

            let after = fe_get_b32_normalized(&mut a);
            assert_eq!(after, [0u8; 32]);
        }
    }
}
