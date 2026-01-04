// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_normalize.rs ]
crate::ix!();

pub fn fe_normalize(r: *mut Fe5x52)  {

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
        verify_check((t4 >> 49) == 0);

        /* At most a single final reduction is needed; check if the value is >= the field characteristic */
        x = (t4 >> 48)
            | (((t4 == 0x0FFFFFFFFFFFF_u64) as u64)
                & ((m == 0xFFFFFFFFFFFFF_u64) as u64)
                & ((t0 >= 0xFFFFEFFFFFC2F_u64) as u64));

        /* Apply the final reduction (for constant-time behaviour, we do it always) */
        t0 = t0.wrapping_add(x.wrapping_mul(0x1000003D1_u64));
        t1 = t1.wrapping_add(t0 >> 52); t0 &= 0xFFFFFFFFFFFFF_u64;
        t2 = t2.wrapping_add(t1 >> 52); t1 &= 0xFFFFFFFFFFFFF_u64;
        t3 = t3.wrapping_add(t2 >> 52); t2 &= 0xFFFFFFFFFFFFF_u64;
        t4 = t4.wrapping_add(t3 >> 52); t3 &= 0xFFFFFFFFFFFFF_u64;

        /* If t4 didn't carry to bit 48 already, then it should have after any final reduction */
        verify_check((t4 >> 48) == x);

        /* Mask off the possible multiple of 2^256 from the final reduction */
        t4 &= 0x0FFFFFFFFFFFF_u64;

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
mod fe_normalize_rs_exhaustive_tests {
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
    fn fe_normalize_reduces_known_noncanonical_representations_to_expected_canonical_bytes() {
        tracing::info!("testing fe_normalize reduction on crafted noncanonical inputs");

        unsafe {
            tracing::debug!("p normalizes to 0");
            let mut a = Fe5x52::new();
            a.n[0] = P0; a.n[1] = P1; a.n[2] = P2; a.n[3] = P3; a.n[4] = P4;
            crate::fe_normalize(&mut a as *mut Fe5x52);
            assert_eq!(fe_to_b32_assuming_normalized(&a), [0u8; 32]);

            tracing::debug!("p+1 normalizes to 1");
            let mut b = Fe5x52::new();
            b.n[0] = P0.wrapping_add(1); b.n[1] = P1; b.n[2] = P2; b.n[3] = P3; b.n[4] = P4;
            crate::fe_normalize(&mut b as *mut Fe5x52);
            assert_eq!(fe_to_b32_assuming_normalized(&b), u64_to_be32(1));

            tracing::debug!("2^256 normalizes to 0x1000003D1");
            let mut c = Fe5x52::new();
            c.n[0] = 0; c.n[1] = 0; c.n[2] = 0; c.n[3] = 0; c.n[4] = 1u64 << 48;
            crate::fe_normalize(&mut c as *mut Fe5x52);
            assert_eq!(fe_to_b32_assuming_normalized(&c), val_2pow256_mod_p_b32());

            tracing::debug!("idempotence: normalizing a normalized value yields itself");
            let mut d = Fe5x52::new();
            crate::fe_set_int(&mut d as *mut Fe5x52, 7);
            crate::fe_normalize(&mut d as *mut Fe5x52);
            let before = fe_to_b32_assuming_normalized(&d);
            crate::fe_normalize(&mut d as *mut Fe5x52);
            let after = fe_to_b32_assuming_normalized(&d);
            assert_eq!(before, after);
        }
    }
}
