// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/ecmult_gen.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult_gen.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult_gen_impl.h]

/// Multiply with the generator: R = a*G
pub fn ecmult_gen(
    ctx: *const EcMultGenContext,
    r:   *mut Gej,
    gn:  *const Scalar)  {

    unsafe {
        let mut add: Ge = core::mem::zeroed();
        let mut adds: GeStorage = core::mem::zeroed();
        let mut gnb: Scalar = Scalar::new();
        let mut bits: u32;

        core::ptr::copy_nonoverlapping(core::ptr::from_ref((*ctx).initial()), r, 1);

        /* Blind scalar/point multiplication by computing (n-b)G + bG instead of nG. */
        scalar_add(
            core::ptr::addr_of_mut!(gnb),
            gn,
            core::ptr::from_ref((*ctx).blind()),
        );

        for j in 0..ECMULT_GEN_PREC_N {
            bits = scalar_get_bits(
                core::ptr::addr_of!(gnb),
                (j * ECMULT_GEN_PREC_B) as u32,
                ECMULT_GEN_PREC_B as u32,
            );

            for i in 0..ECMULT_GEN_PREC_G {
                /** This uses a conditional move to avoid any secret data in array indexes.
                 *   _Any_ use of secret indexes has been demonstrated to result in timing
                 *   sidechannels, even when the cache-line access patterns are uniform.
                 *  See also:
                 *   "A word of warning", CHES 2013 Rump Session, by Daniel J. Bernstein and Peter Schwabe
                 *    (https://cryptojedi.org/peter/data/chesrump-20130822.pdf) and
                 *   "Cache Attacks and Countermeasures: the Case of AES", RSA 2006,
                 *    by Dag Arne Osvik, Adi Shamir, and Eran Tromer
                 *    (https://www.tau.ac.il/~tromer/papers/cache.pdf)
                 */
                ge_storage_cmov(
                    core::ptr::addr_of_mut!(adds),
                    core::ptr::addr_of!((*(*ctx).prec())[j][i]),
                    ((i as u32) == bits) as i32,
                );
            }

            ge_from_storage(core::ptr::addr_of_mut!(add), core::ptr::addr_of!(adds));
            gej_add_ge(r, r, core::ptr::addr_of!(add));
        }

        bits = 0;
        ge_clear(core::ptr::addr_of_mut!(add));
        scalar_clear(core::ptr::addr_of_mut!(gnb));
    }
}

#[cfg(test)]
mod ecmult_gen_generator_multiplication_suite {
    use super::*;

    fn aligned_prealloc_region_for_ecmult_gen() -> (Vec<u8>, *mut c_void, *mut u8) {
        let align = core::mem::align_of::<EcMultGenContextPrec>().max(16);
        let len = ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE;

        let mut mem: Vec<u8> = vec![0u8; len.saturating_add(align)];
        let base = mem.as_mut_ptr();

        let aligned_usize = (base as usize).wrapping_add(align - 1) & !(align - 1);
        let aligned = aligned_usize as *mut u8;

        tracing::debug!(
            align,
            len,
            base = (base as usize),
            aligned = (aligned as usize),
            "allocated aligned prealloc region"
        );

        (mem, aligned as *mut c_void, aligned)
    }

    unsafe fn build_test_context() -> (EcMultGenContext, Vec<u8>) {
        let mut ctx = EcMultGenContext::new();
        ecmult_gen_context_init(&mut ctx);

        #[cfg(not(USE_ECMULT_STATIC_PRECOMPUTATION))]
        {
            let (mem, mut prealloc_cursor, _aligned) = aligned_prealloc_region_for_ecmult_gen();
            let prealloc_ptr: *mut *mut c_void = core::ptr::addr_of_mut!(prealloc_cursor);

            ecmult_gen_context_build(&mut ctx, prealloc_ptr);

            assert!(ecmult_gen_context_is_built((&ctx) as *const EcMultGenContext) != 0);
            assert!(!ctx.prec().is_null());

            return (ctx, mem);
        }

        #[cfg(USE_ECMULT_STATIC_PRECOMPUTATION)]
        {
            let mut mem: Vec<u8> = Vec::new();
            ecmult_gen_context_build(&mut ctx, core::ptr::null_mut());
            assert!(ecmult_gen_context_is_built((&ctx) as *const EcMultGenContext) != 0);
            assert!(!ctx.prec().is_null());
            return (ctx, mem);
        }
    }

    unsafe fn gej_points_are_equal(a: *const Gej, b: *const Gej) -> bool {
        let mut neg_b: Gej = Gej::new();
        gej_neg(&mut neg_b, b);

        let mut diff: Gej = Gej::new();
        gej_add_var(&mut diff, a, (&neg_b) as *const Gej, core::ptr::null_mut());

        gej_is_infinity((&diff) as *const Gej) != 0
    }

