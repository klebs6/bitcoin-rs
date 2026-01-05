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
        (*ctx).prec = (&ecmult_static_context as *const EcMultGenContextPrec) as *mut EcMultGenContextPrec;
    }

    ecmult_gen_blind(ctx, core::ptr::null());
}
