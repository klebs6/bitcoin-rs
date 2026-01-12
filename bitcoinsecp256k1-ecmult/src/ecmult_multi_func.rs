// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_multi_func.rs ]
crate::ix!();

pub type EcMultMultiFunc = fn(
        error_callback: *const Callback,
        _1:             *const EcMultContext,
        _2:             *mut Scratch,
        _3:             *mut Gej,
        _4:             *const Scalar,
        cb:             EcMultMultiCallback,
        _6:             *mut c_void,
        _7:             usize
) -> i32;

#[cfg(test)]
mod ecmult_multi_func_type_contract_suite {
    use super::*;

    #[traced_test]
    fn multi_func_type_accepts_expected_signature() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "multi_func_type_accepts_expected_signature"
        );

        fn dummy_cb(_sc: *mut Scalar, _pt: *mut Ge, _idx: usize, _data: *mut c_void) -> i32 {
            0
        }

        fn dummy_multi_func(
            _error_callback: *const Callback,
            _actx: *const EcMultContext,
            _scratch: *mut Scratch,
            _r: *mut Gej,
            _inp_g_sc: *const Scalar,
            _cb: EcMultMultiCallback,
            _cbdata: *mut c_void,
            _n: usize,
        ) -> i32 {
            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                "dummy_multi_func invoked"
            );
            0
        }

        let f: EcMultMultiFunc = dummy_multi_func;
        let _ = f as usize;
        let _ = dummy_cb as EcMultMultiCallback;
        assert!(true);
    }
}
