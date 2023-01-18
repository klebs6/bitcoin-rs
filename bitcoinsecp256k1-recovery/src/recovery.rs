crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/include/recovery.h]

/** 
 | Opaque data structured that holds a parsed
 | ECDSA signature, supporting pubkey recovery.
 |
 |  The exact representation of data inside is
 |  implementation defined and not guaranteed to
 |  be portable between different platforms or
 |  versions. It is however guaranteed to be 65
 |  bytes in size, and can be safely copied/moved.
 |
 |  If you need to convert to a format suitable
 |  for storage or transmission, use the
 |  ecdsa_signature_serialize_* and
 |  ecdsa_signature_parse_* functions.
 |
 |  Furthermore, it is guaranteed that identical
 |  signatures (including their recoverability)
 |  will have identical representation, so they
 |  can be memcmp'ed.
 */
pub struct EcdsaRecoverableSignature {
    data: [u8; 65],
}

/** 
 | Parse a compact ECDSA signature (64 bytes
 | + recovery id).
 |
 |  Returns: 1 when the signature could be parsed,
 |           0 otherwise
 |
 |  Args: ctx:     a secp256k1 context object
 |
 |  Out:  sig:     a pointer to a signature object
 |
 |  In:   input64: a pointer to a 64-byte compact
 |                 signature
 |
 |        recid:   the recovery id (0, 1, 2 or 3)
 */
lazy_static!{
    /*
    int ecdsa_recoverable_signature_parse_compact(
        const context* ctx,
        ecdsa_recoverable_signature* sig,
        const unsigned char *input64,
        int recid
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Convert a recoverable signature into a normal
 | signature.
 |
 |  Returns: 1
 |  Out: sig:    a pointer to a normal signature
 |               (cannot be NULL).
 |
 |  In:  sigin:  a pointer to a recoverable
 |               signature (cannot be NULL).
 |
 */
lazy_static!{
    /*
    int ecdsa_recoverable_signature_convert(
        const context* ctx,
        ecdsa_signature* sig,
        const ecdsa_recoverable_signature* sigin
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Serialize an ECDSA signature in compact format
 | (64 bytes + recovery id).
 |
 |  Returns: 1
 |
 |  Args: ctx:      a secp256k1 context object
 |
 |  Out:  output64: a pointer to a 64-byte array
 |                  of the compact signature
 |                  (cannot be NULL)
 |
 |        recid:    a pointer to an integer to
 |                  hold the recovery id (can be
 |                  NULL).
 |
 |  In:   sig:      a pointer to an initialized
 |                  signature object (cannot be NULL)
 */
lazy_static!{
    /*
    int ecdsa_recoverable_signature_serialize_compact(
        const context* ctx,
        unsigned char *output64,
        int *recid,
        const ecdsa_recoverable_signature* sig
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3) ARG_NONNULL(4);
    */
}

/** 
 | Create a recoverable ECDSA signature.
 |
 |  Returns: 1: signature created
 |
 |           0: the nonce generation function
 |           failed, or the secret key was
 |           invalid.
 |
 |  Args:    ctx:       pointer to a context
 |                      object, initialized for
 |                      signing (cannot be NULL)
 |
 |  Out:     sig:       pointer to an array where
 |                      the signature will be
 |                      placed (cannot be NULL)
 |
 |  In:      msghash32: the 32-byte message hash
 |                      being signed (cannot be
 |                      NULL)
 |
 |           seckey:    pointer to a 32-byte
 |                      secret key (cannot be
 |                      NULL)
 |
 |           noncefp:   pointer to a nonce generation 
 |                      function. If NULL, 
 |                      nonce_function_default is used
 |
 |           ndata:     pointer to arbitrary data
 |                      used by the nonce
 |                      generation function (can
 |                      be NULL)
 */
lazy_static!{
    /*
    int ecdsa_sign_recoverable(
        const context* ctx,
        ecdsa_recoverable_signature *sig,
        const unsigned char *msghash32,
        const unsigned char *seckey,
        nonce_function noncefp,
        const c_void *ndata
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3) ARG_NONNULL(4);
    */
}

/** 
 | Recover an ECDSA public key from a signature.
 |
 |  Returns: 1: public key successfully recovered
 |              (which guarantees a correct
 |              signature).
 |
 |           0: otherwise.
 |
 |  Args:    ctx:       pointer to a context
 |                      object, initialized for
 |                      verification (cannot be
 |                      NULL)
 |
 |  Out:     pubkey:    pointer to the recovered
 |                      public key (cannot be
 |                      NULL)
 |
 |  In:      sig:       pointer to initialized
 |                      signature that supports
 |                      pubkey recovery (cannot be
 |                      NULL)
 |
 |           msghash32: the 32-byte message hash
 |                      assumed to be signed
 |                      (cannot be NULL)
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ecdsa_recover(
        const context* ctx,
        pubkey *pubkey,
        const ecdsa_recoverable_signature *sig,
        const unsigned char *msghash32
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3) ARG_NONNULL(4);
    */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/modules/recovery/main_impl.h]

pub fn ecdsa_recoverable_signature_load(
        ctx:   *const Secp256k1Context,
        r:     *mut Scalar,
        s:     *mut Scalar,
        recid: *mut i32,
        sig:   *const EcdsaRecoverableSignature)  {
    
    todo!();
        /*
            (c_void)ctx;
        if (sizeof(scalar) == 32) {
            /* When the scalar type is exactly 32 byte, use its
             * representation inside ecdsa_signature, as conversion is very fast.
             * Note that ecdsa_signature_save must use the same representation. */
            memcpy(r, &sig->data[0], 32);
            memcpy(s, &sig->data[32], 32);
        } else {
            scalar_set_b32(r, &sig->data[0], NULL);
            scalar_set_b32(s, &sig->data[32], NULL);
        }
        *recid = sig->data[64];
        */
}

