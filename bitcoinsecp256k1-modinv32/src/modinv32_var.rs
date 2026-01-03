// ---------------- [ File: bitcoinsecp256k1-modinv32/src/modinv32_var.rs ]
crate::ix!();

/// Compute the inverse of x modulo modinfo->modulus, and replace x with it (variable time).
/// 
/// Replace x with its modular inverse mod modinfo->modulus. 
///
/// x must be in range [0, modulus).
/// 
/// If x is zero, the result will be zero as well. 
///
/// If not, the inverse must exist (i.e., the gcd of x and modulus must be 1). 
///
/// These rules are automatically satisfied if the modulus is prime.
/// 
/// On output, all of x's limbs will be in [0, 2^30).
/// 
pub fn modinv32_var(x: *mut ModInv32Signed30, modinfo: *const ModInv32ModInfo) {
    unsafe {
        /* Start with d=0, e=1, f=modulus, g=x, eta=-1. */
        let mut d = ModInv32Signed30 { v: [0i32; 9] };
        let mut e = ModInv32Signed30 { v: [0i32; 9] };
        e.v[0] = 1;
        let mut f = std::ptr::read(std::ptr::addr_of!((*modinfo).modulus));
        let mut g = std::ptr::read(x as *const ModInv32Signed30);
        #[cfg(VERIFY)]
        let mut i: i32 = 0;
        let mut j: i32;
        let mut len: i32 = 9;
        let mut eta: i32 = -1; /* eta = -delta; delta is initially 1 (faster for the variable-time code) */
        let mut cond: i32;
        let mut fn_: i32;
        let mut gn_: i32;

        /* Do iterations of 30 divsteps each until g=0. */
        loop {
            /* Compute transition matrix and new eta after 30 divsteps. */
            let mut t = std::mem::MaybeUninit::<ModInv32Trans2x2>::uninit();
            eta = modinv32_divsteps_30_var(eta, f.v[0] as u32, g.v[0] as u32, t.as_mut_ptr());
            let t = t.assume_init();
            /* Update d,e using that transition matrix. */
            modinv32_update_de_30(&mut d, &mut e, &t, modinfo);
            /* Update f,g using that transition matrix. */
            #[cfg(VERIFY)]
            {
                VERIFY_CHECK!(modinv32_mul_cmp_30(&f, len, &(*modinfo).modulus, -1) > 0); /* f > -modulus */
                VERIFY_CHECK!(modinv32_mul_cmp_30(&f, len, &(*modinfo).modulus, 1) <= 0); /* f <= modulus */
                VERIFY_CHECK!(modinv32_mul_cmp_30(&g, len, &(*modinfo).modulus, -1) > 0); /* g > -modulus */
                VERIFY_CHECK!(modinv32_mul_cmp_30(&g, len, &(*modinfo).modulus, 1) < 0); /* g <  modulus */
            }
            modinv32_update_fg_30_var(len, &mut f, &mut g, &t);
            /* If the bottom limb of g is 0, there is a chance g=0. */
            if g.v[0] == 0 {
                cond = 0;
                /* Check if all other limbs are also 0. */
                j = 1;
                while j < len {
                    cond |= g.v[j as usize];
                    j += 1;
                }
                /* If so, we're done. */
                if cond == 0 {
                    break;
                }
            }

            /* Determine if len>1 and limb (len-1) of both f and g is 0 or -1. */
            fn_ = f.v[(len - 1) as usize];
            gn_ = g.v[(len - 1) as usize];
            cond = (len - 2) >> 31;
            cond |= fn_ ^ (fn_ >> 31);
            cond |= gn_ ^ (gn_ >> 31);
            /* If so, reduce length, propagating the sign of f and g's top limb into the one below. */
            if cond == 0 {
                let idx = (len - 2) as usize;
                f.v[idx] = ((f.v[idx] as u32) | ((fn_ as u32) << 30)) as i32;
                g.v[idx] = ((g.v[idx] as u32) | ((gn_ as u32) << 30)) as i32;
                len -= 1;
            }
            #[cfg(VERIFY)]
            {
                i += 1;
                VERIFY_CHECK!(i < 25); /* We should never need more than 25*30 = 750 divsteps */
                VERIFY_CHECK!(modinv32_mul_cmp_30(&f, len, &(*modinfo).modulus, -1) > 0); /* f > -modulus */
                VERIFY_CHECK!(modinv32_mul_cmp_30(&f, len, &(*modinfo).modulus, 1) <= 0); /* f <= modulus */
                VERIFY_CHECK!(modinv32_mul_cmp_30(&g, len, &(*modinfo).modulus, -1) > 0); /* g > -modulus */
                VERIFY_CHECK!(modinv32_mul_cmp_30(&g, len, &(*modinfo).modulus, 1) < 0); /* g <  modulus */
            }
        }

        /* At this point g is 0 and (if g was not originally 0) f must now equal +/- GCD of
         * the initial f, g values i.e. +/- 1, and d now contains +/- the modular inverse. */
        #[cfg(VERIFY)]
        {
            /* g == 0 */
            VERIFY_CHECK!(modinv32_mul_cmp_30(&g, len, &SIGNED30_ONE, 0) == 0);
            /* |f| == 1, or (x == 0 and d == 0 and |f|=modulus) */
            VERIFY_CHECK!(
                modinv32_mul_cmp_30(&f, len, &SIGNED30_ONE, -1) == 0
                    || modinv32_mul_cmp_30(&f, len, &SIGNED30_ONE, 1) == 0
                    || (modinv32_mul_cmp_30(x as *const ModInv32Signed30, 9, &SIGNED30_ONE, 0)
                        == 0
                        && modinv32_mul_cmp_30(&d, 9, &SIGNED30_ONE, 0) == 0
                        && (modinv32_mul_cmp_30(&f, len, &(*modinfo).modulus, 1) == 0
                            || modinv32_mul_cmp_30(&f, len, &(*modinfo).modulus, -1) == 0))
            );
        }

        /* Optionally negate d, normalize to [0,modulus), and return it. */
        modinv32_normalize_30(&mut d, f.v[(len - 1) as usize], modinfo);
        std::ptr::write(x, d);
    }

    /*
        /* Start with d=0, e=1, f=modulus, g=x, eta=-1. */
    modinv32_signed30 d = {{0, 0, 0, 0, 0, 0, 0, 0, 0}};
    modinv32_signed30 e = {{1, 0, 0, 0, 0, 0, 0, 0, 0}};
    modinv32_signed30 f = modinfo->modulus;
    modinv32_signed30 g = *x;
#ifdef VERIFY
    int i = 0;
#endif
    int j, len = 9;
    int32_t eta = -1; /* eta = -delta; delta is initially 1 (faster for the variable-time code) */
    int32_t cond, fn, gn;

    /* Do iterations of 30 divsteps each until g=0. */
    while (1) {
        /* Compute transition matrix and new eta after 30 divsteps. */
        modinv32_trans2x2 t;
        eta = modinv32_divsteps_30_var(eta, f.v[0], g.v[0], &t);
        /* Update d,e using that transition matrix. */
        modinv32_update_de_30(&d, &e, &t, modinfo);
        /* Update f,g using that transition matrix. */
#ifdef VERIFY
        VERIFY_CHECK(modinv32_mul_cmp_30(&f, len, &modinfo->modulus, -1) > 0); /* f > -modulus */
        VERIFY_CHECK(modinv32_mul_cmp_30(&f, len, &modinfo->modulus, 1) <= 0); /* f <= modulus */
        VERIFY_CHECK(modinv32_mul_cmp_30(&g, len, &modinfo->modulus, -1) > 0); /* g > -modulus */
        VERIFY_CHECK(modinv32_mul_cmp_30(&g, len, &modinfo->modulus, 1) < 0);  /* g <  modulus */
#endif
        modinv32_update_fg_30_var(len, &f, &g, &t);
        /* If the bottom limb of g is 0, there is a chance g=0. */
        if (g.v[0] == 0) {
            cond = 0;
            /* Check if all other limbs are also 0. */
            for (j = 1; j < len; ++j) {
                cond |= g.v[j];
            }
            /* If so, we're done. */
            if (cond == 0) break;
        }

        /* Determine if len>1 and limb (len-1) of both f and g is 0 or -1. */
        fn = f.v[len - 1];
        gn = g.v[len - 1];
        cond = ((int32_t)len - 2) >> 31;
        cond |= fn ^ (fn >> 31);
        cond |= gn ^ (gn >> 31);
        /* If so, reduce length, propagating the sign of f and g's top limb into the one below. */
        if (cond == 0) {
            f.v[len - 2] |= (uint32_t)fn << 30;
            g.v[len - 2] |= (uint32_t)gn << 30;
            --len;
        }
#ifdef VERIFY
        VERIFY_CHECK(++i < 25); /* We should never need more than 25*30 = 750 divsteps */
        VERIFY_CHECK(modinv32_mul_cmp_30(&f, len, &modinfo->modulus, -1) > 0); /* f > -modulus */
        VERIFY_CHECK(modinv32_mul_cmp_30(&f, len, &modinfo->modulus, 1) <= 0); /* f <= modulus */
        VERIFY_CHECK(modinv32_mul_cmp_30(&g, len, &modinfo->modulus, -1) > 0); /* g > -modulus */
        VERIFY_CHECK(modinv32_mul_cmp_30(&g, len, &modinfo->modulus, 1) < 0);  /* g <  modulus */
#endif
    }

    /* At this point g is 0 and (if g was not originally 0) f must now equal +/- GCD of
     * the initial f, g values i.e. +/- 1, and d now contains +/- the modular inverse. */
#ifdef VERIFY
    /* g == 0 */
    VERIFY_CHECK(modinv32_mul_cmp_30(&g, len, &SIGNED30_ONE, 0) == 0);
    /* |f| == 1, or (x == 0 and d == 0 and |f|=modulus) */
    VERIFY_CHECK(modinv32_mul_cmp_30(&f, len, &SIGNED30_ONE, -1) == 0 ||
                 modinv32_mul_cmp_30(&f, len, &SIGNED30_ONE, 1) == 0 ||
                 (modinv32_mul_cmp_30(x, 9, &SIGNED30_ONE, 0) == 0 &&
                  modinv32_mul_cmp_30(&d, 9, &SIGNED30_ONE, 0) == 0 &&
                  (modinv32_mul_cmp_30(&f, len, &modinfo->modulus, 1) == 0 ||
                   modinv32_mul_cmp_30(&f, len, &modinfo->modulus, -1) == 0)));
#endif

    /* Optionally negate d, normalize to [0,modulus), and return it. */
    modinv32_normalize_30(&d, f.v[len - 1], modinfo);
    *x = d;
    */
}

