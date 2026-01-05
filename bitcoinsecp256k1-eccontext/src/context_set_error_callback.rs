// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_set_error_callback.rs ]
crate::ix!();

pub fn context_set_error_callback(
    ctx: *mut Secp256k1Context,
    fun: Option<fn(message: *const u8, data: *mut c_void)>,
    data: *const c_void,
) {
    unsafe {
        arg_check_no_return!(
            ctx != (&CONTEXT_NO_PRECOMP as *const Secp256k1Context as *mut Secp256k1Context)
        );

        let mut fun = fun;
        if fun.is_none() {
            fun = Some(default_error_callback_fn);
        }

        (*ctx).error_callback = Callback::new(fun.unwrap(), data);
    }
}

#[cfg(test)]
mod context_set_error_callback_api_contract_suite {
    use super::*;

    fn noop_error_callback(_message: *const u8, _data: *mut libc::c_void) {}

    #[traced_test]
    fn context_set_error_callback_accepts_none_and_does_not_crash() {
        let flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_VERIFY;
        tracing::info!(flags, "creating context for set_error_callback(None) test");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        tracing::info!("setting error callback to default via None");
        context_set_error_callback(ctx, None, core::ptr::null());

        context_destroy(ctx);
    }

    #[traced_test]
    fn context_set_error_callback_accepts_custom_callback_and_data_pointer() {
        let flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_VERIFY;
        tracing::info!(flags, "creating context for set_error_callback(Some) test");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        let data = 0x1usize as *const libc::c_void;

        tracing::info!("setting custom error callback");
        context_set_error_callback(ctx, Some(noop_error_callback), data);

        tracing::info!("cloning context after setting error callback");
        let cloned = context_clone(ctx);
        tracing::debug!(cloned = ?cloned, "context_clone returned");
        assert!(!cloned.is_null());

        context_destroy(cloned);
        context_destroy(ctx);
    }
}
