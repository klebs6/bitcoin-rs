// ---------------- [ File: bitcoinsecp256k1-modinv32/src/modinv32_normalize_30.rs ]
crate::ix!();

/// Take as input a signed30 number in range (-2*modulus,modulus), and add a multiple of the
/// modulus to it to bring it to range [0,modulus). 
/// 
/// If sign < 0, the input will also be negated in the process. 
/// 
/// The input must have limbs in range (-2^30,2^30). The output will have limbs in range [0,2^30). 
///
pub fn modinv32_normalize_30(r: *mut ModInv32Signed30, sign: i32, modinfo: *const ModInv32ModInfo) {
    unsafe {
        const M30: i32 = (u32::MAX >> 2) as i32;
        let mut r0: i32 = (*r).v[0];
        let mut r1: i32 = (*r).v[1];
        let mut r2: i32 = (*r).v[2];
        let mut r3: i32 = (*r).v[3];
        let mut r4: i32 = (*r).v[4];
        let mut r5: i32 = (*r).v[5];
        let mut r6: i32 = (*r).v[6];
        let mut r7: i32 = (*r).v[7];
        let mut r8: i32 = (*r).v[8];
        let mut cond_add: i32;
        let mut cond_negate: i32;

        #[cfg(VERIFY)]
        {
            /* Verify that all limbs are in range (-2^30,2^30). */
            let mut i: i32;
            i = 0;
            while i < 9 {
                VERIFY_CHECK!((*r).v[i as usize] >= -M30);
                VERIFY_CHECK!((*r).v[i as usize] <= M30);
                i += 1;
            }
            VERIFY_CHECK!(modinv32_mul_cmp_30(r as *const ModInv32Signed30, 9, &(*modinfo).modulus, -2) > 0); /* r > -2*modulus */
            VERIFY_CHECK!(modinv32_mul_cmp_30(r as *const ModInv32Signed30, 9, &(*modinfo).modulus, 1) < 0); /* r < modulus */
        }

        /* In a first step, add the modulus if the input is negative, and then negate if requested.
         * This brings r from range (-2*modulus,modulus) to range (-modulus,modulus). As all input
         * limbs are in range (-2^30,2^30), this cannot overflow an int32_t. Note that the right
         * shifts below are signed sign-extending shifts (see assumptions.h for tests that that is
         * indeed the behavior of the right shift operator). */
        cond_add = r8 >> 31;
        r0 = r0.wrapping_add((*modinfo).modulus.v[0] & cond_add);
        r1 = r1.wrapping_add((*modinfo).modulus.v[1] & cond_add);
        r2 = r2.wrapping_add((*modinfo).modulus.v[2] & cond_add);
        r3 = r3.wrapping_add((*modinfo).modulus.v[3] & cond_add);
        r4 = r4.wrapping_add((*modinfo).modulus.v[4] & cond_add);
        r5 = r5.wrapping_add((*modinfo).modulus.v[5] & cond_add);
        r6 = r6.wrapping_add((*modinfo).modulus.v[6] & cond_add);
        r7 = r7.wrapping_add((*modinfo).modulus.v[7] & cond_add);
        r8 = r8.wrapping_add((*modinfo).modulus.v[8] & cond_add);
        cond_negate = sign >> 31;
        r0 = (r0 ^ cond_negate).wrapping_sub(cond_negate);
        r1 = (r1 ^ cond_negate).wrapping_sub(cond_negate);
        r2 = (r2 ^ cond_negate).wrapping_sub(cond_negate);
        r3 = (r3 ^ cond_negate).wrapping_sub(cond_negate);
        r4 = (r4 ^ cond_negate).wrapping_sub(cond_negate);
        r5 = (r5 ^ cond_negate).wrapping_sub(cond_negate);
        r6 = (r6 ^ cond_negate).wrapping_sub(cond_negate);
        r7 = (r7 ^ cond_negate).wrapping_sub(cond_negate);
        r8 = (r8 ^ cond_negate).wrapping_sub(cond_negate);
        /* Propagate the top bits, to bring limbs back to range (-2^30,2^30). */
        r1 = r1.wrapping_add(r0 >> 30);
        r0 &= M30;
        r2 = r2.wrapping_add(r1 >> 30);
        r1 &= M30;
        r3 = r3.wrapping_add(r2 >> 30);
        r2 &= M30;
        r4 = r4.wrapping_add(r3 >> 30);
        r3 &= M30;
        r5 = r5.wrapping_add(r4 >> 30);
        r4 &= M30;
        r6 = r6.wrapping_add(r5 >> 30);
        r5 &= M30;
        r7 = r7.wrapping_add(r6 >> 30);
        r6 &= M30;
        r8 = r8.wrapping_add(r7 >> 30);
        r7 &= M30;

        /* In a second step add the modulus again if the result is still negative, bringing r to range
         * [0,modulus). */
        cond_add = r8 >> 31;
        r0 = r0.wrapping_add((*modinfo).modulus.v[0] & cond_add);
        r1 = r1.wrapping_add((*modinfo).modulus.v[1] & cond_add);
        r2 = r2.wrapping_add((*modinfo).modulus.v[2] & cond_add);
        r3 = r3.wrapping_add((*modinfo).modulus.v[3] & cond_add);
        r4 = r4.wrapping_add((*modinfo).modulus.v[4] & cond_add);
        r5 = r5.wrapping_add((*modinfo).modulus.v[5] & cond_add);
        r6 = r6.wrapping_add((*modinfo).modulus.v[6] & cond_add);
        r7 = r7.wrapping_add((*modinfo).modulus.v[7] & cond_add);
        r8 = r8.wrapping_add((*modinfo).modulus.v[8] & cond_add);
        /* And propagate again. */
        r1 = r1.wrapping_add(r0 >> 30);
        r0 &= M30;
        r2 = r2.wrapping_add(r1 >> 30);
        r1 &= M30;
        r3 = r3.wrapping_add(r2 >> 30);
        r2 &= M30;
        r4 = r4.wrapping_add(r3 >> 30);
        r3 &= M30;
        r5 = r5.wrapping_add(r4 >> 30);
        r4 &= M30;
        r6 = r6.wrapping_add(r5 >> 30);
        r5 &= M30;
        r7 = r7.wrapping_add(r6 >> 30);
        r6 &= M30;
        r8 = r8.wrapping_add(r7 >> 30);
        r7 &= M30;

        (*r).v[0] = r0;
        (*r).v[1] = r1;
        (*r).v[2] = r2;
        (*r).v[3] = r3;
        (*r).v[4] = r4;
        (*r).v[5] = r5;
        (*r).v[6] = r6;
        (*r).v[7] = r7;
        (*r).v[8] = r8;

        #[cfg(VERIFY)]
        {
            VERIFY_CHECK!(r0 >> 30 == 0);
            VERIFY_CHECK!(r1 >> 30 == 0);
            VERIFY_CHECK!(r2 >> 30 == 0);
            VERIFY_CHECK!(r3 >> 30 == 0);
            VERIFY_CHECK!(r4 >> 30 == 0);
            VERIFY_CHECK!(r5 >> 30 == 0);
            VERIFY_CHECK!(r6 >> 30 == 0);
            VERIFY_CHECK!(r7 >> 30 == 0);
            VERIFY_CHECK!(r8 >> 30 == 0);
            VERIFY_CHECK!(modinv32_mul_cmp_30(r as *const ModInv32Signed30, 9, &(*modinfo).modulus, 0) >= 0); /* r >= 0 */
            VERIFY_CHECK!(modinv32_mul_cmp_30(r as *const ModInv32Signed30, 9, &(*modinfo).modulus, 1) < 0); /* r < modulus */
        }
    }

    /*
        const int32_t M30 = (int32_t)(UINT32_MAX >> 2);
    int32_t r0 = r->v[0], r1 = r->v[1], r2 = r->v[2], r3 = r->v[3], r4 = r->v[4],
            r5 = r->v[5], r6 = r->v[6], r7 = r->v[7], r8 = r->v[8];
    int32_t cond_add, cond_negate;

#ifdef VERIFY
    /* Verify that all limbs are in range (-2^30,2^30). */
    int i;
    for (i = 0; i < 9; ++i) {
        VERIFY_CHECK(r->v[i] >= -M30);
        VERIFY_CHECK(r->v[i] <= M30);
    }
    VERIFY_CHECK(modinv32_mul_cmp_30(r, 9, &modinfo->modulus, -2) > 0); /* r > -2*modulus */
    VERIFY_CHECK(modinv32_mul_cmp_30(r, 9, &modinfo->modulus, 1) < 0); /* r < modulus */
#endif

    /* In a first step, add the modulus if the input is negative, and then negate if requested.
     * This brings r from range (-2*modulus,modulus) to range (-modulus,modulus). As all input
     * limbs are in range (-2^30,2^30), this cannot overflow an int32_t. Note that the right
     * shifts below are signed sign-extending shifts (see assumptions.h for tests that that is
     * indeed the behavior of the right shift operator). */
    cond_add = r8 >> 31;
    r0 += modinfo->modulus.v[0] & cond_add;
    r1 += modinfo->modulus.v[1] & cond_add;
    r2 += modinfo->modulus.v[2] & cond_add;
    r3 += modinfo->modulus.v[3] & cond_add;
    r4 += modinfo->modulus.v[4] & cond_add;
    r5 += modinfo->modulus.v[5] & cond_add;
    r6 += modinfo->modulus.v[6] & cond_add;
    r7 += modinfo->modulus.v[7] & cond_add;
    r8 += modinfo->modulus.v[8] & cond_add;
    cond_negate = sign >> 31;
    r0 = (r0 ^ cond_negate) - cond_negate;
    r1 = (r1 ^ cond_negate) - cond_negate;
    r2 = (r2 ^ cond_negate) - cond_negate;
    r3 = (r3 ^ cond_negate) - cond_negate;
    r4 = (r4 ^ cond_negate) - cond_negate;
    r5 = (r5 ^ cond_negate) - cond_negate;
    r6 = (r6 ^ cond_negate) - cond_negate;
    r7 = (r7 ^ cond_negate) - cond_negate;
    r8 = (r8 ^ cond_negate) - cond_negate;
    /* Propagate the top bits, to bring limbs back to range (-2^30,2^30). */
    r1 += r0 >> 30; r0 &= M30;
    r2 += r1 >> 30; r1 &= M30;
    r3 += r2 >> 30; r2 &= M30;
    r4 += r3 >> 30; r3 &= M30;
    r5 += r4 >> 30; r4 &= M30;
    r6 += r5 >> 30; r5 &= M30;
    r7 += r6 >> 30; r6 &= M30;
    r8 += r7 >> 30; r7 &= M30;

    /* In a second step add the modulus again if the result is still negative, bringing r to range
     * [0,modulus). */
    cond_add = r8 >> 31;
    r0 += modinfo->modulus.v[0] & cond_add;
    r1 += modinfo->modulus.v[1] & cond_add;
    r2 += modinfo->modulus.v[2] & cond_add;
    r3 += modinfo->modulus.v[3] & cond_add;
    r4 += modinfo->modulus.v[4] & cond_add;
    r5 += modinfo->modulus.v[5] & cond_add;
    r6 += modinfo->modulus.v[6] & cond_add;
    r7 += modinfo->modulus.v[7] & cond_add;
    r8 += modinfo->modulus.v[8] & cond_add;
    /* And propagate again. */
    r1 += r0 >> 30; r0 &= M30;
    r2 += r1 >> 30; r1 &= M30;
    r3 += r2 >> 30; r2 &= M30;
    r4 += r3 >> 30; r3 &= M30;
    r5 += r4 >> 30; r4 &= M30;
    r6 += r5 >> 30; r5 &= M30;
    r7 += r6 >> 30; r6 &= M30;
    r8 += r7 >> 30; r7 &= M30;

    r->v[0] = r0;
    r->v[1] = r1;
    r->v[2] = r2;
    r->v[3] = r3;
    r->v[4] = r4;
    r->v[5] = r5;
    r->v[6] = r6;
    r->v[7] = r7;
    r->v[8] = r8;

#ifdef VERIFY
    VERIFY_CHECK(r0 >> 30 == 0);
    VERIFY_CHECK(r1 >> 30 == 0);
    VERIFY_CHECK(r2 >> 30 == 0);
    VERIFY_CHECK(r3 >> 30 == 0);
    VERIFY_CHECK(r4 >> 30 == 0);
    VERIFY_CHECK(r5 >> 30 == 0);
    VERIFY_CHECK(r6 >> 30 == 0);
    VERIFY_CHECK(r7 >> 30 == 0);
    VERIFY_CHECK(r8 >> 30 == 0);
    VERIFY_CHECK(modinv32_mul_cmp_30(r, 9, &modinfo->modulus, 0) >= 0); /* r >= 0 */
    VERIFY_CHECK(modinv32_mul_cmp_30(r, 9, &modinfo->modulus, 1) < 0); /* r < modulus */
#endif
    */
}

