// ---------------- [ File: bitcoinsecp256k1-bench/src/bench_internal.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/bench_internal.c]

pub struct BenchInv {
    scalar: [Scalar; 2],
    fe:     [Fe; 4],
    ge:     [Ge; 2],
    gej:    [Gej; 2],
    data:   [u8; 64],
    wnaf:   [i32; 256],
}

pub fn bench_setup(arg: *mut c_void)  {
    
    todo!();
        /*
            bench_inv *data = (bench_inv*)arg;

        static const unsigned char init[4][32] = {
            /* Initializer for scalar[0], fe[0], first half of data, the X coordinate of ge[0],
               and the (implied affine) X coordinate of gej[0]. */
            {
                0x02, 0x03, 0x05, 0x07, 0x0b, 0x0d, 0x11, 0x13,
                0x17, 0x1d, 0x1f, 0x25, 0x29, 0x2b, 0x2f, 0x35,
                0x3b, 0x3d, 0x43, 0x47, 0x49, 0x4f, 0x53, 0x59,
                0x61, 0x65, 0x67, 0x6b, 0x6d, 0x71, 0x7f, 0x83
            },
            /* Initializer for scalar[1], fe[1], first half of data, the X coordinate of ge[1],
               and the (implied affine) X coordinate of gej[1]. */
            {
                0x82, 0x83, 0x85, 0x87, 0x8b, 0x8d, 0x81, 0x83,
                0x97, 0xad, 0xaf, 0xb5, 0xb9, 0xbb, 0xbf, 0xc5,
                0xdb, 0xdd, 0xe3, 0xe7, 0xe9, 0xef, 0xf3, 0xf9,
                0x11, 0x15, 0x17, 0x1b, 0x1d, 0xb1, 0xbf, 0xd3
            },
            /* Initializer for fe[2] and the Z coordinate of gej[0]. */
            {
                0x3d, 0x2d, 0xef, 0xf4, 0x25, 0x98, 0x4f, 0x5d,
                0xe2, 0xca, 0x5f, 0x41, 0x3f, 0x3f, 0xce, 0x44,
                0xaa, 0x2c, 0x53, 0x8a, 0xc6, 0x59, 0x1f, 0x38,
                0x38, 0x23, 0xe4, 0x11, 0x27, 0xc6, 0xa0, 0xe7
            },
            /* Initializer for fe[3] and the Z coordinate of gej[1]. */
            {
                0xbd, 0x21, 0xa5, 0xe1, 0x13, 0x50, 0x73, 0x2e,
                0x52, 0x98, 0xc8, 0x9e, 0xab, 0x00, 0xa2, 0x68,
                0x43, 0xf5, 0xd7, 0x49, 0x80, 0x72, 0xa7, 0xf3,
                0xd7, 0x60, 0xe6, 0xab, 0x90, 0x92, 0xdf, 0xc5
            }
        };

        scalar_set_b32(&data->scalar[0], init[0], NULL);
        scalar_set_b32(&data->scalar[1], init[1], NULL);
        fe_set_b32(&data->fe[0], init[0]);
        fe_set_b32(&data->fe[1], init[1]);
        fe_set_b32(&data->fe[2], init[2]);
        fe_set_b32(&data->fe[3], init[3]);
        CHECK(ge_set_xo_var(&data->ge[0], &data->fe[0], 0));
        CHECK(ge_set_xo_var(&data->ge[1], &data->fe[1], 1));
        gej_set_ge(&data->gej[0], &data->ge[0]);
        gej_rescale(&data->gej[0], &data->fe[2]);
        gej_set_ge(&data->gej[1], &data->ge[1]);
        gej_rescale(&data->gej[1], &data->fe[3]);
        memcpy(data->data, init[0], 32);
        memcpy(data->data + 32, init[1], 32);
        */
}

