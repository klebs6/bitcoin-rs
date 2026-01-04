// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_is_odd.rs ]
crate::ix!();

#[inline] pub fn fe_is_odd(a: *const Fe10x26) -> i32 {
    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!((*a).normalized != 0);
            fe_verify(a);
        }

        ((*a).n[0] & 1) as i32
    }
}

#[cfg(test)]
mod fe_is_odd_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    #[traced_test]
    fn fe_is_odd_reports_expected_parity_for_boundary_values() {
        info!("checking parity for 0,1,2,p-1,p-2");
        let mut z = fe_from_be_bytes_checked(&BYTES_ZERO);
        let mut o = fe_from_be_bytes_checked(&BYTES_ONE);
        let mut t = fe_from_be_bytes_checked(&BYTES_TWO);
        let mut pm1 = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);
        let mut pm2 = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_TWO_BYTES_BE);

        fe_normalize_in_place(&mut z);
        fe_normalize_in_place(&mut o);
        fe_normalize_in_place(&mut t);
        fe_normalize_in_place(&mut pm1);
        fe_normalize_in_place(&mut pm2);

        let z_odd = unsafe { fe_is_odd(&z as *const Fe10x26) };
        let o_odd = unsafe { fe_is_odd(&o as *const Fe10x26) };
        let t_odd = unsafe { fe_is_odd(&t as *const Fe10x26) };
        let pm1_odd = unsafe { fe_is_odd(&pm1 as *const Fe10x26) };
        let pm2_odd = unsafe { fe_is_odd(&pm2 as *const Fe10x26) };

        debug!(z_odd, o_odd, t_odd, pm1_odd, pm2_odd, "parity bits");
        assert_eq!(z_odd, 0);
        assert_eq!(o_odd, 1);
        assert_eq!(t_odd, 0);
        assert_eq!(pm1_odd, 0);
        assert_eq!(pm2_odd, 1);
    }
}
