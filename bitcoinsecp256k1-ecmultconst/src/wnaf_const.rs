// ---------------- [ File: bitcoinsecp256k1-ecmultconst/src/wnaf_const.rs ]
crate::ix!();

/// Convert a number to WNAF notation.
/// 
///  The number becomes represented by sum(2^{wi} * wnaf[i], i=0..WNAF_SIZE(w)+1) - return_val.
/// 
///  It has the following guarantees:
/// 
///  - each wnaf[i] an odd integer between -(1 << w) and (1 << w)
/// 
///  - each wnaf[i] is nonzero
/// 
///  - the number of words set is always WNAF_SIZE(w) + 1
/// 
///  Adapted from `The Width-w NAF Method Provides Small Memory and Fast Elliptic Scalar
///  Multiplications Secure against Side Channel Attacks`, Okeya and Tagaki. M. Joye (Ed.) CT-RSA
///  2003, LNCS 2612, pp. 328-443,
///  2003. Springer-Verlag Berlin Heidelberg 2003
/// 
///  Numbers reference steps of `Algorithm SPA-resistant Width-w NAF with Odd Scalar` on pp. 335
///
pub fn wnaf_const(wnaf: *mut i32, scalar: *const Scalar, w: i32, size: i32) -> i32 {
    let mut global_sign: i32;
    let mut skew: i32 = 0;
    let mut word: i32 = 0;

    /* 1 2 3 */
    let mut u_last: i32;
    let mut u: i32;

    let mut flip: i32;
    let mut bit: i32;
    let mut s: Scalar = Scalar::new();
    let mut not_neg_one: i32;

    verify_check!(w > 0);
    verify_check!(size > 0);

    unsafe {
        /* Note that we cannot handle even numbers by negating them to be odd, as is
         * done in other implementations, since if our scalars were specified to have
         * width < 256 for performance reasons, their negations would have width 256
         * and we'd lose any performance benefit. Instead, we use a technique from
         * Section 4.2 of the Okeya/Tagaki paper, which is to add either 1 (for even)
         * or 2 (for odd) to the number we are encoding, returning a skew value indicating
         * this, and having the caller compensate after doing the multiplication.
         *
         * In fact, we _do_ want to negate numbers to minimize their bit-lengths (and in
         * particular, to ensure that the outputs from the endomorphism-split fit into
         * 128 bits). If we negate, the parity of our number flips, inverting which of
         * {1, 2} we want to add to the scalar when ensuring that it's odd. Further
         * complicating things, -1 interacts badly with `scalar_cadd_bit` and
         * we need to special-case it in this logic. */
        flip = scalar_is_high(scalar);
        /* We add 1 to even numbers, 2 to odd ones, noting that negation flips parity */
        bit = flip ^ ((scalar_is_even(scalar) == 0) as i32);
        /* We check for negative one, since adding 2 to it will cause an overflow */
        scalar_negate(&mut s, scalar);
        not_neg_one = (scalar_is_one(&s) == 0) as i32;
        core::ptr::copy_nonoverlapping(scalar, &mut s as *mut Scalar, 1);
        scalar_cadd_bit(&mut s, bit as u32, not_neg_one);
        /* If we had negative one, flip == 1, s.d[0] == 0, bit == 1, so caller expects
         * that we added two to it and flipped it. In fact for -1 these operations are
         * identical. We only flipped, but since skewing is required (in the sense that
         * the skew must be 1 or 2, never zero) and flipping is not, we need to change
         * our flags to claim that we only skewed. */
        global_sign = scalar_cond_negate(&mut s, flip);
        global_sign *= not_neg_one * 2 - 1;
        skew = 1i32 << (bit as u32);

        /* 4 */
        u_last = scalar_shr_int(&mut s, w);
        loop {
            let mut even: i32;

            /* 4.1 4.4 */
            u = scalar_shr_int(&mut s, w);
            /* 4.2 */
            even = (((u & 1) == 0) as i32);
            /* In contrast to the original algorithm, u_last is always > 0 and
             * therefore we do not need to check its sign. In particular, it's easy
             * to see that u_last is never < 0 because u is never < 0. Moreover,
             * u_last is never = 0 because u is never even after a loop
             * iteration. The same holds analogously for the initial value of
             * u_last (in the first loop iteration). */
            verify_check!(u_last > 0);
            verify_check!((u_last & 1) == 1);
            u += even;
            u_last -= even * (1i32 << (w as u32));

            /* 4.3, adapted for global sign change */
            *wnaf.offset(word as isize) = u_last * global_sign;
            word += 1;

            u_last = u;

            if !(word * w < size) {
                break;
            }
        }
        *wnaf.offset(word as isize) = u * global_sign;

        verify_check!(scalar_is_zero(&s) != 0);
        verify_check!(word == (WNAF_SIZE_BITS!(size, w) as i32));
        skew
    }
}

