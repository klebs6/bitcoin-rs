// ---------------- [ File: bitcoinsecp256k1-modinv64/src/modinv64_var.rs ]
crate::ix!();

/// Compute the inverse of x modulo modinfo->modulus, and replace x with it (variable time).
/// 
/// Replace x with its modular inverse mod modinfo->modulus. x must be in range [0, modulus).
/// 
/// If x is zero, the result will be zero as well. 
///
/// If not, the inverse must exist (i.e., the gcd of x and modulus must be 1).
/// 
/// These rules are automatically satisfied if the modulus is prime.
/// 
/// On output, all of x's limbs will be in [0, 2^62).
/// 
pub fn modinv64_var(
        x:       *mut ModInv64Signed62,
        modinfo: *const ModInv64ModInfo)  {

    unsafe {
        /* Start with d=0, e=1, f=modulus, g=x, eta=-1. */
        let mut d = ModInv64Signed62::from_limbs([0, 0, 0, 0, 0]);
        let mut e = ModInv64Signed62::from_limbs([1, 0, 0, 0, 0]);
        let mut f = (*modinfo).modulus().clone();
        let mut g = *x;
        #[cfg(VERIFY)]
        let mut i: i32 = 0;
        let mut j: i32;
        let mut len: i32 = 5;
        let mut eta: i64 = -1; /* eta = -delta; delta is initially 1 */
        let mut cond: i64;
        let mut fn_: i64;
        let mut gn_: i64;

        /* Do iterations of 62 divsteps each until g=0. */
        loop {
            /* Compute transition matrix and new eta after 62 divsteps. */
            let mut t = core::mem::MaybeUninit::<ModInv64Trans2x2>::uninit();
            eta = modinv64_divsteps_62_var(eta, f.v()[0] as u64, g.v()[0] as u64, t.as_mut_ptr());
            /* Update d,e using that transition matrix. */
            modinv64_update_de_62(&mut d as *mut _, &mut e as *mut _, t.as_ptr(), modinfo);
            /* Update f,g using that transition matrix. */
            #[cfg(VERIFY)]
            {
                verify_check!(modinv64_mul_cmp_62(&f as *const _, len, (*modinfo).modulus() as *const _, -1) > 0); /* f > -modulus */
                verify_check!(modinv64_mul_cmp_62(&f as *const _, len, (*modinfo).modulus() as *const _, 1) <= 0); /* f <= modulus */
                verify_check!(modinv64_mul_cmp_62(&g as *const _, len, (*modinfo).modulus() as *const _, -1) > 0); /* g > -modulus */
                verify_check!(modinv64_mul_cmp_62(&g as *const _, len, (*modinfo).modulus() as *const _, 1) < 0);  /* g <  modulus */
            }
            modinv64_update_fg_62_var(len, &mut f as *mut _, &mut g as *mut _, t.as_ptr());
            /* If the bottom limb of g is zero, there is a chance that g=0. */
            if g.v()[0] == 0 {
                cond = 0;
                /* Check if the other limbs are also 0. */
                j = 1;
                while j < len {
                    cond |= g.v()[j as usize];
                    j += 1;
                }
                /* If so, we're done. */
                if cond == 0 { break; }
            }

            /* Determine if len>1 and limb (len-1) of both f and g is 0 or -1. */
            fn_ = f.v()[(len - 1) as usize];
            gn_ = g.v()[(len - 1) as usize];
            cond = ((len as i64) - 2) >> 63;
            cond |= fn_ ^ (fn_ >> 63);
            cond |= gn_ ^ (gn_ >> 63);
            /* If so, reduce length, propagating the sign of f and g's top limb into the one below. */
            if cond == 0 {
                let idx = (len - 2) as usize;
                f.v_mut()[idx] = ((f.v()[idx] as u64) | ((fn_ as u64) << 62)) as i64;
                g.v_mut()[idx] = ((g.v()[idx] as u64) | ((gn_ as u64) << 62)) as i64;
                len -= 1;
            }
            #[cfg(VERIFY)]
            {
                i += 1;
                verify_check!(i < 12); /* We should never need more than 12*62 = 744 divsteps */
                verify_check!(modinv64_mul_cmp_62(&f as *const _, len, (*modinfo).modulus() as *const _, -1) > 0); /* f > -modulus */
                verify_check!(modinv64_mul_cmp_62(&f as *const _, len, (*modinfo).modulus() as *const _, 1) <= 0); /* f <= modulus */
                verify_check!(modinv64_mul_cmp_62(&g as *const _, len, (*modinfo).modulus() as *const _, -1) > 0); /* g > -modulus */
                verify_check!(modinv64_mul_cmp_62(&g as *const _, len, (*modinfo).modulus() as *const _, 1) < 0);  /* g <  modulus */
            }
        }

        /* At this point g is 0 and (if g was not originally 0) f must now equal +/- GCD of
         * the initial f, g values i.e. +/- 1, and d now contains +/- the modular inverse. */
        #[cfg(VERIFY)]
        {
            /* g == 0 */
            verify_check!(modinv64_mul_cmp_62(&g as *const _, len, &SIGNED62_ONE as *const _, 0) == 0);
            /* |f| == 1, or (x == 0 and d == 0 and |f|=modulus) */
            verify_check!(modinv64_mul_cmp_62(&f as *const _, len, &SIGNED62_ONE as *const _, -1) == 0 ||
                         modinv64_mul_cmp_62(&f as *const _, len, &SIGNED62_ONE as *const _, 1) == 0 ||
                         (modinv64_mul_cmp_62(x as *const _, 5, &SIGNED62_ONE as *const _, 0) == 0 &&
                          modinv64_mul_cmp_62(&d as *const _, 5, &SIGNED62_ONE as *const _, 0) == 0 &&
                          (modinv64_mul_cmp_62(&f as *const _, len, (*modinfo).modulus() as *const _, 1) == 0 ||
                           modinv64_mul_cmp_62(&f as *const _, len, (*modinfo).modulus() as *const _, -1) == 0)));
        }

        /* Optionally negate d, normalize to [0,modulus), and return it. */
        modinv64_normalize_62(&mut d as *mut _, f.v()[(len - 1) as usize], modinfo);
        *x = d;
    }
}

