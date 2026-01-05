// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_multi_callback.rs ]
crate::ix!();

pub type EcMultMultiCallback = fn(
    sc:   *mut Scalar,
    pt:   *mut Ge,
    idx:  usize,
    data: *mut c_void
) -> i32;
