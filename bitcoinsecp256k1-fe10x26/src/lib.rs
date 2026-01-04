// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/lib.rs ]
#![allow(unused_parens)]
#[macro_use] mod imports; use imports::*;

x!{constants}
x!{fe_add}
x!{fe_clear}
x!{fe_cmov}
x!{fe_cmp_var}
x!{fe_const}
x!{fe_from_signed30}
x!{fe_from_storage}
x!{fe_get_b32}
x!{fe_inv}
x!{fe_inv_var}
x!{fe_is_odd}
x!{fe_is_zero}
x!{fe_mul}
x!{fe_mul_int}
x!{fe_negate}
x!{fe_normalize}
x!{fe_normalize_var}
x!{fe_normalize_weak}
x!{fe_normalizes_to_zero}
x!{fe_normalizes_to_zero_var}
x!{fe_set_b32}
x!{fe_set_int}
x!{fe_sqr}
x!{fe_storage}
x!{fe_storage_cmov}
x!{fe_to_signed30}
x!{fe_to_storage}
x!{fe_verify}
x!{field_10x26}
x!{verify_bits}

#[cfg(test)]
x!{fe10x26_test_support}

#[cfg(test)]
mod crate_root_api_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    #[traced_test]
    fn crate_root_exports_allow_basic_field_roundtrip_smoke() {
        info!("creating field element from bytes and roundtripping via fe_get_b32");
        let mut a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let out = fe_to_be_bytes_normalized(&mut a);
        debug!(?out, "normalized bytes");
        assert_eq!(out, BYTES_PATTERN_A);

        trace!("ensuring fe_set_int produces canonical 1");
        let mut one = fe_from_u32(1);
        let out_one = fe_to_be_bytes_normalized(&mut one);
        assert_eq!(out_one, BYTES_ONE);
    }
}

#[cfg(test)]
mod randomized_reference_consistency_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info, trace, warn};

    struct XorShift64Star {
        state: u64,
    }

    impl XorShift64Star {
        fn new(seed: u64) -> Self {
            Self { state: seed }
        }

        fn next_u64(&mut self) -> u64 {
            let mut x = self.state;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            self.state = x;
            x.wrapping_mul(2685821657736338717u64)
        }

        fn fill_bytes(&mut self, out: &mut [u8]) {
            for chunk in out.chunks_mut(8) {
                let v = self.next_u64().to_be_bytes();
                for (i, b) in chunk.iter_mut().enumerate() {
                    *b = v[i];
                }
            }
        }

        fn next_bytes32(&mut self) -> [u8; 32] {
            let mut out = [0u8; 32];
            self.fill_bytes(&mut out);
            out
        }
    }

    fn sample_fe_bytes_and_value(rng: &mut XorShift64Star) -> ([u8; 32], Fe10x26) {
        loop {
            let bytes = rng.next_bytes32();
            let (fe, ret) = fe_from_be_bytes_ret(&bytes);
            if ret != 0 {
                return (bytes, fe);
            }
            warn!(?bytes, ret, "fe_set_b32 rejected sampled bytes; resampling");
        }
    }

    #[traced_test]
    fn randomized_arithmetic_matches_reference_mod_p_for_many_samples() {
        info!("checking add/mul/sqr/neg vs reference reduction over many randomized samples");
        let mut rng = XorShift64Star::new(0xC0FFE_BADC0DE_D00Du64);

        let samples: usize = 64;
        for i in 0..samples {
            let (a_bytes, a) = sample_fe_bytes_and_value(&mut rng);
            let (b_bytes, b) = sample_fe_bytes_and_value(&mut rng);

            let a_words = words_le_from_be_bytes(&a_bytes);
            let b_words = words_le_from_be_bytes(&b_bytes);

            let expected_add = add_mod_p(&a_words, &b_words);
            let mut got_add_fe = fe_clone_value(&a);
            fe_add_in_place(&mut got_add_fe, &b);
            let got_add = fe_to_words_le_normalized(&mut got_add_fe);

            let expected_mul = mul_mod_p(&a_words, &b_words);
            let got_mul = fe_mul_to_words_le_normalized(&a, &b);

            let expected_sqr = sqr_mod_p(&a_words);
            let got_sqr = fe_sqr_to_words_le_normalized(&a);

            let expected_neg = neg_mod_p(&a_words);
            let got_neg = fe_negate_to_words_le_normalized(&a, 1);

            trace!(
                sample_index = i,
                a0 = a_words[0],
                b0 = b_words[0],
                "sampled inputs"
            );

            assert_eq!(got_add, expected_add);
            assert_eq!(got_mul, expected_mul);
            assert_eq!(got_sqr, expected_sqr);
            assert_eq!(got_neg, expected_neg);

            if i == 0 {
                debug!(
                    ?a_bytes,
                    ?b_bytes,
                    got_add0 = got_add[0],
                    got_mul0 = got_mul[0],
                    got_sqr0 = got_sqr[0],
                    got_neg0 = got_neg[0],
                    "first randomized check snapshot"
                );
            }
        }
    }

    #[traced_test]
    fn randomized_signed30_and_storage_roundtrip_hold_for_many_samples() {
        info!("checking fe_to_storage/fe_from_storage and fe_to_signed30/fe_from_signed30 roundtrips over many randomized samples");
        let mut rng = XorShift64Star::new(0x1234_5678_9ABC_DEF0u64);

        let samples: usize = 64;
        for i in 0..samples {
            let (bytes, mut a) = sample_fe_bytes_and_value(&mut rng);
            fe_normalize_in_place(&mut a);

            let mut stor = Fe10x26Storage { n: [0u32; 8] };
            unsafe { fe_to_storage(&mut stor as *mut Fe10x26Storage, &a as *const Fe10x26) };

            let mut from_stor = Fe10x26::new();
            unsafe { fe_from_storage(&mut from_stor as *mut Fe10x26, &stor as *const Fe10x26Storage) };
            let out_stor = fe_to_be_bytes_normalized(&mut from_stor);

            let mut s: ModInv32Signed30 = unsafe { core::mem::zeroed() };
            unsafe { fe_to_signed30(&mut s as *mut ModInv32Signed30, &a as *const Fe10x26) };

            let mut from_s30 = Fe10x26::new();
            unsafe { fe_from_signed30(&mut from_s30 as *mut Fe10x26, &s as *const ModInv32Signed30) };
            let out_s30 = fe_to_be_bytes_normalized(&mut from_s30);

            trace!(sample_index = i, "roundtrip sample");
            assert_eq!(out_stor, bytes);
            assert_eq!(out_s30, bytes);
        }
    }
}

