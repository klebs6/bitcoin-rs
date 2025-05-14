// ---------------- [ File: bitcoinsecp256k1-bench/src/bench_ecmult.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/bench_ecmult.c]

pub const POINTS: usize = 32768;

pub fn help(argv: *mut *mut u8)  {
    
    todo!();
        /*
            printf("Benchmark EC multiplication algorithms\n");
        printf("\n");
        printf("Usage: %s <help|pippenger_wnaf|strauss_wnaf|simple>\n", argv[0]);
        printf("The output shows the number of multiplied and summed points right after the\n");
        printf("function name. The letter 'g' indicates that one of the points is the generator.\n");
        printf("The benchmarks are divided by the number of points.\n");
        printf("\n");
        printf("default (ecmult_multi): picks pippenger_wnaf or strauss_wnaf depending on the\n");
        printf("                        batch size\n");
        printf("pippenger_wnaf:         for all batch sizes\n");
        printf("strauss_wnaf:           for all batch sizes\n");
        printf("simple:                 multiply and sum each point individually\n");
        */
}

pub struct BenchData {

    /**
      | Setup once in advance
      |
      */
    ctx:             *mut Secp256k1Context,

    scratch:         *mut Scratch,
    scalars:         *mut Scalar,
    pubkeys:         *mut Ge,
    pubkeys_gej:     *mut Gej,
    seckeys:         *mut Scalar,
    expected_output: *mut Gej,
    ecmult_multi:    EcMultMultiFunc,

    /**
      | Changes per benchmark
      |
      */
    count:           usize,

    includes_g:      i32,

    /**
      | Changes per benchmark iteration, used
      | to pick different scalars and pubkeys
      | in each run.
      |
      */
    offset1:         usize,

    offset2:         usize,

    /**
      | Benchmark output.
      |
      */
    output:          *mut Gej,
}

/**
  | Hashes x into [0, POINTS) twice and store
  | the result in offset1 and offset2.
  |
  */
pub fn hash_into_offset(
        data: *mut BenchData,
        x:    usize)  {
    
    todo!();
        /*
            data->offset1 = (x * 0x537b7f6f + 0x8f66a481) % POINTS;
        data->offset2 = (x * 0x7f6f537b + 0x6a1a8f49) % POINTS;
        */
}

/**
  | Check correctness of the benchmark
  | by computing
  | 
  | sum(outputs) ?= (sum(scalars_gen)
  | + sum(seckeys)*sum(scalars))*G
  |
  */
pub fn bench_ecmult_teardown_helper(
        data:              *mut BenchData,
        seckey_offset:     *mut usize,
        scalar_offset:     *mut usize,
        scalar_gen_offset: *mut usize,
        iters:             i32)  {
    
    todo!();
        /*
            int i;
        gej sum_output, tmp;
        scalar sum_scalars;

        gej_set_infinity(&sum_output);
        scalar_clear(&sum_scalars);
        for (i = 0; i < iters; ++i) {
            gej_add_var(&sum_output, &sum_output, &data->output[i], NULL);
            if (scalar_gen_offset != NULL) {
                scalar_add(&sum_scalars, &sum_scalars, &data->scalars[(*scalar_gen_offset+i) % POINTS]);
            }
            if (seckey_offset != NULL) {
                scalar s = data->seckeys[(*seckey_offset+i) % POINTS];
                scalar_mul(&s, &s, &data->scalars[(*scalar_offset+i) % POINTS]);
                scalar_add(&sum_scalars, &sum_scalars, &s);
            }
        }
        ecmult_gen(&data->ctx->ecmult_gen_ctx, &tmp, &sum_scalars);
        gej_neg(&tmp, &tmp);
        gej_add_var(&tmp, &tmp, &sum_output, NULL);
        CHECK(gej_is_infinity(&tmp));
        */
}

pub fn bench_ecmult_setup(arg: *mut c_void)  {
    
    todo!();
        /*
            bench_data* data = (bench_data*)arg;
        /* Re-randomize offset to ensure that we're using different scalars and
         * group elements in each run. */
        hash_into_offset(data, data->offset1);
        */
}