pub fn ecdsa_recoverable_signature_save(
        sig:   *mut EcdsaRecoverableSignature,
        r:     *const Scalar,
        s:     *const Scalar,
        recid: i32)  {
    
    todo!();
        /*
            if (sizeof(scalar) == 32) {
            memcpy(&sig->data[0], r, 32);
            memcpy(&sig->data[32], s, 32);
        } else {
            scalar_get_b32(&sig->data[0], r);
            scalar_get_b32(&sig->data[32], s);
        }
        sig->data[64] = recid;
        */
}

pub fn ecdsa_recoverable_signature_parse_compact(
        ctx:     *const Secp256k1Context,
        sig:     *mut EcdsaRecoverableSignature,
        input64: *const u8,
        recid:   i32) -> i32 {
    
    todo!();
        /*
            scalar r, s;
        int ret = 1;
        int overflow = 0;

        (c_void)ctx;
        ARG_CHECK(sig != NULL);
        ARG_CHECK(input64 != NULL);
        ARG_CHECK(recid >= 0 && recid <= 3);

        scalar_set_b32(&r, &input64[0], &overflow);
        ret &= !overflow;
        scalar_set_b32(&s, &input64[32], &overflow);
        ret &= !overflow;
        if (ret) {
            ecdsa_recoverable_signature_save(sig, &r, &s, recid);
        } else {
            memset(sig, 0, sizeof(*sig));
        }
        return ret;
        */
}

pub fn ecdsa_recoverable_signature_serialize_compact(
        ctx:      *const Secp256k1Context,
        output64: *mut u8,
        recid:    *mut i32,
        sig:      *const EcdsaRecoverableSignature) -> i32 {
    
    todo!();
        /*
            scalar r, s;

        (c_void)ctx;
        ARG_CHECK(output64 != NULL);
        ARG_CHECK(sig != NULL);
        ARG_CHECK(recid != NULL);

        ecdsa_recoverable_signature_load(ctx, &r, &s, recid, sig);
        scalar_get_b32(&output64[0], &r);
        scalar_get_b32(&output64[32], &s);
        return 1;
        */
}

pub fn ecdsa_recoverable_signature_convert(
        ctx:   *const Secp256k1Context,
        sig:   *mut Secp256k1EcdsaSignature,
        sigin: *const EcdsaRecoverableSignature) -> i32 {
    
    todo!();
        /*
            scalar r, s;
        int recid;

        (c_void)ctx;
        ARG_CHECK(sig != NULL);
        ARG_CHECK(sigin != NULL);

        ecdsa_recoverable_signature_load(ctx, &r, &s, &recid, sigin);
        ecdsa_signature_save(sig, &r, &s);
        return 1;
        */
}

pub fn ecdsa_sig_recover(
        ctx:     *const EcMultContext,
        sigr:    *const Scalar,
        sigs:    *const Scalar,
        pubkey:  *mut Ge,
        message: *const Scalar,
        recid:   i32) -> i32 {
    
    todo!();
        /*
            unsigned char brx[32];
        fe fx;
        ge x;
        gej xj;
        scalar rn, u1, u2;
        gej qj;
        int r;

        if (scalar_is_zero(sigr) || scalar_is_zero(sigs)) {
            return 0;
        }

        scalar_get_b32(brx, sigr);
        r = fe_set_b32(&fx, brx);
        (c_void)r;
        VERIFY_CHECK(r); /* brx comes from a scalar, so is less than the order; certainly less than p */
        if (recid & 2) {
            if (fe_cmp_var(&fx, &ecdsa_const_p_minus_order) >= 0) {
                return 0;
            }
            fe_add(&fx, &ecdsa_const_order_as_fe);
        }
        if (!ge_set_xo_var(&x, &fx, recid & 1)) {
            return 0;
        }
        gej_set_ge(&xj, &x);
        scalar_inverse_var(&rn, sigr);
        scalar_mul(&u1, &rn, message);
        scalar_negate(&u1, &u1);
        scalar_mul(&u2, &rn, sigs);
        ecmult(ctx, &qj, &xj, &u2, &u1);
        ge_set_gej_var(pubkey, &qj);
        return !gej_is_infinity(&qj);
        */
}

