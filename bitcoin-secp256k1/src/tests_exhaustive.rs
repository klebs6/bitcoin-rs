// ---------------- [ File: bitcoin-secp256k1/src/tests_exhaustive.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/tests_exhaustive.c]

pub const USE_ECMULT_STATIC_PRECOMPUTATION: bool = false;

/**
  | see group_impl.h for allowable values
  |
  */
#[cfg(not(EXHAUSTIVE_TEST_ORDER))]
pub const EXHAUSTIVE_TEST_ORDER: usize = 13;

lazy_static!{
    /*
    static int count = 2;
    */
}

/**
  | stolen from tests.c
  |
  */
pub fn ge_equals_ge(
        a: *const Ge,
        b: *const Ge)  {
    
    todo!();
        /*
            CHECK(a->infinity == b->infinity);
        if (a->infinity) {
            return;
        }
        CHECK(fe_equal_var(&a->x, &b->x));
        CHECK(fe_equal_var(&a->y, &b->y));
        */
}

pub fn ge_equals_gej(
        a: *const Ge,
        b: *const Gej)  {
    
    todo!();
        /*
            fe z2s;
        fe u1, u2, s1, s2;
        CHECK(a->infinity == b->infinity);
        if (a->infinity) {
            return;
        }
        /* Check a.x * b.z^2 == b.x && a.y * b.z^3 == b.y, to avoid inverses. */
        fe_sqr(&z2s, &b->z);
        fe_mul(&u1, &a->x, &z2s);
        u2 = b->x; fe_normalize_weak(&u2);
        fe_mul(&s1, &a->y, &z2s); fe_mul(&s1, &s1, &b->z);
        s2 = b->y; fe_normalize_weak(&s2);
        CHECK(fe_equal_var(&u1, &u2));
        CHECK(fe_equal_var(&s1, &s2));
        */
}

pub fn random_fe(x: *mut Fe)  {
    
    todo!();
        /*
            unsigned char bin[32];
        do {
            testrand256(bin);
            if (fe_set_b32(x, bin)) {
                return;
            }
        } while(1);
        */
}

/** END stolen from tests.c */

lazy_static!{
    /*
    static uint32_t num_cores = 1;
    static uint32_t this_core = 0;
    */
}

#[inline] pub fn skip_section(iter: *mut u64) -> i32 {
    
    todo!();
        /*
            if (num_cores == 1) return 0;
        *iter += 0xe7037ed1a0b428dbULL;
        return ((((uint32_t)*iter ^ (*iter >> 32)) * num_cores) >> 32) != this_core;
        */
}

pub fn nonce_function_smallint(
        nonce32: *mut u8,
        msg32:   *const u8,
        key32:   *const u8,
        algo16:  *const u8,
        data:    *mut c_void,
        attempt: u32) -> i32 {
    
    todo!();
        /*
            scalar s;
        int *idata = data;
        (c_void)msg32;
        (c_void)key32;
        (c_void)algo16;
        /* Some nonces cannot be used because they'd cause s and/or r to be zero.
         * The signing function has retry logic here that just re-calls the nonce
         * function with an increased `attempt`. So if attempt > 0 this means we
         * need to change the nonce to avoid an infinite loop. */
        if (attempt > 0) {
            *idata = (*idata + 1) % EXHAUSTIVE_TEST_ORDER;
        }
        scalar_set_int(&s, *idata);
        scalar_get_b32(nonce32, &s);
        return 1;
        */
}

pub fn test_exhaustive_endomorphism(group: *const Ge)  {
    
    todo!();
        /*
            int i;
        for (i = 0; i < EXHAUSTIVE_TEST_ORDER; i++) {
            ge res;
            ge_mul_lambda(&res, &group[i]);
            ge_equals_ge(&group[i * EXHAUSTIVE_TEST_LAMBDA % EXHAUSTIVE_TEST_ORDER], &res);
        }
        */
}

