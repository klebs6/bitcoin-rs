// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_context.rs ]
crate::ix!();

/// For accelerating the computation of a*P + b*G:
///
/// NOTE: these vectors were slices before we wanted a const constructor.
///
/// perhaps this will cause problems... perhaps not
///
pub struct EcMultContext {

    /// odd multiples of the generator
    ///
    pre_g:     Vec<*mut GeStorage>,

    /// odd multiples of 2^128*generator
    ///
    pre_g_128: Vec<*mut GeStorage>,
}

impl EcMultContext {

    pub const fn new() -> Self {
        Self {
            pre_g:     vec![],
            pre_g_128: vec![],
        }
    }
}
