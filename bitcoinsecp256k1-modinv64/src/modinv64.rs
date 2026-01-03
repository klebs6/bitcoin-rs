// ---------------- [ File: bitcoinsecp256k1-modinv64/src/modinv64.rs ]
//! This file implements modular inversion based on the paper "Fast constant-time gcd computation
//! and modular inversion" by Daniel J. Bernstein and Bo-Yin Yang.
//! 
//! For an explanation of the algorithm, see doc/safegcd_implementation.md. This file contains an
//! implementation for N=62, using 62-bit signed limbs represented as int64_t.

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/modinv64.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/modinv64_impl.h]

#[cfg(VERIFY)]
pub const SIGNED62_ONE: ModInv64Signed62 = ModInv64Signed62::from_limbs([1, 0, 0, 0, 0]);

/// Compute the inverse of x modulo modinfo->modulus, and replace x with it (constant time in x).
/// 
/// Same as modinv64_var, but constant time in x (not in the modulus).
/// 
pub fn modinv64(
        x:       *mut ModInv64Signed62,
        modinfo: *const ModInv64ModInfo)  {

    unsafe {
        /* Start with d=0, e=1, f=modulus, g=x, zeta=-1. */
        let mut d = ModInv64Signed62::from_limbs([0, 0, 0, 0, 0]);
        let mut e = ModInv64Signed62::from_limbs([1, 0, 0, 0, 0]);
        let mut f = (*modinfo).modulus().clone();
        let mut g = *x;
        let mut i: i32;
        let mut zeta: i64 = -1; /* zeta = -(delta+1/2); delta starts at 1/2. */

        /* Do 10 iterations of 59 divsteps each = 590 divsteps. This suffices for 256-bit inputs. */
        i = 0;
        while i < 10 {
            /* Compute transition matrix and new zeta after 59 divsteps. */
            let mut t = core::mem::MaybeUninit::<ModInv64Trans2x2>::uninit();
            zeta = modinv64_divsteps_59(zeta, f.v()[0] as u64, g.v()[0] as u64, t.as_mut_ptr());
            /* Update d,e using that transition matrix. */
            modinv64_update_de_62(&mut d as *mut _, &mut e as *mut _, t.as_ptr(), modinfo);
            /* Update f,g using that transition matrix. */
            #[cfg(VERIFY)]
            {
                VERIFY_CHECK!(modinv64_mul_cmp_62(&f as *const _, 5, (*modinfo).modulus() as *const _, -1) > 0); /* f > -modulus */
                VERIFY_CHECK!(modinv64_mul_cmp_62(&f as *const _, 5, (*modinfo).modulus() as *const _, 1) <= 0); /* f <= modulus */
                VERIFY_CHECK!(modinv64_mul_cmp_62(&g as *const _, 5, (*modinfo).modulus() as *const _, -1) > 0); /* g > -modulus */
                VERIFY_CHECK!(modinv64_mul_cmp_62(&g as *const _, 5, (*modinfo).modulus() as *const _, 1) < 0);  /* g <  modulus */
            }
            modinv64_update_fg_62(&mut f as *mut _, &mut g as *mut _, t.as_ptr());
            #[cfg(VERIFY)]
            {
                VERIFY_CHECK!(modinv64_mul_cmp_62(&f as *const _, 5, (*modinfo).modulus() as *const _, -1) > 0); /* f > -modulus */
                VERIFY_CHECK!(modinv64_mul_cmp_62(&f as *const _, 5, (*modinfo).modulus() as *const _, 1) <= 0); /* f <= modulus */
                VERIFY_CHECK!(modinv64_mul_cmp_62(&g as *const _, 5, (*modinfo).modulus() as *const _, -1) > 0); /* g > -modulus */
                VERIFY_CHECK!(modinv64_mul_cmp_62(&g as *const _, 5, (*modinfo).modulus() as *const _, 1) < 0);  /* g <  modulus */
            }
            i += 1;
        }

        /* At this point sufficient iterations have been performed that g must have reached 0
         * and (if g was not originally 0) f must now equal +/- GCD of the initial f, g
         * values i.e. +/- 1, and d now contains +/- the modular inverse. */
        #[cfg(VERIFY)]
        {
            /* g == 0 */
            VERIFY_CHECK!(modinv64_mul_cmp_62(&g as *const _, 5, &SIGNED62_ONE as *const _, 0) == 0);
            /* |f| == 1, or (x == 0 and d == 0 and |f|=modulus) */
            VERIFY_CHECK!(modinv64_mul_cmp_62(&f as *const _, 5, &SIGNED62_ONE as *const _, -1) == 0 ||
                         modinv64_mul_cmp_62(&f as *const _, 5, &SIGNED62_ONE as *const _, 1) == 0 ||
                         (modinv64_mul_cmp_62(x as *const _, 5, &SIGNED62_ONE as *const _, 0) == 0 &&
                          modinv64_mul_cmp_62(&d as *const _, 5, &SIGNED62_ONE as *const _, 0) == 0 &&
                          (modinv64_mul_cmp_62(&f as *const _, 5, (*modinfo).modulus() as *const _, 1) == 0 ||
                           modinv64_mul_cmp_62(&f as *const _, 5, (*modinfo).modulus() as *const _, -1) == 0)));
        }

        /* Optionally negate d, normalize to [0,modulus), and return it. */
        modinv64_normalize_62(&mut d as *mut _, f.v()[4], modinfo);
        *x = d;
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) mod modinv64_mod_info_contract {
    use super::*;

    pub(crate) const LIMB_BITS: u32 = 62;
    pub(crate) const LIMB_MASK_U64: u64 = u64::MAX >> 2;
    pub(crate) const LIMB_MASK_U128: u128 = LIMB_MASK_U64 as u128;
    pub(crate) const LIMB_BASE_U128: u128 = 1u128 << LIMB_BITS;

    pub(crate) fn splitmix64_next(state: &mut u64) -> u64 {
        let mut z = state.wrapping_add(0x9E3779B97F4A7C15);
        *state = z;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    pub(crate) fn splitmix128_next(state: &mut u64) -> u128 {
        let lo = splitmix64_next(state) as u128;
        let hi = splitmix64_next(state) as u128;
        (hi << 64) | lo
    }

    pub(crate) fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }

    pub(crate) fn add_mod_u128(a: u128, b: u128, modulus: u128) -> u128 {
        debug_assert!(modulus != 0);
        let (sum, overflow) = a.overflowing_add(b);
        let mut sum = if overflow { sum.wrapping_sub(modulus) } else { sum };
        if sum >= modulus {
            sum = sum.wrapping_sub(modulus);
        }
        sum
    }

    pub(crate) fn sub_mod_u128(a: u128, b: u128, modulus: u128) -> u128 {
        debug_assert!(modulus != 0);
        if a >= b {
            a - b
        } else {
            modulus - (b - a)
        }
    }

    pub(crate) fn mul_mod_u128(mut a: u128, mut b: u128, modulus: u128) -> u128 {
        debug_assert!(modulus != 0);
        a %= modulus;
        b %= modulus;
        let mut acc: u128 = 0;
        while b != 0 {
            if (b & 1) != 0 {
                acc = add_mod_u128(acc, a, modulus);
            }
            b >>= 1;
            a = add_mod_u128(a, a, modulus);
        }
        acc
    }

    pub(crate) fn modinv_u128(a: u128, modulus: u128) -> u128 {
        debug_assert!(modulus != 0);
        debug_assert!(modulus <= (i128::MAX as u128));
        debug_assert!(a < modulus);

        let mut t: i128 = 0;
        let mut new_t: i128 = 1;
        let mut r: i128 = modulus as i128;
        let mut new_r: i128 = a as i128;

        while new_r != 0 {
            let q: i128 = r / new_r;

            let tmp_t = t - q * new_t;
            t = new_t;
            new_t = tmp_t;

            let tmp_r = r - q * new_r;
            r = new_r;
            new_r = tmp_r;
        }

        debug_assert!(r == 1);
        if t < 0 {
            t += modulus as i128;
        }
        (t as u128) % modulus
    }

    pub(crate) fn modulus_inv62_for_odd_u64(modulus0: u64) -> u64 {
        debug_assert!((modulus0 & 1) == 1);
        debug_assert!(modulus0 <= LIMB_MASK_U64);

        let m: i128 = 1i128 << LIMB_BITS;
        let mut t: i128 = 0;
        let mut new_t: i128 = 1;
        let mut r: i128 = m;
        let mut new_r: i128 = modulus0 as i128;

        while new_r != 0 {
            let q: i128 = r / new_r;

            let tmp_t = t - q * new_t;
            t = new_t;
            new_t = tmp_t;

            let tmp_r = r - q * new_r;
            r = new_r;
            new_r = tmp_r;
        }

        debug_assert!(r == 1);
        if t < 0 {
            t += m;
        }
        (t as u64) & LIMB_MASK_U64
    }

    pub(crate) fn signed62_from_u128(mut value: u128) -> ModInv64Signed62 {
        let mut limbs = [0i64; 5];
        let mut i: usize = 0;
        while i < 5 {
            limbs[i] = (value & LIMB_MASK_U128) as i64;
            value >>= LIMB_BITS;
            i += 1;
        }
        ModInv64Signed62::from_limbs(limbs)
    }

    pub(crate) fn signed62_from_i128(mut value: i128) -> ModInv64Signed62 {
        let base: i128 = 1i128 << LIMB_BITS;
        let mut limbs = [0i64; 5];

        let mut i: usize = 0;
        while i < 4 {
            let rem = value.rem_euclid(base);
            limbs[i] = rem as i64; /* 0..2^62-1 */
            value = (value - rem) / base;
            i += 1;
        }
        limbs[4] = value as i64;

        ModInv64Signed62::from_limbs(limbs)
    }

    pub(crate) fn signed62_is_fully_normalized_nonnegative(value: &ModInv64Signed62) -> bool {
        let v = value.v();
        let mut i: usize = 0;
        while i < 5 {
            if v[i] < 0 {
                return false;
            }
            if (v[i] as u64) > LIMB_MASK_U64 {
                return false;
            }
            i += 1;
        }
        true
    }

    pub(crate) fn signed62_cmp_nonnegative(a: &ModInv64Signed62, b: &ModInv64Signed62) -> Ordering {
        debug_assert!(signed62_is_fully_normalized_nonnegative(a));
        debug_assert!(signed62_is_fully_normalized_nonnegative(b));

        let av = a.v();
        let bv = b.v();

        let mut i: i32 = 4;
        while i >= 0 {
            let ia = i as usize;
            if av[ia] < bv[ia] {
                return Ordering::Less;
            }
            if av[ia] > bv[ia] {
                return Ordering::Greater;
            }
            i -= 1;
        }
        Ordering::Equal
    }

    pub(crate) fn signed62_to_u128_assuming_nonnegative_and_fit(value: &ModInv64Signed62) -> u128 {
        debug_assert!(signed62_is_fully_normalized_nonnegative(value));
        let v = value.v();
        debug_assert!(v[2] == 0);
        debug_assert!(v[3] == 0);
        debug_assert!(v[4] == 0);

        let lo = v[0] as u128;
        let hi = v[1] as u128;
        lo | (hi << LIMB_BITS)
    }

    pub(crate) fn signed62_mod_u128(value: &ModInv64Signed62, modulus: u128) -> u128 {
        debug_assert!(modulus != 0);
        debug_assert!(modulus <= (i128::MAX as u128));

        let base_mod = LIMB_BASE_U128 % modulus;
        let mut pow: u128 = 1;
        let mut acc: u128 = 0;

        let v = value.v();
        let mut i: usize = 0;
        while i < 5 {
            let limb_i: i128 = v[i] as i128;
            let limb_mod: u128 = (limb_i.rem_euclid(modulus as i128)) as u128;
            let term = mul_mod_u128(limb_mod, pow, modulus);
            acc = add_mod_u128(acc, term, modulus);
            pow = mul_mod_u128(pow, base_mod, modulus);
            i += 1;
        }
        acc
    }

    pub(crate) fn build_modinfo_from_u128(modulus: u128) -> ModInv64ModInfo {
        debug_assert!(modulus >= 3);
        debug_assert!((modulus & 1) == 1);
        debug_assert!(modulus <= (i128::MAX as u128));

        let modulus_s62 = signed62_from_u128(modulus);
        let modulus0 = (modulus_s62.v()[0] as u64) & LIMB_MASK_U64;
        debug_assert!((modulus0 & 1) == 1);

        let inv62 = modulus_inv62_for_odd_u64(modulus0);

        ModInv64ModInfo::from_modulus_and_inv62(modulus_s62, inv62)
    }

    pub(crate) fn sample_odd_modulus_up_to_120_bits(seed: &mut u64) -> u128 {
        let mask_120: u128 = (1u128 << 120) - 1;
        let mut m = splitmix128_next(seed) & mask_120;
        m |= 1;
        if m < 3 {
            m = 3;
        }
        debug_assert!((m & 1) == 1);
        m
    }

    pub(crate) fn sample_nonzero_coprime_u128(seed: &mut u64, modulus: u128) -> u128 {
        debug_assert!(modulus >= 3);
        debug_assert!((modulus & 1) == 1);

        let mut attempts: u32 = 0;
        loop {
            let x = splitmix128_next(seed) % modulus;
            if x != 0 && gcd_u128(x, modulus) == 1 {
                return x;
            }
            attempts = attempts.wrapping_add(1);
            debug_assert!(attempts != 0);
        }
    }

    pub(crate) fn assert_signed62_lt_modulus(x: &ModInv64Signed62, modulus: &ModInv64Signed62) {
        debug_assert!(signed62_is_fully_normalized_nonnegative(x));
        debug_assert!(signed62_is_fully_normalized_nonnegative(modulus));
        debug_assert!(signed62_cmp_nonnegative(x, modulus) == Ordering::Less);
    }

    #[traced_test]
    fn modulus_inv62_matches_definition_for_random_odd_values() {
        let mut seed: u64 = 0xA4C6_6D8D_4A1F_4B3D;
        let mut i: usize = 0;

        while i < 512 {
            let mut a = splitmix64_next(&mut seed) & LIMB_MASK_U64;
            a |= 1;

            let inv = modulus_inv62_for_odd_u64(a);
            let prod = a.wrapping_mul(inv) & LIMB_MASK_U64;

            trace!(iter = i, a = a, inv = inv, prod = prod);
            assert!(prod == 1);

            i += 1;
        }
    }

    #[traced_test]
    fn signed62_u128_round_trip_for_random_values_up_to_120_bits() {
        let mut seed: u64 = 0x9B8D_11D1_5C4A_7E33;
        let mask_120: u128 = (1u128 << 120) - 1;

        let mut i: usize = 0;
        while i < 512 {
            let x = splitmix128_next(&mut seed) & mask_120;
            let s = signed62_from_u128(x);

            trace!(iter = i, x = x, limbs = ?s.v());
            assert!(signed62_is_fully_normalized_nonnegative(&s));
            assert!(s.v()[2] == 0 && s.v()[3] == 0 && s.v()[4] == 0);

            let y = signed62_to_u128_assuming_nonnegative_and_fit(&s);
            assert!(x == y);

            i += 1;
        }
    }

    #[traced_test]
    fn build_modinfo_populates_expected_modulus_and_inverse() {
        let mut seed: u64 = 0xD0A1_3B52_118C_7F21;

        let modulus = sample_odd_modulus_up_to_120_bits(&mut seed);
        let modinfo = build_modinfo_from_u128(modulus);

        info!(modulus = modulus, modulus_inv62 = modinfo.modulus_inv62());

        let expected_modulus = signed62_from_u128(modulus);
        assert!(*modinfo.modulus() .v() == *expected_modulus.v());

        let modulus0 = (expected_modulus.v()[0] as u64) & LIMB_MASK_U64;
        let expected_inv62 = modulus_inv62_for_odd_u64(modulus0);
        assert!(modinfo.modulus_inv62() == expected_inv62);

        let prod = modulus0.wrapping_mul(modinfo.modulus_inv62()) & LIMB_MASK_U64;
        assert!(prod == 1);

        debug!(size = mem::size_of::<ModInv64ModInfo>(), align = mem::align_of::<ModInv64ModInfo>());
    }
}