    unsafe fn assert_gej_eq(a: *const Gej, b: *const Gej) {
        assert!(gej_points_are_equal(a, b));
    }

    #[traced_test]
    fn ecmult_gen_maps_zero_to_infinity_and_one_to_generator() {
        unsafe {
            let (ctx, _mem) = build_test_context();

            let mut s0: Scalar = Scalar::new();
            scalar_set_int(&mut s0, 0);

            let mut r0: Gej = Gej::new();
            ecmult_gen((&ctx) as *const EcMultGenContext, &mut r0, (&s0) as *const Scalar);

            tracing::info!("computed 0*G; checking infinity");
            assert!(gej_is_infinity((&r0) as *const Gej) != 0);

            let mut s1: Scalar = Scalar::new();
            scalar_set_int(&mut s1, 1);

            let mut r1: Gej = Gej::new();
            ecmult_gen((&ctx) as *const EcMultGenContext, &mut r1, (&s1) as *const Scalar);

            tracing::info!("computed 1*G; checking equals generator");
            let mut expected_g: Gej = Gej::new();
            gej_set_ge(&mut expected_g, &ge_const_g);

            assert_gej_eq((&r1) as *const Gej, (&expected_g) as *const Gej);
        }
    }

    #[traced_test]
    fn ecmult_gen_matches_small_multiples_two_and_three_via_group_operations() {
        unsafe {
            let (ctx, _mem) = build_test_context();

            let mut s1: Scalar = Scalar::new();
            scalar_set_int(&mut s1, 1);
            let mut r1: Gej = Gej::new();
            ecmult_gen((&ctx) as *const EcMultGenContext, &mut r1, (&s1) as *const Scalar);

            let mut s2: Scalar = Scalar::new();
            scalar_set_int(&mut s2, 2);
            let mut r2: Gej = Gej::new();
            ecmult_gen((&ctx) as *const EcMultGenContext, &mut r2, (&s2) as *const Scalar);

            let mut expected2: Gej = Gej::new();
            gej_set_ge(&mut expected2, &ge_const_g);
            gej_double_var(&mut expected2, (&expected2) as *const Gej, core::ptr::null_mut());

            tracing::info!("checking 2*G");
            assert_gej_eq((&r2) as *const Gej, (&expected2) as *const Gej);

            let mut s3: Scalar = Scalar::new();
            scalar_set_int(&mut s3, 3);
            let mut r3: Gej = Gej::new();
            ecmult_gen((&ctx) as *const EcMultGenContext, &mut r3, (&s3) as *const Scalar);

            let mut expected3: Gej = Gej::new();
            core::ptr::copy_nonoverlapping((&expected2) as *const Gej, &mut expected3, 1);
            gej_add_ge(&mut expected3, (&expected3) as *const Gej, &ge_const_g);

            tracing::info!("checking 3*G");
            assert_gej_eq((&r3) as *const Gej, (&expected3) as *const Gej);

            tracing::debug!("checking determinism for scalar=2 within same context");
            let mut r2_again: Gej = Gej::new();
            ecmult_gen((&ctx) as *const EcMultGenContext, &mut r2_again, (&s2) as *const Scalar);
            assert_gej_eq((&r2) as *const Gej, (&r2_again) as *const Gej);

            tracing::debug!("checking that computed points are valid curve points (non-infinity cases)");
            {
                let mut a1: Ge = Ge::new();
                ge_set_gej_var(&mut a1, (&mut r1) as *mut Gej);
                assert!(ge_is_valid_var((&a1) as *const Ge) != 0);
                assert!(ge_is_in_correct_subgroup((&a1) as *const Ge) != 0);

                let mut a2: Ge = Ge::new();
                ge_set_gej_var(&mut a2, (&mut r2) as *mut Gej);
                assert!(ge_is_valid_var((&a2) as *const Ge) != 0);
                assert!(ge_is_in_correct_subgroup((&a2) as *const Ge) != 0);

                let mut a3: Ge = Ge::new();
                ge_set_gej_var(&mut a3, (&mut r3) as *mut Gej);
                assert!(ge_is_valid_var((&a3) as *const Ge) != 0);
                assert!(ge_is_in_correct_subgroup((&a3) as *const Ge) != 0);
            }
        }
    }