pub fn test_exhaustive_addition(
        group:  *const Ge,
        groupj: *const Gej)  {
    
    todo!();
        /*
            int i, j;
        uint64_t iter = 0;

        /* Sanity-check (and check infinity functions) */
        CHECK(ge_is_infinity(&group[0]));
        CHECK(gej_is_infinity(&groupj[0]));
        for (i = 1; i < EXHAUSTIVE_TEST_ORDER; i++) {
            CHECK(!ge_is_infinity(&group[i]));
            CHECK(!gej_is_infinity(&groupj[i]));
        }

        /* Check all addition formulae */
        for (j = 0; j < EXHAUSTIVE_TEST_ORDER; j++) {
            fe fe_inv;
            if (skip_section(&iter)) continue;
            fe_inv(&fe_inv, &groupj[j].z);
            for (i = 0; i < EXHAUSTIVE_TEST_ORDER; i++) {
                ge zless_gej;
                gej tmp;
                /* add_var */
                gej_add_var(&tmp, &groupj[i], &groupj[j], NULL);
                ge_equals_gej(&group[(i + j) % EXHAUSTIVE_TEST_ORDER], &tmp);
                /* add_ge */
                if (j > 0) {
                    gej_add_ge(&tmp, &groupj[i], &group[j]);
                    ge_equals_gej(&group[(i + j) % EXHAUSTIVE_TEST_ORDER], &tmp);
                }
                /* add_ge_var */
                gej_add_ge_var(&tmp, &groupj[i], &group[j], NULL);
                ge_equals_gej(&group[(i + j) % EXHAUSTIVE_TEST_ORDER], &tmp);
                /* add_zinv_var */
                zless_gej.infinity = groupj[j].infinity;
                zless_gej.x = groupj[j].x;
                zless_gej.y = groupj[j].y;
                gej_add_zinv_var(&tmp, &groupj[i], &zless_gej, &fe_inv);
                ge_equals_gej(&group[(i + j) % EXHAUSTIVE_TEST_ORDER], &tmp);
            }
        }

        /* Check doubling */
        for (i = 0; i < EXHAUSTIVE_TEST_ORDER; i++) {
            gej tmp;
            gej_double(&tmp, &groupj[i]);
            ge_equals_gej(&group[(2 * i) % EXHAUSTIVE_TEST_ORDER], &tmp);
            gej_double_var(&tmp, &groupj[i], NULL);
            ge_equals_gej(&group[(2 * i) % EXHAUSTIVE_TEST_ORDER], &tmp);
        }

        /* Check negation */
        for (i = 1; i < EXHAUSTIVE_TEST_ORDER; i++) {
            ge tmp;
            gej tmpj;
            ge_neg(&tmp, &group[i]);
            ge_equals_ge(&group[EXHAUSTIVE_TEST_ORDER - i], &tmp);
            gej_neg(&tmpj, &groupj[i]);
            ge_equals_gej(&group[EXHAUSTIVE_TEST_ORDER - i], &tmpj);
        }
        */
}

pub fn test_exhaustive_ecmult(
        ctx:    *const Secp256k1Context,
        group:  *const Ge,
        groupj: *const Gej)  {
    
    todo!();
        /*
            int i, j, r_log;
        uint64_t iter = 0;
        for (r_log = 1; r_log < EXHAUSTIVE_TEST_ORDER; r_log++) {
            for (j = 0; j < EXHAUSTIVE_TEST_ORDER; j++) {
                if (skip_section(&iter)) continue;
                for (i = 0; i < EXHAUSTIVE_TEST_ORDER; i++) {
                    gej tmp;
                    scalar na, ng;
                    scalar_set_int(&na, i);
                    scalar_set_int(&ng, j);

                    ecmult(&ctx->ecmult_ctx, &tmp, &groupj[r_log], &na, &ng);
                    ge_equals_gej(&group[(i * r_log + j) % EXHAUSTIVE_TEST_ORDER], &tmp);

                    if (i > 0) {
                        ecmult_const(&tmp, &group[i], &ng, 256);
                        ge_equals_gej(&group[(i * j) % EXHAUSTIVE_TEST_ORDER], &tmp);
                    }
                }
            }
        }
        */
}

