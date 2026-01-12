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

#[cfg(test)]
mod pippenger_batch_single_interface_contract_suite {
    use super::*;

    #[traced_test]
    fn pippenger_batch_single_matches_ecmult_multi_func_interface() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "pippenger_batch_single_matches_ecmult_multi_func_interface"
        );

        let f: EcMultMultiFunc = ecmult_pippenger_batch_single;
        let _addr = f as usize;

        tracing::debug!(
            target: "secp256k1::ecmult::tests",
            f_addr = _addr,
            "captured ecmult_pippenger_batch_single fn pointer"
        );

        assert!(_addr != 0);
    }
}
