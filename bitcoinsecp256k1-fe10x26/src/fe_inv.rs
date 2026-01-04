// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_inv.rs ]
crate::ix!();

pub fn fe_inv(
    r: *mut Fe10x26,
    x: *const Fe10x26)  {

    unsafe {
        let mut tmp: Fe10x26 = core::ptr::read(x);
        let mut s: ModInv32Signed30 = core::mem::zeroed();

        fe_normalize(&mut tmp as *mut Fe10x26);
        fe_to_signed30(&mut s as *mut ModInv32Signed30, &tmp as *const Fe10x26);
        modinv32(&mut s as *mut ModInv32Signed30, &*const_modinfo_fe as *const ModInv32ModInfo);
        fe_from_signed30(r, &s as *const ModInv32Signed30);

        verify_check!(fe_normalizes_to_zero(r as *const Fe10x26) == fe_normalizes_to_zero(&tmp as *const Fe10x26));
    }
}

#[cfg(test)]
mod fe_inv_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    fn inv_mul_is_one(bytes: &[u8; 32]) {
        let a = fe_from_be_bytes_checked(bytes);

        let mut inv = Fe10x26::new();
        unsafe { fe_inv(&mut inv as *mut Fe10x26, &a as *const Fe10x26) };

        let got = fe_mul_to_words_le_normalized(&a, &inv);
        let expected = words_le_from_be_bytes(&BYTES_ONE);

        debug!(?got, ?expected, "a * inv(a)");
        assert_eq!(got, expected);
    }

    #[traced_test]
    fn fe_inv_of_zero_is_zero() {
        info!("fe_inv(0) should normalize to 0");
        let zero = fe_from_be_bytes_checked(&BYTES_ZERO);

        let mut inv = Fe10x26::new();
        unsafe { fe_inv(&mut inv as *mut Fe10x26, &zero as *const Fe10x26) };

        let out = fe_to_be_bytes_normalized(&mut inv);
        assert_eq!(out, BYTES_ZERO);
    }

    #[traced_test]
    fn fe_inv_of_one_is_one() {
        info!("fe_inv(1) should normalize to 1");
        let one = fe_from_be_bytes_checked(&BYTES_ONE);

        let mut inv = Fe10x26::new();
        unsafe { fe_inv(&mut inv as *mut Fe10x26, &one as *const Fe10x26) };

        let out = fe_to_be_bytes_normalized(&mut inv);
        assert_eq!(out, BYTES_ONE);
    }

    #[traced_test]
    fn fe_inv_multiplicative_property_holds_for_representative_vectors() {
        info!("for non-zero inputs, a * inv(a) == 1 (mod p)");
        inv_mul_is_one(&BYTES_TWO);
        inv_mul_is_one(&BYTES_THREE);
        inv_mul_is_one(&BYTES_PATTERN_A);
        inv_mul_is_one(&BYTES_2_POW_255);
        inv_mul_is_one(&FIELD_PRIME_MINUS_TWO_BYTES_BE);
    }

    #[traced_test]
    fn fe_inv_does_not_mutate_input() {
        info!("fe_inv should not mutate x input");
        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let mut a_copy = fe_clone_value(&a);

        let mut inv = Fe10x26::new();
        unsafe { fe_inv(&mut inv as *mut Fe10x26, &a as *const Fe10x26) };

        let after = fe_to_be_bytes_normalized(&mut a_copy);
        trace!(?after, "input bytes after fe_inv");
        assert_eq!(after, BYTES_PATTERN_A);
    }
}

#[cfg(test)]
mod fe_inv_additional_identities_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info, trace};

    fn inv_then_inv_roundtrips(bytes: &[u8; 32]) {
        let a = fe_from_be_bytes_checked(bytes);

        let mut inv = Fe10x26::new();
        unsafe { fe_inv(&mut inv as *mut Fe10x26, &a as *const Fe10x26) };

        let mut inv_inv = Fe10x26::new();
        unsafe { fe_inv(&mut inv_inv as *mut Fe10x26, &inv as *const Fe10x26) };

        let a_norm = {
            let mut tmp = fe_clone_value(&a);
            fe_to_be_bytes_normalized(&mut tmp)
        };
        let inv_inv_norm = fe_to_be_bytes_normalized(&mut inv_inv);

        trace!(?a_norm, ?inv_inv_norm, "inv(inv(a)) vs a");
        assert_eq!(inv_inv_norm, a_norm);
    }

    #[traced_test]
    fn fe_inv_is_involution_for_representative_nonzero_values() {
        info!("checking inv(inv(a)) == a for representative nonzero elements");
        inv_then_inv_roundtrips(&BYTES_ONE);
        inv_then_inv_roundtrips(&BYTES_TWO);
        inv_then_inv_roundtrips(&BYTES_THREE);
        inv_then_inv_roundtrips(&BYTES_PATTERN_A);
        inv_then_inv_roundtrips(&FIELD_PRIME_MINUS_TWO_BYTES_BE);
    }

    #[traced_test]
    fn fe_inv_of_p_minus_one_is_p_minus_one() {
        info!("checking inv(p-1) == (p-1)");
        let a = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);

        let mut inv = Fe10x26::new();
        unsafe { fe_inv(&mut inv as *mut Fe10x26, &a as *const Fe10x26) };

        let out = fe_to_be_bytes_normalized(&mut inv);
        debug!(?out, "inv(p-1)");
        assert_eq!(out, FIELD_PRIME_MINUS_ONE_BYTES_BE);
    }
}