pub struct EcMultMultiData {
    sc: [Scalar; 2],
    pt: [Ge; 2],
}

pub fn ecmult_multi_callback(
        sc:     *mut Scalar,
        pt:     *mut Ge,
        idx:    usize,
        cbdata: *mut c_void) -> i32 {
    
    todo!();
        /*
            ecmult_multi_data *data = (ecmult_multi_data*) cbdata;
        *sc = data->sc[idx];
        *pt = data->pt[idx];
        return 1;
        */
}

pub fn test_exhaustive_ecmult_multi(
        ctx:   *const Secp256k1Context,
        group: *const Ge)  {
    
    todo!();
        /*
            int i, j, k, x, y;
        uint64_t iter = 0;
        scratch *scratch = scratch_create(&ctx->error_callback, 4096);
        for (i = 0; i < EXHAUSTIVE_TEST_ORDER; i++) {
            for (j = 0; j < EXHAUSTIVE_TEST_ORDER; j++) {
                for (k = 0; k < EXHAUSTIVE_TEST_ORDER; k++) {
                    for (x = 0; x < EXHAUSTIVE_TEST_ORDER; x++) {
                        if (skip_section(&iter)) continue;
                        for (y = 0; y < EXHAUSTIVE_TEST_ORDER; y++) {
                            gej tmp;
                            scalar g_sc;
                            ecmult_multi_data data;

                            scalar_set_int(&data.sc[0], i);
                            scalar_set_int(&data.sc[1], j);
                            scalar_set_int(&g_sc, k);
                            data.pt[0] = group[x];
                            data.pt[1] = group[y];

                            ecmult_multi_var(&ctx->error_callback, &ctx->ecmult_ctx, scratch, &tmp, &g_sc, ecmult_multi_callback, &data, 2);
                            ge_equals_gej(&group[(i * x + j * y + k) % EXHAUSTIVE_TEST_ORDER], &tmp);
                        }
                    }
                }
            }
        }
        scratch_destroy(&ctx->error_callback, scratch);
        */
}

pub fn r_from_k(
        r:        *mut Scalar,
        group:    *const Ge,
        k:        i32,
        overflow: *mut i32)  {
    
    todo!();
        /*
            fe x;
        unsigned char x_bin[32];
        k %= EXHAUSTIVE_TEST_ORDER;
        x = group[k].x;
        fe_normalize(&x);
        fe_get_b32(x_bin, &x);
        scalar_set_b32(r, x_bin, overflow);
        */
}

