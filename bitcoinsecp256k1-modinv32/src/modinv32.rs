// ---------------- [ File: bitcoinsecp256k1-modinv32/src/modinv32.rs ]
/*!
This file implements modular inversion based on the paper "Fast constant-time gcd computation and
modular inversion" by Daniel J. Bernstein and Bo-Yin Yang.

For an explanation of the algorithm, see doc/safegcd_implementation.md. This file contains an
implementation for N=30, using 30-bit signed limbs represented as int32_t.
*/

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/modinv32.h]

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/modinv32_impl.h]

#[cfg(VERIFY)]
pub const SIGNED30_ONE: ModInv32Signed30 = ModInv32Signed30 {
    v: [1, 0, 0, 0, 0, 0, 0, 0, 0],
};

/// Compute the inverse of x modulo modinfo->modulus, and replace x with it (constant time in x).
/// 
/// Same as secp256k1_modinv32_var, but constant time in x (not in the modulus).
/// 
pub fn modinv32(x: *mut ModInv32Signed30, modinfo: *const ModInv32ModInfo) {
    unsafe {
        /* Start with d=0, e=1, f=modulus, g=x, zeta=-1. */
        let mut d = ModInv32Signed30 { v: [0i32; 9] };
        let mut e = ModInv32Signed30 { v: [0i32; 9] };
        e.v[0] = 1;
        let mut f = std::ptr::read(std::ptr::addr_of!((*modinfo).modulus));
        let mut g = std::ptr::read(x as *const ModInv32Signed30);
        let mut i: i32;
        let mut zeta: i32 = -1; /* zeta = -(delta+1/2); delta is initially 1/2. */

        /* Do 20 iterations of 30 divsteps each = 600 divsteps. 590 suffices for 256-bit inputs. */
        i = 0;
        while i < 20 {
            /* Compute transition matrix and new zeta after 30 divsteps. */
            let mut t = std::mem::MaybeUninit::<ModInv32Trans2x2>::uninit();
            zeta = modinv32_divsteps_30(zeta, f.v[0] as u32, g.v[0] as u32, t.as_mut_ptr());
            let t = t.assume_init();
            /* Update d,e using that transition matrix. */
            modinv32_update_de_30(&mut d, &mut e, &t, modinfo);
            /* Update f,g using that transition matrix. */
            #[cfg(VERIFY)]
            {
                verify_check!(modinv32_mul_cmp_30(&f, 9, &(*modinfo).modulus, -1) > 0); /* f > -modulus */
                verify_check!(modinv32_mul_cmp_30(&f, 9, &(*modinfo).modulus, 1) <= 0); /* f <= modulus */
                verify_check!(modinv32_mul_cmp_30(&g, 9, &(*modinfo).modulus, -1) > 0); /* g > -modulus */
                verify_check!(modinv32_mul_cmp_30(&g, 9, &(*modinfo).modulus, 1) < 0); /* g <  modulus */
            }
            modinv32_update_fg_30(&mut f, &mut g, &t);
            #[cfg(VERIFY)]
            {
                verify_check!(modinv32_mul_cmp_30(&f, 9, &(*modinfo).modulus, -1) > 0); /* f > -modulus */
                verify_check!(modinv32_mul_cmp_30(&f, 9, &(*modinfo).modulus, 1) <= 0); /* f <= modulus */
                verify_check!(modinv32_mul_cmp_30(&g, 9, &(*modinfo).modulus, -1) > 0); /* g > -modulus */
                verify_check!(modinv32_mul_cmp_30(&g, 9, &(*modinfo).modulus, 1) < 0); /* g <  modulus */
            }
            i += 1;
        }

        /* At this point sufficient iterations have been performed that g must have reached 0
         * and (if g was not originally 0) f must now equal +/- GCD of the initial f, g
         * values i.e. +/- 1, and d now contains +/- the modular inverse. */
        #[cfg(VERIFY)]
        {
            /* g == 0 */
            verify_check!(modinv32_mul_cmp_30(&g, 9, &SIGNED30_ONE, 0) == 0);
            /* |f| == 1, or (x == 0 and d == 0 and |f|=modulus) */
            verify_check!(
                modinv32_mul_cmp_30(&f, 9, &SIGNED30_ONE, -1) == 0
                    || modinv32_mul_cmp_30(&f, 9, &SIGNED30_ONE, 1) == 0
                    || (modinv32_mul_cmp_30(x as *const ModInv32Signed30, 9, &SIGNED30_ONE, 0)
                        == 0
                        && modinv32_mul_cmp_30(&d, 9, &SIGNED30_ONE, 0) == 0
                        && (modinv32_mul_cmp_30(&f, 9, &(*modinfo).modulus, 1) == 0
                            || modinv32_mul_cmp_30(&f, 9, &(*modinfo).modulus, -1) == 0))
            );
        }

        /* Optionally negate d, normalize to [0,modulus), and return it. */
        modinv32_normalize_30(&mut d, f.v[8], modinfo);
        std::ptr::write(x, d);
    }
}

