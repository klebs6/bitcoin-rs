// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_from_storage.rs ]
crate::ix!();

#[inline] pub fn fe_from_storage(
    r: *mut Fe10x26,
    a: *const Fe10x26Storage)  {

    unsafe {
        (*r).n[0] = (*a).n[0] & 0x3FFFFFFu32;
        (*r).n[1] = (*a).n[0] >> 26 | (((*a).n[1] << 6)  & 0x3FFFFFFu32);
        (*r).n[2] = (*a).n[1] >> 20 | (((*a).n[2] << 12) & 0x3FFFFFFu32);
        (*r).n[3] = (*a).n[2] >> 14 | (((*a).n[3] << 18) & 0x3FFFFFFu32);
        (*r).n[4] = (*a).n[3] >>  8 | (((*a).n[4] << 24) & 0x3FFFFFFu32);
        (*r).n[5] = ((*a).n[4] >> 2) & 0x3FFFFFFu32;
        (*r).n[6] = (*a).n[4] >> 28 | (((*a).n[5] << 4)  & 0x3FFFFFFu32);
        (*r).n[7] = (*a).n[5] >> 22 | (((*a).n[6] << 10) & 0x3FFFFFFu32);
        (*r).n[8] = (*a).n[6] >> 16 | (((*a).n[7] << 16) & 0x3FFFFFFu32);
        (*r).n[9] = (*a).n[7] >> 10;

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = 1;
            (*r).normalized = 1;
        }
    }
}

#[cfg(test)]
mod fe_from_storage_roundtrip_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    fn roundtrip_storage(bytes: &[u8; 32]) {
        let mut a = fe_from_be_bytes_checked(bytes);
        fe_normalize_in_place(&mut a);

        let mut stor = Fe10x26Storage { n: [0u32; 8] };
        unsafe { fe_to_storage(&mut stor as *mut Fe10x26Storage, &a as *const Fe10x26) };

        let mut b = Fe10x26::new();
        unsafe { fe_from_storage(&mut b as *mut Fe10x26, &stor as *const Fe10x26Storage) };

        let out = fe_to_be_bytes_normalized(&mut b);
        debug!(?out, "roundtrip bytes");
        assert_eq!(&out, bytes);
    }

    #[traced_test]
    fn fe_to_storage_then_fe_from_storage_roundtrips_representative_values() {
        info!("roundtripping via Fe10x26Storage encoding");
        roundtrip_storage(&BYTES_ZERO);
        roundtrip_storage(&BYTES_ONE);
        roundtrip_storage(&BYTES_PATTERN_A);
        roundtrip_storage(&BYTES_2_POW_255);
        roundtrip_storage(&FIELD_PRIME_MINUS_ONE_BYTES_BE);
    }

    #[traced_test]
    fn fe_from_storage_sets_verify_metadata_when_enabled() {
        info!("under secp256k1-verify, fe_from_storage sets magnitude=1 normalized=1");
        let a = fe_from_be_bytes_checked(&BYTES_TWO);
        let mut stor = Fe10x26Storage { n: [0u32; 8] };
        unsafe { fe_to_storage(&mut stor as *mut Fe10x26Storage, &a as *const Fe10x26) };

        let mut r = Fe10x26::new();
        unsafe { fe_from_storage(&mut r as *mut Fe10x26, &stor as *const Fe10x26Storage) };

        #[cfg(feature = "secp256k1-verify")]
        {
            assert_eq!(r.magnitude, 1);
            assert_eq!(r.normalized, 1);
        }
    }
}
