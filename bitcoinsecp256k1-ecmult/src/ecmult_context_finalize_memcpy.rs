// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_context_finalize_memcpy.rs ]
crate::ix!();

pub fn ecmult_context_finalize_memcpy(
    dst: *mut EcMultContext,
    src: *const EcMultContext,
) {
    tracing::trace!(target: "secp256k1::ecmult", "ecmult_context_finalize_memcpy");

    unsafe {
        if !(*src).pre_g().is_null() {
            /* We cast to c_void* first to suppress a -Wcast-align warning. */
            let dst_u8 = dst as *mut u8;
            let src_u8 = src as *const u8;
            let off =
                ((*(*src).pre_g() as *const u8 as usize).wrapping_sub(src_u8 as usize));
            (*dst).set_pre_g(dst_u8.add(off) as *mut GeStorage);
        }
        if !(*src).pre_g_128().is_null() {
            let dst_u8 = dst as *mut u8;
            let src_u8 = src as *const u8;
            let off = ((*(*src).pre_g_128() as *const u8 as usize)
                .wrapping_sub(src_u8 as usize));
            (*dst).set_pre_g_128(dst_u8.add(off) as *mut GeStorage);
        }
    }
}
