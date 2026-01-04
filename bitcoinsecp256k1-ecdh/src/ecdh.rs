// ---------------- [ File: bitcoinsecp256k1-ecdh/src/ecdh.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/include/ecdh.h]

/** 
 | A pointer to a function that hashes an EC point
 | to obtain an ECDH secret
 |
 |  Returns: 1 if the point was successfully
 |           hashed.
 |
 |           0 will cause ecdh to fail and return 0.
 |
 |           Other return values are not allowed,
 |           and the behaviour of ecdh is
 |           undefined for other return values.
 |
 |  Out:     output:     pointer to an array to be
 |                       filled by the function
 |
 |  In:      x32:        pointer to a 32-byte
 |                       x coordinate
 |
 |           y32:        pointer to a 32-byte
 |                       y coordinate
 |
 |           data:       arbitrary data pointer
 |                       that is passed through
 */
pub type EcdhHashFunction = fn(
        output: *mut u8,
        x32:    *const u8,
        y32:    *const u8,
        data:   *mut c_void
) -> i32;

/**
  | An implementation of SHA256 hash function
  | that applies to compressed public key.
  | 
  | Populates the output parameter with
  | 32 bytes.
  |
  */
lazy_static!{
    /*
    extern const ecdh_hash_function ecdh_hash_function_sha256;
    */
}

/**
  | A default ECDH hash function (currently
  | equal to ecdh_hash_function_sha256).
  | 
  | Populates the output parameter with
  | 32 bytes.
  |
  */
lazy_static!{
    /*
    extern const ecdh_hash_function ecdh_hash_function_default;
    */
}

/** 
 | Compute an EC Diffie-Hellman secret in constant
 | time
 |
 |  Returns: 1: exponentiation was successful
 |
 |           0: scalar was invalid (zero or
 |              overflow) or hashfp returned 0
 |
 |  Args:    ctx:        pointer to a context
 |                       object (cannot be NULL)
 |
 |  Out:     output:     pointer to an array to be
 |                       filled by hashfp
 |
 |  In:      pubkey:     a pointer to a pubkey
 |                       containing an initialized
 |                       public key
 |
 |           seckey:     a 32-byte scalar with
 |                       which to multiply the
 |                       point
 |
 |           hashfp:     pointer to a hash
 |                       function. If NULL,
 |                       ecdh_hash_function_sha256
 |                       is used (in which case,
 |                       32 bytes will be written
 |                       to output)
 |
 |           data:       arbitrary data pointer
 |                       that is passed through to hashfp
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ecdh(
      const context* ctx,
      unsigned char *output,
      const pubkey *pubkey,
      const unsigned char *seckey,
      ecdh_hash_function hashfp,
      c_void *data
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3) ARG_NONNULL(4);
    */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/modules/ecdh/main_impl.h]

pub fn ecdh_hash_function_sha256(
        output: *mut u8,
        x32:    *const u8,
        y32:    *const u8,
        data:   *mut c_void) -> i32 {
    
    todo!();
        /*
            unsigned char version = (y32[31] & 0x01) | 0x02;
        sha256 sha;
        (c_void)data;

        sha256_initialize(&sha);
        sha256_write(&sha, &version, 1);
        sha256_write(&sha, x32, 32);
        sha256_finalize(&sha, output);

        return 1;
        */
}

pub const ECDH_HASH_FUNCTION_SHA256:  EcdhHashFunction = ecdh_hash_function_sha256;
pub const ECDH_HASH_FUNCTION_DEFAULT: EcdhHashFunction = ecdh_hash_function_sha256;

pub fn ecdh(
        ctx:    *const Secp256k1Context,
        output: *mut u8,
        point:  *const PubKey,
        scalar: *const u8,
        hashfp: EcdhHashFunction,
        data:   *mut c_void) -> i32 {
    
    todo!();
        /*
        int ret = 0;
        int overflow = 0;
        gej res;
        ge pt;
        scalar s;
        unsigned char x[32];
        unsigned char y[32];

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(output != NULL);
        ARG_CHECK(point != NULL);
        ARG_CHECK(scalar != NULL);

        if (hashfp == NULL) {
            hashfp = ecdh_hash_function_default;
        }

        pubkey_load(ctx, &pt, point);
        scalar_set_b32(&s, scalar, &overflow);

        overflow |= scalar_is_zero(&s);
        scalar_cmov(&s, &scalar_one, overflow);

        ecmult_const(&res, &pt, &s, 256);
        ge_set_gej(&pt, &res);

        /* Compute a hash of the point */
        fe_normalize(&pt.x);
        fe_normalize(&pt.y);
        fe_get_b32(x, &pt.x);
        fe_get_b32(y, &pt.y);

        ret = hashfp(output, x, y, data);

        memset(x, 0, 32);
        memset(y, 0, 32);
        scalar_clear(&s);

        return !!ret & !overflow;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/modules/ecdh/tests_impl.h]

pub fn ecdh_hash_function_test_fail(
    output: *mut u8,
    x:      *const u8,
    y:      *const u8,
    data:   *mut c_void) -> i32 {
    
    todo!();
        /*
            (c_void)output;
        (c_void)x;
        (c_void)y;
        (c_void)data;
        return 0;
        */
}

