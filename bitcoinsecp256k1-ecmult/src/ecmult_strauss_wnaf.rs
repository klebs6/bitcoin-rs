// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_strauss_wnaf.rs ]
crate::ix!();

pub fn ecmult_strauss_wnaf(
        ctx:   *const EcMultContext,
        state: *const StraussState,
        r:     *mut Gej,
        num:   usize,
        a:     *const Gej,
        na:    *const Scalar,
        ng:    *const Scalar)  {
    
    todo!();
        /*
        ge tmpa;
        fe Z;
        /* Splitted G factors. */
        scalar ng_1, ng_128;
        int wnaf_ng_1[129];
        int bits_ng_1 = 0;
        int wnaf_ng_128[129];
        int bits_ng_128 = 0;
        int i;
        int bits = 0;
        size_t np;
        size_t no = 0;

        for (np = 0; np < num; ++np) {
            if (scalar_is_zero(&na[np]) || gej_is_infinity(&a[np])) {
                continue;
            }
            state->ps[no].input_pos = np;
            /* split na into na_1 and na_lam (where na = na_1 + na_lam*lambda, and na_1 and na_lam are ~128 bit) */
            scalar_split_lambda(&state->ps[no].na_1, &state->ps[no].na_lam, &na[np]);

            /* build wnaf representation for na_1 and na_lam. */
            state->ps[no].bits_na_1   = ecmult_wnaf(state->ps[no].wnaf_na_1,   129, &state->ps[no].na_1,   WINDOW_A);
            state->ps[no].bits_na_lam = ecmult_wnaf(state->ps[no].wnaf_na_lam, 129, &state->ps[no].na_lam, WINDOW_A);
            VERIFY_CHECK(state->ps[no].bits_na_1 <= 129);
            VERIFY_CHECK(state->ps[no].bits_na_lam <= 129);
            if (state->ps[no].bits_na_1 > bits) {
                bits = state->ps[no].bits_na_1;
            }
            if (state->ps[no].bits_na_lam > bits) {
                bits = state->ps[no].bits_na_lam;
            }
            ++no;
        }

        /* Calculate odd multiples of a.
         * All multiples are brought to the same Z 'denominator', which is stored
         * in Z. Due to secp256k1' isomorphism we can do all operations pretending
         * that the Z coordinate was 1, use affine addition formulae, and correct
         * the Z coordinate of the result once at the end.
         * The exception is the precomputed G table points, which are actually
         * affine. Compared to the base used for other points, they have a Z ratio
         * of 1/Z, so we can use gej_add_zinv_var, which uses the same
         * isomorphism to efficiently add with a known Z inverse.
         */
        if (no > 0) {
            /* Compute the odd multiples in Jacobian form. */
            ecmult_odd_multiples_table(ECMULT_TABLE_SIZE(WINDOW_A), state->prej, state->zr, &a[state->ps[0].input_pos]);
            for (np = 1; np < no; ++np) {
                gej tmp = a[state->ps[np].input_pos];
    #ifdef VERIFY
                fe_normalize_var(&(state->prej[(np - 1) * ECMULT_TABLE_SIZE(WINDOW_A) + ECMULT_TABLE_SIZE(WINDOW_A) - 1].z));
    #endif
                gej_rescale(&tmp, &(state->prej[(np - 1) * ECMULT_TABLE_SIZE(WINDOW_A) + ECMULT_TABLE_SIZE(WINDOW_A) - 1].z));
                ecmult_odd_multiples_table(ECMULT_TABLE_SIZE(WINDOW_A), state->prej + np * ECMULT_TABLE_SIZE(WINDOW_A), state->zr + np * ECMULT_TABLE_SIZE(WINDOW_A), &tmp);
                fe_mul(state->zr + np * ECMULT_TABLE_SIZE(WINDOW_A), state->zr + np * ECMULT_TABLE_SIZE(WINDOW_A), &(a[state->ps[np].input_pos].z));
            }
            /* Bring them to the same Z denominator. */
            ge_globalz_set_table_gej(ECMULT_TABLE_SIZE(WINDOW_A) * no, state->pre_a, &Z, state->prej, state->zr);
        } else {
            fe_set_int(&Z, 1);
        }

        for (np = 0; np < no; ++np) {
            for (i = 0; i < ECMULT_TABLE_SIZE(WINDOW_A); i++) {
                ge_mul_lambda(&state->pre_a_lam[np * ECMULT_TABLE_SIZE(WINDOW_A) + i], &state->pre_a[np * ECMULT_TABLE_SIZE(WINDOW_A) + i]);
            }
        }

        if (ng) {
            /* split ng into ng_1 and ng_128 (where gn = gn_1 + gn_128*2^128, and gn_1 and gn_128 are ~128 bit) */
            scalar_split_128(&ng_1, &ng_128, ng);

            /* Build wnaf representation for ng_1 and ng_128 */
            bits_ng_1   = ecmult_wnaf(wnaf_ng_1,   129, &ng_1,   WINDOW_G);
            bits_ng_128 = ecmult_wnaf(wnaf_ng_128, 129, &ng_128, WINDOW_G);
            if (bits_ng_1 > bits) {
                bits = bits_ng_1;
            }
            if (bits_ng_128 > bits) {
                bits = bits_ng_128;
            }
        }

        gej_set_infinity(r);

        for (i = bits - 1; i >= 0; i--) {
            int n;
            gej_double_var(r, r, NULL);
            for (np = 0; np < no; ++np) {
                if (i < state->ps[np].bits_na_1 && (n = state->ps[np].wnaf_na_1[i])) {
                    ECMULT_TABLE_GET_GE(&tmpa, state->pre_a + np * ECMULT_TABLE_SIZE(WINDOW_A), n, WINDOW_A);
                    gej_add_ge_var(r, r, &tmpa, NULL);
                }
                if (i < state->ps[np].bits_na_lam && (n = state->ps[np].wnaf_na_lam[i])) {
                    ECMULT_TABLE_GET_GE(&tmpa, state->pre_a_lam + np * ECMULT_TABLE_SIZE(WINDOW_A), n, WINDOW_A);
                    gej_add_ge_var(r, r, &tmpa, NULL);
                }
            }
            if (i < bits_ng_1 && (n = wnaf_ng_1[i])) {
                ECMULT_TABLE_GET_GE_STORAGE(&tmpa, *ctx->pre_g, n, WINDOW_G);
                gej_add_zinv_var(r, r, &tmpa, &Z);
            }
            if (i < bits_ng_128 && (n = wnaf_ng_128[i])) {
                ECMULT_TABLE_GET_GE_STORAGE(&tmpa, *ctx->pre_g_128, n, WINDOW_G);
                gej_add_zinv_var(r, r, &tmpa, &Z);
            }
        }

        if (!r->infinity) {
            fe_mul(&r->z, &r->z, &Z);
        }
        */
}