pub fn bench_ecmult_gen(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            bench_data* data = (bench_data*)arg;
        int i;

        for (i = 0; i < iters; ++i) {
            ecmult_gen(&data->ctx->ecmult_gen_ctx, &data->output[i], &data->scalars[(data->offset1+i) % POINTS]);
        }
        */
}

pub fn bench_ecmult_gen_teardown(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            bench_data* data = (bench_data*)arg;
        bench_ecmult_teardown_helper(data, NULL, NULL, &data->offset1, iters);
        */
}

pub fn bench_ecmult_const(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            bench_data* data = (bench_data*)arg;
        int i;

        for (i = 0; i < iters; ++i) {
            ecmult_const(&data->output[i], &data->pubkeys[(data->offset1+i) % POINTS], &data->scalars[(data->offset2+i) % POINTS], 256);
        }
        */
}

pub fn bench_ecmult_const_teardown(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        bench_data* data = (bench_data*)arg;
        bench_ecmult_teardown_helper(data, &data->offset1, &data->offset2, NULL, iters);
        */
}

pub fn bench_ecmult_1(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            bench_data* data = (bench_data*)arg;
        int i;

        for (i = 0; i < iters; ++i) {
            ecmult(&data->ctx->ecmult_ctx, &data->output[i], &data->pubkeys_gej[(data->offset1+i) % POINTS], &data->scalars[(data->offset2+i) % POINTS], NULL);
        }
        */
}

pub fn bench_ecmult_1_teardown(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            bench_data* data = (bench_data*)arg;
        bench_ecmult_teardown_helper(data, &data->offset1, &data->offset2, NULL, iters);
        */
}

pub fn bench_ecmult_1g(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            bench_data* data = (bench_data*)arg;
        scalar zero;
        int i;

        scalar_set_int(&zero, 0);
        for (i = 0; i < iters; ++i) {
            ecmult(&data->ctx->ecmult_ctx, &data->output[i], NULL, &zero, &data->scalars[(data->offset1+i) % POINTS]);
        }
        */
}

pub fn bench_ecmult_1g_teardown(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        bench_data* data = (bench_data*)arg;
        bench_ecmult_teardown_helper(data, NULL, NULL, &data->offset1, iters);
        */
}

pub fn bench_ecmult_2g(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        bench_data* data = (bench_data*)arg;
        int i;

        for (i = 0; i < iters/2; ++i) {
            ecmult(&data->ctx->ecmult_ctx, &data->output[i], &data->pubkeys_gej[(data->offset1+i) % POINTS], &data->scalars[(data->offset2+i) % POINTS], &data->scalars[(data->offset1+i) % POINTS]);
        }
        */
}

pub fn bench_ecmult_2g_teardown(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        bench_data* data = (bench_data*)arg;
        bench_ecmult_teardown_helper(data, &data->offset1, &data->offset2, &data->offset1, iters/2);
        */
}

pub fn run_ecmult_bench(
        data:  *mut BenchData,
        iters: i32)  {
    
    todo!();
        /*
        char str[32];
        sprintf(str, "ecmult_gen");
        run_benchmark(str, bench_ecmult_gen, bench_ecmult_setup, bench_ecmult_gen_teardown, data, 10, iters);
        sprintf(str, "ecmult_const");
        run_benchmark(str, bench_ecmult_const, bench_ecmult_setup, bench_ecmult_const_teardown, data, 10, iters);
        /* ecmult with non generator point */
        sprintf(str, "ecmult 1");
        run_benchmark(str, bench_ecmult_1, bench_ecmult_setup, bench_ecmult_1_teardown, data, 10, iters);
        /* ecmult with generator point */
        sprintf(str, "ecmult 1g");
        run_benchmark(str, bench_ecmult_1g, bench_ecmult_setup, bench_ecmult_1g_teardown, data, 10, iters);
        /* ecmult with generator and non-generator point. The reported time is per point. */
        sprintf(str, "ecmult 2g");
        run_benchmark(str, bench_ecmult_2g, bench_ecmult_setup, bench_ecmult_2g_teardown, data, 10, 2*iters);
        */
}