pub fn bench_scalar_add(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        int i, j = 0;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            j += scalar_add(&data->scalar[0], &data->scalar[0], &data->scalar[1]);
        }
        CHECK(j <= iters);
        */
}

pub fn bench_scalar_negate(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        int i;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            scalar_negate(&data->scalar[0], &data->scalar[0]);
        }
        */
}

pub fn bench_scalar_mul(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        int i;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            scalar_mul(&data->scalar[0], &data->scalar[0], &data->scalar[1]);
        }
        */
}

pub fn bench_scalar_split(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        int i, j = 0;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            scalar_split_lambda(&data->scalar[0], &data->scalar[1], &data->scalar[0]);
            j += scalar_add(&data->scalar[0], &data->scalar[0], &data->scalar[1]);
        }
        CHECK(j <= iters);
        */
}

pub fn bench_scalar_inverse(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        int i, j = 0;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            scalar_inverse(&data->scalar[0], &data->scalar[0]);
            j += scalar_add(&data->scalar[0], &data->scalar[0], &data->scalar[1]);
        }
        CHECK(j <= iters);
        */
}

pub fn bench_scalar_inverse_var(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            int i, j = 0;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            scalar_inverse_var(&data->scalar[0], &data->scalar[0]);
            j += scalar_add(&data->scalar[0], &data->scalar[0], &data->scalar[1]);
        }
        CHECK(j <= iters);
        */
}

pub fn bench_field_normalize(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        int i;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            fe_normalize(&data->fe[0]);
        }
        */
}

pub fn bench_field_normalize_weak(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        int i;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            fe_normalize_weak(&data->fe[0]);
        }
        */
}

pub fn bench_field_mul(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        int i;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            fe_mul(&data->fe[0], &data->fe[0], &data->fe[1]);
        }
        */
}

pub fn bench_field_sqr(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        int i;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            fe_sqr(&data->fe[0], &data->fe[0]);
        }
        */
}

pub fn bench_field_inverse(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            int i;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            fe_inv(&data->fe[0], &data->fe[0]);
            fe_add(&data->fe[0], &data->fe[1]);
        }
        */
}

pub fn bench_field_inverse_var(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            int i;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            fe_inv_var(&data->fe[0], &data->fe[0]);
            fe_add(&data->fe[0], &data->fe[1]);
        }
        */
}

pub fn bench_field_sqrt(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            int i, j = 0;
        bench_inv *data = (bench_inv*)arg;
        fe t;

        for (i = 0; i < iters; i++) {
            t = data->fe[0];
            j += fe_sqrt(&data->fe[0], &t);
            fe_add(&data->fe[0], &data->fe[1]);
        }
        CHECK(j <= iters);
        */
}

pub fn bench_group_double_var(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            int i;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            gej_double_var(&data->gej[0], &data->gej[0], NULL);
        }
        */
}

pub fn bench_group_add_var(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            int i;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            gej_add_var(&data->gej[0], &data->gej[0], &data->gej[1], NULL);
        }
        */
}

pub fn bench_group_add_affine(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            int i;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            gej_add_ge(&data->gej[0], &data->gej[0], &data->ge[1]);
        }
        */
}

pub fn bench_group_add_affine_var(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            int i;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            gej_add_ge_var(&data->gej[0], &data->gej[0], &data->ge[1], NULL);
        }
        */
}

pub fn bench_group_to_affine_var(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        int i;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; ++i) {
            ge_set_gej_var(&data->ge[1], &data->gej[0]);
            /* Use the output affine X/Y coordinates to vary the input X/Y/Z coordinates.
               Note that the resulting coordinates will generally not correspond to a point
               on the curve, but this is not a problem for the code being benchmarked here.
               Adding and normalizing have less overhead than EC operations (which could
               guarantee the point remains on the curve). */
            fe_add(&data->gej[0].x, &data->ge[1].y);
            fe_add(&data->gej[0].y, &data->fe[2]);
            fe_add(&data->gej[0].z, &data->ge[1].x);
            fe_normalize_var(&data->gej[0].x);
            fe_normalize_var(&data->gej[0].y);
            fe_normalize_var(&data->gej[0].z);
        }
        */
}

