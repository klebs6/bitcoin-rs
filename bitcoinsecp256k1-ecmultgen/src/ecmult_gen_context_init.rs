// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/ecmult_gen_context_init.rs ]
crate::ix!();

pub fn ecmult_gen_context_init(ctx: *mut EcMultGenContext)  {
    
    unsafe {
        (*ctx).set_prec(null_mut());
    }
}
