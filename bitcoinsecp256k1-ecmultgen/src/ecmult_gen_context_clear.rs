// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/ecmult_gen_context_clear.rs ]
crate::ix!();

pub fn ecmult_gen_context_clear(ctx: *mut EcMultGenContext)  {

    unsafe {
        scalar_clear((*ctx).blind_mut());
        gej_clear((*ctx).initial_mut());
        (*ctx).set_prec(core::ptr::null_mut());
    }
}

#[cfg(test)]
mod ecmult_gen_context_clear_contract_suite {
    use super::*;

    fn aligned_prealloc_region_for_ecmult_gen() -> (Vec<u8>, *mut c_void) {
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

        (mem, aligned as *mut c_void)
    }

    unsafe fn scalar_to_bytes32(s: *const Scalar) -> [u8; 32] {
        let mut out = [0u8; 32];
        scalar_get_b32(out.as_mut_ptr(), s);
        out
    }

    #[traced_test]
    fn ecmult_gen_context_clear_resets_built_flag_and_zeroizes_sensitive_state() {
        unsafe {
            let mut ctx = EcMultGenContext::new();
            ecmult_gen_context_init(&mut ctx);

            #[cfg(not(USE_ECMULT_STATIC_PRECOMPUTATION))]
            let (_mem, mut prealloc_cursor) = aligned_prealloc_region_for_ecmult_gen();

            #[cfg(not(USE_ECMULT_STATIC_PRECOMPUTATION))]
            ecmult_gen_context_build(&mut ctx, core::ptr::addr_of_mut!(prealloc_cursor));

            #[cfg(USE_ECMULT_STATIC_PRECOMPUTATION)]
            ecmult_gen_context_build(&mut ctx, core::ptr::null_mut());

            assert!(ecmult_gen_context_is_built((&ctx) as *const EcMultGenContext) != 0);
            assert!(!ctx.prec().is_null());

            let blind_before = scalar_to_bytes32(ctx.blind() as *const Scalar);
            tracing::debug!(
                blind_before_is_zero = (blind_before == [0u8; 32]),
                "captured blind scalar before clear"
            );
            assert_ne!(blind_before, [0u8; 32]);

            {
                let mut initial_affine_before: Ge = Ge::new();
                ge_set_gej_var(&mut initial_affine_before, ctx.initial_mut() as *mut Gej);

                let initial_valid_before =
                    ge_is_valid_var((&initial_affine_before) as *const Ge);
                let initial_in_subgroup_before =
                    ge_is_in_correct_subgroup((&initial_affine_before) as *const Ge);

                tracing::debug!(
                    initial_valid_before,
                    initial_in_subgroup_before,
                    "initial point validity before clear"
                );

                assert_ne!(initial_valid_before, 0);
                assert_ne!(initial_in_subgroup_before, 0);
            }

            tracing::info!(
                prec = (ctx.prec() as usize),
                "clearing built ecmult-gen context"
            );

            ecmult_gen_context_clear(&mut ctx);

            assert!(ecmult_gen_context_is_built((&ctx) as *const EcMultGenContext) == 0);
            assert!(ctx.prec().is_null());

            let blind_after = scalar_to_bytes32(ctx.blind() as *const Scalar);
            tracing::debug!(
                blind_after_is_zero = (blind_after == [0u8; 32]),
                "captured blind scalar after clear"
            );
            assert_eq!(blind_after, [0u8; 32]);

            {
                let mut initial_affine_after: Ge = Ge::new();
                ge_set_gej_var(&mut initial_affine_after, ctx.initial_mut() as *mut Gej);

                let initial_valid_after = ge_is_valid_var((&initial_affine_after) as *const Ge);

                tracing::debug!(
                    initial_valid_after,
                    "initial point validity after clear (expected cleared/invalid)"
                );

                assert_eq!(initial_valid_after, 0);
            }

            tracing::debug!("calling clear twice should be safe and idempotent on pointers");
            ecmult_gen_context_clear(&mut ctx);
            assert!(ctx.prec().is_null());
            assert!(ecmult_gen_context_is_built((&ctx) as *const EcMultGenContext) == 0);
        }
    }
}
