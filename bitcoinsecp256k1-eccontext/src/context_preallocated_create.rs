// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_preallocated_create.rs ]
crate::ix!();

pub fn context_preallocated_create(prealloc: *mut c_void, flags: u32) -> *mut Secp256k1Context {
    unsafe {
        let base: *mut c_void = prealloc;
        let mut prealloc_size: usize = 0;
        let mut ret: *mut Secp256k1Context = core::ptr::null_mut();

        if selftest() == 0 {
            callback_call(&*default_error_callback, b"self test failed\0".as_ptr());
        }

        prealloc_size = context_preallocated_size(flags);
        if prealloc_size == 0 {
            return core::ptr::null_mut();
        }
        verify_check!(!prealloc.is_null());

        let mut prealloc: *mut c_void = prealloc;
        ret = manual_alloc(
            &mut prealloc,
            core::mem::size_of::<Secp256k1Context>(),
            base,
            prealloc_size,
        ) as *mut Secp256k1Context;

        (*ret).illegal_callback = *default_illegal_callback;
        (*ret).error_callback = *default_error_callback;

        ecmult_context_init(&mut (*ret).ecmult_ctx);
        ecmult_gen_context_init(&mut (*ret).ecmult_gen_ctx);

        /* Flags have been checked by context_preallocated_size. */
        verify_check!((flags & FLAGS_TYPE_MASK) == FLAGS_TYPE_CONTEXT);
        if (flags & FLAGS_BIT_CONTEXT_SIGN) != 0 {
            ecmult_gen_context_build(&mut (*ret).ecmult_gen_ctx, &mut prealloc);
        }
        if (flags & FLAGS_BIT_CONTEXT_VERIFY) != 0 {
            ecmult_context_build(&mut (*ret).ecmult_ctx, &mut prealloc);
        }
        (*ret).declassify = ((flags & FLAGS_BIT_CONTEXT_DECLASSIFY) != 0) as i32;

        ret
    }
}

#[cfg(test)]
mod context_preallocated_create_api_contract_suite {
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
    fn context_preallocated_create_returns_null_for_invalid_flags_even_with_null_prealloc() {
        let flags: u32 = 0;
        tracing::info!(flags, "calling context_preallocated_create with invalid flags and NULL prealloc");

        let ctx = context_preallocated_create(core::ptr::null_mut(), flags);
        tracing::debug!(ctx = ?ctx, "context_preallocated_create returned");
        assert!(ctx.is_null());
    }

    #[traced_test]
    fn context_preallocated_create_succeeds_with_valid_flags_and_sized_buffer() {
        let flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_SIGN | FLAGS_BIT_CONTEXT_VERIFY;
        let size = context_preallocated_size(flags);

        tracing::info!(flags, size, "allocating buffer for context_preallocated_create");
        assert!(size != 0);

        let (buf, layout) = alloc_zeroed_aligned(size, 64);

        let ctx = context_preallocated_create(buf, flags);
        tracing::debug!(ctx = ?ctx, buf = ?buf, "context_preallocated_create returned");
        assert!(!ctx.is_null());

        let buf_start = buf as usize;
        let buf_end = buf_start + size;
        let ctx_addr = ctx as usize;

        tracing::debug!(buf_start, buf_end, ctx_addr, "address range check for returned context pointer");
        assert!(ctx_addr >= buf_start);
        assert!(ctx_addr < buf_end);

        context_preallocated_destroy(ctx);
        dealloc_aligned(buf, layout);
    }

    #[traced_test]
    fn context_preallocated_create_succeeds_with_declassify_flag_set() {
        let flags = FLAGS_TYPE_CONTEXT
            | FLAGS_BIT_CONTEXT_SIGN
            | FLAGS_BIT_CONTEXT_VERIFY
            | FLAGS_BIT_CONTEXT_DECLASSIFY;

        let size = context_preallocated_size(flags);
        tracing::info!(flags, size, "allocating buffer for declassify preallocated context");
        assert!(size != 0);

        let (buf, layout) = alloc_zeroed_aligned(size, 64);

        let ctx = context_preallocated_create(buf, flags);
        tracing::debug!(ctx = ?ctx, "context_preallocated_create returned");
        assert!(!ctx.is_null());

        context_preallocated_destroy(ctx);
        dealloc_aligned(buf, layout);
    }
}
