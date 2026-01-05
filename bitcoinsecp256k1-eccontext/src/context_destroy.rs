// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_destroy.rs ]
crate::ix!();

/**
  | Destroy a secp256k1 context object
  | (created in dynamically allocated
  | memory).
  | 
  | The context pointer may not be used afterwards.
  | 
  | The context to destroy must have been
  | created using context_create or context_clone.
  | If the context has instead been created
  | using context_preallocated_create
  | or context_preallocated_clone, the
  | behaviour is undefined. In that case,
  | context_preallocated_destroy must
  | be used instead.
  | 
  | Args: ctx: an existing context to destroy,
  |            constructed using context_create
  |            or context_clone
  |
  */
pub fn context_destroy(ctx: *mut Secp256k1Context) {
    unsafe {
        if !ctx.is_null() {
            context_preallocated_destroy(ctx);
            libc::free(ctx as *mut c_void);
        }
    }
}

#[cfg(test)]
mod context_destroy_api_contract_suite {
    use super::*;

    #[traced_test]
    fn context_destroy_accepts_null_pointer() {
        tracing::info!("calling context_destroy with NULL pointer");
        context_destroy(core::ptr::null_mut());
        tracing::debug!("context_destroy(NULL) returned");
    }

    #[traced_test]
    fn context_destroy_frees_valid_context_created_via_context_create() {
        let flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_VERIFY;
        tracing::info!(flags, "creating context for destroy test");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        tracing::info!("destroying context");
        context_destroy(ctx);
    }
}