pub fn ecdh_hash_function_custom(
    output: *mut u8,
    x:      *const u8,
    y:      *const u8,
    data:   *mut c_void) -> i32 {
    
    todo!();
        /*
            (c_void)data;
        /* Save x and y as uncompressed public key */
        output[0] = 0x04;
        memcpy(output + 1, x, 32);
        memcpy(output + 33, y, 32);
        return 1;
        */
}

pub fn test_ecdh_api()  {
    
    todo!();
        /*
        /* Setup context that just counts errors */
        context *tctx = context_create(CONTEXT_SIGN);
        pubkey point;
        unsigned char res[32];
        unsigned char s_one[32] = { 0 };
        int32_t ecount = 0;
        s_one[31] = 1;

        context_set_error_callback(tctx, counting_illegal_callback_fn, &ecount);
        context_set_illegal_callback(tctx, counting_illegal_callback_fn, &ecount);
        CHECK(ec_pubkey_create(tctx, &point, s_one) == 1);

        /* Check all NULLs are detected */
        CHECK(ecdh(tctx, res, &point, s_one, NULL, NULL) == 1);
        CHECK(ecount == 0);
        CHECK(ecdh(tctx, NULL, &point, s_one, NULL, NULL) == 0);
        CHECK(ecount == 1);
        CHECK(ecdh(tctx, res, NULL, s_one, NULL, NULL) == 0);
        CHECK(ecount == 2);
        CHECK(ecdh(tctx, res, &point, NULL, NULL, NULL) == 0);
        CHECK(ecount == 3);
        CHECK(ecdh(tctx, res, &point, s_one, NULL, NULL) == 1);
        CHECK(ecount == 3);

        /* Cleanup */
        context_destroy(tctx);
        */
}

pub fn test_ecdh_generator_basepoint()  {
    
    todo!();
        /*
            unsigned char s_one[32] = { 0 };
        pubkey point[2];
        int i;

        s_one[31] = 1;
        /* Check against pubkey creation when the basepoint is the generator */
        for (i = 0; i < 100; ++i) {
            sha256 sha;
            unsigned char s_b32[32];
            unsigned char output_ecdh[65];
            unsigned char output_ser[32];
            unsigned char point_ser[65];
            size_t point_ser_len = sizeof(point_ser);
            scalar s;

            random_scalar_order(&s);
            scalar_get_b32(s_b32, &s);

            CHECK(ec_pubkey_create(ctx, &point[0], s_one) == 1);
            CHECK(ec_pubkey_create(ctx, &point[1], s_b32) == 1);

            /* compute using ECDH function with custom hash function */
            CHECK(ecdh(ctx, output_ecdh, &point[0], s_b32, ecdh_hash_function_custom, NULL) == 1);
            /* compute "explicitly" */
            CHECK(ec_pubkey_serialize(ctx, point_ser, &point_ser_len, &point[1], EC_UNCOMPRESSED) == 1);
            /* compare */
            CHECK(memcmp_var(output_ecdh, point_ser, 65) == 0);

            /* compute using ECDH function with default hash function */
            CHECK(ecdh(ctx, output_ecdh, &point[0], s_b32, NULL, NULL) == 1);
            /* compute "explicitly" */
            CHECK(ec_pubkey_serialize(ctx, point_ser, &point_ser_len, &point[1], EC_COMPRESSED) == 1);
            sha256_initialize(&sha);
            sha256_write(&sha, point_ser, point_ser_len);
            sha256_finalize(&sha, output_ser);
            /* compare */
            CHECK(memcmp_var(output_ecdh, output_ser, 32) == 0);
        }
        */
}

pub fn test_bad_scalar()  {
    
    todo!();
        /*
            unsigned char s_zero[32] = { 0 };
        unsigned char s_overflow[32] = {
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
            0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b,
            0xbf, 0xd2, 0x5e, 0x8c, 0xd0, 0x36, 0x41, 0x41
        };
        unsigned char s_rand[32] = { 0 };
        unsigned char output[32];
        scalar rand;
        pubkey point;

        /* Create random point */
        random_scalar_order(&rand);
        scalar_get_b32(s_rand, &rand);
        CHECK(ec_pubkey_create(ctx, &point, s_rand) == 1);

        /* Try to multiply it by bad values */
        CHECK(ecdh(ctx, output, &point, s_zero, NULL, NULL) == 0);
        CHECK(ecdh(ctx, output, &point, s_overflow, NULL, NULL) == 0);
        /* ...and a good one */
        s_overflow[31] -= 1;
        CHECK(ecdh(ctx, output, &point, s_overflow, NULL, NULL) == 1);

        /* Hash function failure results in ecdh failure */
        CHECK(ecdh(ctx, output, &point, s_overflow, ecdh_hash_function_test_fail, NULL) == 0);
        */
}

pub fn run_ecdh_tests()  {
    
    todo!();
        /*
            test_ecdh_api();
        test_ecdh_generator_basepoint();
        test_bad_scalar();
        */
}