#[cfg(test)]
mod randomized_algebraic_identity_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info, trace, warn};

    struct XorShift64Star {
        state: u64,
    }

    impl XorShift64Star {
        fn new(seed: u64) -> Self {
            Self { state: seed }
        }

        fn next_u64(&mut self) -> u64 {
            let mut x = self.state;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            self.state = x;
            x.wrapping_mul(2685821657736338717u64)
        }

        fn fill_bytes(&mut self, out: &mut [u8]) {
            for chunk in out.chunks_mut(8) {
                let v = self.next_u64().to_be_bytes();
                for (i, b) in chunk.iter_mut().enumerate() {
                    *b = v[i];
                }
            }
        }

        fn next_bytes32(&mut self) -> [u8; 32] {
            let mut out = [0u8; 32];
            self.fill_bytes(&mut out);
            out
        }
    }

    fn sample_fe(rng: &mut XorShift64Star) -> Fe10x26 {
        loop {
            let bytes = rng.next_bytes32();
            let (fe, ret) = fe_from_be_bytes_ret(&bytes);
            if ret != 0 {
                return fe;
            }
            warn!(?bytes, ret, "fe_set_b32 rejected sampled bytes; resampling");
        }
    }

    fn norm_bytes(x: &Fe10x26) -> [u8; 32] {
        let mut t = fe_clone_value(x);
        fe_to_be_bytes_normalized(&mut t)
    }

    fn is_zero_value(x: &Fe10x26) -> bool {
        let z = unsafe { fe_normalizes_to_zero(x as *const Fe10x26) };
        z == 1
    }

    #[traced_test]
    fn randomized_group_ring_identities_hold_for_many_samples() {
        info!("checking associativity/commutativity/distributivity, square-vs-mul, negate, and inversion identities over randomized samples");
        let mut rng = XorShift64Star::new(0xDEAD_BEEF_CAFE_BABEu64);

        let samples: usize = 24;
        for i in 0..samples {
            let a = sample_fe(&mut rng);
            let b = sample_fe(&mut rng);
            let c = sample_fe(&mut rng);

            trace!(sample_index = i, "identity sample");

            // (a + b) + c == a + (b + c)
            let mut left_add = fe_clone_value(&a);
            fe_add_in_place(&mut left_add, &b);
            fe_add_in_place(&mut left_add, &c);
            let left_add_bytes = fe_to_be_bytes_normalized(&mut left_add);

            let mut bc = fe_clone_value(&b);
            fe_add_in_place(&mut bc, &c);
            let mut right_add = fe_clone_value(&a);
            fe_add_in_place(&mut right_add, &bc);
            let right_add_bytes = fe_to_be_bytes_normalized(&mut right_add);

            assert_eq!(left_add_bytes, right_add_bytes);

            // a + b == b + a
            let mut ab_add = fe_clone_value(&a);
            fe_add_in_place(&mut ab_add, &b);
            let mut ba_add = fe_clone_value(&b);
            fe_add_in_place(&mut ba_add, &a);
            assert_eq!(fe_to_be_bytes_normalized(&mut ab_add), fe_to_be_bytes_normalized(&mut ba_add));

            // (a * b) * c == a * (b * c)
            let mut ab = Fe10x26::new();
            let mut b_mut = fe_clone_value(&b);
            unsafe { fe_mul(&mut ab as *mut Fe10x26, &a as *const Fe10x26, &mut b_mut as *mut Fe10x26) };

            let mut left_mul = Fe10x26::new();
            let mut c_mut = fe_clone_value(&c);
            unsafe { fe_mul(&mut left_mul as *mut Fe10x26, &ab as *const Fe10x26, &mut c_mut as *mut Fe10x26) };
            let left_mul_bytes = fe_to_be_bytes_normalized(&mut left_mul);

            let mut bc_mul = Fe10x26::new();
            let mut c_mut2 = fe_clone_value(&c);
            let mut b_mut2 = fe_clone_value(&b);
            unsafe { fe_mul(&mut bc_mul as *mut Fe10x26, &b_mut2 as *const Fe10x26, &mut c_mut2 as *mut Fe10x26) };

            let mut right_mul = Fe10x26::new();
            let mut bc_mut = fe_clone_value(&bc_mul);
            unsafe { fe_mul(&mut right_mul as *mut Fe10x26, &a as *const Fe10x26, &mut bc_mut as *mut Fe10x26) };
            let right_mul_bytes = fe_to_be_bytes_normalized(&mut right_mul);

            assert_eq!(left_mul_bytes, right_mul_bytes);

            // a * b == b * a
            let ab_words = fe_mul_to_words_le_normalized(&a, &b);
            let ba_words = fe_mul_to_words_le_normalized(&b, &a);
            assert_eq!(ab_words, ba_words);

            // a * (b + c) == a*b + a*c
            let mut b_plus_c = fe_clone_value(&b);
            fe_add_in_place(&mut b_plus_c, &c);

            let mut left_dist = Fe10x26::new();
            let mut bpc_mut = fe_clone_value(&b_plus_c);
            unsafe { fe_mul(&mut left_dist as *mut Fe10x26, &a as *const Fe10x26, &mut bpc_mut as *mut Fe10x26) };
            let left_dist_bytes = fe_to_be_bytes_normalized(&mut left_dist);

            let mut ab2 = Fe10x26::new();
            let mut b_mut3 = fe_clone_value(&b);
            unsafe { fe_mul(&mut ab2 as *mut Fe10x26, &a as *const Fe10x26, &mut b_mut3 as *mut Fe10x26) };

            let mut ac2 = Fe10x26::new();
            let mut c_mut3 = fe_clone_value(&c);
            unsafe { fe_mul(&mut ac2 as *mut Fe10x26, &a as *const Fe10x26, &mut c_mut3 as *mut Fe10x26) };

            fe_add_in_place(&mut ab2, &ac2);
            let right_dist_bytes = fe_to_be_bytes_normalized(&mut ab2);

            assert_eq!(left_dist_bytes, right_dist_bytes);

            // sqr(a) == a*a
            let sqr_words = fe_sqr_to_words_le_normalized(&a);
            let mul_words = fe_mul_to_words_le_normalized(&a, &a);
            assert_eq!(sqr_words, mul_words);

            // a + (-a) == 0
            let mut neg = Fe10x26::new();
            unsafe { fe_negate(&mut neg as *mut Fe10x26, &a as *const Fe10x26, 1) };
            fe_add_in_place(&mut neg, &a);
            assert_eq!(fe_to_be_bytes_normalized(&mut neg), BYTES_ZERO);

            // a * inv(a) == 1 and inv(inv(a)) == a, for a != 0
            if !is_zero_value(&a) {
                let mut inv = Fe10x26::new();
                unsafe { fe_inv(&mut inv as *mut Fe10x26, &a as *const Fe10x26) };

                let mut prod = Fe10x26::new();
                let mut inv_mut = fe_clone_value(&inv);
                unsafe { fe_mul(&mut prod as *mut Fe10x26, &a as *const Fe10x26, &mut inv_mut as *mut Fe10x26) };
                assert_eq!(fe_to_be_bytes_normalized(&mut prod), BYTES_ONE);

                let mut inv_inv = Fe10x26::new();
                unsafe { fe_inv(&mut inv_inv as *mut Fe10x26, &inv as *const Fe10x26) };

                let a_norm = norm_bytes(&a);
                let inv_inv_norm = fe_to_be_bytes_normalized(&mut inv_inv);

                debug!(
                    sample_index = i,
                    a0 = a_norm[0],
                    invinv0 = inv_inv_norm[0],
                    "inversion roundtrip snapshot"
                );
                assert_eq!(inv_inv_norm, a_norm);
            }
        }
    }
}

