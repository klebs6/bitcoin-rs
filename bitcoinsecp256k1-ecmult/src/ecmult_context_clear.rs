// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_context_clear.rs ]
crate::ix!();

pub fn ecmult_context_clear(ctx: *mut EcMultContext) {
    tracing::trace!(target: "secp256k1::ecmult", "ecmult_context_clear");

    unsafe {
        ecmult_context_init(ctx);
    }
}
