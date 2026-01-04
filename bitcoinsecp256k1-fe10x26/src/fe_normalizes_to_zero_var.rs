// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_normalizes_to_zero_var.rs ]
crate::ix!();

pub fn fe_normalizes_to_zero_var(r: *const Fe10x26) -> i32 {
    unsafe {
        let mut t0: u32;
        let mut t1: u32;
        let mut t2: u32;
        let mut t3: u32;
        let mut t4: u32;
        let mut t5: u32;
        let mut t6: u32;
        let mut t7: u32;
        let mut t8: u32;
        let mut t9: u32;

        let mut z0: u32;
        let mut z1: u32;
        let x: u32;

        t0 = (*r).n[0];
        t9 = (*r).n[9];

        /* Reduce t9 at the start so there will be at most a single carry from the first pass */
        x = t9 >> 22;

        /* The first pass ensures the magnitude is 1, ... */
        t0 = t0.wrapping_add(x.wrapping_mul(0x3D1u32));

        /* z0 tracks a possible raw value of 0, z1 tracks a possible raw value of P */
        z0 = t0 & 0x3FFFFFFu32;
        z1 = z0 ^ 0x3D0u32;

        /* Fast return path should catch the majority of cases */
        if ((z0 != 0u32) as u32) & ((z1 != 0x3FFFFFFu32) as u32) != 0 {
            return 0;
        }

        t1 = (*r).n[1];
        t2 = (*r).n[2];
        t3 = (*r).n[3];
        t4 = (*r).n[4];
        t5 = (*r).n[5];
        t6 = (*r).n[6];
        t7 = (*r).n[7];
        t8 = (*r).n[8];

        t9 &= 0x03FFFFFu32;
        t1 = t1.wrapping_add(x << 6);

        t1 = t1.wrapping_add(t0 >> 26);
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
mod fe_normalizes_to_zero_var_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn fe_normalizes_to_zero_var_matches_fe_normalizes_to_zero_on_representative_cases() {
        info!("var and constant-time variants should agree on representative cases");
        let mut p = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);
        let one = fe_from_be_bytes_checked(&BYTES_ONE);
        fe_add_in_place(&mut p, &one);

        let cases: [Fe10x26; 4] = [
            fe_from_be_bytes_checked(&BYTES_ZERO),
            fe_from_be_bytes_checked(&BYTES_ONE),
            fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE),
            p,
        ];

        for c in cases {
            let a = c;
            let ct = unsafe { fe_normalizes_to_zero(&a as *const Fe10x26) };
            let vt = unsafe { fe_normalizes_to_zero_var(&a as *const Fe10x26) };
            debug!(ct, vt, "ct vs var");
            assert_eq!(ct, vt);
        }
    }
}
