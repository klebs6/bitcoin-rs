// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/ecmult_gen_context_clear.rs ]
crate::ix!();

pub fn ecmult_gen_context_clear(ctx: *mut EcMultGenContext)  {
    
    unsafe {
        scalar_clear(core::ptr::addr_of_mut!((*ctx).blind()));
        gej_clear(core::ptr::addr_of_mut!((*ctx).initial()));
        (*ctx).set_prec(null_mut());
    }
}
