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
