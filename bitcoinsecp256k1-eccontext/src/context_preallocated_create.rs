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
