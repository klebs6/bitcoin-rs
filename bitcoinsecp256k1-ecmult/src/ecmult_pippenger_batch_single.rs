// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_pippenger_batch_single.rs ]
crate::ix!();

/// Wrapper for ecmult_multi_func interface
///
pub fn ecmult_pippenger_batch_single(
    error_callback: *const Callback,
    actx:           *const EcMultContext,
    scratch:        *mut Scratch,
    r:              *mut Gej,
    inp_g_sc:       *const Scalar,
    cb:             EcMultMultiCallback,
    cbdata:         *mut c_void,
    n:              usize,
) -> i32 {
    tracing::trace!(target: "secp256k1::ecmult", n = n, "ecmult_pippenger_batch_single");

    ecmult_pippenger_batch(error_callback, actx, scratch, r, inp_g_sc, cb, cbdata, n, 0)
}
