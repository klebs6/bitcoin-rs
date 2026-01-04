// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_is_odd.rs ]
crate::ix!();

#[inline] pub fn fe_is_odd(a: *const Fe5x52) -> i32 {

    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            verify_check((*a).normalized != 0);
            fe_verify(a);
        }

        ((*a).n[0] & 1u64) as i32
    }
}

#[cfg(test)]
mod fe_is_odd_rs_exhaustive_tests {
    use super::*;

    const FIELD_P_MINUS_1_B32: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x2E,
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

    #[traced_test]
    fn fe_is_odd_reports_lsb_of_normalized_value_across_cases() {
        tracing::info!("testing fe_is_odd for representative values");

        unsafe {
            let mut z = fe_from_b32_checked(&u64_to_be32(0));
            crate::fe_normalize(&mut z as *mut Fe5x52);
            assert_eq!(crate::fe_is_odd(&z as *const Fe5x52), 0);

            let mut one = fe_from_b32_checked(&u64_to_be32(1));
            crate::fe_normalize(&mut one as *mut Fe5x52);
            assert_eq!(crate::fe_is_odd(&one as *const Fe5x52), 1);

            let mut two = fe_from_b32_checked(&u64_to_be32(2));
            crate::fe_normalize(&mut two as *mut Fe5x52);
            assert_eq!(crate::fe_is_odd(&two as *const Fe5x52), 0);

            let mut pm1 = fe_from_b32_checked(&FIELD_P_MINUS_1_B32);
            crate::fe_normalize(&mut pm1 as *mut Fe5x52);
            assert_eq!(crate::fe_is_odd(&pm1 as *const Fe5x52), 0);
        }
    }
}
