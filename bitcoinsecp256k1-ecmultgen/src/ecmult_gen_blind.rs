// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/ecmult_gen_blind.rs ]
crate::ix!();

/// Setup blinding values for ecmult_gen.
pub fn ecmult_gen_blind(
    ctx:    *mut EcMultGenContext,
    seed32: *const u8)  {

    unsafe {
        let mut b: Scalar = Scalar::new();
        let mut gb: Gej = Gej::new();
        let mut s: Fe = core::mem::zeroed();
        let mut nonce32: [u8; 32] = [0u8; 32];
        let mut rng: Rfc6979HmacSha256 = core::mem::zeroed();
        let mut overflow: i32;
        let mut keydata: [u8; 64] = [0u8; 64];

        if seed32.is_null() {
            /* When seed is NULL, reset the initial point and blinding value. */
            let initial: *mut Gej = (*ctx).initial_mut();
            gej_set_ge(initial, &ge_const_g);
            gej_neg(initial, initial as *const Gej);

            let blind: *mut Scalar = (*ctx).blind_mut();
            scalar_set_int(blind, 1);
        }

        /* The prior blinding value (if not reset) is chained forward by including it in the hash. */
        scalar_get_b32(nonce32.as_mut_ptr(), core::ptr::from_ref((*ctx).blind()));

        /** Using a CSPRNG allows a failure free interface, avoids needing large amounts of random data,
        *   and guards against weak or adversarial seeds.  This is a simpler and safer interface than
        *   asking the caller for blinding values directly and expecting them to retry on failure.
        */
        core::ptr::copy_nonoverlapping(nonce32.as_ptr(), keydata.as_mut_ptr(), 32);
        if !seed32.is_null() {
            core::ptr::copy_nonoverlapping(seed32, keydata.as_mut_ptr().add(32), 32);
        }

        rfc6979_hmac_sha256_initialize(
            core::ptr::addr_of_mut!(rng),
            keydata.as_ptr(),
            if !seed32.is_null() { 64 } else { 32 },
        );
        keydata.fill(0);

        /* Accept unobservably small non-uniformity. */
        rfc6979_hmac_sha256_generate(core::ptr::addr_of_mut!(rng), nonce32.as_mut_ptr(), 32);

        overflow = (fe_set_b32(core::ptr::addr_of_mut!(s), nonce32.as_ptr()) == 0) as i32;
        overflow |= fe_is_zero(core::ptr::addr_of!(s));

        let mut fe_one_local: Fe = core::mem::zeroed();
        fe_set_int(core::ptr::addr_of_mut!(fe_one_local), 1);
        fe_cmov(core::ptr::addr_of_mut!(s), core::ptr::addr_of!(fe_one_local), overflow);

        /* Randomize the projection to defend against multiplier sidechannels. */
        gej_rescale((*ctx).initial_mut(), core::ptr::addr_of!(s));
        fe_clear(core::ptr::addr_of_mut!(s));

        rfc6979_hmac_sha256_generate(core::ptr::addr_of_mut!(rng), nonce32.as_mut_ptr(), 32);
        scalar_set_b32(core::ptr::addr_of_mut!(b), nonce32.as_ptr(), core::ptr::null_mut());

        /* A blinding value of 0 works, but would undermine the projection hardening. */
            {
                let mut scalar_one_local: Scalar = Scalar::new();
                scalar_set_int(core::ptr::addr_of_mut!(scalar_one_local), 1);
                let b_is_zero: i32 = scalar_is_zero(core::ptr::addr_of!(b));
                scalar_cmov(
                    core::ptr::addr_of_mut!(b),
                    core::ptr::addr_of!(scalar_one_local),
                    b_is_zero,
                );
            }

            rfc6979_hmac_sha256_finalize(core::ptr::addr_of_mut!(rng));
            nonce32.fill(0);

            ecmult_gen(
                ctx as *const EcMultGenContext,
                core::ptr::addr_of_mut!(gb),
                core::ptr::addr_of!(b),
            );
            scalar_negate(core::ptr::addr_of_mut!(b), core::ptr::addr_of!(b));

            core::ptr::copy_nonoverlapping(core::ptr::addr_of!(b), (*ctx).blind_mut(), 1);
            core::ptr::copy_nonoverlapping(core::ptr::addr_of!(gb), (*ctx).initial_mut(), 1);

            scalar_clear(core::ptr::addr_of_mut!(b));
            gej_clear(core::ptr::addr_of_mut!(gb));
    }

}

#[cfg(test)]
mod ecmult_gen_blinding_refresh_suite {
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

