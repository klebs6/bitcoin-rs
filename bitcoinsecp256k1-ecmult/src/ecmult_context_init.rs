// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_context_init.rs ]
crate::ix!();

pub fn ecmult_context_init(ctx: *mut EcMultContext) {
    tracing::trace!(target: "secp256k1::ecmult", "ecmult_context_init");

    unsafe {
        (*ctx).set_pre_g(core::ptr::null_mut());
        (*ctx).set_pre_g_128(core::ptr::null_mut());
    }
}