pub fn ecdsa_sign_recoverable(
        ctx:       *const Secp256k1Context,
        signature: *mut EcdsaRecoverableSignature,
        msghash32: *const u8,
        seckey:    *const u8,
        noncefp:   NonceFunction,
        noncedata: *const c_void) -> i32 {
    
    todo!();
        /*
            scalar r, s;
        int ret, recid;
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(ecmult_gen_context_is_built(&ctx->ecmult_gen_ctx));
        ARG_CHECK(msghash32 != NULL);
        ARG_CHECK(signature != NULL);
        ARG_CHECK(seckey != NULL);

        ret = ecdsa_sign_inner(ctx, &r, &s, &recid, msghash32, seckey, noncefp, noncedata);
        ecdsa_recoverable_signature_save(signature, &r, &s, recid);
        return ret;
        */
}

pub fn ecdsa_recover(
        ctx:       *const Secp256k1Context,
        pubkey:    *mut PubKey,
        signature: *const EcdsaRecoverableSignature,
        msghash32: *const u8) -> i32 {
    
    todo!();
        /*
            ge q;
        scalar r, s;
        scalar m;
        int recid;
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(ecmult_context_is_built(&ctx->ecmult_ctx));
        ARG_CHECK(msghash32 != NULL);
        ARG_CHECK(signature != NULL);
        ARG_CHECK(pubkey != NULL);

        ecdsa_recoverable_signature_load(ctx, &r, &s, &recid, signature);
        VERIFY_CHECK(recid >= 0 && recid < 4);  /* should have been caught in parse_compact */
        scalar_set_b32(&m, msghash32, NULL);
        if (ecdsa_sig_recover(&ctx->ecmult_ctx, &r, &s, &q, &m, recid)) {
            pubkey_save(pubkey, &q);
            return 1;
        } else {
            memset(pubkey, 0, sizeof(*pubkey));
            return 0;
        }
        */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/modules/recovery/tests_impl.h]

pub fn recovery_test_nonce_function(
        nonce32: *mut u8,
        msg32:   *const u8,
        key32:   *const u8,
        algo16:  *const u8,
        data:    *mut c_void,
        counter: u32) -> i32 {
    
    todo!();
        /*
            (c_void) msg32;
        (c_void) key32;
        (c_void) algo16;
        (c_void) data;

        /* On the first run, return 0 to force a second run */
        if (counter == 0) {
            memset(nonce32, 0, 32);
            return 1;
        }
        /* On the second run, return an overflow to force a third run */
        if (counter == 1) {
            memset(nonce32, 0xff, 32);
            return 1;
        }
        /* On the next run, return a valid nonce, but flip a coin as to whether or not to fail signing. */
        memset(nonce32, 1, 32);
        return testrand_bits(1);
        */
}

