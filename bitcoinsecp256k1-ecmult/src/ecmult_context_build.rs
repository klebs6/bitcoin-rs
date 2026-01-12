// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_context_build.rs ]
crate::ix!();

pub fn ecmult_context_build(
    ctx:      *mut EcMultContext,
    prealloc: *mut *mut c_void,
) {
    tracing::trace!(target: "secp256k1::ecmult", "ecmult_context_build");

    unsafe {
        let base: *mut c_void = *prealloc;
        let prealloc_size: usize = *ECMULT_CONTEXT_PREALLOCATED_SIZE;

        if !(*ctx).pre_g().is_null() {
            return;
        }

        /* get the generator */
        let mut gj = Gej::new();
        gej_set_ge(core::ptr::addr_of_mut!(gj), core::ptr::addr_of!(ge_const_g));

        {
            let size: usize = core::mem::size_of::<GeStorage>()
                .wrapping_mul(ecmult_table_size!(WINDOW_G));
            /* check for overflow */
            verify_check!(
                size / core::mem::size_of::<GeStorage>() == ecmult_table_size!(WINDOW_G)
            );
            (*ctx).set_pre_g(manual_alloc(
                prealloc,
                core::mem::size_of::<GeStorage>() * ecmult_table_size!(WINDOW_G),
                base,
                prealloc_size,
            ) as *mut GeStorage);
        }

        /* precompute the tables with odd multiples */
        ecmult_odd_multiples_table_storage_var(
            ecmult_table_size!(WINDOW_G) as i32,
            *(*ctx).pre_g(),
            core::ptr::addr_of!(gj),
        );

        {
            let mut g_128j: Gej;
            let mut i: i32;

            let size: usize = core::mem::size_of::<GeStorage>()
                .wrapping_mul(ecmult_table_size!(WINDOW_G));
            /* check for overflow */
            verify_check!(
                size / core::mem::size_of::<GeStorage>() == ecmult_table_size!(WINDOW_G)
            );
            (*ctx).set_pre_g_128(manual_alloc(
                prealloc,
                core::mem::size_of::<GeStorage>() * ecmult_table_size!(WINDOW_G),
                base,
                prealloc_size,
            ) as *mut GeStorage);

            /* calculate 2^128*generator */
            g_128j = gj;
            i = 0;
            while i < 128 {
                gej_double_var(
                    core::ptr::addr_of_mut!(g_128j),
                    core::ptr::addr_of!(g_128j),
                    core::ptr::null_mut(),
                );
                i += 1;
            }
            ecmult_odd_multiples_table_storage_var(
                ecmult_table_size!(WINDOW_G) as i32,
                *(*ctx).pre_g_128(),
                core::ptr::addr_of!(g_128j),
            );
        }
    }
}

#[cfg(test)]
mod ecmult_context_build_contract_suite {
    use super::*;

    use crate::ecmult_test_harness::*;

    #[traced_test]
    fn ecmult_context_build_initializes_tables_and_advances_cursor() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "ecmult_context_build_initializes_tables_and_advances_cursor"
        );

        unsafe {
            let (buf, layout, ctx, cursor_end, ctx_offset) = alloc_and_build_ecmult_context_preallocated();

            let pre_g = *(*ctx).pre_g();
            let pre_g_128 = *(*ctx).pre_g_128();

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                pre_g_is_null = pre_g.is_null(),
                pre_g_128_is_null = pre_g_128.is_null(),
                "built context pointers"
            );

            assert!(!pre_g.is_null());
            assert!(!pre_g_128.is_null());

            let expected_end = buf.add(ctx_offset + *ECMULT_CONTEXT_PREALLOCATED_SIZE) as *mut c_void;
            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                cursor_end = cursor_end as usize,
                expected_end = expected_end as usize,
                "prealloc cursor end"
            );
            assert_eq!(cursor_end, expected_end);

            let table_size = ecmult_table_size!(WINDOW_G);
            let mut i = 0usize;
            while i < table_size {
                let mut ge = Ge::new();
                ge_from_storage(core::ptr::addr_of_mut!(ge), pre_g.add(i));
                assert_eq!(ge_is_infinity(core::ptr::addr_of!(ge)), 0);

                ge_from_storage(core::ptr::addr_of_mut!(ge), pre_g_128.add(i));
                assert_eq!(ge_is_infinity(core::ptr::addr_of!(ge)), 0);

                i += 1;
            }

            dealloc_aligned(buf, layout);
        }
    }

    #[traced_test]
    fn ecmult_context_build_is_idempotent_when_already_built() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "ecmult_context_build_is_idempotent_when_already_built"
        );

        unsafe {
            let (buf, layout, ctx, cursor_end, ctx_offset) = alloc_and_build_ecmult_context_preallocated();

            let before_pre_g = *(*ctx).pre_g();
            let before_pre_g_128 = *(*ctx).pre_g_128();
            let mut cursor = cursor_end;
            let cursor_ptr: *mut *mut c_void = core::ptr::addr_of_mut!(cursor);

            ecmult_context_build(ctx, cursor_ptr);

            let after_pre_g = *(*ctx).pre_g();
            let after_pre_g_128 = *(*ctx).pre_g_128();

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                before_pre_g = before_pre_g as usize,
                after_pre_g = after_pre_g as usize,
                before_pre_g_128 = before_pre_g_128 as usize,
                after_pre_g_128 = after_pre_g_128 as usize,
                "idempotence pointer check"
            );

            assert_eq!(before_pre_g, after_pre_g);
            assert_eq!(before_pre_g_128, after_pre_g_128);

            let expected_end = buf.add(ctx_offset + *ECMULT_CONTEXT_PREALLOCATED_SIZE) as *mut c_void;
            assert_eq!(cursor, expected_end);

            dealloc_aligned(buf, layout);
        }
    }
}
