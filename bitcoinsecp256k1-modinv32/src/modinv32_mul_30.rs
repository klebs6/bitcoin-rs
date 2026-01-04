// ---------------- [ File: bitcoinsecp256k1-modinv32/src/modinv32_mul_30.rs ]
crate::ix!();

/// Compute a*factor and put it in r. 
///
/// All but the top limb in r will be in range [0,2^30).
/// 
#[cfg(VERIFY)]
pub fn modinv32_mul_30(r: *mut ModInv32Signed30, a: *const ModInv32Signed30, alen: i32, factor: i32) {
    unsafe {
        const M30: i32 = (u32::MAX >> 2) as i32;
        let mut c: i64 = 0;
        let mut i: i32;

        i = 0;
        while i < 8 {
            if i < alen {
                c += (i64::from((*a).v[i as usize])) * (i64::from(factor));
            }
            (*r).v[i as usize] = (c as i32) & M30;
            c >>= 30;
            i += 1;
        }
        if 8 < alen {
            c += (i64::from((*a).v[8])) * (i64::from(factor));
        }
        verify_check!(c == (c as i32) as i64);
        (*r).v[8] = c as i32;
    }
}

#[cfg(test)]
mod modinv32_mul_30_arithmetic_validation {
    use super::*;

    #[cfg(VERIFY)]
    #[traced_test]
    fn mul_30_matches_reference_digit_extraction_for_various_lengths_and_factors() {
        let factors: [i32; 7] = [-9, -2, -1, 0, 1, 2, 9];
        let alens: [i32; 5] = [1, 2, 3, 8, 9];

        let mut seed: u64 = 0xDEAD_BEEF_1234_5678u64;

        for &alen in alens.iter() {
            for &factor in factors.iter() {
                for _ in 0..64 {
                    let mut a = ModInv32Signed30 { v: [0i32; 9] };
                    let mut i: usize = 0;
                    while i < 9 {
                        let r = support::xorshift64_star(&mut seed);
                        let limb = ((r % 2001) as i64) - 1000;
                        a.v[i] = limb as i32;
                        i += 1;
                    }

                    /* Ensure limbs stay within (-2^30,2^30) (strictly within for these bounds). */
                    support::assert_signed30_limbs_within_signed_bound(&a);

                    let mut out = ModInv32Signed30 { v: [0i32; 9] };
                    modinv32_mul_30(
                        (&mut out) as *mut ModInv32Signed30,
                        (&a) as *const ModInv32Signed30,
                        alen,
                        factor,
                    );

                    /* Reference: reproduce the C digit extraction logic in i64. */
                    const M30: i32 = (u32::MAX >> 2) as i32;
                    let mut c: i64 = 0;
                    let mut exp = [0i32; 9];

                    let mut idx: i32 = 0;
                    while idx < 8 {
                        if idx < alen {
                            c += (a.v[idx as usize] as i64) * (factor as i64);
                        }
                        exp[idx as usize] = (c as i32) & M30;
                        c >>= 30;
                        idx += 1;
                    }

                    if 8 < alen {
                        c += (a.v[8] as i64) * (factor as i64);
                    }
                    exp[8] = c as i32;

                    tracing::debug!(
                        alen,
                        factor,
                        "mul_30 case validated (reference digits)"
                    );

                    assert!(out.v == exp);

                    let mut k: usize = 0;
                    while k < 8 {
                        assert!((out.v[k] >> 30) == 0);
                        k += 1;
                    }
                }
            }
        }
    }
}