    unsafe fn build_test_context() -> (EcMultGenContext, Vec<u8>) {
        let mut ctx = EcMultGenContext::new();
        ecmult_gen_context_init(&mut ctx);

        #[cfg(not(USE_ECMULT_STATIC_PRECOMPUTATION))]
        {
            let (mem, mut prealloc_cursor) = aligned_prealloc_region_for_ecmult_gen();
            ecmult_gen_context_build(&mut ctx, core::ptr::addr_of_mut!(prealloc_cursor));
            return (ctx, mem);
        }

        #[cfg(USE_ECMULT_STATIC_PRECOMPUTATION)]
        {
            let mem: Vec<u8> = Vec::new();
            ecmult_gen_context_build(&mut ctx, core::ptr::null_mut());
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

    unsafe fn scalar_to_bytes32(s: *const Scalar) -> [u8; 32] {
        let mut out = [0u8; 32];
        scalar_get_b32(out.as_mut_ptr(), s);
        out
    }

    #[traced_test]
    fn ecmult_gen_blind_with_null_seed_is_deterministic_and_preserves_correctness() {
        unsafe {
            let (mut ctx, _mem) = build_test_context();

            let mut one: Scalar = Scalar::new();
            scalar_set_int(&mut one, 1);

            let mut r_before: Gej = Gej::new();
            ecmult_gen((&ctx) as *const EcMultGenContext, &mut r_before, (&one) as *const Scalar);

            ecmult_gen_blind(&mut ctx, core::ptr::null());

            let mut r_after: Gej = Gej::new();
            ecmult_gen((&ctx) as *const EcMultGenContext, &mut r_after, (&one) as *const Scalar);

            tracing::info!("checking that blinding refresh does not change 1*G");
            assert_gej_eq((&r_after) as *const Gej, (&r_before) as *const Gej);

            let blind_after_first = scalar_to_bytes32(ctx.blind() as *const Scalar);

            ecmult_gen_blind(&mut ctx, core::ptr::null());
            let blind_after_second = scalar_to_bytes32(ctx.blind() as *const Scalar);

            tracing::info!("checking deterministic output for null seed reblinding");
            assert_eq!(blind_after_first, blind_after_second);

            tracing::debug!("checking basic non-degeneracy of blinding scalar");
            assert!(scalar_is_zero(ctx.blind() as *const Scalar) == 0);

            let mut expected_g: Gej = Gej::new();
            gej_set_ge(&mut expected_g, &ge_const_g);

            tracing::debug!("checking 1*G equals generator after deterministic null-seed reblinding");
            assert_gej_eq((&r_after) as *const Gej, (&expected_g) as *const Gej);
        }
    }

    #[traced_test]
    fn ecmult_gen_blind_with_explicit_seed_is_deterministic_given_same_start_state() {
        unsafe {
            let seed: [u8; 32] = [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
                0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
                0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
            ];

            let (mut ctx1, _mem1) = build_test_context();
            let (mut ctx2, _mem2) = build_test_context();

            ecmult_gen_blind(&mut ctx1, seed.as_ptr());
            ecmult_gen_blind(&mut ctx2, seed.as_ptr());

            let b1 = scalar_to_bytes32(ctx1.blind() as *const Scalar);
            let b2 = scalar_to_bytes32(ctx2.blind() as *const Scalar);

            tracing::info!("checking deterministic blinded state across identical start states");
            assert_eq!(b1, b2);

            tracing::debug!("checking deterministic initial point across identical start states");
            assert_gej_eq(ctx1.initial() as *const Gej, ctx2.initial() as *const Gej);

            let mut one: Scalar = Scalar::new();
            scalar_set_int(&mut one, 1);

            let mut r1: Gej = Gej::new();
            ecmult_gen((&ctx1) as *const EcMultGenContext, &mut r1, (&one) as *const Scalar);

            let mut r2: Gej = Gej::new();
            ecmult_gen((&ctx2) as *const EcMultGenContext, &mut r2, (&one) as *const Scalar);

            tracing::debug!("checking 1*G remains generator after explicit-seed reblinding");
            let mut expected_g: Gej = Gej::new();
            gej_set_ge(&mut expected_g, &ge_const_g);

            assert_gej_eq((&r1) as *const Gej, (&expected_g) as *const Gej);
            assert_gej_eq((&r2) as *const Gej, (&expected_g) as *const Gej);
        }
    }

    #[traced_test]
    fn ecmult_gen_blind_chains_previous_blind_when_seed_is_reused() {
        unsafe {
            let seed: [u8; 32] = [0x42u8; 32];
            let (mut ctx, _mem) = build_test_context();

            ecmult_gen_blind(&mut ctx, seed.as_ptr());
            let blind1 = scalar_to_bytes32(ctx.blind() as *const Scalar);
            let mut initial1: Gej = Gej::new();
            core::ptr::copy_nonoverlapping(ctx.initial() as *const Gej, &mut initial1, 1);

            ecmult_gen_blind(&mut ctx, seed.as_ptr());
            let blind2 = scalar_to_bytes32(ctx.blind() as *const Scalar);
            let mut initial2: Gej = Gej::new();
            core::ptr::copy_nonoverlapping(ctx.initial() as *const Gej, &mut initial2, 1);

            tracing::info!("checking that reuse of seed chains (state changes) rather than repeating");
            let same_blind = blind1 == blind2;
            let same_initial = gej_points_are_equal((&initial1) as *const Gej, (&initial2) as *const Gej);

            assert!(!(same_blind && same_initial));

            let mut one: Scalar = Scalar::new();
            scalar_set_int(&mut one, 1);

            let mut r: Gej = Gej::new();
            ecmult_gen((&ctx) as *const EcMultGenContext, &mut r, (&one) as *const Scalar);

            let mut expected_g: Gej = Gej::new();
            gej_set_ge(&mut expected_g, &ge_const_g);

            tracing::debug!("checking correctness preserved after chained reblinding");
            assert_gej_eq((&r) as *const Gej, (&expected_g) as *const Gej);
        }
    }
}
