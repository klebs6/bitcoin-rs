// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_clone.rs ]
crate::ix!();

pub fn context_clone(ctx: *const Secp256k1Context) -> *mut Secp256k1Context {
    unsafe {
        let mut ret: *mut Secp256k1Context = core::ptr::null_mut();
        let mut prealloc_size: usize = 0;

        verify_check!(!ctx.is_null());
        prealloc_size = context_preallocated_clone_size(ctx);

        ret = checked_malloc(&(*ctx).error_callback, prealloc_size) as *mut Secp256k1Context;
        ret = context_preallocated_clone(ctx, ret as *mut c_void);

        ret
    }
}