#[cfg(test)]
mod wnaf_const_exhaustive_vector_suite {
    use super::*;

    fn scalar_from_u32_for_wnaf_vectors(v: u32) -> Scalar {
        let mut s = Scalar::new();
        scalar_set_int(&mut s as *mut Scalar, v);
        s
    }

    fn scalar_from_b32_for_wnaf_vectors(bytes: &[u8; 32]) -> Scalar {
        let mut s = Scalar::new();
        let mut overflow: i32 = 0;
        scalar_set_b32(
            &mut s as *mut Scalar,
            bytes.as_ptr(),
            &mut overflow as *mut i32,
        );
        s
    }

    fn scalar_negated_for_wnaf_vectors(s: &Scalar) -> Scalar {
        let mut neg = Scalar::new();
        scalar_negate(&mut neg as *mut Scalar, s as *const Scalar);
        neg
    }

    fn scalar_bit_length_for_wnaf_vectors(s: &Scalar) -> i32 {
        let sp: *const Scalar = s as *const Scalar;
        let mut i: i32 = 255;
        while i >= 0 {
            if scalar_get_bits_var(sp, i as u32, 1) != 0 {
                return i + 1;
            }
            i -= 1;
        }
        0
    }

    fn scalar_abs_bit_length_for_wnaf_vectors(s: &Scalar) -> i32 {
        let sp: *const Scalar = s as *const Scalar;
        if scalar_is_high(sp) != 0 {
            let mut tmp = Scalar::new();
            scalar_negate(&mut tmp as *mut Scalar, sp);
            scalar_bit_length_for_wnaf_vectors(&tmp)
        } else {
            scalar_bit_length_for_wnaf_vectors(s)
        }
    }

    fn size_requirement_for_wnaf_vectors(s: &Scalar) -> i32 {
        let bl = scalar_abs_bit_length_for_wnaf_vectors(s);
        let size = bl + 1;
        if size > 0 { size } else { 1 }
    }

    fn expected_skew_for_wnaf_vectors(s: &Scalar) -> i32 {
        let sp: *const Scalar = s as *const Scalar;
        let flip: i32 = scalar_is_high(sp);
        let even: i32 = scalar_is_even(sp);
        let bit: i32 = flip ^ ((even == 0) as i32);
        1i32 << (bit as u32)
    }

    fn scalar_from_signed_digit_for_wnaf_vectors(d: i32) -> Scalar {
        let mut out = Scalar::new();
        let abs: u32 = if d < 0 { (-d) as u32 } else { d as u32 };
        scalar_set_int(&mut out as *mut Scalar, abs);
        if d < 0 {
            let mut neg = Scalar::new();
            scalar_negate(&mut neg as *mut Scalar, &out as *const Scalar);
            out = neg;
        }
        out
    }