pub fn bench_ecmult_wnaf(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            int i, bits = 0, overflow = 0;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            bits += ecmult_wnaf(data->wnaf, 256, &data->scalar[0], WINDOW_A);
            overflow += scalar_add(&data->scalar[0], &data->scalar[0], &data->scalar[1]);
        }
        CHECK(overflow >= 0);
        CHECK(bits <= 256*iters);
        */
}

pub fn bench_wnaf_const(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            int i, bits = 0, overflow = 0;
        bench_inv *data = (bench_inv*)arg;

        for (i = 0; i < iters; i++) {
            bits += wnaf_const(data->wnaf, &data->scalar[0], WINDOW_A, 256);
            overflow += scalar_add(&data->scalar[0], &data->scalar[0], &data->scalar[1]);
        }
        CHECK(overflow >= 0);
        CHECK(bits <= 256*iters);
        */
}

pub fn bench_sha256(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            int i;
        bench_inv *data = (bench_inv*)arg;
        sha256 sha;

        for (i = 0; i < iters; i++) {
            sha256_initialize(&sha);
            sha256_write(&sha, data->data, 32);
            sha256_finalize(&sha, data->data);
        }
        */
}

pub fn bench_hmac_sha256(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        int i;
        bench_inv *data = (bench_inv*)arg;
        hmac_sha256 hmac;

        for (i = 0; i < iters; i++) {
            hmac_sha256_initialize(&hmac, data->data, 32);
            hmac_sha256_write(&hmac, data->data, 32);
            hmac_sha256_finalize(&hmac, data->data);
        }
        */
}

pub fn bench_rfc6979_hmac_sha256(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        int i;
        bench_inv *data = (bench_inv*)arg;
        rfc6979_hmac_sha256 rng;

        for (i = 0; i < iters; i++) {
            rfc6979_hmac_sha256_initialize(&rng, data->data, 64);
            rfc6979_hmac_sha256_generate(&rng, data->data, 32);
        }
        */
}

pub fn bench_context_verify(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        int i;
        (c_void)arg;
        for (i = 0; i < iters; i++) {
            context_destroy(context_create(CONTEXT_VERIFY));
        }
        */
}

pub fn bench_context_sign(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            int i;
        (c_void)arg;
        for (i = 0; i < iters; i++) {
            context_destroy(context_create(CONTEXT_SIGN));
        }
        */
}

