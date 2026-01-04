// ---------------- [ File: bitcoinsecp256k1-modinv64/src/modinv64_normalize_62.rs ]
crate::ix!();

/// Take as input a signed62 number in range (-2*modulus,modulus), and add a multiple of the
/// modulus to it to bring it to range [0,modulus).
/// 
/// If sign < 0, the input will also be negated in the process. 
///
/// The input must have limbs in range (-2^62,2^62). The output will have limbs in range [0,2^62).
/// 
pub fn modinv64_normalize_62(
        r:       *mut ModInv64Signed62,
        sign:    i64,
        modinfo: *const ModInv64ModInfo)  {

    const M62: i64 = (u64::MAX >> 2) as i64;

    unsafe {
        let mut r0: i64 = (*r).v()[0];
        let mut r1: i64 = (*r).v()[1];
        let mut r2: i64 = (*r).v()[2];
        let mut r3: i64 = (*r).v()[3];
        let mut r4: i64 = (*r).v()[4];
        let modulus = (*modinfo).modulus();

        let mut cond_add: i64;
        let mut cond_negate: i64;

        #[cfg(VERIFY)]
        {
            /* Verify that all limbs are in range (-2^62,2^62). */
            let mut i: i32 = 0;
            while i < 5 {
                verify_check!((*r).v()[i as usize] >= -M62);
                verify_check!((*r).v()[i as usize] <= M62);
                i += 1;
            }
            verify_check!(modinv64_mul_cmp_62(r as *const _, 5, modulus as *const _, -2) > 0); /* r > -2*modulus */
            verify_check!(modinv64_mul_cmp_62(r as *const _, 5, modulus as *const _, 1) < 0); /* r < modulus */
        }

        /* In a first step, add the modulus if the input is negative, and then negate if requested.
         * This brings r from range (-2*modulus,modulus) to range (-modulus,modulus). As all input
         * limbs are in range (-2^62,2^62), this cannot overflow an int64_t. Note that the right
         * shifts below are signed sign-extending shifts (see assumptions.h for tests that that is
         * indeed the behavior of the right shift operator). */
        cond_add = r4 >> 63;
        r0 = r0.wrapping_add(modulus.v()[0] & cond_add);
        r1 = r1.wrapping_add(modulus.v()[1] & cond_add);
        r2 = r2.wrapping_add(modulus.v()[2] & cond_add);
        r3 = r3.wrapping_add(modulus.v()[3] & cond_add);
        r4 = r4.wrapping_add(modulus.v()[4] & cond_add);
        cond_negate = sign >> 63;
        r0 = (r0 ^ cond_negate).wrapping_sub(cond_negate);
        r1 = (r1 ^ cond_negate).wrapping_sub(cond_negate);
        r2 = (r2 ^ cond_negate).wrapping_sub(cond_negate);
        r3 = (r3 ^ cond_negate).wrapping_sub(cond_negate);
        r4 = (r4 ^ cond_negate).wrapping_sub(cond_negate);
        /* Propagate the top bits, to bring limbs back to range (-2^62,2^62). */
        r1 = r1.wrapping_add(r0 >> 62); r0 &= M62;
        r2 = r2.wrapping_add(r1 >> 62); r1 &= M62;
        r3 = r3.wrapping_add(r2 >> 62); r2 &= M62;
        r4 = r4.wrapping_add(r3 >> 62); r3 &= M62;

        /* In a second step add the modulus again if the result is still negative, bringing
         * r to range [0,modulus). */
        cond_add = r4 >> 63;
        r0 = r0.wrapping_add(modulus.v()[0] & cond_add);
        r1 = r1.wrapping_add(modulus.v()[1] & cond_add);
        r2 = r2.wrapping_add(modulus.v()[2] & cond_add);
        r3 = r3.wrapping_add(modulus.v()[3] & cond_add);
        r4 = r4.wrapping_add(modulus.v()[4] & cond_add);
        /* And propagate again. */
        r1 = r1.wrapping_add(r0 >> 62); r0 &= M62;
        r2 = r2.wrapping_add(r1 >> 62); r1 &= M62;
        r3 = r3.wrapping_add(r2 >> 62); r2 &= M62;
        r4 = r4.wrapping_add(r3 >> 62); r3 &= M62;

        (*r).v_mut()[0] = r0;
        (*r).v_mut()[1] = r1;
        (*r).v_mut()[2] = r2;
        (*r).v_mut()[3] = r3;
        (*r).v_mut()[4] = r4;

        #[cfg(VERIFY)]
        {
            verify_check!(r0 >> 62 == 0);
            verify_check!(r1 >> 62 == 0);
            verify_check!(r2 >> 62 == 0);
            verify_check!(r3 >> 62 == 0);
            verify_check!(r4 >> 62 == 0);
            verify_check!(modinv64_mul_cmp_62(r as *const _, 5, modulus as *const _, 0) >= 0); /* r >= 0 */
            verify_check!(modinv64_mul_cmp_62(r as *const _, 5, modulus as *const _, 1) < 0); /* r < modulus */
        }
    }
}