#[cfg(test)]
mod modinv32_normalize_30_range_validation {
    use super::*;

    #[traced_test]
    fn normalize_30_maps_values_to_canonical_range_and_preserves_residue() {
        let moduli: [u64; 6] = [3, 5, 101, 257, 65537, 1_000_000_007];

        for &modulus in moduli.iter() {
            let modinfo = support::modinfo_from_u64(modulus);
            tracing::info!(modulus, "validating modinv32_normalize_30");

            let test_values: [i128; 10] = [
                (-(2i128) * (modulus as i128)) + 1,
                -(modulus as i128),
                -(modulus as i128) + 1,
                -1,
                0,
                1,
                (modulus as i128) - 2,
                (modulus as i128) - 1,
                -2,
                2,
            ];
            let sign_values: [i32; 3] = [0, 1, -1];

            for &sign in sign_values.iter() {
                for &v in test_values.iter() {
                    let mut r = support::signed30_from_i128_sign_extended(v);

                    tracing::trace!(
                        modulus,
                        sign,
                        v,
                        r_in = ?r.v,
                        "normalize_30 input (sign-extended signed30)"
                    );

                    modinv32_normalize_30(
                        (&mut r) as *mut ModInv32Signed30,
                        sign,
                        (&modinfo) as *const ModInv32ModInfo,
                    );

                    tracing::trace!(
                        modulus,
                        sign,
                        v,
                        r_out = ?r.v,
                        "normalize_30 output"
                    );

                    support::assert_signed30_limbs_are_normalized(&r);
                    let out_u128 = support::signed30_to_u128_horner(&r);
                    assert!(out_u128 < modulus as u128);

                    let expected = support::normalize_mod_u64(if sign < 0 { -v } else { v }, modulus) as u128;

                    tracing::debug!(
                        modulus,
                        sign,
                        v,
                        out = out_u128,
                        expected,
                        "normalize_30 canonical residue check"
                    );

                    assert!(out_u128 == expected);
                }
            }

            /* Non-canonical negative one input. */
            {
                let sign_values_nc: [i32; 3] = [0, 1, -1];
                for &sign in sign_values_nc.iter() {
                    let mut r = support::noncanonical_negative_one();

                    tracing::trace!(
                        modulus,
                        sign,
                        r_in = ?r.v,
                        "normalize_30 noncanonical -1 input"
                    );

                    modinv32_normalize_30(
                        (&mut r) as *mut ModInv32Signed30,
                        sign,
                        (&modinfo) as *const ModInv32ModInfo,
                    );

                    tracing::trace!(
                        modulus,
                        sign,
                        r_out = ?r.v,
                        "normalize_30 noncanonical -1 output"
                    );

                    support::assert_signed30_limbs_are_normalized(&r);
                    let out_u128 = support::signed30_to_u128_horner(&r);
                    assert!(out_u128 < modulus as u128);

                    let expected = support::normalize_mod_u64(if sign < 0 { 1 } else { -1 }, modulus) as u128;

                    tracing::debug!(
                        modulus,
                        sign,
                        out = out_u128,
                        expected,
                        "normalize_30 noncanonical -1 residue check"
                    );

                    assert!(out_u128 == expected);
                }
            }
        }
    }
}
