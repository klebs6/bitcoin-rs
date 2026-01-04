// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_verify.rs ]
crate::ix!();

#[cfg(feature="secp256k1-verify")]
pub fn fe_verify(a: *const Fe5x52)  {

    unsafe {
        let d0: u64 = (*a).n[0];
        let d1: u64 = (*a).n[1];
        let d2: u64 = (*a).n[2];
        let d3: u64 = (*a).n[3];
        let d4: u64 = (*a).n[4];

        let m: i32 = if (*a).normalized != 0 { 1 } else { (*a).magnitude.wrapping_mul(2) };
        let mut r: i32 = 1;

        /* secp256k1 'p' value defined in "Standards for Efficient Cryptography" (SEC2) 2.7.1. */
        let mu: u64 = m as u64;

        r &= ((d0 <= 0xFFFFFFFFFFFFF_u64.wrapping_mul(mu)) as i32);
        r &= ((d1 <= 0xFFFFFFFFFFFFF_u64.wrapping_mul(mu)) as i32);
        r &= ((d2 <= 0xFFFFFFFFFFFFF_u64.wrapping_mul(mu)) as i32);
        r &= ((d3 <= 0xFFFFFFFFFFFFF_u64.wrapping_mul(mu)) as i32);
        r &= ((d4 <= 0x0FFFFFFFFFFFF_u64.wrapping_mul(mu)) as i32);
        r &= (((*a).magnitude >= 0) as i32);
        r &= (((*a).magnitude <= 2048) as i32);

        if (*a).normalized != 0 {
            r &= (((*a).magnitude <= 1) as i32);
            if (r != 0)
                & (d4 == 0x0FFFFFFFFFFFF_u64)
                & (((d3 & d2 & d1) == 0xFFFFFFFFFFFFF_u64))
            {
                r &= ((d0 < 0xFFFFEFFFFFC2F_u64) as i32);
            }
        }

        verify_check!(r == 1);
    }
}

#[cfg(all(test, feature = "secp256k1-verify"))]
mod fe_verify_rs_exhaustive_tests {
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
    fn fe_verify_accepts_known_good_normalized_values_under_verify_feature() {
        tracing::info!("testing fe_verify success path on known-good normalized inputs");

        unsafe {
            let samples: [&[u8; 32]; 4] = [&u64_to_be32(0), &u64_to_be32(1), &u64_to_be32(2), &u64_to_be32(65536)];

            for (idx, s) in samples.iter().enumerate() {
                tracing::debug!(sample_index = idx, "verifying");
                let a = fe_from_b32_checked(s);
                crate::fe_verify(&a as *const Fe5x52);
            }
        }
    }
}
