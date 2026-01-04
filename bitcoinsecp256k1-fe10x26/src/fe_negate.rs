// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_negate.rs ]
crate::ix!();

#[inline] pub fn fe_negate(
    r: *mut Fe10x26,
    a: *const Fe10x26,
    m: i32)  {

    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!((*a).magnitude <= m);
            fe_verify(a);
        }

        let mm: u64 = (m as u64).wrapping_add(1);
        let f: u64 = mm.wrapping_mul(2);

        (*r).n[0] = (0x3FFFC2Fu64.wrapping_mul(f).wrapping_sub((*a).n[0] as u64)) as u32;
        (*r).n[1] = (0x3FFFFBFu64.wrapping_mul(f).wrapping_sub((*a).n[1] as u64)) as u32;
        (*r).n[2] = (0x3FFFFFFu64.wrapping_mul(f).wrapping_sub((*a).n[2] as u64)) as u32;
        (*r).n[3] = (0x3FFFFFFu64.wrapping_mul(f).wrapping_sub((*a).n[3] as u64)) as u32;
        (*r).n[4] = (0x3FFFFFFu64.wrapping_mul(f).wrapping_sub((*a).n[4] as u64)) as u32;
        (*r).n[5] = (0x3FFFFFFu64.wrapping_mul(f).wrapping_sub((*a).n[5] as u64)) as u32;
        (*r).n[6] = (0x3FFFFFFu64.wrapping_mul(f).wrapping_sub((*a).n[6] as u64)) as u32;
        (*r).n[7] = (0x3FFFFFFu64.wrapping_mul(f).wrapping_sub((*a).n[7] as u64)) as u32;
        (*r).n[8] = (0x3FFFFFFu64.wrapping_mul(f).wrapping_sub((*a).n[8] as u64)) as u32;
        (*r).n[9] = (0x03FFFFFu64.wrapping_mul(f).wrapping_sub((*a).n[9] as u64)) as u32;

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = m.wrapping_add(1);
            (*r).normalized = 0;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_negate_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    #[traced_test]
    fn fe_negate_zero_is_zero_after_normalization() {
        info!("negating 0 yields 0");
        let z = fe_from_be_bytes_checked(&BYTES_ZERO);
        let got = fe_negate_to_words_le_normalized(&z, 1);
        assert_eq!(got, [0u32; 8]);
    }

    #[traced_test]
    fn fe_negate_one_is_p_minus_one_after_normalization() {
        info!("negating 1 yields p-1");
        let one = fe_from_be_bytes_checked(&BYTES_ONE);
        let got = fe_negate_to_words_le_normalized(&one, 1);
        let expected = words_le_from_be_bytes(&FIELD_PRIME_MINUS_ONE_BYTES_BE);

        debug!(?got, ?expected, "neg(1)");
        assert_eq!(got, expected);
    }

    #[traced_test]
    fn fe_negate_p_minus_one_is_one_after_normalization() {
        info!("negating (p-1) yields 1");
        let pm1 = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);
        let got = fe_negate_to_words_le_normalized(&pm1, 1);
        let expected = words_le_from_be_bytes(&BYTES_ONE);

        assert_eq!(got, expected);
    }

    #[traced_test]
    fn fe_negate_adds_back_to_zero_mod_p_for_representative_value() {
        info!("(-a) + a == 0 mod p after normalization");
        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);

        let mut neg = Fe10x26::new();
        unsafe { fe_negate(&mut neg as *mut Fe10x26, &a as *const Fe10x26, 1) };

        fe_add_in_place(&mut neg, &a);
        let out = fe_to_be_bytes_normalized(&mut neg);

        trace!(?out, "(-a)+a normalized bytes");
        assert_eq!(out, BYTES_ZERO);
    }

    #[traced_test]
    fn fe_negate_matches_reference_mod_p() {
        info!("fe_negate should match reference negation mod p after normalization");
        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let got = fe_negate_to_words_le_normalized(&a, 1);

        let a_words = words_le_from_be_bytes(&BYTES_PATTERN_A);
        let expected = neg_mod_p(&a_words);

        debug!(?got, ?expected, "negation reference compare");
        assert_eq!(got, expected);
    }

    #[traced_test]
    fn fe_negate_sets_verify_metadata_when_enabled() {
        info!("under secp256k1-verify, fe_negate sets magnitude=m+1 normalized=0");
        let a = fe_from_be_bytes_checked(&BYTES_TWO);

        let mut r = Fe10x26::new();
        unsafe { fe_negate(&mut r as *mut Fe10x26, &a as *const Fe10x26, 5) };

        #[cfg(feature = "secp256k1-verify")]
        {
            assert_eq!(r.magnitude, 6);
            assert_eq!(r.normalized, 0);
        }
    }
}
