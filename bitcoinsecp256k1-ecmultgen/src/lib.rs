// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{constants}
x!{ecmult_gen}
x!{ecmult_gen_blind}
x!{ecmult_gen_contex}
x!{ecmult_gen_context_build}
x!{ecmult_gen_context_clear}
x!{ecmult_gen_context_finalize_memcpy}
x!{ecmult_gen_context_init}
x!{ecmult_gen_context_is_built}

#[cfg(test)]
mod imports_visibility_smoke_suite {
    use super::*;

    #[traced_test]
    fn imports_expose_core_secp256k1_types_and_functions() {
        tracing::info!("smoke-checking imports module bindings");

        let mut s: Scalar = Scalar::new();
        unsafe {
            scalar_set_int(&mut s, 1);
            assert!(scalar_is_zero((&s) as *const Scalar) == 0);

            let mut p: Gej = Gej::new();
            gej_set_ge(&mut p, &ge_const_g);
            assert!(gej_is_infinity((&p) as *const Gej) == 0);

            let mut q: Gej = Gej::new();
            gej_double_var(&mut q, (&p) as *const Gej, core::ptr::null_mut());
            assert!(gej_is_infinity((&q) as *const Gej) == 0);
        }
    }
}

#[cfg(test)]
mod ecmultgen_public_api_contract_smoke_suite {
    use super::*;

    fn aligned_prealloc_region_for_ecmult_gen() -> (Vec<u8>, *mut c_void) {
        let align = core::mem::align_of::<EcMultGenContextPrec>().max(16);
        let len = ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE;
        let mut mem: Vec<u8> = vec![0u8; len.saturating_add(align)];
        let base = mem.as_mut_ptr();
        let aligned_usize = (base as usize).wrapping_add(align - 1) & !(align - 1);
        let aligned = aligned_usize as *mut u8;
        (mem, aligned as *mut c_void)
    }

    #[traced_test]
    fn ecmultgen_public_functions_build_blind_and_multiply_end_to_end() {
        unsafe {
            let mut ctx = EcMultGenContext::new();
            ecmult_gen_context_init(&mut ctx);

            #[cfg(not(USE_ECMULT_STATIC_PRECOMPUTATION))]
            {
                let (_mem, mut prealloc_cursor) = aligned_prealloc_region_for_ecmult_gen();
                ecmult_gen_context_build(&mut ctx, core::ptr::addr_of_mut!(prealloc_cursor));
            }

            #[cfg(USE_ECMULT_STATIC_PRECOMPUTATION)]
            {
                ecmult_gen_context_build(&mut ctx, core::ptr::null_mut());
            }

            assert!(ecmult_gen_context_is_built((&ctx) as *const EcMultGenContext) != 0);

            let seed: [u8; 32] = [0x99u8; 32];
            ecmult_gen_blind(&mut ctx, seed.as_ptr());

            let mut one: Scalar = Scalar::new();
            scalar_set_int(&mut one, 1);

            let mut r: Gej = Gej::new();
            ecmult_gen((&ctx) as *const EcMultGenContext, &mut r, (&one) as *const Scalar);

            let mut expected_g: Gej = Gej::new();
            gej_set_ge(&mut expected_g, &ge_const_g);

            let mut neg_expected: Gej = Gej::new();
            gej_neg(&mut neg_expected, (&expected_g) as *const Gej);

            let mut diff: Gej = Gej::new();
            gej_add_var(&mut diff, (&r) as *const Gej, (&neg_expected) as *const Gej, core::ptr::null_mut());

            tracing::info!("checking 1*G equals generator after build+blind");
            assert!(gej_is_infinity((&diff) as *const Gej) != 0);

            ecmult_gen_context_clear(&mut ctx);
            assert!(ecmult_gen_context_is_built((&ctx) as *const EcMultGenContext) == 0);
        }
    }
}
