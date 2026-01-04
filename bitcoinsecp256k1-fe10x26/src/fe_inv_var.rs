// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_inv_var.rs ]
crate::ix!();

pub fn fe_inv_var(
    r: *mut Fe10x26,
    x: *const Fe10x26)  {

    unsafe {
        let mut tmp: Fe10x26 = core::ptr::read(x);
        let mut s: ModInv32Signed30 = core::mem::zeroed();

        fe_normalize_var(&mut tmp as *mut Fe10x26);
        fe_to_signed30(&mut s as *mut ModInv32Signed30, &tmp as *const Fe10x26);
        modinv32_var(&mut s as *mut ModInv32Signed30, &*const_modinfo_fe as *const ModInv32ModInfo);
        fe_from_signed30(r, &s as *const ModInv32Signed30);

        verify_check!(fe_normalizes_to_zero(r as *const Fe10x26) == fe_normalizes_to_zero(&tmp as *const Fe10x26));
    }
}

#[cfg(test)]
mod fe_inv_var_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    #[traced_test]
    fn fe_inv_var_matches_fe_inv_for_representative_vectors() {
        info!("fe_inv_var and fe_inv should agree on normalized inputs");
        let vectors: [&[u8; 32]; 6] = [
            &BYTES_ONE,
            &BYTES_TWO,
            &BYTES_THREE,
            &BYTES_PATTERN_A,
            &BYTES_2_POW_255,
            &FIELD_PRIME_MINUS_TWO_BYTES_BE,
        ];

        for v in vectors {
            let a = fe_from_be_bytes_checked(v);

            let mut inv_ct = Fe10x26::new();
            unsafe { fe_inv(&mut inv_ct as *mut Fe10x26, &a as *const Fe10x26) };

            let mut inv_vt = Fe10x26::new();
            unsafe { fe_inv_var(&mut inv_vt as *mut Fe10x26, &a as *const Fe10x26) };

            let out_ct = fe_to_be_bytes_normalized(&mut inv_ct);
            let out_vt = fe_to_be_bytes_normalized(&mut inv_vt);

            debug!(?out_ct, ?out_vt, "inv vs inv_var");
            assert_eq!(out_ct, out_vt);
        }
    }

    #[traced_test]
    fn fe_inv_var_of_zero_is_zero() {
        info!("fe_inv_var(0) should normalize to 0");
        let zero = fe_from_be_bytes_checked(&BYTES_ZERO);

        let mut inv = Fe10x26::new();
        unsafe { fe_inv_var(&mut inv as *mut Fe10x26, &zero as *const Fe10x26) };

        let out = fe_to_be_bytes_normalized(&mut inv);
        assert_eq!(out, BYTES_ZERO);
    }
}
