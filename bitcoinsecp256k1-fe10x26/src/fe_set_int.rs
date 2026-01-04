// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_set_int.rs ]
crate::ix!();

#[inline] pub fn fe_set_int(
    r: *mut Fe10x26,
    a: i32)  {

    unsafe {
        (*r).n[0] = a as u32;
        (*r).n[1] = 0;
        (*r).n[2] = 0;
        (*r).n[3] = 0;
        (*r).n[4] = 0;
        (*r).n[5] = 0;
        (*r).n[6] = 0;
        (*r).n[7] = 0;
        (*r).n[8] = 0;
        (*r).n[9] = 0;

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = 1;
            (*r).normalized = 1;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_set_int_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info};

    fn set_int_and_get_bytes(v: u32) -> [u8; 32] {
        let mut r = Fe10x26::new();
        unsafe { fe_set_int(&mut r as *mut Fe10x26, v as i32) };
        fe_to_be_bytes_normalized(&mut r)
    }

    #[traced_test]
    fn fe_set_int_sets_small_values_correctly() {
        info!("fe_set_int should set small values canonically");
        let z = set_int_and_get_bytes(0);
        let o = set_int_and_get_bytes(1);
        let t = set_int_and_get_bytes(2);

        debug!(?z, ?o, ?t, "set_int outputs");
        assert_eq!(z, BYTES_ZERO);
        assert_eq!(o, BYTES_ONE);
        assert_eq!(t, BYTES_TWO);
    }

    #[traced_test]
    fn fe_set_int_sets_verify_metadata_when_enabled() {
        info!("under secp256k1-verify, fe_set_int sets magnitude=1 normalized=1");
        let mut r = Fe10x26::new();
        unsafe { fe_set_int(&mut r as *mut Fe10x26, 7) };

        #[cfg(feature = "secp256k1-verify")]
        {
            assert_eq!(r.magnitude, 1);
            assert_eq!(r.normalized, 1);
        }
    }
}
