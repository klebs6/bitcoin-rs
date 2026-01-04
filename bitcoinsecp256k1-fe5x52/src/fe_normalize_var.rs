// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_normalize_var.rs ]
crate::ix!();

pub fn fe_normalize_var(r: *mut Fe5x52)  {

    unsafe {
        let mut t0: u64 = (*r).n[0];
        let mut t1: u64 = (*r).n[1];
        let mut t2: u64 = (*r).n[2];
        let mut t3: u64 = (*r).n[3];
        let mut t4: u64 = (*r).n[4];

        /* Reduce t4 at the start so there will be at most a single carry from the first pass */
        let mut m: u64;
        let mut x: u64 = t4 >> 48;
        t4 &= 0x0FFFFFFFFFFFF_u64;

        /* The first pass ensures the magnitude is 1, ... */
        t0 = t0.wrapping_add(x.wrapping_mul(0x1000003D1_u64));
        t1 = t1.wrapping_add(t0 >> 52); t0 &= 0xFFFFFFFFFFFFF_u64;
        t2 = t2.wrapping_add(t1 >> 52); t1 &= 0xFFFFFFFFFFFFF_u64; m = t1;
        t3 = t3.wrapping_add(t2 >> 52); t2 &= 0xFFFFFFFFFFFFF_u64; m &= t2;
        t4 = t4.wrapping_add(t3 >> 52); t3 &= 0xFFFFFFFFFFFFF_u64; m &= t3;

        /* ... except for a possible carry at bit 48 of t4 (i.e. bit 256 of the field element) */
        verify_check!((t4 >> 49) == 0);

        /* At most a single final reduction is needed; check if the value is >= the field characteristic */
        x = (t4 >> 48)
            | (((t4 == 0x0FFFFFFFFFFFF_u64) as u64)
                & ((m == 0xFFFFFFFFFFFFF_u64) as u64)
                & ((t0 >= 0xFFFFEFFFFFC2F_u64) as u64));

        if x != 0 {
            t0 = t0.wrapping_add(0x1000003D1_u64);
            t1 = t1.wrapping_add(t0 >> 52); t0 &= 0xFFFFFFFFFFFFF_u64;
            t2 = t2.wrapping_add(t1 >> 52); t1 &= 0xFFFFFFFFFFFFF_u64;
            t3 = t3.wrapping_add(t2 >> 52); t2 &= 0xFFFFFFFFFFFFF_u64;
            t4 = t4.wrapping_add(t3 >> 52); t3 &= 0xFFFFFFFFFFFFF_u64;

            /* If t4 didn't carry to bit 48 already, then it should have after any final reduction */
            verify_check!((t4 >> 48) == x);

            /* Mask off the possible multiple of 2^256 from the final reduction */
            t4 &= 0x0FFFFFFFFFFFF_u64;
        }

        (*r).n[0] = t0; (*r).n[1] = t1; (*r).n[2] = t2; (*r).n[3] = t3; (*r).n[4] = t4;

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = 1;
            (*r).normalized = 1;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_normalize_var_rs_exhaustive_tests {
    use super::*;

    const P0: u64 = 0xFFFFEFFFFFC2F_u64;
    const P1: u64 = 0xFFFFFFFFFFFFF_u64;
    const P2: u64 = 0xFFFFFFFFFFFFF_u64;
    const P3: u64 = 0xFFFFFFFFFFFFF_u64;
    const P4: u64 = 0x0FFFFFFFFFFFF_u64;

    fn u64_to_be32(v: u64) -> [u8; 32] {
        let mut out = [0u8; 32];
        out[24..32].copy_from_slice(&v.to_be_bytes());
        out
    }

    fn val_2pow256_mod_p_b32() -> [u8; 32] {
        let mut out = [0u8; 32];
        out[27] = 0x01;
        out[28] = 0x00;
        out[29] = 0x00;
        out[30] = 0x03;
        out[31] = 0xD1;
        out
    }

    unsafe fn fe_to_b32_assuming_normalized(a: &Fe5x52) -> [u8; 32] {
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), a as *const Fe5x52);
        out
    }

    #[traced_test]
    fn fe_normalize_var_matches_fe_normalize_outputs_for_crafted_cases() {
        tracing::info!("testing fe_normalize_var outputs match fe_normalize for representative inputs");

        unsafe {
            let mut inputs: [Fe5x52; 4] = [
                { let mut x = Fe5x52::new(); crate::fe_set_int(&mut x as *mut Fe5x52, 0); x },
                { let mut x = Fe5x52::new(); crate::fe_set_int(&mut x as *mut Fe5x52, 7); x },
                { let mut x = Fe5x52::new(); x.n[0]=P0; x.n[1]=P1; x.n[2]=P2; x.n[3]=P3; x.n[4]=P4; x },
                { let mut x = Fe5x52::new(); x.n[0]=0; x.n[1]=0; x.n[2]=0; x.n[3]=0; x.n[4]=1u64<<48; x },
            ];

            for (idx, inp) in inputs.iter().enumerate() {
                tracing::debug!(case_index = idx, "comparing normalize vs normalize_var");

                let mut a = *inp;
                let mut b = *inp;

                crate::fe_normalize(&mut a as *mut Fe5x52);
                crate::fe_normalize_var(&mut b as *mut Fe5x52);

                let ba = fe_to_b32_assuming_normalized(&a);
                let bb = fe_to_b32_assuming_normalized(&b);
                assert_eq!(ba, bb);

                if idx == 3 {
                    assert_eq!(ba, val_2pow256_mod_p_b32());
                }
                if idx == 2 {
                    assert_eq!(ba, u64_to_be32(0));
                }
            }
        }
    }
}