pub fn test_exhaustive_verify(
        ctx:   *const Secp256k1Context,
        group: *const Ge)  {
    
    todo!();
        /*
            int s, r, msg, key;
        uint64_t iter = 0;
        for (s = 1; s < EXHAUSTIVE_TEST_ORDER; s++) {
            for (r = 1; r < EXHAUSTIVE_TEST_ORDER; r++) {
                for (msg = 1; msg < EXHAUSTIVE_TEST_ORDER; msg++) {
                    for (key = 1; key < EXHAUSTIVE_TEST_ORDER; key++) {
                        ge nonconst_ge;
                        ecdsa_signature sig;
                        pubkey pk;
                        scalar sk_s, msg_s, r_s, s_s;
                        scalar s_times_k_s, msg_plus_r_times_sk_s;
                        int k, should_verify;
                        unsigned char msg32[32];

                        if (skip_section(&iter)) continue;

                        scalar_set_int(&s_s, s);
                        scalar_set_int(&r_s, r);
                        scalar_set_int(&msg_s, msg);
                        scalar_set_int(&sk_s, key);

                        /* Verify by hand */
                        /* Run through every k value that gives us this r and check that *one* works.
                         * Note there could be none, there could be multiple, ECDSA is weird. */
                        should_verify = 0;
                        for (k = 0; k < EXHAUSTIVE_TEST_ORDER; k++) {
                            scalar check_x_s;
                            r_from_k(&check_x_s, group, k, NULL);
                            if (r_s == check_x_s) {
                                scalar_set_int(&s_times_k_s, k);
                                scalar_mul(&s_times_k_s, &s_times_k_s, &s_s);
                                scalar_mul(&msg_plus_r_times_sk_s, &r_s, &sk_s);
                                scalar_add(&msg_plus_r_times_sk_s, &msg_plus_r_times_sk_s, &msg_s);
                                should_verify |= scalar_eq(&s_times_k_s, &msg_plus_r_times_sk_s);
                            }
                        }
                        /* nb we have a "high s" rule */
                        should_verify &= !scalar_is_high(&s_s);

                        /* Verify by calling verify */
                        ecdsa_signature_save(&sig, &r_s, &s_s);
                        memcpy(&nonconst_ge, &group[sk_s], sizeof(nonconst_ge));
                        pubkey_save(&pk, &nonconst_ge);
                        scalar_get_b32(msg32, &msg_s);
                        CHECK(should_verify ==
                              ecdsa_verify(ctx, &sig, msg32, &pk));
                    }
                }
            }
        }
        */
}

pub fn test_exhaustive_sign(
        ctx:   *const Secp256k1Context,
        group: *const Ge)  {
    
    todo!();
        /*
            int i, j, k;
        uint64_t iter = 0;

        /* Loop */
        for (i = 1; i < EXHAUSTIVE_TEST_ORDER; i++) {  /* message */
            for (j = 1; j < EXHAUSTIVE_TEST_ORDER; j++) {  /* key */
                if (skip_section(&iter)) continue;
                for (k = 1; k < EXHAUSTIVE_TEST_ORDER; k++) {  /* nonce */
                    const int starting_k = k;
                    int ret;
                    ecdsa_signature sig;
                    scalar sk, msg, r, s, expected_r;
                    unsigned char sk32[32], msg32[32];
                    scalar_set_int(&msg, i);
                    scalar_set_int(&sk, j);
                    scalar_get_b32(sk32, &sk);
                    scalar_get_b32(msg32, &msg);

                    ret = ecdsa_sign(ctx, &sig, msg32, sk32, nonce_function_smallint, &k);
                    CHECK(ret == 1);

                    ecdsa_signature_load(ctx, &r, &s, &sig);
                    /* Note that we compute expected_r *after* signing -- this is important
                     * because our nonce-computing function function might change k during
                     * signing. */
                    r_from_k(&expected_r, group, k, NULL);
                    CHECK(r == expected_r);
                    CHECK((k * s) % EXHAUSTIVE_TEST_ORDER == (i + r * j) % EXHAUSTIVE_TEST_ORDER ||
                          (k * (EXHAUSTIVE_TEST_ORDER - s)) % EXHAUSTIVE_TEST_ORDER == (i + r * j) % EXHAUSTIVE_TEST_ORDER);

                    /* Overflow means we've tried every possible nonce */
                    if (k < starting_k) {
                        break;
                    }
                }
            }
        }

        /* We would like to verify zero-knowledge here by counting how often every
         * possible (s, r) tuple appears, but because the group order is larger
         * than the field order, when coercing the x-values to scalar values, some
         * appear more often than others, so we are actually not zero-knowledge.
         * (This effect also appears in the real code, but the difference is on the
         * order of 1/2^128th the field order, so the deviation is not useful to a
         * computationally bounded attacker.)
         */
        */
}

lazy_static!{
    /*
    #ifdef ENABLE_MODULE_RECOVERY
    #include "src/modules/recovery/tests_exhaustive_impl.h"
    #endif

    #ifdef ENABLE_MODULE_EXTRAKEYS
    #include "src/modules/extrakeys/tests_exhaustive_impl.h"
    #endif

    #ifdef ENABLE_MODULE_SCHNORRSIG
    #include "src/modules/schnorrsig/tests_exhaustive_impl.h"
    #endif
    */
}

