// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_clear.rs ]
crate::ix!();

#[inline] pub fn fe_clear(a: *mut Fe10x26)  {
    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            (*a).magnitude = 0;
            (*a).normalized = 1;
        }

        for i in 0..10 {
            (*a).n[i] = 0;
        }
    }
}

#[cfg(test)]
mod fe_clear_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    #[traced_test]
    fn fe_clear_zeroes_all_limbs_and_yields_zero_value() {
        info!("fe_clear should zero the element");
        let mut a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);

        unsafe { fe_clear(&mut a as *mut Fe10x26) };

        let out = fe_to_be_bytes_normalized(&mut a);
        debug!(?out, "cleared bytes");
        assert_eq!(out, BYTES_ZERO);
    }

    #[traced_test]
    fn fe_clear_sets_verify_flags_when_enabled() {
        info!("under secp256k1-verify, fe_clear should set magnitude=0 and normalized=1");
        let mut a = fe_from_be_bytes_checked(&BYTES_TWO);

        unsafe { fe_clear(&mut a as *mut Fe10x26) };

        #[cfg(feature = "secp256k1-verify")]
        {
            assert_eq!(a.magnitude, 0);
            assert_eq!(a.normalized, 1);
        }
    }
}
