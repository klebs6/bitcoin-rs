// ---------------- [ File: bitcoinsecp256k1-eccontext/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{context}
x!{context_clone}
x!{context_create}
x!{context_destroy}
x!{context_preallocated_clone}
x!{context_preallocated_clone_size}
x!{context_preallocated_create}
x!{context_preallocated_destroy}
x!{context_preallocated_size}
x!{context_randomize}
x!{context_set_error_callback}
x!{context_set_illegal_callback}
x!{get_verify_context}

#[cfg(test)]
mod imports_surface_area_contract_suite {
    use super::*;

    #[traced_test]
    fn imports_expose_expected_core_types_to_this_crate() {
        tracing::info!("verifying imported symbol surface compiles and is usable");

        let _ = core::mem::size_of::<Callback>();
        let _ = core::mem::size_of::<Scalar>();
        let _ = core::mem::size_of::<Ge>();
        let _ = core::mem::size_of::<Gej>();
        let _ = core::mem::size_of::<Scratch>();

        tracing::debug!("imported types are available");
    }
}

#[cfg(test)]
mod eccontext_crate_integration_contract_suite {
    use crate::*;
    use core::sync::atomic::{AtomicUsize, Ordering};

    fn counting_illegal_callback(_message: *const u8, data: *mut libc::c_void) {
        if data.is_null() {
            tracing::error!("counting_illegal_callback invoked with NULL data");
            return;
        }
        let counter = unsafe { &*(data as *const AtomicUsize) };
        counter.fetch_add(1, Ordering::SeqCst);
    }

    #[traced_test]
    fn end_to_end_context_lifecycle_create_randomize_clone_preallocated_clone_and_destroy() {
        let flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_SIGN | FLAGS_BIT_CONTEXT_VERIFY;
        tracing::info!(flags, "creating primary context");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        let seed = [1u8; 32];
        let rand_ret = context_randomize(ctx, seed.as_ptr());
        tracing::debug!(rand_ret, "context_randomize returned");
        assert_eq!(rand_ret, 1);

        let counter = Box::new(AtomicUsize::new(0));
        let counter_ptr = (&*counter as *const AtomicUsize) as *const libc::c_void;

        tracing::info!("installing counting illegal callback");
        context_set_illegal_callback(ctx, Some(counting_illegal_callback), counter_ptr);

        tracing::info!("triggering illegal callback via preallocated clone with NULL prealloc");
        let prealloc_clone_null = context_preallocated_clone(ctx, core::ptr::null_mut());
        tracing::debug!(prealloc_clone_null = ?prealloc_clone_null, "context_preallocated_clone returned");
        assert!(prealloc_clone_null.is_null());

        let calls = counter.load(Ordering::SeqCst);
        tracing::debug!(calls, "illegal callback call count observed");
        assert_eq!(calls, 1);

        tracing::info!("cloning context via context_clone");
        let cloned = context_clone(ctx);
        tracing::debug!(cloned = ?cloned, "context_clone returned");
        assert!(!cloned.is_null());
        assert_ne!(cloned, ctx);

        tracing::info!("destroying cloned context via context_destroy");
        context_destroy(cloned);

        tracing::info!("creating and destroying preallocated clone with sized buffer");
        let clone_size = context_preallocated_clone_size(ctx);
        tracing::debug!(clone_size, "context_preallocated_clone_size returned");

        let layout = std::alloc::Layout::from_size_align(clone_size.max(1), 64).unwrap();
        let buf = unsafe { std::alloc::alloc_zeroed(layout) } as *mut libc::c_void;
        tracing::debug!(buf = ?buf, "allocated buffer for preallocated clone");
        assert!(!buf.is_null());

        let prealloc_cloned = context_preallocated_clone(ctx, buf);
        tracing::debug!(prealloc_cloned = ?prealloc_cloned, "context_preallocated_clone returned");
        assert!(!prealloc_cloned.is_null());

        context_preallocated_destroy(prealloc_cloned);

        unsafe { std::alloc::dealloc(buf as *mut u8, layout) };

        tracing::info!("destroying primary context via context_destroy");
        context_destroy(ctx);
    }
}
