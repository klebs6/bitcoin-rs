// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_context_is_built.rs ]
crate::ix!();

pub fn ecmult_context_is_built(ctx: *const EcMultContext) -> i32 {
    tracing::trace!(target: "secp256k1::ecmult", "ecmult_context_is_built");

    unsafe { (!(*ctx).pre_g().is_null()) as i32 }
}
