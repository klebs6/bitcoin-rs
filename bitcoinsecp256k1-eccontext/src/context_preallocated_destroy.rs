// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_preallocated_destroy.rs ]
crate::ix!();

pub fn context_preallocated_destroy(ctx: *mut Secp256k1Context) {
    unsafe {
        arg_check_no_return!(
            ctx != (&CONTEXT_NO_PRECOMP as *const Secp256k1Context as *mut Secp256k1Context)
        );
        if !ctx.is_null() {
            ecmult_context_clear(&mut (*ctx).ecmult_ctx);
            ecmult_gen_context_clear(&mut (*ctx).ecmult_gen_ctx);
        }
    }
}

#[cfg(test)]
mod context_preallocated_destroy_api_contract_suite {
    use super::*;

    fn alloc_zeroed_aligned(size: usize, align: usize) -> (*mut libc::c_void, std::alloc::Layout) {
        let size = size.max(1);
        let layout = std::alloc::Layout::from_size_align(size, align).unwrap();
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) } as *mut libc::c_void;
        tracing::debug!(size, align, ptr = ?ptr, "allocated prealloc buffer");
        assert!(!ptr.is_null());
        (ptr, layout)
    }

    fn dealloc_aligned(ptr: *mut libc::c_void, layout: std::alloc::Layout) {
        tracing::debug!(ptr = ?ptr, size = layout.size(), align = layout.align(), "deallocating prealloc buffer");
        unsafe { std::alloc::dealloc(ptr as *mut u8, layout) }
    }

    #[traced_test]
    fn context_preallocated_destroy_accepts_null_pointer() {
        tracing::info!("calling context_preallocated_destroy with NULL pointer");
        context_preallocated_destroy(core::ptr::null_mut());
        tracing::debug!("context_preallocated_destroy(NULL) returned");
    }

    #[traced_test]
    fn context_preallocated_destroy_clears_preallocated_context_created_via_context_preallocated_create() {
        let flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_SIGN | FLAGS_BIT_CONTEXT_VERIFY;
        let size = context_preallocated_size(flags);

        tracing::info!(flags, size, "allocating buffer for preallocated create/destroy roundtrip");
        assert!(size != 0);

        let (buf, layout) = alloc_zeroed_aligned(size, 64);

        let ctx = context_preallocated_create(buf, flags);
        tracing::debug!(ctx = ?ctx, "context_preallocated_create returned");
        assert!(!ctx.is_null());

        tracing::info!("destroying preallocated context");
        context_preallocated_destroy(ctx);

        dealloc_aligned(buf, layout);
    }
}
