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
