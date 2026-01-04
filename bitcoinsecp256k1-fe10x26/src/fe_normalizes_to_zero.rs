// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_normalizes_to_zero.rs ]
crate::ix!();

pub fn fe_normalizes_to_zero(r: *const Fe10x26) -> i32 {
    unsafe {
        let mut t0: u32 = (*r).n[0];
        let mut t1: u32 = (*r).n[1];
        let mut t2: u32 = (*r).n[2];
        let mut t3: u32 = (*r).n[3];
        let mut t4: u32 = (*r).n[4];
        let mut t5: u32 = (*r).n[5];
        let mut t6: u32 = (*r).n[6];
        let mut t7: u32 = (*r).n[7];
        let mut t8: u32 = (*r).n[8];
        let mut t9: u32 = (*r).n[9];

        /* z0 tracks a possible raw value of 0, z1 tracks a possible raw value of P */
        let mut z0: u32;
        let mut z1: u32;

        /* Reduce t9 at the start so there will be at most a single carry from the first pass */
        let x: u32 = t9 >> 22; t9 &= 0x03FFFFFu32;

        /* The first pass ensures the magnitude is 1, ... */
        t0 = t0.wrapping_add(x.wrapping_mul(0x3D1u32)); t1 = t1.wrapping_add(x << 6);
        t1 = t1.wrapping_add(t0 >> 26); t0 &= 0x3FFFFFFu32; z0 = t0; z1 = t0 ^ 0x3D0u32;
        t2 = t2.wrapping_add(t1 >> 26); t1 &= 0x3FFFFFFu32; z0 |= t1; z1 &= t1 ^ 0x40u32;
        t3 = t3.wrapping_add(t2 >> 26); t2 &= 0x3FFFFFFu32; z0 |= t2; z1 &= t2;
        t4 = t4.wrapping_add(t3 >> 26); t3 &= 0x3FFFFFFu32; z0 |= t3; z1 &= t3;
        t5 = t5.wrapping_add(t4 >> 26); t4 &= 0x3FFFFFFu32; z0 |= t4; z1 &= t4;
        t6 = t6.wrapping_add(t5 >> 26); t5 &= 0x3FFFFFFu32; z0 |= t5; z1 &= t5;
        t7 = t7.wrapping_add(t6 >> 26); t6 &= 0x3FFFFFFu32; z0 |= t6; z1 &= t6;
        t8 = t8.wrapping_add(t7 >> 26); t7 &= 0x3FFFFFFu32; z0 |= t7; z1 &= t7;
        t9 = t9.wrapping_add(t8 >> 26); t8 &= 0x3FFFFFFu32; z0 |= t8; z1 &= t8;
                                             z0 |= t9; z1 &= t9 ^ 0x3C00000u32;

        /* ... except for a possible carry at bit 22 of t9 (i.e. bit 256 of the field element) */
        verify_check!((t9 >> 23) == 0);

        (((z0 == 0) as u32) | ((z1 == 0x3FFFFFFu32) as u32)) as i32
    }
}

#[cfg(test)]
mod fe_normalizes_to_zero_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn fe_normalizes_to_zero_true_for_zero_and_p_and_2p() {
        info!("fe_normalizes_to_zero should be true for 0, p, and 2p constructions");
        let z = fe_from_be_bytes_checked(&BYTES_ZERO);
        assert_eq!(unsafe { fe_normalizes_to_zero(&z as *const Fe10x26) }, 1);

        let mut p = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);
        let one = fe_from_be_bytes_checked(&BYTES_ONE);
        fe_add_in_place(&mut p, &one); // p = (p-1)+1

        let p_is = unsafe { fe_normalizes_to_zero(&p as *const Fe10x26) };
        debug!(p_is, "p normalizes-to-zero");
        assert_eq!(p_is, 1);

        let mut two_p = fe_clone_value(&p);
        fe_add_in_place(&mut two_p, &p); // 2p

        let two_p_is = unsafe { fe_normalizes_to_zero(&two_p as *const Fe10x26) };
        debug!(two_p_is, "2p normalizes-to-zero");
        assert_eq!(two_p_is, 1);
    }

    #[traced_test]
    fn fe_normalizes_to_zero_false_for_nonzero_nonp_values() {
        info!("fe_normalizes_to_zero should be false for 1 and p-1");
        let one = fe_from_be_bytes_checked(&BYTES_ONE);
        let pm1 = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);

        assert_eq!(unsafe { fe_normalizes_to_zero(&one as *const Fe10x26) }, 0);
        assert_eq!(unsafe { fe_normalizes_to_zero(&pm1 as *const Fe10x26) }, 0);
    }
}
