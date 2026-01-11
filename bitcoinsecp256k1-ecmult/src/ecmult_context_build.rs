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
            (*ctx).pre_g(),
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
                (*ctx).pre_g_128(),
                core::ptr::addr_of!(g_128j),
            );
        }
    }

        /*
            gej gj;
        c_void* const base = *prealloc;
        size_t const prealloc_size = ECMULT_CONTEXT_PREALLOCATED_SIZE;

        if (ctx->pre_g != NULL) {
            return;
        }

        /* get the generator */
        gej_set_ge(&gj, &ge_const_g);

        {
            size_t size = sizeof((*ctx->pre_g)[0]) * ((size_t)ECMULT_TABLE_SIZE(WINDOW_G));
            /* check for overflow */
            VERIFY_CHECK(size / sizeof((*ctx->pre_g)[0]) == ((size_t)ECMULT_TABLE_SIZE(WINDOW_G)));
            ctx->pre_g = (ge_storage (*)[])manual_alloc(prealloc, sizeof((*ctx->pre_g)[0]) * ECMULT_TABLE_SIZE(WINDOW_G), base, prealloc_size);
        }

        /* precompute the tables with odd multiples */
        ecmult_odd_multiples_table_storage_var(ECMULT_TABLE_SIZE(WINDOW_G), *ctx->pre_g, &gj);

        {
            gej g_128j;
            int i;

            size_t size = sizeof((*ctx->pre_g_128)[0]) * ((size_t) ECMULT_TABLE_SIZE(WINDOW_G));
            /* check for overflow */
            VERIFY_CHECK(size / sizeof((*ctx->pre_g_128)[0]) == ((size_t)ECMULT_TABLE_SIZE(WINDOW_G)));
            ctx->pre_g_128 = (ge_storage (*)[])manual_alloc(prealloc, sizeof((*ctx->pre_g_128)[0]) * ECMULT_TABLE_SIZE(WINDOW_G), base, prealloc_size);

            /* calculate 2^128*generator */
            g_128j = gj;
            for (i = 0; i < 128; i++) {
                gej_double_var(&g_128j, &g_128j, NULL);
            }
            ecmult_odd_multiples_table_storage_var(ECMULT_TABLE_SIZE(WINDOW_G), *ctx->pre_g_128, &g_128j);
        }
        */
}

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
