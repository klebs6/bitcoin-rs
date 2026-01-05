// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_preallocated_clone.rs ]
crate::ix!();

pub fn context_preallocated_clone(
    ctx: *const Secp256k1Context,
    prealloc: *mut c_void,
) -> *mut Secp256k1Context {
    unsafe {
        let mut prealloc_size: usize = 0;
        let mut ret: *mut Secp256k1Context = core::ptr::null_mut();

        verify_check!(!ctx.is_null());
        arg_check!(!prealloc.is_null());

        prealloc_size = context_preallocated_clone_size(ctx);
        ret = prealloc as *mut Secp256k1Context;

        libc::memcpy(ret as *mut c_void, ctx as *const c_void, prealloc_size);

        ecmult_gen_context_finalize_memcpy(&mut (*ret).ecmult_gen_ctx, &(*ctx).ecmult_gen_ctx);
        ecmult_context_finalize_memcpy(&mut (*ret).ecmult_ctx, &(*ctx).ecmult_ctx);

        ret
    }
}

#[cfg(test)]
mod context_preallocated_clone_api_contract_suite {
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
    fn context_preallocated_clone_roundtrips_and_can_be_preallocated_destroyed() {
        let flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_SIGN | FLAGS_BIT_CONTEXT_VERIFY;
        tracing::info!(flags, "creating source context for preallocated clone");

        let ctx = context_create(flags);
        tracing::debug!(ctx = ?ctx, "context_create returned");
        assert!(!ctx.is_null());

        let clone_size = context_preallocated_clone_size(ctx);
        tracing::info!(clone_size, "computed clone preallocation size");

        let (buf, layout) = alloc_zeroed_aligned(clone_size, 64);

        let cloned = context_preallocated_clone(ctx, buf);
        tracing::debug!(cloned = ?cloned, buf = ?buf, "context_preallocated_clone returned");
        assert!(!cloned.is_null());
        assert_eq!(cloned as *mut libc::c_void, buf);

        tracing::info!("destroying cloned preallocated context");
        context_preallocated_destroy(cloned);

        dealloc_aligned(buf, layout);

        tracing::info!("destroying original context");
        context_destroy(ctx);
    }
}
