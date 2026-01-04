// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_to_signed30.rs ]
crate::ix!();

pub fn fe_to_signed30(
    r: *mut ModInv32Signed30,
    a: *const Fe10x26)  {

    unsafe {
        let m30: u32 = u32::MAX >> 2;

        let a0: u64 = (*a).n[0] as u64;
        let a1: u64 = (*a).n[1] as u64;
        let a2: u64 = (*a).n[2] as u64;
        let a3: u64 = (*a).n[3] as u64;
        let a4: u64 = (*a).n[4] as u64;
        let a5: u64 = (*a).n[5] as u64;
        let a6: u64 = (*a).n[6] as u64;
        let a7: u64 = (*a).n[7] as u64;
        let a8: u64 = (*a).n[8] as u64;
        let a9: u64 = (*a).n[9] as u64;

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!((*a).normalized != 0);
        }

        (*r).v[0] = ((a0       | a1 << 26) & (m30 as u64)) as i32;
        (*r).v[1] = ((a1 >>  4 | a2 << 22) & (m30 as u64)) as i32;
        (*r).v[2] = ((a2 >>  8 | a3 << 18) & (m30 as u64)) as i32;
        (*r).v[3] = ((a3 >> 12 | a4 << 14) & (m30 as u64)) as i32;
        (*r).v[4] = ((a4 >> 16 | a5 << 10) & (m30 as u64)) as i32;
        (*r).v[5] = ((a5 >> 20 | a6 <<  6) & (m30 as u64)) as i32;
        (*r).v[6] = ((a6 >> 24 | a7 <<  2 | a8 << 28) & (m30 as u64)) as i32;
        (*r).v[7] = ((a8 >>  2 | a9 << 24) & (m30 as u64)) as i32;
        (*r).v[8] = (a9 >>  6) as i32;
    }
}

#[cfg(test)]
mod fe_to_signed30_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn fe_to_signed30_outputs_limbs_in_expected_ranges() {
        info!("fe_to_signed30 should output 30-bit limbs (top limb <= 16 bits)");
        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);

        let mut s: ModInv32Signed30 = unsafe { core::mem::zeroed() };
        unsafe { fe_to_signed30(&mut s as *mut ModInv32Signed30, &a as *const Fe10x26) };

        debug!(?s.v, "signed30 output");
        for i in 0..8 {
            assert_eq!(((s.v[i] as u32) >> 30), 0u32);
        }
        assert_eq!(((s.v[8] as u32) >> 16), 0u32);
    }

    #[traced_test]
    fn fe_to_signed30_and_back_roundtrips() {
        info!("fe_to_signed30 then fe_from_signed30 should roundtrip representative values");
        let vectors: [&[u8; 32]; 5] = [
            &BYTES_ZERO,
            &BYTES_ONE,
            &BYTES_2_POW_255,
            &BYTES_PATTERN_A,
            &FIELD_PRIME_MINUS_ONE_BYTES_BE,
        ];

        for v in vectors {
            let mut a = fe_from_be_bytes_checked(v);

            let mut s: ModInv32Signed30 = unsafe { core::mem::zeroed() };
            unsafe { fe_to_signed30(&mut s as *mut ModInv32Signed30, &a as *const Fe10x26) };

            let mut b = Fe10x26::new();
            unsafe { fe_from_signed30(&mut b as *mut Fe10x26, &s as *const ModInv32Signed30) };

            let out = fe_to_be_bytes_normalized(&mut b);
            assert_eq!(&out, v);
        }
    }
}

