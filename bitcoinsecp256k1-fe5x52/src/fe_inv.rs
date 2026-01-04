// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_inv.rs ]
crate::ix!();

pub fn fe_inv(
        r: *mut Fe5x52,
        x: *const Fe5x52)  {

    unsafe {
        let mut tmp: Fe5x52 = core::ptr::read(x);
        let mut s = core::mem::MaybeUninit::<ModInv64Signed62>::uninit();

        fe_normalize(&mut tmp as *mut Fe5x52);
        fe_to_signed62(s.as_mut_ptr(), &tmp as *const Fe5x52);

        modinv64(&mut *s.as_mut_ptr(), &const_modinfo_fe);

        fe_from_signed62(r, s.as_ptr());

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check(fe_normalizes_to_zero(r) == fe_normalizes_to_zero(&tmp as *const Fe5x52));
        }
    }
}

#[cfg(test)]
mod fe_inv_rs_exhaustive_tests {
    use super::*;

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

    unsafe fn fe_from_u64(v: u64) -> Fe5x52 {
        fe_from_b32_checked(&u64_to_be32(v))
    }

    unsafe fn fe_to_b32_normalized(fe: &mut Fe5x52) -> [u8; 32] {
        crate::fe_normalize(fe as *mut Fe5x52);
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), fe as *const Fe5x52);
        out
    }

    #[traced_test]
    fn fe_inv_satisfies_multiplicative_identity_for_nonzero_and_returns_zero_for_zero() {
        tracing::info!("testing fe_inv correctness identities on representative values");

        unsafe {
            let values: [u64; 12] = [0, 1, 2, 3, 5, 7, 11, 13, 17, 19, 255, 65536];

            for &v in values.iter() {
                tracing::debug!(value_u64 = v, "testing inversion identity x*inv(x)");
                let x = fe_from_u64(v);

                let mut inv = Fe5x52::new();
                crate::fe_inv(&mut inv as *mut Fe5x52, &x as *const Fe5x52);

                let mut prod = Fe5x52::new();
                crate::fe_mul(&mut prod as *mut Fe5x52, &x as *const Fe5x52, &inv as *const Fe5x52);

                let got = fe_to_b32_normalized(&mut prod);

                if v == 0 {
                    assert_eq!(got, [0u8; 32]);
                } else {
                    assert_eq!(got, u64_to_be32(1));
                }
            }

            tracing::debug!("testing inv(p-1) == p-1");
            let x = fe_from_b32_checked(&FIELD_P_MINUS_1_B32);
            let mut inv = Fe5x52::new();
            crate::fe_inv(&mut inv as *mut Fe5x52, &x as *const Fe5x52);
            assert_eq!(fe_to_b32_normalized(&mut inv), FIELD_P_MINUS_1_B32);
        }
    }
}
