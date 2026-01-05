// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_randomize.rs ]
crate::ix!();

pub fn context_randomize(ctx: *mut Secp256k1Context, seed32: *const u8) -> i32 {
    unsafe {
        verify_check!(!ctx.is_null());
        if ecmult_gen_context_is_built(&(*ctx).ecmult_gen_ctx) != 0 {
            ecmult_gen_blind(&mut (*ctx).ecmult_gen_ctx, seed32);
        }
        1
    }
}
