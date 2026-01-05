// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_context_build.rs ]
crate::ix!();

pub fn ecmult_context_build(
        ctx:      *mut EcMultContext,
        prealloc: *mut *mut c_void)  {
    
    todo!();
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
