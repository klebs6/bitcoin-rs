// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_to_signed62.rs ]
crate::ix!();

pub fn fe_to_signed62(
        r: *mut ModInv64Signed62,
        a: *const Fe5x52)  {

    unsafe {
        const M62: u64 = u64::MAX >> 2;

        let a0: u64 = (*a).n[0];
        let a1: u64 = (*a).n[1];
        let a2: u64 = (*a).n[2];
        let a3: u64 = (*a).n[3];
        let a4: u64 = (*a).n[4];

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check((*a).normalized != 0);
        }

        let rv: *mut i64 = r as *mut i64;

        *rv.add(0) = ((a0 | (a1 << 52)) & M62) as i64;
        *rv.add(1) = (((a1 >> 10) | (a2 << 42)) & M62) as i64;
        *rv.add(2) = (((a2 >> 20) | (a3 << 32)) & M62) as i64;
        *rv.add(3) = (((a3 >> 30) | (a4 << 22)) & M62) as i64;
        *rv.add(4) = ( a4 >> 40) as i64;
    }
}

#[cfg(test)]
mod fe_to_signed62_rs_exhaustive_tests {
    use super::*;

    const SAMPLE_B32: [u8; 32] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x01,
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
    fn fe_to_signed62_produces_roundtrippable_representation_with_fe_from_signed62() {
        tracing::info!("testing fe_to_signed62 + fe_from_signed62 roundtrip");

        unsafe {
            let mut a = fe_from_b32_checked(&SAMPLE_B32);
            crate::fe_normalize(&mut a as *mut Fe5x52);

            let mut s = core::mem::MaybeUninit::<ModInv64Signed62>::uninit();
            crate::fe_to_signed62(s.as_mut_ptr(), &a as *const Fe5x52);

            let mut b = Fe5x52::new();
            crate::fe_from_signed62(&mut b as *mut Fe5x52, s.as_ptr());

            let got = fe_to_b32_normalized(&mut b);
            let expected = fe_to_b32_normalized(&mut a);

            assert_eq!(got, expected);
        }
    }
}
