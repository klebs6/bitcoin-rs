// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_preallocated_clone_size.rs ]
crate::ix!();

pub fn context_preallocated_clone_size(ctx: *const Secp256k1Context) -> usize {
    let mut ret: usize = round_to_align!(core::mem::size_of::<Secp256k1Context>());
    verify_check!(!ctx.is_null());

    unsafe {
        if ecmult_gen_context_is_built(&(*ctx).ecmult_gen_ctx) != 0 {
            ret += ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE;
        }
        if ecmult_context_is_built(&(*ctx).ecmult_ctx) != 0 {
            ret += ECMULT_CONTEXT_PREALLOCATED_SIZE;
        }
    }

    ret
}