pub fn test_ecdsa_recovery_api()  {
    
    todo!();
        /*
            /* Setup contexts that just count errors */
        context *none = context_create(CONTEXT_NONE);
        context *sign = context_create(CONTEXT_SIGN);
        context *vrfy = context_create(CONTEXT_VERIFY);
        context *both = context_create(CONTEXT_SIGN | CONTEXT_VERIFY);
        pubkey pubkey;
        pubkey recpubkey;
        ecdsa_signature normal_sig;
        ecdsa_recoverable_signature recsig;
        unsigned char privkey[32] = { 1 };
        unsigned char message[32] = { 2 };
        int32_t ecount = 0;
        int recid = 0;
        unsigned char sig[74];
        unsigned char zero_privkey[32] = { 0 };
        unsigned char over_privkey[32] = { 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                                           0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                                           0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                                           0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff };

        context_set_error_callback(none, counting_illegal_callback_fn, &ecount);
        context_set_error_callback(sign, counting_illegal_callback_fn, &ecount);
        context_set_error_callback(vrfy, counting_illegal_callback_fn, &ecount);
        context_set_error_callback(both, counting_illegal_callback_fn, &ecount);
        context_set_illegal_callback(none, counting_illegal_callback_fn, &ecount);
        context_set_illegal_callback(sign, counting_illegal_callback_fn, &ecount);
        context_set_illegal_callback(vrfy, counting_illegal_callback_fn, &ecount);
        context_set_illegal_callback(both, counting_illegal_callback_fn, &ecount);

        /* Construct and verify corresponding public key. */
        CHECK(ec_seckey_verify(ctx, privkey) == 1);
        CHECK(ec_pubkey_create(ctx, &pubkey, privkey) == 1);

        /* Check bad contexts and NULLs for signing */
        ecount = 0;
        CHECK(ecdsa_sign_recoverable(none, &recsig, message, privkey, NULL, NULL) == 0);
        CHECK(ecount == 1);
        CHECK(ecdsa_sign_recoverable(sign, &recsig, message, privkey, NULL, NULL) == 1);
        CHECK(ecount == 1);
        CHECK(ecdsa_sign_recoverable(vrfy, &recsig, message, privkey, NULL, NULL) == 0);
        CHECK(ecount == 2);
        CHECK(ecdsa_sign_recoverable(both, &recsig, message, privkey, NULL, NULL) == 1);
        CHECK(ecount == 2);
        CHECK(ecdsa_sign_recoverable(both, NULL, message, privkey, NULL, NULL) == 0);
        CHECK(ecount == 3);
        CHECK(ecdsa_sign_recoverable(both, &recsig, NULL, privkey, NULL, NULL) == 0);
        CHECK(ecount == 4);
        CHECK(ecdsa_sign_recoverable(both, &recsig, message, NULL, NULL, NULL) == 0);
        CHECK(ecount == 5);
        /* This will fail or succeed randomly, and in either case will not ARG_CHECK failure */
        ecdsa_sign_recoverable(both, &recsig, message, privkey, recovery_test_nonce_function, NULL);
        CHECK(ecount == 5);
        /* These will all fail, but not in ARG_CHECK way */
        CHECK(ecdsa_sign_recoverable(both, &recsig, message, zero_privkey, NULL, NULL) == 0);
        CHECK(ecdsa_sign_recoverable(both, &recsig, message, over_privkey, NULL, NULL) == 0);
        /* This one will succeed. */
        CHECK(ecdsa_sign_recoverable(both, &recsig, message, privkey, NULL, NULL) == 1);
        CHECK(ecount == 5);

        /* Check signing with a goofy nonce function */

        /* Check bad contexts and NULLs for recovery */
        ecount = 0;
        CHECK(ecdsa_recover(none, &recpubkey, &recsig, message) == 0);
        CHECK(ecount == 1);
        CHECK(ecdsa_recover(sign, &recpubkey, &recsig, message) == 0);
        CHECK(ecount == 2);
        CHECK(ecdsa_recover(vrfy, &recpubkey, &recsig, message) == 1);
        CHECK(ecount == 2);
        CHECK(ecdsa_recover(both, &recpubkey, &recsig, message) == 1);
        CHECK(ecount == 2);
        CHECK(ecdsa_recover(both, NULL, &recsig, message) == 0);
        CHECK(ecount == 3);
        CHECK(ecdsa_recover(both, &recpubkey, NULL, message) == 0);
        CHECK(ecount == 4);
        CHECK(ecdsa_recover(both, &recpubkey, &recsig, NULL) == 0);
        CHECK(ecount == 5);

        /* Check NULLs for conversion */
        CHECK(ecdsa_sign(both, &normal_sig, message, privkey, NULL, NULL) == 1);
        ecount = 0;
        CHECK(ecdsa_recoverable_signature_convert(both, NULL, &recsig) == 0);
        CHECK(ecount == 1);
        CHECK(ecdsa_recoverable_signature_convert(both, &normal_sig, NULL) == 0);
        CHECK(ecount == 2);
        CHECK(ecdsa_recoverable_signature_convert(both, &normal_sig, &recsig) == 1);

        /* Check NULLs for de/serialization */
        CHECK(ecdsa_sign_recoverable(both, &recsig, message, privkey, NULL, NULL) == 1);
        ecount = 0;
        CHECK(ecdsa_recoverable_signature_serialize_compact(both, NULL, &recid, &recsig) == 0);
        CHECK(ecount == 1);
        CHECK(ecdsa_recoverable_signature_serialize_compact(both, sig, NULL, &recsig) == 0);
        CHECK(ecount == 2);
        CHECK(ecdsa_recoverable_signature_serialize_compact(both, sig, &recid, NULL) == 0);
        CHECK(ecount == 3);
        CHECK(ecdsa_recoverable_signature_serialize_compact(both, sig, &recid, &recsig) == 1);

        CHECK(ecdsa_recoverable_signature_parse_compact(both, NULL, sig, recid) == 0);
        CHECK(ecount == 4);
        CHECK(ecdsa_recoverable_signature_parse_compact(both, &recsig, NULL, recid) == 0);
        CHECK(ecount == 5);
        CHECK(ecdsa_recoverable_signature_parse_compact(both, &recsig, sig, -1) == 0);
        CHECK(ecount == 6);
        CHECK(ecdsa_recoverable_signature_parse_compact(both, &recsig, sig, 5) == 0);
        CHECK(ecount == 7);
        /* overflow in signature will fail but not affect ecount */
        memcpy(sig, over_privkey, 32);
        CHECK(ecdsa_recoverable_signature_parse_compact(both, &recsig, sig, recid) == 0);
        CHECK(ecount == 7);

        /* cleanup */
        context_destroy(none);
        context_destroy(sign);
        context_destroy(vrfy);
        context_destroy(both);
        */
}

