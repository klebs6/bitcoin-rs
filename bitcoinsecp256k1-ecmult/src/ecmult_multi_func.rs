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