#[cfg(test)]
use crate::modinv64_mod_info_contract::*;

#[cfg(test)]
mod modinv64_normalize_62_contract {
    use super::*;

    #[traced_test]
    fn normalize_62_places_output_in_canonical_range_for_edge_cases() {
        let modulus: u128 = (1u128 << 119) - 23; /* odd, fits <= 120 bits */
        let modinfo = build_modinfo_from_u128(modulus);

        let cases: &[(i128, i64)] = &[
            (0, 1),
            (1, 1),
            (-1, 1),
            ((modulus as i128) - 1, 1),
            (-(modulus as i128) + 1, 1),
            (-(2 * modulus as i128) + 1, 1),
            (0, -1),
            (1, -1),
            (-1, -1),
            ((modulus as i128) - 1, -1),
            (-(modulus as i128) + 1, -1),
        ];

        let mut idx: usize = 0;
        while idx < cases.len() {
            let (r_in, sign) = cases[idx];
            let mut r = signed62_from_i128(r_in);

            trace!(idx = idx, r_in = r_in, sign = sign, limbs_before = ?r.v());

            modinv64_normalize_62(&mut r as *mut _, sign, &modinfo as *const _);

            trace!(idx = idx, limbs_after = ?r.v());

            assert!(signed62_is_fully_normalized_nonnegative(&r));
            assert!(r.v()[2] == 0 && r.v()[3] == 0 && r.v()[4] == 0);

            let out = signed62_to_u128_assuming_nonnegative_and_fit(&r);
            assert!(out < modulus);

            let effective: i128 = if sign < 0 { -r_in } else { r_in };
            let expected: u128 = (effective.rem_euclid(modulus as i128)) as u128;

            trace!(idx = idx, out = out, expected = expected);
            assert!(out == expected);

            idx += 1;
        }
    }

    #[traced_test]
    fn normalize_62_round_trips_random_inputs_with_reference_modulo() {
        let mut seed: u64 = 0xF4CC_7E9D_1B2A_3C4D;

        let modulus: u128 = ((1u128 << 118) + (1u128 << 77) + 0xC0FFEEu128) | 1;
        let modinfo = build_modinfo_from_u128(modulus);

        let mut i: usize = 0;
        while i < 512 {
            let span: u128 = 3 * modulus;
            let raw = (splitmix128_next(&mut seed) % span) as i128;
            let r_in: i128 = raw - (2 * modulus) as i128;
            let sign: i64 = if (splitmix128_next(&mut seed) & 1) == 0 { 1 } else { -1 };

            let mut r = signed62_from_i128(r_in);

            trace!(iter = i, r_in = r_in, sign = sign, limbs_before = ?r.v());

            modinv64_normalize_62(&mut r as *mut _, sign, &modinfo as *const _);

            trace!(iter = i, limbs_after = ?r.v());
            assert!(signed62_is_fully_normalized_nonnegative(&r));

            let out = signed62_to_u128_assuming_nonnegative_and_fit(&r);
            assert!(out < modulus);

            let effective: i128 = if sign < 0 { -r_in } else { r_in };
            let expected: u128 = (effective.rem_euclid(modulus as i128)) as u128;

            assert!(out == expected);
            i += 1;
        }
    }
}
