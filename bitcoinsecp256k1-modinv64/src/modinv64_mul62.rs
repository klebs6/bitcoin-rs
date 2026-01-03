// ---------------- [ File: bitcoinsecp256k1-modinv64/src/modinv64_mul62.rs ]
crate::ix!();

/// Compute a*factor and put it in r. All but the top limb in r will be in range [0,2^62).
/// 
#[cfg(VERIFY)]
pub fn modinv64_mul_62(
        r:      *mut ModInv64Signed62,
        a:      *const ModInv64Signed62,
        alen:   i32,
        factor: i64)  {

    const M62: i64 = (u64::MAX >> 2) as i64;
    let mut c: i128 = 0;
    let mut i: i32 = 0;

    unsafe {
        while i < 4 {
            if i < alen {
                c += ((*a).v()[i as usize] as i128) * (factor as i128);
            }
            (*r).v_mut()[i as usize] = (c as i64) & M62;
            c >>= 62;
            i += 1;
        }
        if 4 < alen {
            c += ((*a).v()[4] as i128) * (factor as i128);
        }
        VERIFY_CHECK!(c == ((c as i64) as i128));
        (*r).v_mut()[4] = c as i64;
    }
}

#[cfg(all(test, VERIFY))]
mod modinv64_mul_62_contract {
    use super::*;

    #[traced_test]
    fn mul_62_matches_small_integer_reference_and_normalizes_lower_limbs() {
        let mut seed: u64 = 0x1234_5678_90AB_CDEF;
        let base: i128 = 1i128 << LIMB_BITS;
        let m62: i64 = (u64::MAX >> 2) as i64;

        let mut i: usize = 0;
        while i < 512 {
            let a0: i128 = (splitmix128_next(&mut seed) & 0x7FFF_FFFF) as i128;
            let a1: i128 = (splitmix128_next(&mut seed) & 0x000F_FFFF) as i128;
            let factor: i64 = ((splitmix128_next(&mut seed) as i64) % 97) - 48;
            let alen: i32 = if (splitmix128_next(&mut seed) & 1) == 0 { 1 } else { 2 };

            let a_val: i128 = if alen == 1 { a0 } else { a0 + a1 * base };
            let a = signed62_from_i128(a_val);

            let mut out = ModInv64Signed62::from_limbs([0, 0, 0, 0, 0]);
            trace!(iter = i, a_val = a_val, factor = factor, alen = alen, a_limbs = ?a.v());

            modinv64_mul_62(&mut out as *mut _, &a as *const _, alen, factor);

            trace!(iter = i, out_limbs = ?out.v());

            let expected_val: i128 = a_val * (factor as i128);
            let expected = signed62_from_i128(expected_val);

            assert!(*out.v() == *expected.v());

            let mut j: usize = 0;
            while j < 4 {
                assert!(out.v()[j] >= 0);
                assert!((out.v()[j] as u64) <= LIMB_MASK_U64);
                j += 1;
            }
            assert!(out.v()[4] >= -m62 && out.v()[4] <= m62);

            i += 1;
        }
    }
}
