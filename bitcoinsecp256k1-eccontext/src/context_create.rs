// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_create.rs ]
crate::ix!();

/**
  | Create a secp256k1 context object (in
  | dynamically allocated memory).
  | 
  | This function uses malloc to allocate
  | memory. It is guaranteed that malloc
  | is called at most once for every call
  | of this function. If you need to avoid
  | dynamic memory allocation entirely,
  | see the functions in preallocated.h.
  | 
  | Returns: a newly created context object.
  | 
  | In: flags: which parts of the context
  | to initialize.
  | 
  | See also context_randomize.
  |
  */
pub fn context_create(flags: u32) -> *mut Secp256k1Context {
    unsafe {
        let prealloc_size: usize = context_preallocated_size(flags);
        let ctx: *mut Secp256k1Context =
            checked_malloc(&*default_error_callback, prealloc_size) as *mut Secp256k1Context;

        if expect!(
            context_preallocated_create(ctx as *mut c_void, flags).is_null(),
            0
        ) {
            libc::free(ctx as *mut c_void);
            return core::ptr::null_mut();
        }

        ctx
    }
}

#[cfg(test)]
mod context_create_api_contract_suite {
    use super::*;

    #[traced_test]
    fn context_create_returns_null_for_invalid_flags() {
        let flags: u32 = 0;
        tracing::info!(flags, "calling context_create with invalid flags");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(ctx.is_null());
    }

    #[traced_test]
    fn context_create_allocates_and_can_be_destroyed_for_base_flags() {
        let flags = FLAGS_TYPE_CONTEXT;
        tracing::info!(flags, "creating base context");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        tracing::info!("destroying base context");
        context_destroy(ctx);
    }

    #[traced_test]
    fn context_create_allocates_sign_only_context() {
        let flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_SIGN;
        tracing::info!(flags, "creating sign-only context");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        context_destroy(ctx);
    }

    #[traced_test]
    fn context_create_allocates_verify_only_context() {
        let flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_VERIFY;
        tracing::info!(flags, "creating verify-only context");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        context_destroy(ctx);
    }

    #[traced_test]
    fn context_create_allocates_sign_verify_declassify_context() {
        let flags = FLAGS_TYPE_CONTEXT
            | FLAGS_BIT_CONTEXT_SIGN
            | FLAGS_BIT_CONTEXT_VERIFY
            | FLAGS_BIT_CONTEXT_DECLASSIFY;

        tracing::info!(flags, "creating sign+verify+declassify context");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        context_destroy(ctx);
    }
}