#[cfg(test)]
mod modinv32_constant_time_inversion_validation {
    use super::*;

    #[traced_test]
    fn constant_time_inversion_matches_reference_for_small_and_medium_moduli() {
        let moduli: [u64; 16] = [
            3,
            5,
            7,
            11,
            13,
            17,
            19,
            23,
            29,
            31,
            101,
            127,
            257,
            65537,
            1_000_000_007,
            (1u64 << 60) - 93,
        ];

        for &modulus in moduli.iter() {
            let modinfo = support::modinfo_from_u64(modulus);
            tracing::info!(modulus, "validating modinv32 (constant-time)");

            if modulus <= 257 {
                for x in 0..modulus {
                    if x != 0 && support::gcd_u64(x, modulus) != 1 {
                        tracing::debug!(modulus, x, "skipping non-invertible residue");
                        continue;
                    }

                    let mut x_s = support::signed30_from_u64(x);
                    modinv32(
                        (&mut x_s) as *mut ModInv32Signed30,
                        (&modinfo) as *const ModInv32ModInfo,
                    );
                    support::assert_signed30_limbs_are_normalized(&x_s);

                    let inv_u128 = support::signed30_to_u128_horner(&x_s);
                    assert!(inv_u128 < modulus as u128);
                    let inv = inv_u128 as u64;

                    if x == 0 {
                        assert!(inv == 0);
                    } else {
                        let expected = support::modinv_u64(x, modulus);
                        assert!(inv == expected);
                        let prod = ((x as u128) * (inv as u128)) % (modulus as u128);
                        assert!(prod == 1u128);
                    }
                }
            } else {
                let mut seed: u64 = 0xA5A5_A5A5_A5A5_A5A5u64 ^ modulus;
                let mut inputs: Vec<u64> = Vec::new();

                inputs.push(0);
                inputs.push(1);
                inputs.push(2);
                inputs.push(3);
                inputs.push(4);
                inputs.push(5);
                inputs.push(6);
                inputs.push(7);
                inputs.push(8);
                inputs.push(9);
                inputs.push(modulus - 1);
                inputs.push(modulus - 2);
                inputs.push(modulus / 2);
                inputs.push((modulus / 2) + 1);

                for _ in 0..64 {
                    inputs.push(support::xorshift64_star(&mut seed) % modulus);
                }

                for x in inputs.into_iter() {
                    if x != 0 && support::gcd_u64(x, modulus) != 1 {
                        tracing::debug!(modulus, x, "skipping non-invertible residue");
                        continue;
                    }

                    let mut x_s = support::signed30_from_u64(x);
                    modinv32(
                        (&mut x_s) as *mut ModInv32Signed30,
                        (&modinfo) as *const ModInv32ModInfo,
                    );
                    support::assert_signed30_limbs_are_normalized(&x_s);

                    let inv_u128 = support::signed30_to_u128_horner(&x_s);
                    assert!(inv_u128 < modulus as u128);
                    let inv = inv_u128 as u64;

                    if x == 0 {
                        assert!(inv == 0);
                    } else {
                        let expected = support::modinv_u64(x, modulus);
                        assert!(inv == expected);
                        let prod = ((x as u128) * (inv as u128)) % (modulus as u128);
                        assert!(prod == 1u128);
                    }
                }
            }
        }
    }

    #[cfg(VERIFY)]
    #[traced_test]
    fn signed30_one_constant_has_expected_limb_pattern() {
        tracing::info!("validating SIGNED30_ONE limb pattern");
        assert!(SIGNED30_ONE.v == [1, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}