#[cfg(test)]
mod modinv32_variable_time_inversion_validation {
    use super::*;

    #[traced_test]
    fn variable_time_inversion_matches_constant_time_and_reference() {
        let moduli: [u64; 12] = [
            3,
            5,
            7,
            11,
            13,
            17,
            19,
            101,
            257,
            65537,
            1_000_000_007,
            (1u64 << 60) - 93,
        ];

        for &modulus in moduli.iter() {
            let modinfo = support::modinfo_from_u64(modulus);
            tracing::info!(modulus, "validating modinv32_var (variable-time)");

            let mut seed: u64 = 0xC3C3_C3C3_C3C3_C3C3u64 ^ modulus;
            let mut inputs: Vec<u64> = Vec::new();

            if modulus <= 257 {
                for x in 0..modulus {
                    inputs.push(x);
                }
            } else {
                inputs.push(0);
                inputs.push(1);
                inputs.push(2);
                inputs.push(3);
                inputs.push(4);
                inputs.push(5);
                inputs.push(modulus - 1);
                inputs.push(modulus - 2);
                inputs.push(modulus / 2);
                inputs.push((modulus / 2) + 1);
                for _ in 0..96 {
                    inputs.push(support::xorshift64_star(&mut seed) % modulus);
                }
            }

            for x in inputs.into_iter() {
                if x != 0 && support::gcd_u64(x, modulus) != 1 {
                    tracing::debug!(modulus, x, "skipping non-invertible residue");
                    continue;
                }

                let mut x_ct = support::signed30_from_u64(x);
                let mut x_vt = support::signed30_from_u64(x);

                modinv32(
                    (&mut x_ct) as *mut ModInv32Signed30,
                    (&modinfo) as *const ModInv32ModInfo,
                );
                modinv32_var(
                    (&mut x_vt) as *mut ModInv32Signed30,
                    (&modinfo) as *const ModInv32ModInfo,
                );

                support::assert_signed30_limbs_are_normalized(&x_ct);
                support::assert_signed30_limbs_are_normalized(&x_vt);

                let inv_ct = support::signed30_to_u128_horner(&x_ct);
                let inv_vt = support::signed30_to_u128_horner(&x_vt);

                assert!(inv_ct == inv_vt);
                assert!(inv_ct < modulus as u128);

                if x == 0 {
                    assert!(inv_ct == 0);
                } else {
                    let expected = support::modinv_u64(x, modulus);
                    assert!(inv_ct == expected as u128);
                    let prod = ((x as u128) * (inv_ct as u128)) % (modulus as u128);
                    assert!(prod == 1u128);
                }
            }
        }
    }
}