pub fn test_ecdsa_recovery_end_to_end()  {
    
    todo!();
        /*
            unsigned char extra[32] = {0x00};
        unsigned char privkey[32];
        unsigned char message[32];
        ecdsa_signature signature[5];
        ecdsa_recoverable_signature rsignature[5];
        unsigned char sig[74];
        pubkey pubkey;
        pubkey recpubkey;
        int recid = 0;

        /* Generate a random key and message. */
        {
            scalar msg, key;
            random_scalar_order_test(&msg);
            random_scalar_order_test(&key);
            scalar_get_b32(privkey, &key);
            scalar_get_b32(message, &msg);
        }

        /* Construct and verify corresponding public key. */
        CHECK(ec_seckey_verify(ctx, privkey) == 1);
        CHECK(ec_pubkey_create(ctx, &pubkey, privkey) == 1);

        /* Serialize/parse compact and verify/recover. */
        extra[0] = 0;
        CHECK(ecdsa_sign_recoverable(ctx, &rsignature[0], message, privkey, NULL, NULL) == 1);
        CHECK(ecdsa_sign(ctx, &signature[0], message, privkey, NULL, NULL) == 1);
        CHECK(ecdsa_sign_recoverable(ctx, &rsignature[4], message, privkey, NULL, NULL) == 1);
        CHECK(ecdsa_sign_recoverable(ctx, &rsignature[1], message, privkey, NULL, extra) == 1);
        extra[31] = 1;
        CHECK(ecdsa_sign_recoverable(ctx, &rsignature[2], message, privkey, NULL, extra) == 1);
        extra[31] = 0;
        extra[0] = 1;
        CHECK(ecdsa_sign_recoverable(ctx, &rsignature[3], message, privkey, NULL, extra) == 1);
        CHECK(ecdsa_recoverable_signature_serialize_compact(ctx, sig, &recid, &rsignature[4]) == 1);
        CHECK(ecdsa_recoverable_signature_convert(ctx, &signature[4], &rsignature[4]) == 1);
        CHECK(memcmp_var(&signature[4], &signature[0], 64) == 0);
        CHECK(ecdsa_verify(ctx, &signature[4], message, &pubkey) == 1);
        memset(&rsignature[4], 0, sizeof(rsignature[4]));
        CHECK(ecdsa_recoverable_signature_parse_compact(ctx, &rsignature[4], sig, recid) == 1);
        CHECK(ecdsa_recoverable_signature_convert(ctx, &signature[4], &rsignature[4]) == 1);
        CHECK(ecdsa_verify(ctx, &signature[4], message, &pubkey) == 1);
        /* Parse compact (with recovery id) and recover. */
        CHECK(ecdsa_recoverable_signature_parse_compact(ctx, &rsignature[4], sig, recid) == 1);
        CHECK(ecdsa_recover(ctx, &recpubkey, &rsignature[4], message) == 1);
        CHECK(memcmp_var(&pubkey, &recpubkey, sizeof(pubkey)) == 0);
        /* Serialize/destroy/parse signature and verify again. */
        CHECK(ecdsa_recoverable_signature_serialize_compact(ctx, sig, &recid, &rsignature[4]) == 1);
        sig[testrand_bits(6)] += 1 + testrand_int(255);
        CHECK(ecdsa_recoverable_signature_parse_compact(ctx, &rsignature[4], sig, recid) == 1);
        CHECK(ecdsa_recoverable_signature_convert(ctx, &signature[4], &rsignature[4]) == 1);
        CHECK(ecdsa_verify(ctx, &signature[4], message, &pubkey) == 0);
        /* Recover again */
        CHECK(ecdsa_recover(ctx, &recpubkey, &rsignature[4], message) == 0 ||
              memcmp_var(&pubkey, &recpubkey, sizeof(pubkey)) != 0);
        */
}

/**
  | Tests several edge cases.
  |
  */
