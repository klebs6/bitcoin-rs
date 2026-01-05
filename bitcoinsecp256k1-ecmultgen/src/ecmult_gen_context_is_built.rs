// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/ecmult_gen_context_is_built.rs ]
crate::ix!();

pub fn ecmult_gen_context_is_built(ctx: *const EcMultGenContext) -> i32 {
    
    unsafe { (!(*ctx).prec().is_null()) as i32 }
}
