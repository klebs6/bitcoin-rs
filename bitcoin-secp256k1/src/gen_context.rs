crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/gen_context.c]

/**
  | We can't require the precomputed tables
  | when creating them.
  |
  */
pub const USE_ECMULT_STATIC_PRECOMPUTATION: bool = false;

/**
  | In principle we could use ASM, but this
  | yields only a minor speedup in build
  | time and it's very complicated. In particular
  | when cross-compiling, we'd need to
  | build the ASM for the build and the host
  | machine.
  |
  */
pub const USE_EXTERNAL_ASM: bool = false;
pub const USE_ASM_X86_64:   bool = false;

pub fn default_error_callback_fn(
        str_: *const u8,
        data: *mut c_void)  {
    
    todo!();
        /*
            (c_void)data;
        fprintf(stderr, "[libsecp256k1] internal consistency check failed: %s\n", str);
        abort();
        */
}

lazy_static!{
    /*
    static const secp256k1_callback default_error_callback = {
        default_error_callback_fn,
        NULL
    };
    */
}


pub fn secp256k1_gen_context_main(
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            secp256k1_ecmult_gen_context ctx;
        c_void *prealloc, *base;
        int inner;
        int outer;
        FILE* fp;

        (c_void)argc;
        (c_void)argv;

        fp = fopen("src/ecmult_static_context.h","w");
        if (fp == NULL) {
            fprintf(stderr, "Could not open src/ecmult_static_context.h for writing!\n");
            return -1;
        }

        fprintf(fp, "#ifndef SECP256K1_ECMULT_STATIC_CONTEXT_H\n");
        fprintf(fp, "#define SECP256K1_ECMULT_STATIC_CONTEXT_H\n");
        fprintf(fp, "#include \"src/group.h\"\n");
        fprintf(fp, "#define SC SECP256K1_GE_STORAGE_CONST\n");
        fprintf(fp, "#if ECMULT_GEN_PREC_N != %d || ECMULT_GEN_PREC_G != %d\n", ECMULT_GEN_PREC_N, ECMULT_GEN_PREC_G);
        fprintf(fp, "   #error configuration mismatch, invalid ECMULT_GEN_PREC_N, ECMULT_GEN_PREC_G. Try deleting ecmult_static_context.h before the build.\n");
        fprintf(fp, "#endif\n");
        fprintf(fp, "static const secp256k1_ge_storage secp256k1_ecmult_static_context[ECMULT_GEN_PREC_N][ECMULT_GEN_PREC_G] = {\n");

        base = checked_malloc(&default_error_callback, SECP256K1_ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE);
        prealloc = base;
        secp256k1_ecmult_gen_context_init(&ctx);
        secp256k1_ecmult_gen_context_build(&ctx, &prealloc);
        for(outer = 0; outer != ECMULT_GEN_PREC_N; outer++) {
            fprintf(fp,"{\n");
            for(inner = 0; inner != ECMULT_GEN_PREC_G; inner++) {
                fprintf(fp,"    SC(%uu, %uu, %uu, %uu, %uu, %uu, %uu, %uu, %uu, %uu, %uu, %uu, %uu, %uu, %uu, %uu)", SECP256K1_GE_STORAGE_CONST_GET((*ctx.prec)[outer][inner]));
                if (inner != ECMULT_GEN_PREC_G - 1) {
                    fprintf(fp,",\n");
                } else {
                    fprintf(fp,"\n");
                }
            }
            if (outer != ECMULT_GEN_PREC_N - 1) {
                fprintf(fp,"},\n");
            } else {
                fprintf(fp,"}\n");
            }
        }
        fprintf(fp,"};\n");
        secp256k1_ecmult_gen_context_clear(&ctx);
        free(base);

        fprintf(fp, "#undef SC\n");
        fprintf(fp, "#endif\n");
        fclose(fp);

        return 0;
        */
}