pub fn test_ecdsa_recovery_edge_cases()  {
    
    todo!();
        /*
            const unsigned char msg32[32] = {
            'T', 'h', 'i', 's', ' ', 'i', 's', ' ',
            'a', ' ', 'v', 'e', 'r', 'y', ' ', 's',
            'e', 'c', 'r', 'e', 't', ' ', 'm', 'e',
            's', 's', 'a', 'g', 'e', '.', '.', '.'
        };
        const unsigned char sig64[64] = {
            /* Generated by signing the above message with nonce 'This is the nonce we will use...'
             * and secret key 0 (which is not valid), resulting in recid 1. */
            0x67, 0xCB, 0x28, 0x5F, 0x9C, 0xD1, 0x94, 0xE8,
            0x40, 0xD6, 0x29, 0x39, 0x7A, 0xF5, 0x56, 0x96,
            0x62, 0xFD, 0xE4, 0x46, 0x49, 0x99, 0x59, 0x63,
            0x17, 0x9A, 0x7D, 0xD1, 0x7B, 0xD2, 0x35, 0x32,
            0x4B, 0x1B, 0x7D, 0xF3, 0x4C, 0xE1, 0xF6, 0x8E,
            0x69, 0x4F, 0xF6, 0xF1, 0x1A, 0xC7, 0x51, 0xDD,
            0x7D, 0xD7, 0x3E, 0x38, 0x7E, 0xE4, 0xFC, 0x86,
            0x6E, 0x1B, 0xE8, 0xEC, 0xC7, 0xDD, 0x95, 0x57
        };
        pubkey pubkey;
        /* signature (r,s) = (4,4), which can be recovered with all 4 recids. */
        const unsigned char sigb64[64] = {
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
        };
        pubkey pubkeyb;
        ecdsa_recoverable_signature rsig;
        ecdsa_signature sig;
        int recid;

        CHECK(ecdsa_recoverable_signature_parse_compact(ctx, &rsig, sig64, 0));
        CHECK(!ecdsa_recover(ctx, &pubkey, &rsig, msg32));
        CHECK(ecdsa_recoverable_signature_parse_compact(ctx, &rsig, sig64, 1));
        CHECK(ecdsa_recover(ctx, &pubkey, &rsig, msg32));
        CHECK(ecdsa_recoverable_signature_parse_compact(ctx, &rsig, sig64, 2));
        CHECK(!ecdsa_recover(ctx, &pubkey, &rsig, msg32));
        CHECK(ecdsa_recoverable_signature_parse_compact(ctx, &rsig, sig64, 3));
        CHECK(!ecdsa_recover(ctx, &pubkey, &rsig, msg32));

        for (recid = 0; recid < 4; recid++) {
            int i;
            int recid2;
            /* (4,4) encoded in DER. */
            unsigned char sigbder[8] = {0x30, 0x06, 0x02, 0x01, 0x04, 0x02, 0x01, 0x04};
            unsigned char sigcder_zr[7] = {0x30, 0x05, 0x02, 0x00, 0x02, 0x01, 0x01};
            unsigned char sigcder_zs[7] = {0x30, 0x05, 0x02, 0x01, 0x01, 0x02, 0x00};
            unsigned char sigbderalt1[39] = {
                0x30, 0x25, 0x02, 0x20, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x04, 0x02, 0x01, 0x04,
            };
            unsigned char sigbderalt2[39] = {
                0x30, 0x25, 0x02, 0x01, 0x04, 0x02, 0x20, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
            };
            unsigned char sigbderalt3[40] = {
                0x30, 0x26, 0x02, 0x21, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x04, 0x02, 0x01, 0x04,
            };
            unsigned char sigbderalt4[40] = {
                0x30, 0x26, 0x02, 0x01, 0x04, 0x02, 0x21, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
            };
            /* (order + r,4) encoded in DER. */
            unsigned char sigbderlong[40] = {
                0x30, 0x26, 0x02, 0x21, 0x00, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xBA, 0xAE, 0xDC,
                0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E,
                0x8C, 0xD0, 0x36, 0x41, 0x45, 0x02, 0x01, 0x04
            };
            CHECK(ecdsa_recoverable_signature_parse_compact(ctx, &rsig, sigb64, recid) == 1);
            CHECK(ecdsa_recover(ctx, &pubkeyb, &rsig, msg32) == 1);
            CHECK(ecdsa_signature_parse_der(ctx, &sig, sigbder, sizeof(sigbder)) == 1);
            CHECK(ecdsa_verify(ctx, &sig, msg32, &pubkeyb) == 1);
            for (recid2 = 0; recid2 < 4; recid2++) {
                pubkey pubkey2b;
                CHECK(ecdsa_recoverable_signature_parse_compact(ctx, &rsig, sigb64, recid2) == 1);
                CHECK(ecdsa_recover(ctx, &pubkey2b, &rsig, msg32) == 1);
                /* Verifying with (order + r,4) should always fail. */
                CHECK(ecdsa_signature_parse_der(ctx, &sig, sigbderlong, sizeof(sigbderlong)) == 1);
                CHECK(ecdsa_verify(ctx, &sig, msg32, &pubkeyb) == 0);
            }
            /* DER parsing tests. */
            /* Zero length r/s. */
            CHECK(ecdsa_signature_parse_der(ctx, &sig, sigcder_zr, sizeof(sigcder_zr)) == 0);
            CHECK(ecdsa_signature_parse_der(ctx, &sig, sigcder_zs, sizeof(sigcder_zs)) == 0);
            /* Leading zeros. */
            CHECK(ecdsa_signature_parse_der(ctx, &sig, sigbderalt1, sizeof(sigbderalt1)) == 0);
            CHECK(ecdsa_signature_parse_der(ctx, &sig, sigbderalt2, sizeof(sigbderalt2)) == 0);
            CHECK(ecdsa_signature_parse_der(ctx, &sig, sigbderalt3, sizeof(sigbderalt3)) == 0);
            CHECK(ecdsa_signature_parse_der(ctx, &sig, sigbderalt4, sizeof(sigbderalt4)) == 0);
            sigbderalt3[4] = 1;
            CHECK(ecdsa_signature_parse_der(ctx, &sig, sigbderalt3, sizeof(sigbderalt3)) == 1);
            CHECK(ecdsa_verify(ctx, &sig, msg32, &pubkeyb) == 0);
            sigbderalt4[7] = 1;
            CHECK(ecdsa_signature_parse_der(ctx, &sig, sigbderalt4, sizeof(sigbderalt4)) == 1);
            CHECK(ecdsa_verify(ctx, &sig, msg32, &pubkeyb) == 0);
            /* Damage signature. */
            sigbder[7]++;
            CHECK(ecdsa_signature_parse_der(ctx, &sig, sigbder, sizeof(sigbder)) == 1);
            CHECK(ecdsa_verify(ctx, &sig, msg32, &pubkeyb) == 0);
            sigbder[7]--;
            CHECK(ecdsa_signature_parse_der(ctx, &sig, sigbder, 6) == 0);
            CHECK(ecdsa_signature_parse_der(ctx, &sig, sigbder, sizeof(sigbder) - 1) == 0);
            for(i = 0; i < 8; i++) {
                int c;
                unsigned char orig = sigbder[i];
                /*Try every single-byte change.*/
                for (c = 0; c < 256; c++) {
                    if (c == orig ) {
                        continue;
                    }
                    sigbder[i] = c;
                    CHECK(ecdsa_signature_parse_der(ctx, &sig, sigbder, sizeof(sigbder)) == 0 || ecdsa_verify(ctx, &sig, msg32, &pubkeyb) == 0);
                }
                sigbder[i] = orig;
            }
        }

        /* Test r/s equal to zero */
        {
            /* (1,1) encoded in DER. */
            unsigned char sigcder[8] = {0x30, 0x06, 0x02, 0x01, 0x01, 0x02, 0x01, 0x01};
            unsigned char sigc64[64] = {
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            };
            pubkey pubkeyc;
            CHECK(ecdsa_recoverable_signature_parse_compact(ctx, &rsig, sigc64, 0) == 1);
            CHECK(ecdsa_recover(ctx, &pubkeyc, &rsig, msg32) == 1);
            CHECK(ecdsa_signature_parse_der(ctx, &sig, sigcder, sizeof(sigcder)) == 1);
            CHECK(ecdsa_verify(ctx, &sig, msg32, &pubkeyc) == 1);
            sigcder[4] = 0;
            sigc64[31] = 0;
            CHECK(ecdsa_recoverable_signature_parse_compact(ctx, &rsig, sigc64, 0) == 1);
            CHECK(ecdsa_recover(ctx, &pubkeyb, &rsig, msg32) == 0);
            CHECK(ecdsa_signature_parse_der(ctx, &sig, sigcder, sizeof(sigcder)) == 1);
            CHECK(ecdsa_verify(ctx, &sig, msg32, &pubkeyc) == 0);
            sigcder[4] = 1;
            sigcder[7] = 0;
            sigc64[31] = 1;
            sigc64[63] = 0;
            CHECK(ecdsa_recoverable_signature_parse_compact(ctx, &rsig, sigc64, 0) == 1);
            CHECK(ecdsa_recover(ctx, &pubkeyb, &rsig, msg32) == 0);
            CHECK(ecdsa_signature_parse_der(ctx, &sig, sigcder, sizeof(sigcder)) == 1);
            CHECK(ecdsa_verify(ctx, &sig, msg32, &pubkeyc) == 0);
        }
        */
}

