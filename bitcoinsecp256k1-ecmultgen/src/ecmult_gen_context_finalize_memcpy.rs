// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/ecmult_gen_context_finalize_memcpy.rs ]
crate::ix!();

pub fn ecmult_gen_context_finalize_memcpy(
        dst: *mut EcMultGenContext,
        src: *const EcMultGenContext)  {
    
    #[cfg(not(USE_ECMULT_STATIC_PRECOMPUTATION))]
    unsafe {
        if !(*src).prec().is_null() {
            /* We cast to c_void* first to suppress a -Wcast-align warning. */
            let src_base: usize = src as usize;
            let dst_base: usize = dst as usize;
            let src_prec: usize = (*src).prec() as usize;
            let offset: usize = src_prec.wrapping_sub(src_base);
            (*dst).set_prec((dst_base.wrapping_add(offset)) as *mut EcMultGenContextPrec);
        }
    }

    #[cfg(USE_ECMULT_STATIC_PRECOMPUTATION)]
    {
        let _ = dst;
        let _ = src;
    }
}
