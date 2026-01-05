// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_preallocated_clone_size.rs ]
crate::ix!();

pub fn context_preallocated_clone_size(ctx: *const Secp256k1Context) -> usize {
    let mut ret: usize = round_to_align!(core::mem::size_of::<Secp256k1Context>());
    verify_check!(!ctx.is_null());

    unsafe {
        if ecmult_gen_context_is_built(&(*ctx).ecmult_gen_ctx) != 0 {
            ret += ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE;
        }
        if ecmult_context_is_built(&(*ctx).ecmult_ctx) != 0 {
            ret += ECMULT_CONTEXT_PREALLOCATED_SIZE;
        }
    }

    ret
}

#[cfg(test)]
mod context_preallocated_clone_size_api_contract_suite {
    use super::*;

    #[traced_test]
    fn clone_size_for_no_precomp_matches_base_preallocated_size() {
        let ctx_ptr: *const Secp256k1Context = &CONTEXT_NO_PRECOMP as *const Secp256k1Context;

        let clone_size = context_preallocated_clone_size(ctx_ptr);
        let base_size = context_preallocated_size(FLAGS_TYPE_CONTEXT);

        tracing::debug!(clone_size, base_size, "computed sizes");
        assert_eq!(clone_size, base_size);
    }

    #[traced_test]
    fn clone_size_for_minimal_created_context_matches_base_preallocated_size() {
        let flags = FLAGS_TYPE_CONTEXT;
        tracing::info!(flags, "creating minimal context for clone-size test");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        let clone_size = context_preallocated_clone_size(ctx);
        let base_size = context_preallocated_size(flags);

        tracing::debug!(clone_size, base_size, "computed sizes");
        assert_eq!(clone_size, base_size);

        context_destroy(ctx);
    }

    #[traced_test]
    fn clone_size_for_sign_verify_created_context_matches_preallocated_size() {
        let flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_SIGN | FLAGS_BIT_CONTEXT_VERIFY;
        tracing::info!(flags, "creating sign+verify context for clone-size test");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        let clone_size = context_preallocated_clone_size(ctx);
        let expected = context_preallocated_size(flags);

        tracing::debug!(clone_size, expected, "computed sizes");
        assert_eq!(clone_size, expected);

        context_destroy(ctx);
    }
}
