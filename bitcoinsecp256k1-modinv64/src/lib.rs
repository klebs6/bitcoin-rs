// ---------------- [ File: bitcoinsecp256k1-modinv64/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{modinv64}
x!{modinv64_abs}
x!{modinv64_divsteps_59}
x!{modinv64_divsteps_62_var}
x!{modinv64_mod_info}
x!{modinv64_mul62}
x!{modinv64_mul_cmp_62}
x!{modinv64_normalize_62}
x!{modinv64_signed62}
x!{modinv64_trans2x2}
x!{modinv64_update_de_62}
x!{modinv64_update_fg_62}
x!{modinv64_update_fg_62_var}
x!{modinv64_var}

#[cfg(test)]
pub use crate::modinv64_mod_info_contract::*;

#[cfg(test)]
mod modinv64_constant_time_inversion_contract {
    use super::*;

    #[traced_test]
    fn constant_time_inversion_matches_variable_time_for_random_inputs() {
        let mut seed: u64 = 0xFACE_FEED_1234_5678;

        let mut case_idx: usize = 0;
        while case_idx < 256 {
            let modulus = sample_odd_modulus_up_to_120_bits(&mut seed);
            let modinfo = build_modinfo_from_u128(modulus);

            let mut x_val = splitmix128_next(&mut seed) % modulus;
            if x_val != 0 && gcd_u128(x_val, modulus) != 1 {
                x_val = sample_nonzero_coprime_u128(&mut seed, modulus);
            }

            let mut x_ct = signed62_from_u128(x_val);
            let mut x_vt = signed62_from_u128(x_val);

            trace!(case_idx = case_idx, modulus = modulus, x_in = x_val);

            modinv64(&mut x_ct as *mut _, &modinfo as *const _);
            modinv64_var(&mut x_vt as *mut _, &modinfo as *const _);

            trace!(case_idx = case_idx, x_ct = ?x_ct.v(), x_vt = ?x_vt.v());

            assert!(signed62_is_fully_normalized_nonnegative(&x_ct));
            assert!(signed62_is_fully_normalized_nonnegative(&x_vt));

            assert_signed62_lt_modulus(&x_ct, modinfo.modulus());
            assert_signed62_lt_modulus(&x_vt, modinfo.modulus());

            let inv_ct = signed62_to_u128_assuming_nonnegative_and_fit(&x_ct);
            let inv_vt = signed62_to_u128_assuming_nonnegative_and_fit(&x_vt);
            assert!(inv_ct == inv_vt);

            if x_val == 0 {
                assert!(inv_ct == 0);
            } else {
                let check = mul_mod_u128(x_val, inv_ct, modulus);
                assert!(check == 1);
            }

            case_idx += 1;
        }
    }

    #[traced_test]
    fn constant_time_inversion_zero_maps_to_zero() {
        let modulus: u128 = ((1u128 << 119) + 55) | 1;
        let modinfo = build_modinfo_from_u128(modulus);

        let mut x = signed62_from_u128(0);
        modinv64(&mut x as *mut _, &modinfo as *const _);

        trace!(x_out = ?x.v());
        assert!(signed62_is_fully_normalized_nonnegative(&x));
        assert!(signed62_to_u128_assuming_nonnegative_and_fit(&x) == 0);
    }
}
