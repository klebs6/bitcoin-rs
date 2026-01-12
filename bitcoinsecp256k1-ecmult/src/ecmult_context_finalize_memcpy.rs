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
                (*(*src).pre_g() as *const u8 as usize).wrapping_sub(src_u8 as usize);
            (*dst).set_pre_g(dst_u8.add(off) as *mut GeStorage);
        }
        if !(*src).pre_g_128().is_null() {
            let dst_u8 = dst as *mut u8;
            let src_u8 = src as *const u8;
            let off = (*(*src).pre_g_128() as *const u8 as usize).wrapping_sub(src_u8 as usize);
            (*dst).set_pre_g_128(dst_u8.add(off) as *mut GeStorage);
        }
    }
}

#[cfg(test)]
mod ecmult_context_finalize_memcpy_contract_suite {
    use super::*;

    use crate::ecmult_test_harness::*;

    #[traced_test]
    fn ecmult_context_finalize_memcpy_rebases_internal_pointers_after_memcpy() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "ecmult_context_finalize_memcpy_rebases_internal_pointers_after_memcpy"
        );

        unsafe {
            let (src_buf, src_layout, src_ctx, _src_cursor_end, src_ctx_offset) =
                alloc_and_build_ecmult_context_preallocated();

            let total_size = src_ctx_offset + *ECMULT_CONTEXT_PREALLOCATED_SIZE;
            let align = src_layout.align();

            let (dst_buf, dst_layout) = alloc_zeroed_aligned(total_size, align);
            core::ptr::copy_nonoverlapping(src_buf, dst_buf, total_size);

            let dst_ctx = dst_buf as *mut EcMultContext;

            let src_u8 = src_ctx as *const u8;
            let dst_u8 = dst_ctx as *mut u8;

            let src_pre_g = *(*src_ctx).pre_g() as *const u8;
            let src_pre_g_128 = *(*src_ctx).pre_g_128() as *const u8;

            let off_pre_g = (src_pre_g as usize).wrapping_sub(src_u8 as usize);
            let off_pre_g_128 = (src_pre_g_128 as usize).wrapping_sub(src_u8 as usize);

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                off_pre_g = off_pre_g,
                off_pre_g_128 = off_pre_g_128,
                "computed src offsets"
            );

            ecmult_context_finalize_memcpy(dst_ctx, src_ctx as *const EcMultContext);

            let dst_pre_g = *(*dst_ctx).pre_g() as *const u8;
            let dst_pre_g_128 = *(*dst_ctx).pre_g_128() as *const u8;

            let expected_dst_pre_g = dst_u8.add(off_pre_g) as *const u8;
            let expected_dst_pre_g_128 = dst_u8.add(off_pre_g_128) as *const u8;

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                dst_pre_g = dst_pre_g as usize,
                expected_dst_pre_g = expected_dst_pre_g as usize,
                dst_pre_g_128 = dst_pre_g_128 as usize,
                expected_dst_pre_g_128 = expected_dst_pre_g_128 as usize,
                "rebased pointers"
            );

            assert_eq!(dst_pre_g, expected_dst_pre_g);
            assert_eq!(dst_pre_g_128, expected_dst_pre_g_128);

            dealloc_aligned(src_buf, src_layout);
            dealloc_aligned(dst_buf, dst_layout);
        }
    }
}