pub fn bench_ecmult_multi_callback(
        sc:  *mut Scalar,
        ge:  *mut Ge,
        idx: usize,
        arg: *mut c_void) -> i32 {
    
    todo!();
        /*
            bench_data* data = (bench_data*)arg;
        if (data->includes_g) ++idx;
        if (idx == 0) {
            *sc = data->scalars[data->offset1];
            *ge = ge_const_g;
        } else {
            *sc = data->scalars[(data->offset1 + idx) % POINTS];
            *ge = data->pubkeys[(data->offset2 + idx - 1) % POINTS];
        }
        return 1;
        */
}

pub fn bench_ecmult_multi(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
            bench_data* data = (bench_data*)arg;

        int includes_g = data->includes_g;
        int iter;
        int count = data->count;
        iters = iters / data->count;

        for (iter = 0; iter < iters; ++iter) {
            data->ecmult_multi(&data->ctx->error_callback, &data->ctx->ecmult_ctx, data->scratch, &data->output[iter], data->includes_g ? &data->scalars[data->offset1] : NULL, bench_ecmult_multi_callback, arg, count - includes_g);
            data->offset1 = (data->offset1 + count) % POINTS;
            data->offset2 = (data->offset2 + count - 1) % POINTS;
        }
        */
}

pub fn bench_ecmult_multi_setup(arg: *mut c_void)  {
    
    todo!();
        /*
        bench_data* data = (bench_data*)arg;
        hash_into_offset(data, data->count);
        */
}

pub fn bench_ecmult_multi_teardown(
        arg:   *mut c_void,
        iters: i32)  {
    
    todo!();
        /*
        bench_data* data = (bench_data*)arg;
        int iter;
        iters = iters / data->count;
        /* Verify the results in teardown, to avoid doing comparisons while benchmarking. */
        for (iter = 0; iter < iters; ++iter) {
            gej tmp;
            gej_add_var(&tmp, &data->output[iter], &data->expected_output[iter], NULL);
            CHECK(gej_is_infinity(&tmp));
        }
        */
}

pub fn generate_scalar(
        num:    u32,
        scalar: *mut Scalar)  {
    
    todo!();
        /*
        sha256 sha256;
        unsigned char c[10] = {'e', 'c', 'm', 'u', 'l', 't', 0, 0, 0, 0};
        unsigned char buf[32];
        int overflow = 0;
        c[6] = num;
        c[7] = num >> 8;
        c[8] = num >> 16;
        c[9] = num >> 24;
        sha256_initialize(&sha256);
        sha256_write(&sha256, c, sizeof(c));
        sha256_finalize(&sha256, buf);
        scalar_set_b32(scalar, buf, &overflow);
        CHECK(!overflow);
        */
}

pub fn run_ecmult_multi_bench(
        data:       *mut BenchData,
        count:      usize,
        includes_g: i32,
        num_iters:  i32)  {
    
    todo!();
        /*
        char str[32];
        static const scalar zero = SCALAR_CONST(0, 0, 0, 0, 0, 0, 0, 0);
        size_t iters = 1 + num_iters / count;
        size_t iter;

        data->count = count;
        data->includes_g = includes_g;

        /* Compute (the negation of) the expected results directly. */
        hash_into_offset(data, data->count);
        for (iter = 0; iter < iters; ++iter) {
            scalar tmp;
            scalar total = data->scalars[(data->offset1++) % POINTS];
            size_t i = 0;
            for (i = 0; i + 1 < count; ++i) {
                scalar_mul(&tmp, &data->seckeys[(data->offset2++) % POINTS], &data->scalars[(data->offset1++) % POINTS]);
                scalar_add(&total, &total, &tmp);
            }
            scalar_negate(&total, &total);
            ecmult(&data->ctx->ecmult_ctx, &data->expected_output[iter], NULL, &zero, &total);
        }

        /* Run the benchmark. */
        sprintf(str, includes_g ? "ecmult_multi %ig" : "ecmult_multi %i", (int)count);
        run_benchmark(str, bench_ecmult_multi, bench_ecmult_multi_setup, bench_ecmult_multi_teardown, data, 10, count * iters);
        */
}