#[cfg(test)]
mod modinv64_variable_time_inversion_contract {
    use super::*;

    #[traced_test]
    fn variable_time_inversion_zero_maps_to_zero() {
        let modulus: u128 = ((1u128 << 119) + 33) | 1;
        let modinfo = build_modinfo_from_u128(modulus);

        let mut x = signed62_from_u128(0);
        modinv64_var(&mut x as *mut _, &modinfo as *const _);

        trace!(x_out = ?x.v());
        assert!(signed62_is_fully_normalized_nonnegative(&x));
        assert!(x.v()[0] == 0);
        assert!(signed62_to_u128_assuming_nonnegative_and_fit(&x) == 0);
    }

    #[traced_test]
    fn variable_time_inversion_matches_extended_gcd_reference_over_random_cases() {
        let mut seed: u64 = 0x0102_0304_0506_0708;

        let mut case_idx: usize = 0;
        while case_idx < 256 {
            let modulus = sample_odd_modulus_up_to_120_bits(&mut seed);
            let modinfo = build_modinfo_from_u128(modulus);

            let mut x_val = splitmix128_next(&mut seed) % modulus;
            if x_val != 0 && gcd_u128(x_val, modulus) != 1 {
                x_val = sample_nonzero_coprime_u128(&mut seed, modulus);
            }

            let mut x = signed62_from_u128(x_val);

            trace!(case_idx = case_idx, modulus = modulus, x_in = x_val, limbs_in = ?x.v());

            modinv64_var(&mut x as *mut _, &modinfo as *const _);

            trace!(case_idx = case_idx, limbs_out = ?x.v());

            assert!(signed62_is_fully_normalized_nonnegative(&x));
            assert_signed62_lt_modulus(&x, modinfo.modulus());

            let inv = signed62_to_u128_assuming_nonnegative_and_fit(&x);

            if x_val == 0 {
                assert!(inv == 0);
            } else {
                let expected = modinv_u128((x_val % modulus) as u128, modulus);
                assert!(inv == expected);

                let check = mul_mod_u128(x_val, inv, modulus);
                assert!(check == 1);
            }

            case_idx += 1;
        }
    }

    #[traced_test]
    fn variable_time_inversion_handles_edge_values_across_random_moduli() {
        let mut seed: u64 = 0xB16B_00B5_DEAD_BEEF;

        let mut i: usize = 0;
        while i < 128 {
            let modulus = sample_odd_modulus_up_to_120_bits(&mut seed);
            let modinfo = build_modinfo_from_u128(modulus);

            let edge = [
                0u128,
                1u128,
                2u128 % modulus,
                (modulus - 1),
                (modulus / 2),
            ];

            let mut j: usize = 0;
            while j < edge.len() {
                let x_val = edge[j];
                let mut x = signed62_from_u128(x_val);

                trace!(iter = i, j = j, modulus = modulus, x_in = x_val);

                modinv64_var(&mut x as *mut _, &modinfo as *const _);

                assert!(signed62_is_fully_normalized_nonnegative(&x));
                let inv = signed62_to_u128_assuming_nonnegative_and_fit(&x);

                if x_val == 0 {
                    assert!(inv == 0);
                } else if gcd_u128(x_val, modulus) == 1 {
                    let check = mul_mod_u128(x_val, inv, modulus);
                    assert!(check == 1);
                }

                j += 1;
            }

            i += 1;
        }
    }
}
