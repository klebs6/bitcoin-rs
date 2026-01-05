// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_randomize.rs ]
crate::ix!();

pub fn context_randomize(ctx: *mut Secp256k1Context, seed32: *const u8) -> i32 {
    unsafe {
        verify_check!(!ctx.is_null());
        if ecmult_gen_context_is_built(&(*ctx).ecmult_gen_ctx) != 0 {
            ecmult_gen_blind(&mut (*ctx).ecmult_gen_ctx, seed32);
        }
        1
    }
}

#[cfg(test)]
mod context_randomize_api_contract_suite {
    use super::*;

    #[traced_test]
    fn context_randomize_returns_one_for_verify_only_context_with_null_seed() {
        let flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_VERIFY;
        tracing::info!(flags, "creating verify-only context");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        let ret = context_randomize(ctx, core::ptr::null());
        tracing::debug!(ret, "context_randomize returned");
        assert_eq!(ret, 1);

        context_destroy(ctx);
    }

    #[traced_test]
    fn context_randomize_returns_one_for_sign_only_context_with_null_and_non_null_seed() {
        let flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_SIGN;
        tracing::info!(flags, "creating sign-only context");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        let ret_null = context_randomize(ctx, core::ptr::null());
        tracing::debug!(ret_null, "context_randomize(NULL seed) returned");
        assert_eq!(ret_null, 1);

        let seed = [0u8; 32];
        let ret_seed = context_randomize(ctx, seed.as_ptr());
        tracing::debug!(ret_seed, "context_randomize(non-NULL seed) returned");
        assert_eq!(ret_seed, 1);

        context_destroy(ctx);
    }

    #[traced_test]
    fn context_randomize_returns_one_for_base_context_without_sign_precomputation() {
        let flags = FLAGS_TYPE_CONTEXT;
        tracing::info!(flags, "creating base context");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        let seed = [0u8; 32];
        let ret = context_randomize(ctx, seed.as_ptr());
        tracing::debug!(ret, "context_randomize returned");
        assert_eq!(ret, 1);

        context_destroy(ctx);
    }
}
