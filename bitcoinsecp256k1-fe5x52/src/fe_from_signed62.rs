// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_from_signed62.rs ]
crate::ix!();

pub fn fe_from_signed62(
        r: *mut Fe5x52,
        a: *const ModInv64Signed62)  {

    unsafe {
        const M52: u64 = u64::MAX >> 12;

        let av: *const i64 = a as *const i64;
        let a0: u64 = *av.add(0) as u64;
        let a1: u64 = *av.add(1) as u64;
        let a2: u64 = *av.add(2) as u64;
        let a3: u64 = *av.add(3) as u64;
        let a4: u64 = *av.add(4) as u64;

        /* The output from modinv64{_var} should be normalized to range [0,modulus), and
         * have limbs in [0,2^62). The modulus is < 2^256, so the top limb must be below 2^(256-62*4).
         */
        verify_check!((a0 >> 62) == 0);
        verify_check!((a1 >> 62) == 0);
        verify_check!((a2 >> 62) == 0);
        verify_check!((a3 >> 62) == 0);
        verify_check!((a4 >> 8) == 0);

        (*r).n[0] = ( a0                   ) & M52;
        (*r).n[1] = ((a0 >> 52) | (a1 << 10)) & M52;
        (*r).n[2] = ((a1 >> 42) | (a2 << 20)) & M52;
        (*r).n[3] = ((a2 >> 32) | (a3 << 30)) & M52;
        (*r).n[4] = ((a3 >> 22) | (a4 << 40));

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = 1;
            (*r).normalized = 1;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_from_signed62_rs_exhaustive_tests {
    use super::*;

    const SAMPLE_B32: [u8; 32] = [
        0x02, 0x46, 0x8A, 0xCE, 0x10, 0x32, 0x54, 0x76,
        0x98, 0xBA, 0xDC, 0xFE, 0x01, 0x23, 0x45, 0x67,
        0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98,
        0x76, 0x54, 0x32, 0x10, 0x00, 0x00, 0x00, 0x11,
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
    fn signed62_conversion_roundtrips_through_fe_to_signed62_and_fe_from_signed62() {
        tracing::info!("testing fe_to_signed62 -> fe_from_signed62 roundtrips");

        unsafe {
            let samples: [&[u8; 32]; 4] = [&u64_to_be32(0), &u64_to_be32(1), &SAMPLE_B32, &FIELD_P_MINUS_1_B32];

            for (idx, s) in samples.iter().enumerate() {
                tracing::debug!(sample_index = idx, "roundtripping one sample");

                let mut a = fe_from_b32_checked(s);
                crate::fe_normalize(&mut a as *mut Fe5x52);

                let mut sig = core::mem::MaybeUninit::<ModInv64Signed62>::uninit();
                crate::fe_to_signed62(sig.as_mut_ptr(), &a as *const Fe5x52);

                let mut b = Fe5x52::new();
                crate::fe_from_signed62(&mut b as *mut Fe5x52, sig.as_ptr());

                let got = fe_to_b32_normalized(&mut b);
                let expected = fe_to_b32_normalized(&mut a);

                assert_eq!(got, expected);
            }
        }
    }
}