pub fn run_recovery_tests()  {
    
    todo!();
        /*
            int i;
        for (i = 0; i < count; i++) {
            test_ecdsa_recovery_api();
        }
        for (i = 0; i < 64*count; i++) {
            test_ecdsa_recovery_end_to_end();
        }
        test_ecdsa_recovery_edge_cases();
        */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/modules/recovery/tests_exhaustive_impl.h]

pub fn test_exhaustive_recovery_sign(
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
                    secp256k1_fe r_dot_y_normalized;
                    secp256k1_ecdsa_recoverable_signature rsig;
                    secp256k1_ecdsa_signature sig;
                    secp256k1_scalar sk, msg, r, s, expected_r;
                    unsigned char sk32[32], msg32[32];
                    int expected_recid;
                    int recid;
                    int overflow;
                    secp256k1_scalar_set_int(&msg, i);
                    secp256k1_scalar_set_int(&sk, j);
                    secp256k1_scalar_get_b32(sk32, &sk);
                    secp256k1_scalar_get_b32(msg32, &msg);

                    secp256k1_ecdsa_sign_recoverable(ctx, &rsig, msg32, sk32, secp256k1_nonce_function_smallint, &k);

                    /* Check directly */
                    secp256k1_ecdsa_recoverable_signature_load(ctx, &r, &s, &recid, &rsig);
                    r_from_k(&expected_r, group, k, &overflow);
                    CHECK(r == expected_r);
                    CHECK((k * s) % EXHAUSTIVE_TEST_ORDER == (i + r * j) % EXHAUSTIVE_TEST_ORDER ||
                          (k * (EXHAUSTIVE_TEST_ORDER - s)) % EXHAUSTIVE_TEST_ORDER == (i + r * j) % EXHAUSTIVE_TEST_ORDER);
                    /* The recid's second bit is for conveying overflow (R.x value >= group order).
                     * In the actual secp256k1 this is an astronomically unlikely event, but in the
                     * small group used here, it will be the case for all points except the ones where
                     * R.x=1 (which the group is specifically selected to have).
                     * Note that this isn't actually useful; full recovery would need to convey
                     * floor(R.x / group_order), but only one bit is used as that is sufficient
                     * in the real group. */
                    expected_recid = overflow ? 2 : 0;
                    r_dot_y_normalized = group[k].y;
                    secp256k1_fe_normalize(&r_dot_y_normalized);
                    /* Also the recovery id is flipped depending if we hit the low-s branch */
                    if ((k * s) % EXHAUSTIVE_TEST_ORDER == (i + r * j) % EXHAUSTIVE_TEST_ORDER) {
                        expected_recid |= secp256k1_fe_is_odd(&r_dot_y_normalized);
                    } else {
                        expected_recid |= !secp256k1_fe_is_odd(&r_dot_y_normalized);
                    }
                    CHECK(recid == expected_recid);

                    /* Convert to a standard sig then check */
                    secp256k1_ecdsa_recoverable_signature_convert(ctx, &sig, &rsig);
                    secp256k1_ecdsa_signature_load(ctx, &r, &s, &sig);
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
        */
}

