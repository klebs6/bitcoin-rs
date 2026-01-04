// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_add.rs ]
crate::ix!();

#[inline] pub fn fe_add(
    r: *mut Fe10x26,
    a: *const Fe10x26)  {

    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            fe_verify(a);
        }

        (*r).n[0] = (*r).n[0].wrapping_add((*a).n[0]);
        (*r).n[1] = (*r).n[1].wrapping_add((*a).n[1]);
        (*r).n[2] = (*r).n[2].wrapping_add((*a).n[2]);
        (*r).n[3] = (*r).n[3].wrapping_add((*a).n[3]);
        (*r).n[4] = (*r).n[4].wrapping_add((*a).n[4]);
        (*r).n[5] = (*r).n[5].wrapping_add((*a).n[5]);
        (*r).n[6] = (*r).n[6].wrapping_add((*a).n[6]);
        (*r).n[7] = (*r).n[7].wrapping_add((*a).n[7]);
        (*r).n[8] = (*r).n[8].wrapping_add((*a).n[8]);
        (*r).n[9] = (*r).n[9].wrapping_add((*a).n[9]);

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = (*r).magnitude.wrapping_add((*a).magnitude);
            (*r).normalized = 0;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_add_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    #[traced_test]
    fn fe_add_with_zero_is_identity_after_normalization() {
        info!("fe_add(r, 0) should not change value");
        let mut r = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let a0 = fe_from_be_bytes_checked(&BYTES_ZERO);

        fe_add_in_place(&mut r, &a0);
        let out = fe_to_be_bytes_normalized(&mut r);

        debug!(?out, "result bytes");
        assert_eq!(out, BYTES_PATTERN_A);
    }

    #[traced_test]
    fn fe_add_allows_self_alias_and_doubles_value_mod_p() {
        info!("fe_add(r, r) should double r (self-alias via raw pointers)");
        let mut r = fe_from_be_bytes_checked(&BYTES_2_POW_255);
        let mut r_before = fe_clone_value(&r);

        let before_words = fe_to_words_le_normalized(&mut r_before);

        let rptr = &mut r as *mut Fe10x26;
        unsafe { fe_add(rptr, rptr as *const Fe10x26) };

        let got_words = fe_to_words_le_normalized(&mut r);
        let expected_words = add_mod_p(&before_words, &before_words);

        trace!(?before_words, ?expected_words, ?got_words, "doubling check");
        assert_eq!(got_words, expected_words);
    }

    #[traced_test]
    fn fe_add_wraps_p_minus_one_plus_one_to_zero_after_normalization() {
        info!("(p-1) + 1 should normalize to 0");
        let mut r = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);
        let one = fe_from_be_bytes_checked(&BYTES_ONE);

        fe_add_in_place(&mut r, &one);

        let out = fe_to_be_bytes_normalized(&mut r);
        debug!(?out, "normalized result");
        assert_eq!(out, BYTES_ZERO);
    }

    #[traced_test]
    fn fe_add_p_minus_one_twice_normalizes_to_p_minus_two() {
        info!("(p-1) + (p-1) == p-2 (mod p)");
        let mut r = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);
        let a = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);

        fe_add_in_place(&mut r, &a);

        let out = fe_to_be_bytes_normalized(&mut r);
        debug!(?out, "normalized result");
        assert_eq!(out, FIELD_PRIME_MINUS_TWO_BYTES_BE);
    }

    #[traced_test]
    fn fe_add_updates_verify_metadata_when_enabled() {
        info!("under secp256k1-verify, fe_add should update magnitude and clear normalized");
        let mut r = fe_from_be_bytes_checked(&BYTES_FIVE);
        let a = fe_from_be_bytes_checked(&BYTES_THREE);

        fe_add_in_place(&mut r, &a);

        #[cfg(feature = "secp256k1-verify")]
        {
            debug!(magnitude = r.magnitude, normalized = r.normalized, "post-fe_add metadata");
            assert_eq!(r.normalized, 0);
            assert_eq!(r.magnitude, 2);
        }
    }
}
