// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_is_zero.rs ]
crate::ix!();

#[inline] pub fn fe_is_zero(a: *const Fe10x26) -> i32 {
    unsafe {
        let t = &(*a).n;

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!((*a).normalized != 0);
            fe_verify(a);
        }

        if (t[0] | t[1] | t[2] | t[3] | t[4] | t[5] | t[6] | t[7] | t[8] | t[9]) == 0 {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod fe_is_zero_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    #[traced_test]
    fn fe_is_zero_identifies_zero_and_nonzero_values() {
        info!("checking fe_is_zero for 0,1,p-1");
        let mut z = fe_from_be_bytes_checked(&BYTES_ZERO);
        let mut o = fe_from_be_bytes_checked(&BYTES_ONE);
        let mut pm1 = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);

        fe_normalize_in_place(&mut z);
        fe_normalize_in_place(&mut o);
        fe_normalize_in_place(&mut pm1);

        let z_is = unsafe { fe_is_zero(&z as *const Fe10x26) };
        let o_is = unsafe { fe_is_zero(&o as *const Fe10x26) };
        let pm1_is = unsafe { fe_is_zero(&pm1 as *const Fe10x26) };

        debug!(z_is, o_is, pm1_is, "is_zero results");
        assert_eq!(z_is, 1);
        assert_eq!(o_is, 0);
        assert_eq!(pm1_is, 0);
    }

    #[traced_test]
    fn fe_is_zero_after_normalizing_p_yields_one() {
        info!("constructing p via (p-1)+1, normalize, then fe_is_zero==1");
        let mut r = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);
        let one = fe_from_be_bytes_checked(&BYTES_ONE);

        fe_add_in_place(&mut r, &one);
        fe_normalize_in_place(&mut r);

        let isz = unsafe { fe_is_zero(&r as *const Fe10x26) };
        assert_eq!(isz, 1);
    }
}
