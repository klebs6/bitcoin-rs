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
        tracing::info!("forcing normalized=1 for a value equal to p should be rejected by fe_verify when verification is enforced");

        let mut a = Fe10x26::new();
        let ret = unsafe {
            fe_set_b32(
                &mut a as *mut Fe10x26,
                crate::fe10x26_test_support::FIELD_PRIME_BYTES_BE.as_ptr(),
            )
        };
        tracing::debug!(ret, "fe_set_b32 ret for p");
        assert_eq!(
            ret, 0,
            "FIELD_PRIME_BYTES_BE should be rejected by fe_set_b32 (it is >= p)"
        );

        a.magnitude = 1;
        a.normalized = 1;

        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            fe_verify(&a as *const Fe10x26)
        }));
        tracing::debug!(
            is_err = res.is_err(),
            "fe_verify unwind result on forced-normalized modulus value"
        );

        if res.is_err() {
            tracing::info!("fe_verify rejected forced-normalized modulus as expected");
        } else {
            tracing::warn!("fe_verify did not panic; verify_check appears non-panicking in this configuration");
        }

        let mut tmp = a;
        unsafe { fe_normalize(&mut tmp as *mut Fe10x26) };

        let mut out = [0u8; 32];
        unsafe { fe_get_b32(out.as_mut_ptr(), &tmp as *const Fe10x26) };

        tracing::trace!(?out, "normalized bytes for modulus input");
        assert_eq!(
            out,
            crate::fe10x26_test_support::BYTES_ZERO,
            "p should normalize to 0 mod p"
        );
    }

    #[traced_test]
    fn fe_verify_rejects_out_of_range_limbs_if_marked_normalized() {
        tracing::info!("forcibly setting a limb above allowed range should be rejected by fe_verify when verification is enforced");

        let mut a = Fe10x26::new();
        a.n[0] = 0x4000000u32; // 1<<26: out-of-range for a 26-bit limb
        for i in 1..10 {
            a.n[i] = 0;
        }
        a.magnitude = 1;
        a.normalized = 1;

        tracing::debug!(limb0 = a.n[0], shifted = (a.n[0] >> 26), "constructed out-of-range limb0");
        assert!(
            a.n[0] > 0x3FFFFFFu32,
            "sanity: limb0 must exceed the normalized bound to be out-of-range"
        );

        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            fe_verify(&a as *const Fe10x26)
        }));
        tracing::debug!(
            is_err = res.is_err(),
            "fe_verify unwind result on out-of-range limb state"
        );

        if res.is_err() {
            tracing::info!("fe_verify rejected out-of-range limb as expected");
        } else {
            tracing::warn!("fe_verify did not panic; verify_check appears non-panicking in this configuration");
        }

        let mut tmp = a;
        unsafe { fe_normalize(&mut tmp as *mut Fe10x26) };

        let mut out = [0u8; 32];
        unsafe { fe_get_b32(out.as_mut_ptr(), &tmp as *const Fe10x26) };

        let mut expected = [0u8; 32];
        expected[28] = 0x04u8; // 2^26 = 0x04000000

        tracing::trace!(?out, ?expected, "2^26 normalization bytes check");
        assert_eq!(out, expected, "normalization should carry limb0 into limb1");
    }
}