#[cfg(all(test, feature = "secp256k1-verify"))]
mod normalized_api_preconditions_enforcement_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use tracing::{debug, info, trace};

    #[traced_test]
    fn normalized_required_functions_panic_on_unnormalized_inputs_and_succeed_after_normalization() {
        tracing::info!("observing normalized-precondition APIs on unnormalized input and validating deterministic behavior after normalization");

        let mut a = fe_from_be_bytes_checked(&BYTES_TWO);
        let one = fe_from_be_bytes_checked(&BYTES_ONE);

        fe_add_in_place(&mut a, &one);

        tracing::debug!(
            magnitude = a.magnitude,
            normalized = a.normalized,
            "constructed potentially unnormalized input"
        );
        assert_eq!(a.normalized, 0);

        let mut out_raw = [0u8; 32];
        let get_b32_res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            fe_get_b32(out_raw.as_mut_ptr(), &a as *const Fe10x26)
        }));
        tracing::debug!(is_err = get_b32_res.is_err(), ?out_raw, "fe_get_b32 on unnormalized input");

        let is_odd_res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { fe_is_odd(&a as *const Fe10x26) }));
        tracing::debug!(is_err = is_odd_res.is_err(), "fe_is_odd on unnormalized input");

        let is_zero_res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { fe_is_zero(&a as *const Fe10x26) }));
        tracing::debug!(is_err = is_zero_res.is_err(), "fe_is_zero on unnormalized input");

        let cmp_res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { fe_cmp_var(&a as *const Fe10x26, &a as *const Fe10x26) }));
        tracing::debug!(is_err = cmp_res.is_err(), "fe_cmp_var on unnormalized input");

        let to_signed30_res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut s: ModInv32Signed30 = unsafe { core::mem::zeroed() };
            unsafe { fe_to_signed30(&mut s as *mut ModInv32Signed30, &a as *const Fe10x26) };
        }));
        tracing::debug!(is_err = to_signed30_res.is_err(), "fe_to_signed30 on unnormalized input");

        let to_storage_res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stor = Fe10x26Storage { n: [0u32; 8] };
            unsafe { fe_to_storage(&mut stor as *mut Fe10x26Storage, &a as *const Fe10x26) };
        }));
        tracing::debug!(is_err = to_storage_res.is_err(), "fe_to_storage on unnormalized input");

        tracing::trace!("after normalization, calls should succeed and produce canonical results");

        fe_normalize_in_place(&mut a);
        assert_eq!(a.normalized, 1);

        let mut out = [0u8; 32];
        unsafe { fe_get_b32(out.as_mut_ptr(), &a as *const Fe10x26) };
        assert_eq!(out, BYTES_THREE);

        let odd = unsafe { fe_is_odd(&a as *const Fe10x26) };
        assert_eq!(odd, 1);

        let z = unsafe { fe_is_zero(&a as *const Fe10x26) };
        assert_eq!(z, 0);

        let cmp = unsafe { fe_cmp_var(&a as *const Fe10x26, &a as *const Fe10x26) };
        assert_eq!(cmp, 0);

        let mut s: ModInv32Signed30 = unsafe { core::mem::zeroed() };
        unsafe { fe_to_signed30(&mut s as *mut ModInv32Signed30, &a as *const Fe10x26) };

        let mut back = Fe10x26::new();
        unsafe { fe_from_signed30(&mut back as *mut Fe10x26, &s as *const ModInv32Signed30) };
        fe_normalize_in_place(&mut back);

        let mut out_back = [0u8; 32];
        unsafe { fe_get_b32(out_back.as_mut_ptr(), &back as *const Fe10x26) };
        assert_eq!(out_back, BYTES_THREE);

        let mut stor = Fe10x26Storage { n: [0u32; 8] };
        unsafe { fe_to_storage(&mut stor as *mut Fe10x26Storage, &a as *const Fe10x26) };

        let mut from_stor = Fe10x26::new();
        unsafe { fe_from_storage(&mut from_stor as *mut Fe10x26, &stor as *const Fe10x26Storage) };
        fe_normalize_in_place(&mut from_stor);

        let mut out_from_stor = [0u8; 32];
        unsafe { fe_get_b32(out_from_stor.as_mut_ptr(), &from_stor as *const Fe10x26) };
        assert_eq!(out_from_stor, BYTES_THREE);
    }
}
