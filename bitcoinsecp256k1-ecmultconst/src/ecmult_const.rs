// ---------------- [ File: bitcoinsecp256k1-ecmultconst/src/ecmult_const.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult_const.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult_const_impl.h]

/**
  | Multiply: R = q*A (in constant-time)
  |
  | Here `bits` should be set to the maximum
  | bitlength of the _absolute value_ of
  | `q`, plus one because we internally
  | sometimes add 2 to the number during
  | the WNAF conversion.
  |
  */
pub fn ecmult_const(r: *mut Gej, a: *const Ge, scalar: *const Scalar, size: i32) {
    unsafe {
        let mut pre_a: [Ge; ECMULT_TABLE_SIZE!(WINDOW_A)] =
            [core::mem::zeroed(); ECMULT_TABLE_SIZE!(WINDOW_A)];
        let mut tmpa: Ge = core::mem::zeroed();
        let mut Z: Fe = core::mem::zeroed();

        let mut skew_1: i32;
        let mut pre_a_lam: [Ge; ECMULT_TABLE_SIZE!(WINDOW_A)] =
            [core::mem::zeroed(); ECMULT_TABLE_SIZE!(WINDOW_A)];
        let mut wnaf_lam: [i32; 1 + WNAF_SIZE!(WINDOW_A - 1)] =
            [0; 1 + WNAF_SIZE!(WINDOW_A - 1)];
        let mut skew_lam: i32;
        let mut q_1: Scalar = Scalar::new();
        let mut q_lam: Scalar = Scalar::new();
        let mut wnaf_1: [i32; 1 + WNAF_SIZE!(WINDOW_A - 1)] =
            [0; 1 + WNAF_SIZE!(WINDOW_A - 1)];

        let mut i: i32;

        /* build wnaf representation for q. */
        let mut rsize: i32 = size;
        if size > 128 {
            rsize = 128;
            /* split q into q_1 and q_lam (where q = q_1 + q_lam*lambda, and q_1 and q_lam are ~128 bit) */
            scalar_split_lambda(&mut q_1, &mut q_lam, scalar);
            skew_1 = wnaf_const(wnaf_1.as_mut_ptr(), &q_1, WINDOW_A - 1, 128);
            skew_lam = wnaf_const(wnaf_lam.as_mut_ptr(), &q_lam, WINDOW_A - 1, 128);
        } else {
            skew_1 = wnaf_const(wnaf_1.as_mut_ptr(), scalar, WINDOW_A - 1, size);
            skew_lam = 0;
        }

        /* Calculate odd multiples of a.
         * All multiples are brought to the same Z 'denominator', which is stored
         * in Z. Due to secp256k1' isomorphism we can do all operations pretending
         * that the Z coordinate was 1, use affine addition formulae, and correct
         * the Z coordinate of the result once at the end.
         */
        gej_set_ge(r, a);
        ecmult_odd_multiples_table_globalz_windowa(pre_a.as_mut_ptr(), &mut Z, r);
        i = 0;
        while (i as usize) < ECMULT_TABLE_SIZE!(WINDOW_A) {
            fe_normalize_weak(&mut pre_a[i as usize].y);
            i += 1;
        }
        if size > 128 {
            i = 0;
            while (i as usize) < ECMULT_TABLE_SIZE!(WINDOW_A) {
                ge_mul_lambda(&mut pre_a_lam[i as usize], &pre_a[i as usize]);
                i += 1;
            }
        }

        /* first loop iteration (separated out so we can directly set r, rather
         * than having it start at infinity, get doubled several times, then have
         * its new value added to it) */
        i = wnaf_1[WNAF_SIZE_BITS!(rsize, WINDOW_A - 1) as usize];
        verify_check!(i != 0);
        let tmpa_ptr: *mut Ge = core::ptr::addr_of_mut!(tmpa);
        ecmult_const_table_get_ge!(tmpa_ptr, pre_a, i, WINDOW_A);
        gej_set_ge(r, &tmpa);
        if size > 128 {
            i = wnaf_lam[WNAF_SIZE_BITS!(rsize, WINDOW_A - 1) as usize];
            verify_check!(i != 0);
            ecmult_const_table_get_ge!(tmpa_ptr, pre_a_lam, i, WINDOW_A);
            gej_add_ge(r, r, &tmpa);
        }
        /* remaining loop iterations */
        i = (WNAF_SIZE_BITS!(rsize, WINDOW_A - 1) as i32) - 1;
        while i >= 0 {
            let mut n: i32;
            let mut j: i32;

            j = 0;
            while j < WINDOW_A - 1 {
                gej_double(r, r);
                j += 1;
            }

            n = wnaf_1[i as usize];
            ecmult_const_table_get_ge!(tmpa_ptr, pre_a, n, WINDOW_A);
            verify_check!(n != 0);
            gej_add_ge(r, r, &tmpa);
            if size > 128 {
                n = wnaf_lam[i as usize];
                ecmult_const_table_get_ge!(tmpa_ptr, pre_a_lam, n, WINDOW_A);
                verify_check!(n != 0);
                gej_add_ge(r, r, &tmpa);
            }

            i -= 1;
        }

        {
            let z_ptr: *mut Fe = core::ptr::addr_of_mut!((*r).z);
            fe_mul(z_ptr, z_ptr as *const Fe, &Z);
        }

        {
            /* Correct for wNAF skew */
            let mut correction: Ge = core::mem::zeroed();
            core::ptr::copy_nonoverlapping(a, &mut correction as *mut Ge, 1);

            let mut correction_1_stor: core::mem::MaybeUninit<GeStorage> =
                core::mem::MaybeUninit::uninit();
            let mut correction_lam_stor: core::mem::MaybeUninit<GeStorage> =
                core::mem::MaybeUninit::uninit();
            let mut a2_stor: core::mem::MaybeUninit<GeStorage> = core::mem::MaybeUninit::uninit();

            let mut tmpj: Gej = core::mem::zeroed();

            {
                let tmpj_ptr: *mut Gej = core::ptr::addr_of_mut!(tmpj);
                gej_set_ge(tmpj_ptr, &correction);
                gej_double_var(tmpj_ptr, tmpj_ptr as *const Gej, core::ptr::null_mut());
                ge_set_gej(&mut correction as *mut Ge, tmpj_ptr);
            }

            ge_to_storage(correction_1_stor.as_mut_ptr(), a);
            if size > 128 {
                ge_to_storage(correction_lam_stor.as_mut_ptr(), a);
            }
            ge_to_storage(a2_stor.as_mut_ptr(), &correction);

            /* For odd numbers this is 2a (so replace it), for even ones a (so no-op) */
            ge_storage_cmov(
                correction_1_stor.as_mut_ptr(),
                a2_stor.as_ptr(),
                (skew_1 == 2) as i32,
            );
            if size > 128 {
                ge_storage_cmov(
                    correction_lam_stor.as_mut_ptr(),
                    a2_stor.as_ptr(),
                    (skew_lam == 2) as i32,
                );
            }

            /* Apply the correction */
            ge_from_storage(&mut correction as *mut Ge, correction_1_stor.as_ptr());
            {
                let correction_ptr: *mut Ge = core::ptr::addr_of_mut!(correction);
                ge_neg(correction_ptr, correction_ptr as *const Ge);
            }
            gej_add_ge(r, r, &correction);

            if size > 128 {
                ge_from_storage(&mut correction as *mut Ge, correction_lam_stor.as_ptr());
                {
                    let correction_ptr: *mut Ge = core::ptr::addr_of_mut!(correction);
                    ge_neg(correction_ptr, correction_ptr as *const Ge);
                    ge_mul_lambda(correction_ptr, correction_ptr as *const Ge);
                }
                gej_add_ge(r, r, &correction);
            }
        }
    }
}