pub fn secp256k1_tests_exhaustive_main(
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            int i;
        gej groupj[EXHAUSTIVE_TEST_ORDER];
        ge group[EXHAUSTIVE_TEST_ORDER];
        unsigned char rand32[32];
        context *ctx;

        /* Disable buffering for stdout to improve reliability of getting
         * diagnostic information. Happens right at the start of main because
         * setbuf must be used before any other operation on the stream. */
        setbuf(stdout, NULL);
        /* Also disable buffering for stderr because it's not guaranteed that it's
         * unbuffered on all systems. */
        setbuf(stderr, NULL);

        printf("Exhaustive tests for order %lu\n", (unsigned long)EXHAUSTIVE_TEST_ORDER);

        /* find iteration count */
        if (argc > 1) {
            count = strtol(argv[1], NULL, 0);
        }
        printf("test count = %i\n", count);

        /* find random seed */
        testrand_init(argc > 2 ? argv[2] : NULL);

        /* set up split processing */
        if (argc > 4) {
            num_cores = strtol(argv[3], NULL, 0);
            this_core = strtol(argv[4], NULL, 0);
            if (num_cores < 1 || this_core >= num_cores) {
                fprintf(stderr, "Usage: %s [count] [seed] [numcores] [thiscore]\n", argv[0]);
                return 1;
            }
            printf("running tests for core %lu (out of [0..%lu])\n", (unsigned long)this_core, (unsigned long)num_cores - 1);
        }

        while (count--) {
            /* Build context */
            ctx = context_create(CONTEXT_SIGN | CONTEXT_VERIFY);
            testrand256(rand32);
            CHECK(context_randomize(ctx, rand32));

            /* Generate the entire group */
            gej_set_infinity(&groupj[0]);
            ge_set_gej(&group[0], &groupj[0]);
            for (i = 1; i < EXHAUSTIVE_TEST_ORDER; i++) {
                gej_add_ge(&groupj[i], &groupj[i - 1], &ge_const_g);
                ge_set_gej(&group[i], &groupj[i]);
                if (count != 0) {
                    /* Set a different random z-value for each Jacobian point, except z=1
                       is used in the last iteration. */
                    fe z;
                    random_fe(&z);
                    gej_rescale(&groupj[i], &z);
                }

                /* Verify against ecmult_gen */
                {
                    scalar scalar_i;
                    gej generatedj;
                    ge generated;

                    scalar_set_int(&scalar_i, i);
                    ecmult_gen(&ctx->ecmult_gen_ctx, &generatedj, &scalar_i);
                    ge_set_gej(&generated, &generatedj);

                    CHECK(group[i].infinity == 0);
                    CHECK(generated.infinity == 0);
                    CHECK(fe_equal_var(&generated.x, &group[i].x));
                    CHECK(fe_equal_var(&generated.y, &group[i].y));
                }
            }

            /* Run the tests */
            test_exhaustive_endomorphism(group);
            test_exhaustive_addition(group, groupj);
            test_exhaustive_ecmult(ctx, group, groupj);
            test_exhaustive_ecmult_multi(ctx, group);
            test_exhaustive_sign(ctx, group);
            test_exhaustive_verify(ctx, group);

    #ifdef ENABLE_MODULE_RECOVERY
            test_exhaustive_recovery(ctx, group);
    #endif
    #ifdef ENABLE_MODULE_EXTRAKEYS
            test_exhaustive_extrakeys(ctx, group);
    #endif
    #ifdef ENABLE_MODULE_SCHNORRSIG
            test_exhaustive_schnorrsig(ctx);
    #endif

            context_destroy(ctx);
        }

        testrand_finish();

        printf("no problems found\n");
        return 0;
        */
}