    fn reconstruct_scalar_from_wnaf_vectors(wnaf: &[i32], w: i32, skew: i32) -> Scalar {
        let mut acc = Scalar::new();
        scalar_clear(&mut acc as *mut Scalar);

        for &digit in wnaf.iter().rev() {
            let mut j: i32 = 0;
            while j < w {
                let accp: *mut Scalar = &mut acc as *mut Scalar;
                scalar_add(accp, accp as *const Scalar, accp as *const Scalar);
                j += 1;
            }

            let term = scalar_from_signed_digit_for_wnaf_vectors(digit);
            let accp: *mut Scalar = &mut acc as *mut Scalar;
            scalar_add(accp, accp as *const Scalar, &term as *const Scalar);
        }

        let mut skew_s = Scalar::new();
        scalar_set_int(&mut skew_s as *mut Scalar, skew as u32);

        let mut neg_skew = Scalar::new();
        scalar_negate(&mut neg_skew as *mut Scalar, &skew_s as *const Scalar);

        let accp: *mut Scalar = &mut acc as *mut Scalar;
        scalar_add(accp, accp as *const Scalar, &neg_skew as *const Scalar);

        acc
    }

    fn run_single_wnaf_vector_case(s: &Scalar, w: i32, size: i32) {
        let words: usize = WNAF_SIZE_BITS!(size, w) as usize;
        const SENTINEL: i32 = 0x7F7F7F7F_i32;

        let mut wnaf_buf: [i32; 257] = [SENTINEL; 257];

        let skew = wnaf_const(
            wnaf_buf.as_mut_ptr(),
            s as *const Scalar,
            w,
            size,
        );

        tracing::trace!(w = w, size = size, words = words as u32, skew = skew);

        let expected_skew = expected_skew_for_wnaf_vectors(s);
        assert!(skew == expected_skew);
        assert!(skew == 1 || skew == 2);

        let bound: i64 = 1i64 << (w as u32);

        let mut i: usize = 0;
        while i <= words {
            let d = wnaf_buf[i];
            assert!(d != 0);
            assert!((d & 1) != 0);
            let abs: i64 = if d < 0 { -(d as i64) } else { d as i64 };
            assert!(abs <= bound);
            i += 1;
        }

        let mut k: usize = words + 1;
        while k < wnaf_buf.len() {
            assert!(wnaf_buf[k] == SENTINEL);
            k += 1;
        }

        let reconstructed = reconstruct_scalar_from_wnaf_vectors(&wnaf_buf[..=words], w, skew);
        assert!(scalar_eq(&reconstructed as *const Scalar, s as *const Scalar) != 0);
    }

    fn next_u64_for_wnaf_vectors(state: &mut u64) -> u64 {
        let mut x = *state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        *state = x;
        x
    }

    fn random_scalar_for_wnaf_vectors(state: &mut u64) -> Scalar {
        let mut b32 = [0u8; 32];
        let a = next_u64_for_wnaf_vectors(state).to_be_bytes();
        let b = next_u64_for_wnaf_vectors(state).to_be_bytes();
        let c = next_u64_for_wnaf_vectors(state).to_be_bytes();
        let d = next_u64_for_wnaf_vectors(state).to_be_bytes();

        b32[0..8].copy_from_slice(&a);
        b32[8..16].copy_from_slice(&b);
        b32[16..24].copy_from_slice(&c);
        b32[24..32].copy_from_slice(&d);

        scalar_from_b32_for_wnaf_vectors(&b32)
    }