    #[traced_test]
    fn ecmult_gen_handles_window_boundary_scalars_around_prec_g() {
        unsafe {
            let (ctx, _mem) = build_test_context();

            let g = ECMULT_GEN_PREC_G as u32;
            let b = ECMULT_GEN_PREC_B as u32;

            tracing::info!(prec_g = g, prec_b = b, "testing scalars around window boundary");

            let mut sg_minus_1: Scalar = Scalar::new();
            scalar_set_int(&mut sg_minus_1, g.wrapping_sub(1));

            let mut sg: Scalar = Scalar::new();
            scalar_set_int(&mut sg, g);

            let mut sg_plus_1: Scalar = Scalar::new();
            scalar_set_int(&mut sg_plus_1, g.wrapping_add(1));

            let mut rg_minus_1: Gej = Gej::new();
            ecmult_gen((&ctx) as *const EcMultGenContext, &mut rg_minus_1, (&sg_minus_1) as *const Scalar);

            let mut rg: Gej = Gej::new();
            ecmult_gen((&ctx) as *const EcMultGenContext, &mut rg, (&sg) as *const Scalar);

            let mut rg_plus_1: Gej = Gej::new();
            ecmult_gen((&ctx) as *const EcMultGenContext, &mut rg_plus_1, (&sg_plus_1) as *const Scalar);

            let mut expected_pow2b: Gej = Gej::new();
            gej_set_ge(&mut expected_pow2b, &ge_const_g);
            for _ in 0..ECMULT_GEN_PREC_B {
                gej_double_var(&mut expected_pow2b, (&expected_pow2b) as *const Gej, core::ptr::null_mut());
            }

            tracing::debug!("checking (2^B)*G == PREC_G*G");
            assert_gej_eq((&rg) as *const Gej, (&expected_pow2b) as *const Gej);

            let mut expected_pow2b_minus_1: Gej = Gej::new();
            core::ptr::copy_nonoverlapping((&expected_pow2b) as *const Gej, &mut expected_pow2b_minus_1, 1);
            {
                let mut neg_g: Gej = Gej::new();
                gej_set_ge(&mut neg_g, &ge_const_g);
                gej_neg(&mut neg_g, (&neg_g) as *const Gej);
                gej_add_var(
                    &mut expected_pow2b_minus_1,
                    (&expected_pow2b_minus_1) as *const Gej,
                    (&neg_g) as *const Gej,
                    core::ptr::null_mut(),
                );
            }

            tracing::debug!("checking (2^B - 1)*G");
            assert_gej_eq((&rg_minus_1) as *const Gej, (&expected_pow2b_minus_1) as *const Gej);

            let mut expected_pow2b_plus_1: Gej = Gej::new();
            core::ptr::copy_nonoverlapping((&expected_pow2b) as *const Gej, &mut expected_pow2b_plus_1, 1);
            gej_add_ge(&mut expected_pow2b_plus_1, (&expected_pow2b_plus_1) as *const Gej, &ge_const_g);

            tracing::debug!("checking (2^B + 1)*G");
            assert_gej_eq((&rg_plus_1) as *const Gej, (&expected_pow2b_plus_1) as *const Gej);
        }
    }

    #[traced_test]
    fn ecmult_gen_obeys_additive_homomorphism_for_representative_scalars() {
        unsafe {
            let (ctx, _mem) = build_test_context();

            let candidates: [u32; 6] = [
                0,
                1,
                2,
                3,
                (ECMULT_GEN_PREC_G as u32).wrapping_sub(1),
                ECMULT_GEN_PREC_G as u32,
            ];

            for &a_u32 in candidates.iter() {
                for &b_u32 in candidates.iter() {
                    let mut a: Scalar = Scalar::new();
                    scalar_set_int(&mut a, a_u32);

                    let mut b: Scalar = Scalar::new();
                    scalar_set_int(&mut b, b_u32);

                    let mut a_plus_b: Scalar = Scalar::new();
                    scalar_add(&mut a_plus_b, (&a) as *const Scalar, (&b) as *const Scalar);

                    let mut r_a: Gej = Gej::new();
                    ecmult_gen((&ctx) as *const EcMultGenContext, &mut r_a, (&a) as *const Scalar);

                    let mut r_b: Gej = Gej::new();
                    ecmult_gen((&ctx) as *const EcMultGenContext, &mut r_b, (&b) as *const Scalar);

                    let mut r_sum_expected: Gej = Gej::new();
                    gej_add_var(
                        &mut r_sum_expected,
                        (&r_a) as *const Gej,
                        (&r_b) as *const Gej,
                        core::ptr::null_mut(),
                    );

                    let mut r_a_plus_b: Gej = Gej::new();
                    ecmult_gen((&ctx) as *const EcMultGenContext, &mut r_a_plus_b, (&a_plus_b) as *const Scalar);

                    tracing::debug!(a = a_u32, b = b_u32, "checking homomorphism: (a+b)G == aG + bG");
                    assert_gej_eq((&r_a_plus_b) as *const Gej, (&r_sum_expected) as *const Gej);
                }
            }
        }
    }
}
