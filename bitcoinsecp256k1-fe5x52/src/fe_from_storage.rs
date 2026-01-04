// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_from_storage.rs ]
crate::ix!();

#[inline] pub fn fe_from_storage(
        r: *mut Fe5x52,
        a: *const Fe5x52Storage)  {

    unsafe {
        let an: *const u64 = a as *const u64;

        let a0 = *an.add(0);
        let a1 = *an.add(1);
        let a2 = *an.add(2);
        let a3 = *an.add(3);

        (*r).n[0] = a0 & 0xFFFFFFFFFFFFF_u64;
        (*r).n[1] = (a0 >> 52) | ((a1 << 12) & 0xFFFFFFFFFFFFF_u64);
        (*r).n[2] = (a1 >> 40) | ((a2 << 24) & 0xFFFFFFFFFFFFF_u64);
        (*r).n[3] = (a2 >> 28) | ((a3 << 36) & 0xFFFFFFFFFFFFF_u64);
        (*r).n[4] = (a3 >> 16);

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = 1;
            (*r).normalized = 1;
        }
    }
}

#[cfg(test)]
mod fe_from_storage_rs_exhaustive_tests {
    use super::*;

    const SAMPLE_B32: [u8; 32] = [
        0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70,
        0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0,
        0x0F, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88,
        0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x01,
    ];

    fn u64_to_be32(v: u64) -> [u8; 32] {
        let mut out = [0u8; 32];
        out[24..32].copy_from_slice(&v.to_be_bytes());
        out
    }

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
    fn storage_conversion_roundtrips_through_fe_to_storage_and_fe_from_storage() {
        tracing::info!("testing fe_to_storage -> fe_from_storage roundtrip");

        unsafe {
            let samples: [&[u8; 32]; 3] = [&u64_to_be32(0), &u64_to_be32(1), &SAMPLE_B32];

            for (idx, s) in samples.iter().enumerate() {
                tracing::debug!(sample_index = idx, "roundtripping one sample");

                let mut a = fe_from_b32_checked(s);
                crate::fe_normalize(&mut a as *mut Fe5x52);

                let mut stor = core::mem::MaybeUninit::<Fe5x52Storage>::uninit();
                crate::fe_to_storage(stor.as_mut_ptr(), &a as *const Fe5x52);

                let mut b = Fe5x52::new();
                crate::fe_from_storage(&mut b as *mut Fe5x52, stor.as_ptr());

                let got = fe_to_b32_normalized(&mut b);
                let expected = fe_to_b32_normalized(&mut a);

                assert_eq!(got, expected);
            }
        }
    }
}
