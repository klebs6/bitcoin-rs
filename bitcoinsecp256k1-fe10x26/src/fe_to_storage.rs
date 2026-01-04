// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_to_storage.rs ]
crate::ix!();

pub fn fe_to_storage(
    r: *mut Fe10x26Storage,
    a: *const Fe10x26)  {

    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!((*a).normalized != 0);
        }

        (*r).n[0] = (*a).n[0] | ((*a).n[1] << 26);
        (*r).n[1] = ((*a).n[1] >>  6) | ((*a).n[2] << 20);
        (*r).n[2] = ((*a).n[2] >> 12) | ((*a).n[3] << 14);
        (*r).n[3] = ((*a).n[3] >> 18) | ((*a).n[4] <<  8);
        (*r).n[4] = ((*a).n[4] >> 24) | ((*a).n[5] <<  2) | ((*a).n[6] << 28);
        (*r).n[5] = ((*a).n[6] >>  4) | ((*a).n[7] << 22);
        (*r).n[6] = ((*a).n[7] >> 10) | ((*a).n[8] << 16);
        (*r).n[7] = ((*a).n[8] >> 16) | ((*a).n[9] << 10);
    }
}

#[cfg(test)]
mod fe_to_storage_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn fe_to_storage_and_back_roundtrips_representative_values() {
        info!("fe_to_storage then fe_from_storage should roundtrip values");
        let vectors: [&[u8; 32]; 5] = [
            &BYTES_ZERO,
            &BYTES_ONE,
            &BYTES_PATTERN_A,
            &BYTES_2_POW_255,
            &FIELD_PRIME_MINUS_ONE_BYTES_BE,
        ];

        for v in vectors {
            let mut a = fe_from_be_bytes_checked(v);
            fe_normalize_in_place(&mut a);

            let mut stor = Fe10x26Storage { n: [0u32; 8] };
            unsafe { fe_to_storage(&mut stor as *mut Fe10x26Storage, &a as *const Fe10x26) };

            let mut b = Fe10x26::new();
            unsafe { fe_from_storage(&mut b as *mut Fe10x26, &stor as *const Fe10x26Storage) };

            let out = fe_to_be_bytes_normalized(&mut b);
            debug!(?out, "storage roundtrip bytes");
            assert_eq!(&out, v);
        }
    }
}
