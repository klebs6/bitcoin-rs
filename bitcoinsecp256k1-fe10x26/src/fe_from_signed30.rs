// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_from_signed30.rs ]
crate::ix!();

pub fn fe_from_signed30(
    r: *mut Fe10x26,
    a: *const ModInv32Signed30)  {

    unsafe {
        let m26: u32 = u32::MAX >> 6;

        let a0: u32 = (*a).v[0] as u32;
        let a1: u32 = (*a).v[1] as u32;
        let a2: u32 = (*a).v[2] as u32;
        let a3: u32 = (*a).v[3] as u32;
        let a4: u32 = (*a).v[4] as u32;
        let a5: u32 = (*a).v[5] as u32;
        let a6: u32 = (*a).v[6] as u32;
        let a7: u32 = (*a).v[7] as u32;
        let a8: u32 = (*a).v[8] as u32;

        /* The output from modinv32{_var} should be normalized to range [0,modulus), and
         * have limbs in [0,2^30). The modulus is < 2^256, so the top limb must be below 2^(256-30*8).
         */
        verify_check!((a0 >> 30) == 0);
        verify_check!((a1 >> 30) == 0);
        verify_check!((a2 >> 30) == 0);
        verify_check!((a3 >> 30) == 0);
        verify_check!((a4 >> 30) == 0);
        verify_check!((a5 >> 30) == 0);
        verify_check!((a6 >> 30) == 0);
        verify_check!((a7 >> 30) == 0);
        verify_check!((a8 >> 16) == 0);

        (*r).n[0] =  a0                         & m26;
        (*r).n[1] = (a0 >> 26 | a1 <<  4)       & m26;
        (*r).n[2] = (a1 >> 22 | a2 <<  8)       & m26;
        (*r).n[3] = (a2 >> 18 | a3 << 12)       & m26;
        (*r).n[4] = (a3 >> 14 | a4 << 16)       & m26;
        (*r).n[5] = (a4 >> 10 | a5 << 20)       & m26;
        (*r).n[6] = (a5 >>  6 | a6 << 24)       & m26;
        (*r).n[7] = (a6 >>  2)                  & m26;
        (*r).n[8] = (a6 >> 28 | a7 <<  2)       & m26;
        (*r).n[9] = (a7 >> 24 | a8 <<  6);

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = 1;
            (*r).normalized = 1;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_from_signed30_roundtrip_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    fn roundtrip_bytes_via_signed30(bytes: &[u8; 32]) {
        let mut a = fe_from_be_bytes_checked(bytes);

        let mut s: ModInv32Signed30 = unsafe { core::mem::zeroed() };
        unsafe { fe_to_signed30(&mut s as *mut ModInv32Signed30, &a as *const Fe10x26) };

        debug!(?s.v, "signed30 limbs");
        for (i, limb) in s.v.iter().enumerate() {
            let u = *limb as u32;
            if i < 8 {
                assert_eq!(u >> 30, 0u32);
            } else {
                assert_eq!(u >> 16, 0u32);
            }
        }

        let mut b = Fe10x26::new();
        unsafe { fe_from_signed30(&mut b as *mut Fe10x26, &s as *const ModInv32Signed30) };

        let out = fe_to_be_bytes_normalized(&mut b);
        trace!(?out, "roundtrip bytes");
        assert_eq!(&out, bytes);
    }

    #[traced_test]
    fn fe_to_signed30_then_fe_from_signed30_roundtrips_multiple_vectors() {
        info!("roundtripping representative values through signed30 representation");
        roundtrip_bytes_via_signed30(&BYTES_ZERO);
        roundtrip_bytes_via_signed30(&BYTES_ONE);
        roundtrip_bytes_via_signed30(&BYTES_2_POW_255);
        roundtrip_bytes_via_signed30(&BYTES_PATTERN_A);
        roundtrip_bytes_via_signed30(&FIELD_PRIME_MINUS_ONE_BYTES_BE);
    }

    #[traced_test]
    fn fe_from_signed30_sets_verify_metadata_when_enabled() {
        info!("under secp256k1-verify, fe_from_signed30 sets magnitude=1 normalized=1");
        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);

        let mut s: ModInv32Signed30 = unsafe { core::mem::zeroed() };
        unsafe { fe_to_signed30(&mut s as *mut ModInv32Signed30, &a as *const Fe10x26) };

        let mut r = Fe10x26::new();
        unsafe { fe_from_signed30(&mut r as *mut Fe10x26, &s as *const ModInv32Signed30) };

        #[cfg(feature = "secp256k1-verify")]
        {
            assert_eq!(r.magnitude, 1);
            assert_eq!(r.normalized, 1);
        }
    }
}