pub fn secp256k1_bench_internal_main(
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
        bench_inv data;
        int iters = get_iters(20000);

        if (have_flag(argc, argv, "scalar") || have_flag(argc, argv, "add")) run_benchmark("scalar_add", bench_scalar_add, bench_setup, NULL, &data, 10, iters*100);
        if (have_flag(argc, argv, "scalar") || have_flag(argc, argv, "negate")) run_benchmark("scalar_negate", bench_scalar_negate, bench_setup, NULL, &data, 10, iters*100);
        if (have_flag(argc, argv, "scalar") || have_flag(argc, argv, "mul")) run_benchmark("scalar_mul", bench_scalar_mul, bench_setup, NULL, &data, 10, iters*10);
        if (have_flag(argc, argv, "scalar") || have_flag(argc, argv, "split")) run_benchmark("scalar_split", bench_scalar_split, bench_setup, NULL, &data, 10, iters);
        if (have_flag(argc, argv, "scalar") || have_flag(argc, argv, "inverse")) run_benchmark("scalar_inverse", bench_scalar_inverse, bench_setup, NULL, &data, 10, iters);
        if (have_flag(argc, argv, "scalar") || have_flag(argc, argv, "inverse")) run_benchmark("scalar_inverse_var", bench_scalar_inverse_var, bench_setup, NULL, &data, 10, iters);

        if (have_flag(argc, argv, "field") || have_flag(argc, argv, "normalize")) run_benchmark("field_normalize", bench_field_normalize, bench_setup, NULL, &data, 10, iters*100);
        if (have_flag(argc, argv, "field") || have_flag(argc, argv, "normalize")) run_benchmark("field_normalize_weak", bench_field_normalize_weak, bench_setup, NULL, &data, 10, iters*100);
        if (have_flag(argc, argv, "field") || have_flag(argc, argv, "sqr")) run_benchmark("field_sqr", bench_field_sqr, bench_setup, NULL, &data, 10, iters*10);
        if (have_flag(argc, argv, "field") || have_flag(argc, argv, "mul")) run_benchmark("field_mul", bench_field_mul, bench_setup, NULL, &data, 10, iters*10);
        if (have_flag(argc, argv, "field") || have_flag(argc, argv, "inverse")) run_benchmark("field_inverse", bench_field_inverse, bench_setup, NULL, &data, 10, iters);
        if (have_flag(argc, argv, "field") || have_flag(argc, argv, "inverse")) run_benchmark("field_inverse_var", bench_field_inverse_var, bench_setup, NULL, &data, 10, iters);
        if (have_flag(argc, argv, "field") || have_flag(argc, argv, "sqrt")) run_benchmark("field_sqrt", bench_field_sqrt, bench_setup, NULL, &data, 10, iters);

        if (have_flag(argc, argv, "group") || have_flag(argc, argv, "double")) run_benchmark("group_double_var", bench_group_double_var, bench_setup, NULL, &data, 10, iters*10);
        if (have_flag(argc, argv, "group") || have_flag(argc, argv, "add")) run_benchmark("group_add_var", bench_group_add_var, bench_setup, NULL, &data, 10, iters*10);
        if (have_flag(argc, argv, "group") || have_flag(argc, argv, "add")) run_benchmark("group_add_affine", bench_group_add_affine, bench_setup, NULL, &data, 10, iters*10);
        if (have_flag(argc, argv, "group") || have_flag(argc, argv, "add")) run_benchmark("group_add_affine_var", bench_group_add_affine_var, bench_setup, NULL, &data, 10, iters*10);
        if (have_flag(argc, argv, "group") || have_flag(argc, argv, "to_affine")) run_benchmark("group_to_affine_var", bench_group_to_affine_var, bench_setup, NULL, &data, 10, iters);

        if (have_flag(argc, argv, "ecmult") || have_flag(argc, argv, "wnaf")) run_benchmark("wnaf_const", bench_wnaf_const, bench_setup, NULL, &data, 10, iters);
        if (have_flag(argc, argv, "ecmult") || have_flag(argc, argv, "wnaf")) run_benchmark("ecmult_wnaf", bench_ecmult_wnaf, bench_setup, NULL, &data, 10, iters);

        if (have_flag(argc, argv, "hash") || have_flag(argc, argv, "sha256")) run_benchmark("hash_sha256", bench_sha256, bench_setup, NULL, &data, 10, iters);
        if (have_flag(argc, argv, "hash") || have_flag(argc, argv, "hmac")) run_benchmark("hash_hmac_sha256", bench_hmac_sha256, bench_setup, NULL, &data, 10, iters);
        if (have_flag(argc, argv, "hash") || have_flag(argc, argv, "rng6979")) run_benchmark("hash_rfc6979_hmac_sha256", bench_rfc6979_hmac_sha256, bench_setup, NULL, &data, 10, iters);

        if (have_flag(argc, argv, "context") || have_flag(argc, argv, "verify")) run_benchmark("context_verify", bench_context_verify, bench_setup, NULL, &data, 10, 1 + iters/1000);
        if (have_flag(argc, argv, "context") || have_flag(argc, argv, "sign")) run_benchmark("context_sign", bench_context_sign, bench_setup, NULL, &data, 10, 1 + iters/100);

        return 0;
        */
}
