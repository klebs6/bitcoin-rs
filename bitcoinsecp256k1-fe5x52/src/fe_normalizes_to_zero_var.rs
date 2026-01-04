// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_normalizes_to_zero_var.rs ]
crate::ix!();

pub fn fe_normalizes_to_zero_var(r: *const Fe5x52) -> i32 {

    unsafe {
        let mut t0: u64;
        let mut t1: u64;
        let mut t2: u64;
        let mut t3: u64;
        let mut t4: u64;
        let mut z0: u64;
        let mut z1: u64;
        let x: u64;

        t0 = (*r).n[0];
        t4 = (*r).n[4];

        /* Reduce t4 at the start so there will be at most a single carry from the first pass */
        x = t4 >> 48;

        /* The first pass ensures the magnitude is 1, ... */
        t0 = t0.wrapping_add(x.wrapping_mul(0x1000003D1_u64));

        /* z0 tracks a possible raw value of 0, z1 tracks a possible raw value of P */
        z0 = t0 & 0xFFFFFFFFFFFFF_u64;
        z1 = z0 ^ 0x1000003D0_u64;

        /* Fast return path should catch the majority of cases */
        if ((z0 != 0u64) & (z1 != 0xFFFFFFFFFFFFF_u64)) {
            return 0;
        }

        t1 = (*r).n[1];
        t2 = (*r).n[2];
        t3 = (*r).n[3];

        t4 &= 0x0FFFFFFFFFFFF_u64;

        t1 = t1.wrapping_add(t0 >> 52);
        t2 = t2.wrapping_add(t1 >> 52); t1 &= 0xFFFFFFFFFFFFF_u64; z0 |= t1; z1 &= t1;
        t3 = t3.wrapping_add(t2 >> 52); t2 &= 0xFFFFFFFFFFFFF_u64; z0 |= t2; z1 &= t2;
        t4 = t4.wrapping_add(t3 >> 52); t3 &= 0xFFFFFFFFFFFFF_u64; z0 |= t3; z1 &= t3;
        z0 |= t4; z1 &= t4 ^ 0xF000000000000_u64;

        /* ... except for a possible carry at bit 48 of t4 (i.e. bit 256 of the field element) */
        verify_check!((t4 >> 49) == 0);

        (((z0 == 0) | (z1 == 0xFFFFFFFFFFFFF_u64)) as i32)
    }
}

#[cfg(test)]
mod fe_normalizes_to_zero_var_rs_exhaustive_tests {
    use super::*;

    const P0: u64 = 0xFFFFEFFFFFC2F_u64;
    const P1: u64 = 0xFFFFFFFFFFFFF_u64;
    const P2: u64 = 0xFFFFFFFFFFFFF_u64;
    const P3: u64 = 0xFFFFFFFFFFFFF_u64;
    const P4: u64 = 0x0FFFFFFFFFFFF_u64;

    #[traced_test]
    fn fe_normalizes_to_zero_var_agrees_with_constant_time_variant_for_representative_inputs() {
        tracing::info!("testing fe_normalizes_to_zero_var agreement with fe_normalizes_to_zero");

        unsafe {
            let mut samples: [Fe5x52; 6] = [
                { let mut x = Fe5x52::new(); x.n = [0,0,0,0,0]; x },
                { let mut x = Fe5x52::new(); x.n[0]=P0; x.n[1]=P1; x.n[2]=P2; x.n[3]=P3; x.n[4]=P4; x },
                { let mut x = Fe5x52::new(); x.n[0]=P0.wrapping_add(1); x.n[1]=P1; x.n[2]=P2; x.n[3]=P3; x.n[4]=P4; x },
                { let mut x = Fe5x52::new(); crate::fe_set_int(&mut x as *mut Fe5x52, 1); x },
                { let mut x = Fe5x52::new(); x.n[4] = 1u64 << 48; x },
                { let mut x = Fe5x52::new(); x.n[0]=P0; x.n[1]=P1; x.n[2]=P2; x.n[3]=P3; x.n[4]=P4; crate::fe_add(&mut x as *mut Fe5x52, &x as *const Fe5x52); x },
            ];

            for (idx, s) in samples.iter().enumerate() {
                let ct = crate::fe_normalizes_to_zero(s as *const Fe5x52);
                let var = crate::fe_normalizes_to_zero_var(s as *const Fe5x52);
                tracing::debug!(case_index = idx, ct = ct, var = var, "comparing ct vs var");
                assert_eq!(ct, var);
            }
        }
    }
}