    #[traced_test]
    fn wnaf_const_roundtrips_and_invariants_hold_for_window_a_minus_one() {
        tracing::info!("starting wnaf_const exhaustive vectors for WINDOW_A - 1");

        let w: i32 = WINDOW_A - 1;
        assert!(w > 0);

        let size_candidates: [i32; 22] = [
            1, 2, 3, 4, 5, 6, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256,
        ];

        let mut v: u32 = 0;
        while v <= 512 {
            let s = scalar_from_u32_for_wnaf_vectors(v);
            let s_neg = scalar_negated_for_wnaf_vectors(&s);

            let req = size_requirement_for_wnaf_vectors(&s);
            tracing::debug!(v = v, req_size = req);

            let mut si: usize = 0;
            while si < size_candidates.len() {
                let size = size_candidates[si];
                if size >= req {
                    run_single_wnaf_vector_case(&s, w, size);
                    run_single_wnaf_vector_case(&s_neg, w, size);
                }
                si += 1;
            }

            v += 1;
        }

        let mut rng: u64 = 0xD1B54A32D192ED03_u64;
        let mut j: u32 = 0;
        while j < 128 {
            let s = random_scalar_for_wnaf_vectors(&mut rng);
            let s_neg = scalar_negated_for_wnaf_vectors(&s);

            let req = size_requirement_for_wnaf_vectors(&s);
            tracing::debug!(case = j, req_size = req);

            let mut si: usize = 0;
            while si < size_candidates.len() {
                let size = size_candidates[si];
                if size >= req {
                    run_single_wnaf_vector_case(&s, w, size);
                    run_single_wnaf_vector_case(&s_neg, w, size);
                }
                si += 1;
            }

            j += 1;
        }
    }

    #[traced_test]
    fn wnaf_const_roundtrips_across_varied_window_sizes_and_boundary_scalars() {
        tracing::info!("starting wnaf_const varied-window stress suite");

        let w_values: [i32; 15] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let size_candidates: [i32; 22] = [
            1, 2, 3, 4, 5, 6, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256,
        ];

        let interesting_values: [u32; 31] = [
            0, 1, 2, 3, 4, 5, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256,
            257, 511, 512, 513, 1023, 1024, 1025, 2047, 2048,
        ];

        let mut wi: usize = 0;
        while wi < w_values.len() {
            let w = w_values[wi];
            assert!(w > 0);

            let mut vi: usize = 0;
            while vi < interesting_values.len() {
                let v = interesting_values[vi];
                let s = scalar_from_u32_for_wnaf_vectors(v);
                let s_neg = scalar_negated_for_wnaf_vectors(&s);

                let req = size_requirement_for_wnaf_vectors(&s);
                tracing::trace!(w = w, v = v, req_size = req);

                let mut si: usize = 0;
                while si < size_candidates.len() {
                    let size = size_candidates[si];
                    if size >= req {
                        run_single_wnaf_vector_case(&s, w, size);
                        run_single_wnaf_vector_case(&s_neg, w, size);
                    }
                    si += 1;
                }

                vi += 1;
            }

            let mut rng: u64 = 0x9E3779B97F4A7C15_u64 ^ (w as u64);
            let mut r: u32 = 0;
            while r < 32 {
                let s = random_scalar_for_wnaf_vectors(&mut rng);
                let s_neg = scalar_negated_for_wnaf_vectors(&s);

                let req = size_requirement_for_wnaf_vectors(&s);
                tracing::trace!(w = w, rand_case = r, req_size = req);

                let mut si: usize = 0;
                while si < size_candidates.len() {
                    let size = size_candidates[si];
                    if size >= req {
                        run_single_wnaf_vector_case(&s, w, size);
                        run_single_wnaf_vector_case(&s_neg, w, size);
                    }
                    si += 1;
                }

                r += 1;
            }

            wi += 1;
        }
    }

    #[traced_test]
    fn wnaf_const_handles_high_even_negative_one_representation_case() {
        tracing::info!("starting wnaf_const negative-one special-case coverage");

        let w: i32 = WINDOW_A - 1;
        assert!(w > 0);

        let one = scalar_from_u32_for_wnaf_vectors(1);
        let neg_one = scalar_negated_for_wnaf_vectors(&one);

        let size_candidates: [i32; 6] = [1, 2, 8, 128, 129, 256];

        let mut si: usize = 0;
        while si < size_candidates.len() {
            let size = size_candidates[si];
            if size >= size_requirement_for_wnaf_vectors(&neg_one) {
                tracing::debug!(size = size);
                run_single_wnaf_vector_case(&neg_one, w, size);
            }
            si += 1;
        }

        assert!(expected_skew_for_wnaf_vectors(&neg_one) == 1 || expected_skew_for_wnaf_vectors(&neg_one) == 2);
    }
}
