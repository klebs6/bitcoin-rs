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

#[cfg(test)]
mod ecmult_gen_context_finalize_memcpy_behavior_suite {
    use super::*;

    fn align_up(value: usize, align: usize) -> usize {
        (value.wrapping_add(align - 1)) & !(align - 1)
    }

    fn align_ptr(p: *mut u8, align: usize) -> *mut u8 {
        let aligned = (p as usize).wrapping_add(align - 1) & !(align - 1);
        aligned as *mut u8
    }

    unsafe fn gej_points_are_equal(a: *const Gej, b: *const Gej) -> bool {
        let mut neg_b: Gej = Gej::new();
        gej_neg(&mut neg_b, b);

        let mut diff: Gej = Gej::new();
        gej_add_var(&mut diff, a, (&neg_b) as *const Gej, core::ptr::null_mut());

        gej_is_infinity((&diff) as *const Gej) != 0
    }

    #[cfg(not(USE_ECMULT_STATIC_PRECOMPUTATION))]
    #[traced_test]
    fn ecmult_gen_context_finalize_memcpy_adjusts_prec_pointer_on_relocation() {
        unsafe {
            let align = core::mem::align_of::<EcMultGenContextPrec>()
                .max(core::mem::align_of::<EcMultGenContext>())
                .max(16);

            let ctx_size = core::mem::size_of::<EcMultGenContext>();
            let prec_offset = align_up(ctx_size, align);
            let total_size = prec_offset
                .checked_add(ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE)
                .unwrap();

            tracing::info!(
                align,
                ctx_size,
                prec_offset,
                total_size,
                "allocating contiguous src/dst blocks for finalize_memcpy test"
            );

            let mut src_mem: Vec<u8> = vec![0u8; total_size.saturating_add(align)];
            let src_base = align_ptr(src_mem.as_mut_ptr(), align);
            let src_ctx_ptr = src_base as *mut EcMultGenContext;
            let src_prec_base = src_base.add(prec_offset) as *mut c_void;

            core::ptr::write(src_ctx_ptr, EcMultGenContext::new());
            ecmult_gen_context_init(src_ctx_ptr);

            let mut prealloc_cursor: *mut c_void = src_prec_base;
            ecmult_gen_context_build(src_ctx_ptr, core::ptr::addr_of_mut!(prealloc_cursor));

            let src_prec_ptr = (*src_ctx_ptr).prec();
            assert!(!src_prec_ptr.is_null());

            let expected_src_prec = (src_base as usize).wrapping_add(prec_offset) as *mut EcMultGenContextPrec;
            assert_eq!(src_prec_ptr, expected_src_prec);

            let mut dst_mem: Vec<u8> = vec![0u8; total_size.saturating_add(align)];
            let dst_base = align_ptr(dst_mem.as_mut_ptr(), align);

            core::ptr::copy_nonoverlapping(src_base, dst_base, total_size);

            let dst_ctx_ptr = dst_base as *mut EcMultGenContext;

            tracing::debug!(
                src_ctx = (src_ctx_ptr as usize),
                src_prec = (src_prec_ptr as usize),
                dst_ctx = (dst_ctx_ptr as usize),
                "copied context+prec block; corrupting src prec to ensure dst must be fixed"
            );

            core::ptr::write_bytes(src_prec_ptr as *mut u8, 0u8, ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE);

            ecmult_gen_context_finalize_memcpy(dst_ctx_ptr, src_ctx_ptr);

            let src_base_usize = src_ctx_ptr as usize;
            let dst_base_usize = dst_ctx_ptr as usize;
            let src_prec_usize = src_prec_ptr as usize;
            let offset = src_prec_usize.wrapping_sub(src_base_usize);

            let expected_dst_prec = (dst_base_usize.wrapping_add(offset)) as *mut EcMultGenContextPrec;
            let actual_dst_prec = (*dst_ctx_ptr).prec();

            tracing::info!(
                expected_dst_prec = (expected_dst_prec as usize),
                actual_dst_prec = (actual_dst_prec as usize),
                "checking finalize_memcpy pointer adjustment"
            );

            assert_eq!(actual_dst_prec, expected_dst_prec);

            let mut one: Scalar = Scalar::new();
            scalar_set_int(&mut one, 1);

            let mut r: Gej = Gej::new();
            ecmult_gen(dst_ctx_ptr as *const EcMultGenContext, &mut r, (&one) as *const Scalar);

            let mut expected_g: Gej = Gej::new();
            gej_set_ge(&mut expected_g, &ge_const_g);

            tracing::debug!("verifying dst context remains functional after finalize_memcpy");
            assert!(gej_points_are_equal((&r) as *const Gej, (&expected_g) as *const Gej));
        }
    }

    #[traced_test]
    fn ecmult_gen_context_finalize_memcpy_is_noop_when_src_prec_is_null() {
        unsafe {
            let mut src = EcMultGenContext::new();
            ecmult_gen_context_init(&mut src);

            let mut dst = EcMultGenContext::new();
            ecmult_gen_context_init(&mut dst);

            let sentinel: *mut EcMultGenContextPrec = 7usize as *mut EcMultGenContextPrec;
            dst.set_prec(sentinel);

            ecmult_gen_context_finalize_memcpy(&mut dst, (&src) as *const EcMultGenContext);

            tracing::info!(
                src_prec_is_null = src.prec().is_null(),
                dst_prec = (dst.prec() as usize),
                "finalize_memcpy with null src.prec should not modify dst"
            );

            assert!(src.prec().is_null());
            assert_eq!(dst.prec(), sentinel);
        }
    }

    #[cfg(USE_ECMULT_STATIC_PRECOMPUTATION)]
    #[traced_test]
    fn ecmult_gen_context_finalize_memcpy_is_noop_under_static_precomputation() {
        unsafe {
            let mut src = EcMultGenContext::new();
            ecmult_gen_context_init(&mut src);

            let mut dst = EcMultGenContext::new();
            ecmult_gen_context_init(&mut dst);

            let sentinel: *mut EcMultGenContextPrec = 9usize as *mut EcMultGenContextPrec;
            dst.set_prec(sentinel);

            ecmult_gen_context_finalize_memcpy(&mut dst, (&src) as *const EcMultGenContext);

            assert_eq!(dst.prec(), sentinel);
        }
    }
}
