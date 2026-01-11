// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/ecmult_gen_context_build.rs ]
crate::ix!();

pub fn ecmult_gen_context_build(
    ctx:      *mut EcMultGenContext,
    prealloc: *mut *mut c_void)  {

    unsafe {
        if !(*ctx).prec().is_null() {
            return;
        }
    }

    #[cfg(not(USE_ECMULT_STATIC_PRECOMPUTATION))]
    unsafe {
        let mut prec: [Ge; ECMULT_GEN_PREC_N * ECMULT_GEN_PREC_G] =
            [Ge::new(); ECMULT_GEN_PREC_N * ECMULT_GEN_PREC_G];

        let mut gj: Gej = Gej::new();
        let mut nums_gej: Gej = Gej::new();

        let prealloc_size: usize = ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE;
        let base: *mut c_void = *prealloc;

        (*ctx).set_prec(manual_alloc(prealloc, prealloc_size, base, prealloc_size) as *mut EcMultGenContextPrec);

        /* get the generator */
        gej_set_ge(core::ptr::addr_of_mut!(gj), &ge_const_g);

        /* Construct a group element with no known corresponding scalar (nothing up my sleeve). */
        {
            static NUMS_B32: [u8; 33] = *b"The scalar for this x is unknown\0";
            let mut nums_x: Fe = core::mem::zeroed();
            let mut nums_ge: Ge = Ge::new();
            let mut r: i32;

            r = fe_set_b32(core::ptr::addr_of_mut!(nums_x), NUMS_B32.as_ptr());
            let _ = r;
            verify_check!(r != 0);

            r = ge_set_xo_var(core::ptr::addr_of_mut!(nums_ge), core::ptr::addr_of!(nums_x), 0);
            let _ = r;
            verify_check!(r != 0);

            gej_set_ge(core::ptr::addr_of_mut!(nums_gej), core::ptr::addr_of!(nums_ge));
            /* Add G to make the bits in x uniformly distributed. */
            gej_add_ge_var(
                core::ptr::addr_of_mut!(nums_gej),
                core::ptr::addr_of!(nums_gej),
                &ge_const_g,
                null_mut(),
            );
        }

        /* compute prec. */
        {
            let mut precj: [Gej; ECMULT_GEN_PREC_N * ECMULT_GEN_PREC_G] =
                [Gej::new(); ECMULT_GEN_PREC_N * ECMULT_GEN_PREC_G];

            let mut gbase: Gej = Gej::new();
            let mut numsbase: Gej = Gej::new();

            core::ptr::copy_nonoverlapping(core::ptr::addr_of!(gj), core::ptr::addr_of_mut!(gbase), 1);
            core::ptr::copy_nonoverlapping(core::ptr::addr_of!(nums_gej), core::ptr::addr_of_mut!(numsbase), 1);

            for j in 0..ECMULT_GEN_PREC_N {
                /* Set precj[j*PREC_G .. j*PREC_G+(PREC_G-1)] to (numsbase, numsbase + gbase, ..., numsbase + (PREC_G-1)*gbase). */
                core::ptr::copy_nonoverlapping(
                    core::ptr::addr_of!(numsbase),
                    core::ptr::addr_of_mut!(precj[j * ECMULT_GEN_PREC_G]),
                    1,
                );

                for i in 1..ECMULT_GEN_PREC_G {
                    gej_add_var(
                        core::ptr::addr_of_mut!(precj[j * ECMULT_GEN_PREC_G + i]),
                        core::ptr::addr_of!(precj[j * ECMULT_GEN_PREC_G + i - 1]),
                        core::ptr::addr_of!(gbase),
                        null_mut(),
                    );
                }

                /* Multiply gbase by PREC_G. */
                for _i in 0..ECMULT_GEN_PREC_B {
                    gej_double_var(
                        core::ptr::addr_of_mut!(gbase),
                        core::ptr::addr_of!(gbase),
                        null_mut(),
                    );
                }

                /* Multiply numbase by 2. */
                gej_double_var(
                    core::ptr::addr_of_mut!(numsbase),
                    core::ptr::addr_of!(numsbase),
                    null_mut(),
                );

                if j == ECMULT_GEN_PREC_N - 2 {
                    /* In the last iteration, numsbase is (1 - 2^j) * nums instead. */
                    gej_neg(core::ptr::addr_of_mut!(numsbase), core::ptr::addr_of!(numsbase));
                    gej_add_var(
                        core::ptr::addr_of_mut!(numsbase),
                        core::ptr::addr_of!(numsbase),
                        core::ptr::addr_of!(nums_gej),
                        null_mut(),
                    );
                }
            }

            ge_set_all_gej_var(
                prec.as_mut_ptr(),
                precj.as_ptr(),
                ECMULT_GEN_PREC_N * ECMULT_GEN_PREC_G,
            );
        }

        for j in 0..ECMULT_GEN_PREC_N {
            for i in 0..ECMULT_GEN_PREC_G {
                ge_to_storage(
                    core::ptr::addr_of_mut!((*(*ctx).prec())[j][i]),
                    core::ptr::addr_of!(prec[j * ECMULT_GEN_PREC_G + i]),
                );
            }
        }
    }

    #[cfg(USE_ECMULT_STATIC_PRECOMPUTATION)]
    unsafe {
        let _ = prealloc;
        (*ctx).set_prec((&ecmult_static_context as *const EcMultGenContextPrec) as *mut EcMultGenContextPrec);
    }

    ecmult_gen_blind(ctx, core::ptr::null());

}

