// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_verify.rs ]
crate::ix!();

#[cfg(feature="secp256k1-verify")]
pub fn fe_verify(a: *const Fe10x26)  {
    unsafe {
        let d: *const u32 = (*a).n.as_ptr();

        let m: i32 = if (*a).normalized != 0 { 1 } else { 2i32.wrapping_mul((*a).magnitude) };
        let mut r: i32 = 1;

        r &= (((*d.add(0)) as u64 <= (0x3FFFFFFu64).wrapping_mul(m as u64)) as i32);
        r &= (((*d.add(1)) as u64 <= (0x3FFFFFFu64).wrapping_mul(m as u64)) as i32);
        r &= (((*d.add(2)) as u64 <= (0x3FFFFFFu64).wrapping_mul(m as u64)) as i32);
        r &= (((*d.add(3)) as u64 <= (0x3FFFFFFu64).wrapping_mul(m as u64)) as i32);
        r &= (((*d.add(4)) as u64 <= (0x3FFFFFFu64).wrapping_mul(m as u64)) as i32);
        r &= (((*d.add(5)) as u64 <= (0x3FFFFFFu64).wrapping_mul(m as u64)) as i32);
        r &= (((*d.add(6)) as u64 <= (0x3FFFFFFu64).wrapping_mul(m as u64)) as i32);
        r &= (((*d.add(7)) as u64 <= (0x3FFFFFFu64).wrapping_mul(m as u64)) as i32);
        r &= (((*d.add(8)) as u64 <= (0x3FFFFFFu64).wrapping_mul(m as u64)) as i32);
        r &= (((*d.add(9)) as u64 <= (0x03FFFFFu64).wrapping_mul(m as u64)) as i32);

        r &= (((*a).magnitude >= 0) as i32);
        r &= (((*a).magnitude <= 32) as i32);

        if (*a).normalized != 0 {
            r &= (((*a).magnitude <= 1) as i32);
            if (r != 0) && (*d.add(9) == 0x03FFFFFu32) {
                let mid: u32 = *d.add(8) & *d.add(7) & *d.add(6) & *d.add(5) & *d.add(4) & *d.add(3) & *d.add(2);
                if mid == 0x3FFFFFFu32 {
                    r &= ((((*d.add(1)).wrapping_add(0x40u32).wrapping_add(((*d.add(0)).wrapping_add(0x3D1u32)) >> 26)) <= 0x3FFFFFFu32) as i32);
                }
            }
        }

        verify_check!(r == 1);
    }
}

#[cfg(all(test, feature = "secp256k1-verify"))]
mod fe_verify_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn fe_verify_accepts_valid_normalized_values() {
        info!("fe_verify should accept canonical values");
        let mut z = fe_from_be_bytes_checked(&BYTES_ZERO);
        let mut o = fe_from_be_bytes_checked(&BYTES_ONE);
        let mut pm1 = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);

        fe_normalize_in_place(&mut z);
        fe_normalize_in_place(&mut o);
        fe_normalize_in_place(&mut pm1);

        unsafe { fe_verify(&z as *const Fe10x26) };
        unsafe { fe_verify(&o as *const Fe10x26) };
        unsafe { fe_verify(&pm1 as *const Fe10x26) };
    }

    #[traced_test]
    fn fe_verify_rejects_value_equal_to_modulus_if_marked_normalized() {
        info!("forcing normalized=1 for a value equal to p should cause fe_verify failure");
        let (mut p_fe, ret) = fe_from_be_bytes_ret(&FIELD_PRIME_BYTES_BE);
        debug!(ret, "fe_set_b32 ret for p");
        assert_eq!(ret, 0);

        p_fe.magnitude = 1;
        p_fe.normalized = 1;

        let res = std::panic::catch_unwind(|| unsafe { fe_verify(&p_fe as *const Fe10x26) });
        assert!(res.is_err());
    }

    #[traced_test]
    fn fe_verify_rejects_out_of_range_limbs_if_marked_normalized() {
        info!("forcibly setting a limb above allowed range should fail fe_verify");
        let mut a = fe_from_be_bytes_checked(&BYTES_ONE);
        a.magnitude = 1;
        a.normalized = 1;

        a.n[0] = 0xFFFFFFFFu32;

        let res = std::panic::catch_unwind(|| unsafe { fe_verify(&a as *const Fe10x26) });
        assert!(res.is_err());
    }
}