pub fn test_exhaustive_recovery_verify(
        ctx:   *const Secp256k1Context,
        group: *const Ge)  {
    
    todo!();
        /*
            /* This is essentially a copy of test_exhaustive_verify, with recovery added */
        int s, r, msg, key;
        uint64_t iter = 0;
        for (s = 1; s < EXHAUSTIVE_TEST_ORDER; s++) {
            for (r = 1; r < EXHAUSTIVE_TEST_ORDER; r++) {
                for (msg = 1; msg < EXHAUSTIVE_TEST_ORDER; msg++) {
                    for (key = 1; key < EXHAUSTIVE_TEST_ORDER; key++) {
                        secp256k1_ge nonconst_ge;
                        secp256k1_ecdsa_recoverable_signature rsig;
                        secp256k1_ecdsa_signature sig;
                        secp256k1_pubkey pk;
                        secp256k1_scalar sk_s, msg_s, r_s, s_s;
                        secp256k1_scalar s_times_k_s, msg_plus_r_times_sk_s;
                        int recid = 0;
                        int k, should_verify;
                        unsigned char msg32[32];

                        if (skip_section(&iter)) continue;

                        secp256k1_scalar_set_int(&s_s, s);
                        secp256k1_scalar_set_int(&r_s, r);
                        secp256k1_scalar_set_int(&msg_s, msg);
                        secp256k1_scalar_set_int(&sk_s, key);
                        secp256k1_scalar_get_b32(msg32, &msg_s);

                        /* Verify by hand */
                        /* Run through every k value that gives us this r and check that *one* works.
                         * Note there could be none, there could be multiple, ECDSA is weird. */
                        should_verify = 0;
                        for (k = 0; k < EXHAUSTIVE_TEST_ORDER; k++) {
                            secp256k1_scalar check_x_s;
                            r_from_k(&check_x_s, group, k, NULL);
                            if (r_s == check_x_s) {
                                secp256k1_scalar_set_int(&s_times_k_s, k);
                                secp256k1_scalar_mul(&s_times_k_s, &s_times_k_s, &s_s);
                                secp256k1_scalar_mul(&msg_plus_r_times_sk_s, &r_s, &sk_s);
                                secp256k1_scalar_add(&msg_plus_r_times_sk_s, &msg_plus_r_times_sk_s, &msg_s);
                                should_verify |= secp256k1_scalar_eq(&s_times_k_s, &msg_plus_r_times_sk_s);
                            }
                        }
                        /* nb we have a "high s" rule */
                        should_verify &= !secp256k1_scalar_is_high(&s_s);

                        /* We would like to try recovering the pubkey and checking that it matches,
                         * but pubkey recovery is impossible in the exhaustive tests (the reason
                         * being that there are 12 nonzero r values, 12 nonzero points, and no
                         * overlap between the sets, so there are no valid signatures). */

                        /* Verify by converting to a standard signature and calling verify */
                        secp256k1_ecdsa_recoverable_signature_save(&rsig, &r_s, &s_s, recid);
                        secp256k1_ecdsa_recoverable_signature_convert(ctx, &sig, &rsig);
                        memcpy(&nonconst_ge, &group[sk_s], sizeof(nonconst_ge));
                        secp256k1_pubkey_save(&pk, &nonconst_ge);
                        CHECK(should_verify ==
                              secp256k1_ecdsa_verify(ctx, &sig, msg32, &pk));
                    }
                }
            }
        }
        */
}

pub fn test_exhaustive_recovery(
        ctx:   *const Secp256k1Context,
        group: *const Ge)  {
    
    todo!();
        /*
            test_exhaustive_recovery_sign(ctx, group);
        test_exhaustive_recovery_verify(ctx, group);
        */
}