#[cfg(test)]
mod ecmult_const_exhaustive_correctness_suite {
    use super::*;

    fn scalar_from_u32_for_ecmult_vectors(v: u32) -> Scalar {
        let mut s = Scalar::new();
        scalar_set_int(&mut s as *mut Scalar, v);
        s
    }

    fn scalar_from_b32_for_ecmult_vectors(bytes: &[u8; 32]) -> Scalar {
        let mut s = Scalar::new();
        let mut overflow: i32 = 0;
        scalar_set_b32(
            &mut s as *mut Scalar,
            bytes.as_ptr(),
            &mut overflow as *mut i32,
        );
        s
    }

    fn scalar_negated_for_ecmult_vectors(s: &Scalar) -> Scalar {
        let mut neg = Scalar::new();
        scalar_negate(&mut neg as *mut Scalar, s as *const Scalar);
        neg
    }

    fn scalar_bit_length_for_ecmult_vectors(s: &Scalar) -> i32 {
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

    fn scalar_abs_bit_length_for_ecmult_vectors(s: &Scalar) -> i32 {
        let sp: *const Scalar = s as *const Scalar;
        if scalar_is_high(sp) != 0 {
            let mut tmp = Scalar::new();
            scalar_negate(&mut tmp as *mut Scalar, sp);
            scalar_bit_length_for_ecmult_vectors(&tmp)
        } else {
            scalar_bit_length_for_ecmult_vectors(s)
        }
    }

    fn size_requirement_for_ecmult_vectors(s: &Scalar) -> i32 {
        let bl = scalar_abs_bit_length_for_ecmult_vectors(s);
        let size = bl + 1;
        if size > 0 { size } else { 1 }
    }

    fn gej_points_equal_for_ecmult_vectors(a: &Gej, b: &Gej) -> bool {
        let mut neg_b: Gej = unsafe { core::mem::zeroed() };
        gej_neg(&mut neg_b as *mut Gej, b as *const Gej);

        let mut diff: Gej = unsafe { core::mem::zeroed() };
        gej_add_var(
            &mut diff as *mut Gej,
            a as *const Gej,
            &neg_b as *const Gej,
            core::ptr::null_mut(),
        );

        gej_is_infinity(&diff as *const Gej) != 0
    }

    fn ge_from_gej_var_for_ecmult_vectors(a: &Gej) -> Ge {
        let mut out: Ge = unsafe { core::mem::zeroed() };
        let mut tmp: Gej = *a;
        ge_set_gej_var(&mut out as *mut Ge, &mut tmp as *mut Gej);
        out
    }

    fn naive_mul_gej_for_ecmult_vectors(a: &Ge, s: &Scalar) -> Gej {
        let mut r: Gej = unsafe { core::mem::zeroed() };
        gej_set_infinity(&mut r as *mut Gej);

        let sp: *const Scalar = s as *const Scalar;
        let rp: *mut Gej = &mut r as *mut Gej;

        let mut i: i32 = 255;
        while i >= 0 {
            gej_double(rp, rp as *const Gej);
            if scalar_get_bits_var(sp, i as u32, 1) != 0 {
                gej_add_ge(rp, rp as *const Gej, a as *const Ge);
            }
            i -= 1;
        }

        r
    }

    fn find_affine_point_in_correct_subgroup_for_ecmult_vectors() -> Ge {
        let mut x: Fe = unsafe { core::mem::zeroed() };

        let mut candidate: Ge = unsafe { core::mem::zeroed() };
        let mut found: i32 = 0;

        let mut xi: i32 = 1;
        while xi <= 10_000 {
            fe_set_int(&mut x as *mut Fe, xi);

            let mut p: Ge = unsafe { core::mem::zeroed() };
            if ge_set_xo_var(&mut p as *mut Ge, &x as *const Fe, 0) != 0 {
                if ge_is_infinity(&p as *const Ge) == 0
                    && ge_is_valid_var(&p as *const Ge) != 0
                    && ge_is_in_correct_subgroup(&p as *const Ge) != 0
                {
                    candidate = p;
                    found = 1;
                    tracing::debug!(x_int = xi, y_odd = 0);
                    break;
                }
            }

            let mut q: Ge = unsafe { core::mem::zeroed() };
            if ge_set_xo_var(&mut q as *mut Ge, &x as *const Fe, 1) != 0 {
                if ge_is_infinity(&q as *const Ge) == 0
                    && ge_is_valid_var(&q as *const Ge) != 0
                    && ge_is_in_correct_subgroup(&q as *const Ge) != 0
                {
                    candidate = q;
                    found = 1;
                    tracing::debug!(x_int = xi, y_odd = 1);
                    break;
                }
            }

            xi += 1;
        }

        assert!(found != 0);
        candidate
    }

    fn derive_affine_point_by_u32_for_ecmult_vectors(base: &Ge, k: u32) -> Ge {
        let s = scalar_from_u32_for_ecmult_vectors(k);
        let rj = naive_mul_gej_for_ecmult_vectors(base, &s);
        ge_from_gej_var_for_ecmult_vectors(&rj)
    }

    fn next_u64_for_ecmult_vectors(state: &mut u64) -> u64 {
        let mut x = *state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        *state = x;
        x
    }

    fn random_scalar_for_ecmult_vectors(state: &mut u64) -> Scalar {
        let mut b32 = [0u8; 32];
        let a = next_u64_for_ecmult_vectors(state).to_be_bytes();
        let b = next_u64_for_ecmult_vectors(state).to_be_bytes();
        let c = next_u64_for_ecmult_vectors(state).to_be_bytes();
        let d = next_u64_for_ecmult_vectors(state).to_be_bytes();

        b32[0..8].copy_from_slice(&a);
        b32[8..16].copy_from_slice(&b);
        b32[16..24].copy_from_slice(&c);
        b32[24..32].copy_from_slice(&d);

        scalar_from_b32_for_ecmult_vectors(&b32)
    }

    fn scalar_add_for_ecmult_vectors(a: &Scalar, b: &Scalar) -> Scalar {
        let mut out = Scalar::new();
        scalar_add(&mut out as *mut Scalar, a as *const Scalar, b as *const Scalar);
        out
    }

    fn run_single_ecmult_case_for_vectors(a: &Ge, s: &Scalar, size: i32) -> Gej {
        let mut out: Gej = unsafe { core::mem::zeroed() };
        ecmult_const(
            &mut out as *mut Gej,
            a as *const Ge,
            s as *const Scalar,
            size,
        );
        out
    }

    fn assert_ecmult_outputs_match_expected_for_vectors(a: &Ge, s: &Scalar, expected: &Gej, size: i32) {
        let out = run_single_ecmult_case_for_vectors(a, s, size);

        tracing::trace!(size = size, is_inf = gej_is_infinity(&out as *const Gej));

        assert!(gej_points_equal_for_ecmult_vectors(&out, expected));

        if gej_is_infinity(&out as *const Gej) == 0 {
            let out_aff = ge_from_gej_var_for_ecmult_vectors(&out);
            assert!(ge_is_valid_var(&out_aff as *const Ge) != 0);
            assert!(ge_is_in_correct_subgroup(&out_aff as *const Ge) != 0);
        }
    }

    #[traced_test]
    fn ecmult_const_matches_naive_for_edgecase_scalars_on_fixed_point() {
        tracing::info!("starting ecmult_const correctness suite against naive double-and-add");

        let a = find_affine_point_in_correct_subgroup_for_ecmult_vectors();

        let mut a_neg = a;
        ge_neg(&mut a_neg as *mut Ge, &a as *const Ge);

        let a2 = derive_affine_point_by_u32_for_ecmult_vectors(&a, 2);
        let a3 = derive_affine_point_by_u32_for_ecmult_vectors(&a, 3);
        let a7 = derive_affine_point_by_u32_for_ecmult_vectors(&a, 7);

        let points: [Ge; 5] = [a, a_neg, a2, a3, a7];

        let scalar_values: [u32; 31] = [
            0, 1, 2, 3, 4, 5, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255,
            256, 257, 511, 512, 513, 1023, 1024, 1025, 2047, 2048,
        ];

        let size_large: i32 = 256;

        let mut pi: usize = 0;
        while pi < points.len() {
            let p = &points[pi];
            tracing::debug!(point_index = pi as u32);

            let mut vi: usize = 0;
            while vi < scalar_values.len() {
                let v = scalar_values[vi];

                let s = scalar_from_u32_for_ecmult_vectors(v);
                let s_neg = scalar_negated_for_ecmult_vectors(&s);

                let expected = naive_mul_gej_for_ecmult_vectors(p, &s);
                let expected_neg = naive_mul_gej_for_ecmult_vectors(p, &s_neg);

                let size_req = size_requirement_for_ecmult_vectors(&s);
                let size_req_neg = size_requirement_for_ecmult_vectors(&s_neg);

                tracing::trace!(v = v, size_req = size_req, size_req_neg = size_req_neg);

                assert_ecmult_outputs_match_expected_for_vectors(p, &s, &expected, size_req);
                assert_ecmult_outputs_match_expected_for_vectors(p, &s_neg, &expected_neg, size_req_neg);

                assert_ecmult_outputs_match_expected_for_vectors(p, &s, &expected, size_large);
                assert_ecmult_outputs_match_expected_for_vectors(p, &s_neg, &expected_neg, size_large);

                vi += 1;
            }

            pi += 1;
        }
    }

    #[traced_test]
    fn ecmult_const_matches_naive_for_random_scalars_across_multiple_points() {
        tracing::info!("starting ecmult_const randomized stress suite");

        let base = find_affine_point_in_correct_subgroup_for_ecmult_vectors();

        let p2 = derive_affine_point_by_u32_for_ecmult_vectors(&base, 2);
        let p3 = derive_affine_point_by_u32_for_ecmult_vectors(&base, 3);
        let p5 = derive_affine_point_by_u32_for_ecmult_vectors(&base, 5);
        let p11 = derive_affine_point_by_u32_for_ecmult_vectors(&base, 11);

        let points: [Ge; 5] = [base, p2, p3, p5, p11];

        let mut rng: u64 = 0xA0761D6478BD642F_u64;
        let size_large: i32 = 256;

        let mut pi: usize = 0;
        while pi < points.len() {
            let p = &points[pi];
            tracing::debug!(point_index = pi as u32);

            let mut i: u32 = 0;
            while i < 48 {
                let s = random_scalar_for_ecmult_vectors(&mut rng);
                let s_neg = scalar_negated_for_ecmult_vectors(&s);

                let expected = naive_mul_gej_for_ecmult_vectors(p, &s);
                let expected_neg = naive_mul_gej_for_ecmult_vectors(p, &s_neg);

                let size_req = size_requirement_for_ecmult_vectors(&s);
                let size_req_neg = size_requirement_for_ecmult_vectors(&s_neg);

                tracing::trace!(
                    case = i,
                    size_req = size_req,
                    size_req_neg = size_req_neg
                );

                assert_ecmult_outputs_match_expected_for_vectors(p, &s, &expected, size_req);
                assert_ecmult_outputs_match_expected_for_vectors(p, &s_neg, &expected_neg, size_req_neg);

                assert_ecmult_outputs_match_expected_for_vectors(p, &s, &expected, size_large);
                assert_ecmult_outputs_match_expected_for_vectors(p, &s_neg, &expected_neg, size_large);

                i += 1;
            }

            pi += 1;
        }
    }

    #[traced_test]
    fn ecmult_const_respects_additive_homomorphism_for_random_scalar_pairs() {
        tracing::info!("starting ecmult_const additive homomorphism property checks");

        let a = find_affine_point_in_correct_subgroup_for_ecmult_vectors();

        let mut rng: u64 = 0xE7037ED1A0B428DB_u64;
        let size: i32 = 256;

        let mut i: u32 = 0;
        while i < 64 {
            let s1 = random_scalar_for_ecmult_vectors(&mut rng);
            let s2 = random_scalar_for_ecmult_vectors(&mut rng);

            let s12 = scalar_add_for_ecmult_vectors(&s1, &s2);

            let r1 = run_single_ecmult_case_for_vectors(&a, &s1, size);
            let r2 = run_single_ecmult_case_for_vectors(&a, &s2, size);
            let r12 = run_single_ecmult_case_for_vectors(&a, &s12, size);

            let mut rsum: Gej = unsafe { core::mem::zeroed() };
            gej_add_var(
                &mut rsum as *mut Gej,
                &r1 as *const Gej,
                &r2 as *const Gej,
                core::ptr::null_mut(),
            );

            tracing::trace!(case = i);
            assert!(gej_points_equal_for_ecmult_vectors(&rsum, &r12));

            i += 1;
        }
    }
}