pub fn secp256k1_bench_ecmult_main(
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            bench_data data;
        int i, p;
        size_t scratch_size;

        int iters = get_iters(10000);

        data.ecmult_multi = ecmult_multi_var;

        if (argc > 1) {
            if(have_flag(argc, argv, "-h")
               || have_flag(argc, argv, "--help")
               || have_flag(argc, argv, "help")) {
                help(argv);
                return 1;
            } else if(have_flag(argc, argv, "pippenger_wnaf")) {
                printf("Using pippenger_wnaf:\n");
                data.ecmult_multi = ecmult_pippenger_batch_single;
            } else if(have_flag(argc, argv, "strauss_wnaf")) {
                printf("Using strauss_wnaf:\n");
                data.ecmult_multi = ecmult_strauss_batch_single;
            } else if(have_flag(argc, argv, "simple")) {
                printf("Using simple algorithm:\n");
            } else {
                fprintf(stderr, "%s: unrecognized argument '%s'.\n\n", argv[0], argv[1]);
                help(argv);
                return 1;
            }
        }

        data.ctx = context_create(CONTEXT_SIGN | CONTEXT_VERIFY);
        scratch_size = strauss_scratch_size(POINTS) + STRAUSS_SCRATCH_OBJECTS*16;
        if (!have_flag(argc, argv, "simple")) {
            data.scratch = scratch_space_create(data.ctx, scratch_size);
        } else {
            data.scratch = NULL;
        }

        /* Allocate stuff */
        data.scalars = malloc(sizeof(scalar) * POINTS);
        data.seckeys = malloc(sizeof(scalar) * POINTS);
        data.pubkeys = malloc(sizeof(ge) * POINTS);
        data.pubkeys_gej = malloc(sizeof(gej) * POINTS);
        data.expected_output = malloc(sizeof(gej) * (iters + 1));
        data.output = malloc(sizeof(gej) * (iters + 1));

        /* Generate a set of scalars, and private/public keypairs. */
        gej_set_ge(&data.pubkeys_gej[0], &ge_const_g);
        scalar_set_int(&data.seckeys[0], 1);
        for (i = 0; i < POINTS; ++i) {
            generate_scalar(i, &data.scalars[i]);
            if (i) {
                gej_double_var(&data.pubkeys_gej[i], &data.pubkeys_gej[i - 1], NULL);
                scalar_add(&data.seckeys[i], &data.seckeys[i - 1], &data.seckeys[i - 1]);
            }
        }
        ge_set_all_gej_var(data.pubkeys, data.pubkeys_gej, POINTS);

        /* Initialize offset1 and offset2 */
        hash_into_offset(&data, 0);
        run_ecmult_bench(&data, iters);

        for (i = 1; i <= 8; ++i) {
            run_ecmult_multi_bench(&data, i, 1, iters);
        }

        /* This is disabled with low count of iterations because the loop runs 77 times even with iters=1
        * and the higher it goes the longer the computation takes(more points)
        * So we don't run this benchmark with low iterations to prevent slow down */
         if (iters > 2) {
            for (p = 0; p <= 11; ++p) {
                for (i = 9; i <= 16; ++i) {
                    run_ecmult_multi_bench(&data, i << p, 1, iters);
                }
            }
        }

        if (data.scratch != NULL) {
            scratch_space_destroy(data.ctx, data.scratch);
        }
        context_destroy(data.ctx);
        free(data.scalars);
        free(data.pubkeys);
        free(data.pubkeys_gej);
        free(data.seckeys);
        free(data.output);
        free(data.expected_output);

        return(0);
        */
}
