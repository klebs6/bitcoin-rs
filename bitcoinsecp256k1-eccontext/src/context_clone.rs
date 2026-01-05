// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_clone.rs ]
crate::ix!();

pub fn context_clone(ctx: *const Secp256k1Context) -> *mut Secp256k1Context {
    unsafe {
        let mut ret: *mut Secp256k1Context = core::ptr::null_mut();
        let mut prealloc_size: usize = 0;

        verify_check!(!ctx.is_null());
        prealloc_size = context_preallocated_clone_size(ctx);

        ret = checked_malloc(&(*ctx).error_callback, prealloc_size) as *mut Secp256k1Context;
        ret = context_preallocated_clone(ctx, ret as *mut c_void);

        ret
    }
}

#[cfg(test)]
mod context_clone_api_contract_suite {
    use super::*;

    fn all_precomp_context_flags() -> u32 {
        FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_SIGN | FLAGS_BIT_CONTEXT_VERIFY
    }

    #[traced_test]
    fn context_clone_duplicates_context_and_allows_independent_destruction() {
        let flags = all_precomp_context_flags();
        tracing::info!(flags, "creating context for clone test");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        let cloned = context_clone(ctx);
        tracing::debug!(cloned = ?cloned, "context_clone returned");
        assert!(!cloned.is_null());
        assert_ne!(cloned, ctx);

        tracing::info!("destroying cloned context");
        context_destroy(cloned);

        tracing::info!("destroying original context");
        context_destroy(ctx);
    }

    #[traced_test]
    fn context_clone_works_for_base_context_without_precomputation() {
        let flags = FLAGS_TYPE_CONTEXT;
        tracing::info!(flags, "creating minimal context for clone test");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        let cloned = context_clone(ctx);
        tracing::debug!(cloned = ?cloned, "context_clone returned");
        assert!(!cloned.is_null());
        assert_ne!(cloned, ctx);

        context_destroy(cloned);
        context_destroy(ctx);
    }
}
