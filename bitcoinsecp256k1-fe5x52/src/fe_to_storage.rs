// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_to_storage.rs ]
crate::ix!();

pub fn fe_to_storage(
        r: *mut FeStorage,
        a: *const Fe5x52)  {

    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            verify_check((*a).normalized != 0);
        }

        let rn: *mut u64 = r as *mut u64;

        *rn.add(0) = (*a).n[0] | ((*a).n[1] << 52);
        *rn.add(1) = ((*a).n[1] >> 12) | ((*a).n[2] << 40);
        *rn.add(2) = ((*a).n[2] >> 24) | ((*a).n[3] << 28);
        *rn.add(3) = ((*a).n[3] >> 36) | ((*a).n[4] << 16);
    }
}

#[cfg(test)]
mod fe_to_storage_rs_exhaustive_tests {
    use super::*;

    const SAMPLE_B32: [u8; 32] = [
        0x0F, 0xED, 0xCB, 0xA9, 0x87, 0x65, 0x43, 0x21,
        0x10, 0x32, 0x54, 0x76, 0x98, 0xBA, 0xDC, 0xFE,
        0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32, 0x10,
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0x01,
    ];

    unsafe fn fe_from_b32_checked(bytes: &[u8; 32]) -> Fe5x52 {
        let mut fe = Fe5x52::new();
        let ret = crate::fe_set_b32(&mut fe as *mut Fe5x52, bytes.as_ptr());
        assert_eq!(ret, 1);
        fe
    }

    unsafe fn fe_to_b32_normalized(fe: &mut Fe5x52) -> [u8; 32] {
        crate::fe_normalize(fe as *mut Fe5x52);
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), fe as *const Fe5x52);
        out
    }

    #[traced_test]
    fn fe_to_storage_roundtrips_through_fe_from_storage_for_normalized_inputs() {
        tracing::info!("testing fe_to_storage -> fe_from_storage roundtrip with representative input");

        unsafe {
            let mut a = fe_from_b32_checked(&SAMPLE_B32);
            crate::fe_normalize(&mut a as *mut Fe5x52);

            let mut stor = core::mem::MaybeUninit::<FeStorage>::uninit();
            crate::fe_to_storage(stor.as_mut_ptr(), &a as *const Fe5x52);

            let mut b = Fe5x52::new();
            crate::fe_from_storage(&mut b as *mut Fe5x52, stor.as_ptr());

            let got = fe_to_b32_normalized(&mut b);
            let expected = fe_to_b32_normalized(&mut a);

            assert_eq!(got, expected);
        }
    }
}

