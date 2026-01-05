// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_set_illegal_callback.rs ]
crate::ix!();

pub fn context_set_illegal_callback(
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
            fun = Some(default_illegal_callback_fn);
        }

        (*ctx).illegal_callback = Callback::new(fun.unwrap(), data);
    }
}

#[cfg(test)]
mod context_set_illegal_callback_api_contract_suite {
    use super::*;
    use core::sync::atomic::{AtomicUsize, Ordering};

    fn counting_illegal_callback(_message: *const u8, data: *mut libc::c_void) {
        if data.is_null() {
            tracing::error!("counting_illegal_callback invoked with NULL data");
            return;
        }
        let counter = unsafe { &*(data as *const AtomicUsize) };
        let prev = counter.fetch_add(1, Ordering::SeqCst);
        tracing::debug!(prev, next = prev + 1, "illegal callback invocation recorded");
    }

    #[traced_test]
    fn context_set_illegal_callback_is_invoked_by_arg_check_on_null_prealloc() {
        let flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_SIGN | FLAGS_BIT_CONTEXT_VERIFY;
        tracing::info!(flags, "creating context for illegal-callback invocation test");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        let counter = Box::new(AtomicUsize::new(0));
        let counter_ptr = (&*counter as *const AtomicUsize) as *const libc::c_void;

        tracing::info!("installing counting illegal callback");
        context_set_illegal_callback(ctx, Some(counting_illegal_callback), counter_ptr);

        tracing::info!("triggering ARG_CHECK by calling context_preallocated_clone with NULL prealloc");
        let cloned = context_preallocated_clone(ctx, core::ptr::null_mut());
        tracing::debug!(cloned = ?cloned, "context_preallocated_clone returned");
        assert!(cloned.is_null());

        let calls = counter.load(Ordering::SeqCst);
        tracing::debug!(calls, "observed illegal callback call count");
        assert_eq!(calls, 1);

        tracing::info!("resetting illegal callback to default via None");
        context_set_illegal_callback(ctx, None, core::ptr::null());

        tracing::info!("triggering ARG_CHECK again after reset");
        let cloned2 = context_preallocated_clone(ctx, core::ptr::null_mut());
        tracing::debug!(cloned2 = ?cloned2, "context_preallocated_clone returned after reset");
        assert!(cloned2.is_null());

        let calls_after = counter.load(Ordering::SeqCst);
        tracing::debug!(calls_after, "observed illegal callback call count after reset");
        assert_eq!(calls_after, 1);

        context_destroy(ctx);
    }
}