#[cfg(test)]
mod ecmult_gen_context_build_behavior_suite {
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

    #[traced_test]
    fn ecmult_gen_context_build_sets_prec_and_marks_built() {
        unsafe {
            let mut ctx = EcMultGenContext::new();
            ecmult_gen_context_init(&mut ctx);

            #[cfg(not(USE_ECMULT_STATIC_PRECOMPUTATION))]
            {
                let (mem, mut prealloc_cursor, prealloc_base_u8) = aligned_prealloc_region_for_ecmult_gen();
                let base_u8: *mut u8 = prealloc_base_u8;

                ecmult_gen_context_build(&mut ctx, core::ptr::addr_of_mut!(prealloc_cursor));

                tracing::info!(
                    built = ecmult_gen_context_is_built((&ctx) as *const EcMultGenContext),
                    prec = (ctx.prec() as usize),
                    "context built"
                );

                assert!(ecmult_gen_context_is_built((&ctx) as *const EcMultGenContext) != 0);
                assert!(!ctx.prec().is_null());

                let expected_end = base_u8.add(ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE);
                let actual_end = prealloc_cursor as *mut u8;

                tracing::debug!(
                    expected_end = (expected_end as usize),
                    actual_end = (actual_end as usize),
                    "checking prealloc cursor advancement"
                );

                assert_eq!(actual_end, expected_end);

                assert!(scalar_is_zero(ctx.blind() as *const Scalar) == 0);
                assert!(gej_is_infinity(ctx.initial() as *const Gej) == 0);

                core::hint::black_box(mem);
            }

            #[cfg(USE_ECMULT_STATIC_PRECOMPUTATION)]
            {
                ecmult_gen_context_build(&mut ctx, core::ptr::null_mut());

                assert!(ecmult_gen_context_is_built((&ctx) as *const EcMultGenContext) != 0);
                assert!(!ctx.prec().is_null());
                assert!(scalar_is_zero(ctx.blind() as *const Scalar) == 0);
                assert!(gej_is_infinity(ctx.initial() as *const Gej) == 0);
            }
        }
    }

    #[traced_test]
    fn ecmult_gen_context_build_is_idempotent_when_prec_is_already_set() {
        unsafe {
            let mut ctx = EcMultGenContext::new();
            ecmult_gen_context_init(&mut ctx);

            #[cfg(not(USE_ECMULT_STATIC_PRECOMPUTATION))]
            {
                let (_mem, mut prealloc_cursor, _base_u8) = aligned_prealloc_region_for_ecmult_gen();
                ecmult_gen_context_build(&mut ctx, core::ptr::addr_of_mut!(prealloc_cursor));

                let first_prec = ctx.prec();

                tracing::debug!(first_prec = (first_prec as usize), "built once");
                assert!(!first_prec.is_null());

                ecmult_gen_context_build(&mut ctx, core::ptr::null_mut());
                let second_prec = ctx.prec();

                tracing::debug!(
                    first_prec = (first_prec as usize),
                    second_prec = (second_prec as usize),
                    "built twice"
                );

                assert_eq!(first_prec, second_prec);
            }

            #[cfg(USE_ECMULT_STATIC_PRECOMPUTATION)]
            {
                ecmult_gen_context_build(&mut ctx, core::ptr::null_mut());
                let first_prec = ctx.prec();
                ecmult_gen_context_build(&mut ctx, core::ptr::null_mut());
                let second_prec = ctx.prec();
                assert_eq!(first_prec, second_prec);
            }
        }
    }
}
