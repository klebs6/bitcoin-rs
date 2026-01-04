// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_normalize.rs ]
crate::ix!();

pub fn fe_normalize(r: *mut Fe10x26)  {
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

        /* Reduce t9 at the start so there will be at most a single carry from the first pass */
        let mut m: u32;
        let mut x: u32 = t9 >> 22; t9 &= 0x03FFFFFu32;

        /* The first pass ensures the magnitude is 1, ... */
        t0 = t0.wrapping_add(x.wrapping_mul(0x3D1u32)); t1 = t1.wrapping_add(x << 6);
        t1 = t1.wrapping_add(t0 >> 26); t0 &= 0x3FFFFFFu32;
        t2 = t2.wrapping_add(t1 >> 26); t1 &= 0x3FFFFFFu32;
        t3 = t3.wrapping_add(t2 >> 26); t2 &= 0x3FFFFFFu32; m = t2;
        t4 = t4.wrapping_add(t3 >> 26); t3 &= 0x3FFFFFFu32; m &= t3;
        t5 = t5.wrapping_add(t4 >> 26); t4 &= 0x3FFFFFFu32; m &= t4;
        t6 = t6.wrapping_add(t5 >> 26); t5 &= 0x3FFFFFFu32; m &= t5;
        t7 = t7.wrapping_add(t6 >> 26); t6 &= 0x3FFFFFFu32; m &= t6;
        t8 = t8.wrapping_add(t7 >> 26); t7 &= 0x3FFFFFFu32; m &= t7;
        t9 = t9.wrapping_add(t8 >> 26); t8 &= 0x3FFFFFFu32; m &= t8;

        /* ... except for a possible carry at bit 22 of t9 (i.e. bit 256 of the field element) */
        verify_check!((t9 >> 23) == 0);

        /* At most a single final reduction is needed; check if the value is >= the field characteristic */
        x = (t9 >> 22)
          | (((t9 == 0x03FFFFFu32) as u32)
           & ((m == 0x3FFFFFFu32) as u32)
           & (((t1.wrapping_add(0x40u32).wrapping_add((t0.wrapping_add(0x3D1u32)) >> 26)) > 0x3FFFFFFu32) as u32));

        /* Apply the final reduction (for constant-time behaviour, we do it always) */
        t0 = t0.wrapping_add(x.wrapping_mul(0x3D1u32)); t1 = t1.wrapping_add(x << 6);
        t1 = t1.wrapping_add(t0 >> 26); t0 &= 0x3FFFFFFu32;
        t2 = t2.wrapping_add(t1 >> 26); t1 &= 0x3FFFFFFu32;
        t3 = t3.wrapping_add(t2 >> 26); t2 &= 0x3FFFFFFu32;
        t4 = t4.wrapping_add(t3 >> 26); t3 &= 0x3FFFFFFu32;
        t5 = t5.wrapping_add(t4 >> 26); t4 &= 0x3FFFFFFu32;
        t6 = t6.wrapping_add(t5 >> 26); t5 &= 0x3FFFFFFu32;
        t7 = t7.wrapping_add(t6 >> 26); t6 &= 0x3FFFFFFu32;
        t8 = t8.wrapping_add(t7 >> 26); t7 &= 0x3FFFFFFu32;
        t9 = t9.wrapping_add(t8 >> 26); t8 &= 0x3FFFFFFu32;

        /* If t9 didn't carry to bit 22 already, then it should have after any final reduction */
        verify_check!((t9 >> 22) == x);

        /* Mask off the possible multiple of 2^256 from the final reduction */
        t9 &= 0x03FFFFFu32;

        (*r).n[0] = t0; (*r).n[1] = t1; (*r).n[2] = t2; (*r).n[3] = t3; (*r).n[4] = t4;
        (*r).n[5] = t5; (*r).n[6] = t6; (*r).n[7] = t7; (*r).n[8] = t8; (*r).n[9] = t9;

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = 1;
            (*r).normalized = 1;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_normalize_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    #[traced_test]
    fn fe_normalize_is_idempotent_on_normalized_values() {
        info!("fe_normalize(normalized(x)) == normalized(x)");
        let mut x = fe_from_be_bytes_checked(&BYTES_PATTERN_A);

        let once = fe_to_be_bytes_normalized(&mut x);
        let twice = fe_to_be_bytes_normalized(&mut x);

        debug!(?once, ?twice, "normalize idempotence");
        assert_eq!(once, twice);
        assert_eq!(once, BYTES_PATTERN_A);
    }

    #[traced_test]
    fn fe_normalize_reduces_p_to_zero_when_constructed_via_addition() {
        info!("construct p = (p-1)+1, then normalize -> 0");
        let mut r = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);
        let one = fe_from_be_bytes_checked(&BYTES_ONE);

        fe_add_in_place(&mut r, &one);

        let out = fe_to_be_bytes_normalized(&mut r);
        assert_eq!(out, BYTES_ZERO);
    }

    #[traced_test]
    fn fe_normalize_reduces_p_plus_one_to_one_when_constructed_via_addition() {
        info!("construct p+1 = (p-1)+2, normalize -> 1");
        let mut r = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);
        let two = fe_from_be_bytes_checked(&BYTES_TWO);

        fe_add_in_place(&mut r, &two);

        let out = fe_to_be_bytes_normalized(&mut r);
        assert_eq!(out, BYTES_ONE);
    }

    #[traced_test]
    fn fe_normalize_sets_verify_metadata_when_enabled() {
        info!("under secp256k1-verify, fe_normalize sets magnitude=1 normalized=1");
        let mut r = fe_from_be_bytes_checked(&BYTES_PATTERN_A);

        fe_normalize_in_place(&mut r);

        #[cfg(feature = "secp256k1-verify")]
        {
            assert_eq!(r.magnitude, 1);
            assert_eq!(r.normalized, 1);
        }
    }
}
