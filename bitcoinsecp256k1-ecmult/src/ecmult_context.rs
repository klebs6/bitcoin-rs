// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_context.rs ]
crate::ix!();

/// For accelerating the computation of a*P + b*G:
///
/// NOTE: these vectors were slices before we wanted a const constructor.
///
/// perhaps this will cause problems... perhaps not
///
#[repr(C)]
#[derive(Getters,Setters,MutGetters)]
#[getset(get="pub",set="pub",get_mut="pub")]
pub struct EcMultContext {
    /// odd multiples of the generator
    pre_g:     *mut GeStorage,

    /// odd multiples of 2^128*generator
    pre_g_128: *mut GeStorage,
}

impl EcMultContext {
    pub const fn new() -> Self {
        Self {
            pre_g: core::ptr::null_mut(),
            pre_g_128: core::ptr::null_mut(),
        }
    }
}
