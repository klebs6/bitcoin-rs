// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_mul_int.rs ]
crate::ix!();

#[inline] pub fn fe_mul_int(
    r: *mut Fe10x26,
    a: i32)  {

    unsafe {
        let aa: u32 = a as u32;

        (*r).n[0] = (*r).n[0].wrapping_mul(aa);
        (*r).n[1] = (*r).n[1].wrapping_mul(aa);
        (*r).n[2] = (*r).n[2].wrapping_mul(aa);
        (*r).n[3] = (*r).n[3].wrapping_mul(aa);
        (*r).n[4] = (*r).n[4].wrapping_mul(aa);
        (*r).n[5] = (*r).n[5].wrapping_mul(aa);
        (*r).n[6] = (*r).n[6].wrapping_mul(aa);
        (*r).n[7] = (*r).n[7].wrapping_mul(aa);
        (*r).n[8] = (*r).n[8].wrapping_mul(aa);
        (*r).n[9] = (*r).n[9].wrapping_mul(aa);

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = (*r).magnitude.wrapping_mul(a);
            (*r).normalized = 0;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_mul_int_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    #[traced_test]
    fn fe_mul_int_by_zero_yields_zero() {
        info!("fe_mul_int(r,0) should zero the value");
        let mut r = fe_from_be_bytes_checked(&BYTES_PATTERN_A);

        unsafe { fe_mul_int(&mut r as *mut Fe10x26, 0) };

        let out = fe_to_be_bytes_normalized(&mut r);
        debug!(?out, "mul_int(0) output");
        assert_eq!(out, BYTES_ZERO);
    }

    #[traced_test]
    fn fe_mul_int_by_one_is_identity() {
        info!("fe_mul_int(r,1) should not change the value");
        let mut r = fe_from_be_bytes_checked(&BYTES_PATTERN_A);

        unsafe { fe_mul_int(&mut r as *mut Fe10x26, 1) };

        let out = fe_to_be_bytes_normalized(&mut r);
        assert_eq!(out, BYTES_PATTERN_A);
    }

    #[traced_test]
    fn fe_mul_int_by_two_matches_add_self() {
        info!("fe_mul_int by 2 should match r+r (mod p after normalization)");
        let mut r1 = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let mut r2 = fe_from_be_bytes_checked(&BYTES_PATTERN_A);

        unsafe { fe_mul_int(&mut r1 as *mut Fe10x26, 2) };

        let r2_copy = fe_clone_value(&r2);
        fe_add_in_place(&mut r2, &r2_copy);

        let out1 = fe_to_words_le_normalized(&mut r1);
        let out2 = fe_to_words_le_normalized(&mut r2);

        debug!(?out1, ?out2, "mul_int(2) vs add self");
        assert_eq!(out1, out2);
    }

    #[traced_test]
    fn fe_mul_int_matches_reference_scalar_multiplication() {
        info!("fe_mul_int by small scalar should match reference mul_mod_p");
        let scalar: u32 = 7;
        let mut r = fe_from_be_bytes_checked(&BYTES_PATTERN_A);

        unsafe { fe_mul_int(&mut r as *mut Fe10x26, scalar as i32) };

        let got = fe_to_words_le_normalized(&mut r);

        let a_words = words_le_from_be_bytes(&BYTES_PATTERN_A);
        let scalar_words: [u32; 8] = [scalar, 0, 0, 0, 0, 0, 0, 0];
        let expected = mul_mod_p(&a_words, &scalar_words);

        assert_eq!(got, expected);
    }

    #[traced_test]
    fn fe_mul_int_updates_verify_metadata_when_enabled() {
        info!("under secp256k1-verify, fe_mul_int updates magnitude and clears normalized");
        let mut r = fe_from_be_bytes_checked(&BYTES_TWO);

        unsafe { fe_mul_int(&mut r as *mut Fe10x26, 3) };

        #[cfg(feature = "secp256k1-verify")]
        {
            assert_eq!(r.normalized, 0);
            assert_eq!(r.magnitude, 3);
        }
    }
}
