// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_context_finalize_memcpy.rs ]
crate::ix!();

pub fn ecmult_context_finalize_memcpy(
        dst: *mut EcMultContext,
        src: *const EcMultContext)  {
    
    todo!();
        /*
        if (src->pre_g != NULL) {
            /* We cast to c_void* first to suppress a -Wcast-align warning. */
            dst->pre_g = (ge_storage (*)[])(c_void*)((unsigned char*)dst + ((unsigned char*)(src->pre_g) - (unsigned char*)src));
        }
        if (src->pre_g_128 != NULL) {
            dst->pre_g_128 = (ge_storage (*)[])(c_void*)((unsigned char*)dst + ((unsigned char*)(src->pre_g_128) - (unsigned char*)src));
        }
        */
}
