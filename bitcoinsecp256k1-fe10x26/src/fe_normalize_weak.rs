// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_normalize_weak.rs ]
crate::ix!();

pub fn fe_normalize_weak(r: *mut Fe10x26)  {
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
        let x: u32 = t9 >> 22; t9 &= 0x03FFFFFu32;

        /* The first pass ensures the magnitude is 1, ... */
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

        /* ... except for a possible carry at bit 22 of t9 (i.e. bit 256 of the field element) */
        verify_check!((t9 >> 23) == 0);

        (*r).n[0] = t0; (*r).n[1] = t1; (*r).n[2] = t2; (*r).n[3] = t3; (*r).n[4] = t4;
        (*r).n[5] = t5; (*r).n[6] = t6; (*r).n[7] = t7; (*r).n[8] = t8; (*r).n[9] = t9;

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = 1;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_normalize_weak_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn fe_normalize_weak_preserves_value_mod_p_for_normalized_inputs() {
        info!("fe_normalize_weak should preserve value for already-normalized inputs");
        let mut a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);

        let before = fe_to_be_bytes_normalized(&mut fe_clone_value(&a));
        unsafe { fe_normalize_weak(&mut a as *mut Fe10x26) };

        let after = fe_to_be_bytes_normalized(&mut a);
        debug!(?before, ?after, "weak normalize");
        assert_eq!(after, before);
    }

    #[traced_test]
    fn fe_normalize_weak_does_not_break_normalizes_to_zero_for_constructed_p() {
        info!("construct p and ensure it still normalizes-to-zero after fe_normalize_weak");
        let mut r = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);
        let one = fe_from_be_bytes_checked(&BYTES_ONE);

        fe_add_in_place(&mut r, &one);
        unsafe { fe_normalize_weak(&mut r as *mut Fe10x26) };

        let z = unsafe { fe_normalizes_to_zero(&r as *const Fe10x26) };
        trace!(z, "fe_normalizes_to_zero after weak normalize");
        assert_eq!(z, 1);
    }

    #[traced_test]
    fn fe_normalize_weak_sets_verify_magnitude_when_enabled() {
        info!("under secp256k1-verify, fe_normalize_weak sets magnitude=1");
        let mut r = fe_from_be_bytes_checked(&BYTES_PATTERN_A);

        unsafe { fe_normalize_weak(&mut r as *mut Fe10x26) };

        #[cfg(feature = "secp256k1-verify")]
        {
            assert_eq!(r.magnitude, 1);
        }
    }
}
