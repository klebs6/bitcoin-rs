crate::ix!();

/**
  | Opaque data structured that holds a
  | parsed ECDSA signature.
  | 
  | The exact representation of data inside
  | is implementation defined and not guaranteed
  | to be portable between different platforms
  | or versions. It is however guaranteed
  | to be 64 bytes in size, and can be safely
  | copied/moved.
  | 
  | If you need to convert to a format suitable
  | for storage, transmission, or comparison,
  | use the ecdsa_signature_serialize_*
  | and ecdsa_signature_parse_* functions.
  |
  */
pub struct Secp256k1EcdsaSignature {
    data: [u8; 64],
}

pub fn ecdsa_signature_load(
        ctx: *const Secp256k1Context,
        r:   *mut Scalar,
        s:   *mut Scalar,
        sig: *const Secp256k1EcdsaSignature)  {
    
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
        */
}

pub fn ecdsa_signature_save(
        sig: *mut Secp256k1EcdsaSignature,
        r:   *const Scalar,
        s:   *const Scalar)  {
    
    todo!();
        /*
            if (sizeof(scalar) == 32) {
            memcpy(&sig->data[0], r, 32);
            memcpy(&sig->data[32], s, 32);
        } else {
            scalar_get_b32(&sig->data[0], r);
            scalar_get_b32(&sig->data[32], s);
        }
        */
}

pub fn ecdsa_signature_parse_der(
        ctx:      *const Secp256k1Context,
        sig:      *mut Secp256k1EcdsaSignature,
        input:    *const u8,
        inputlen: usize) -> i32 {
    
    todo!();
        /*
            scalar r, s;

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(sig != NULL);
        ARG_CHECK(input != NULL);

        if (ecdsa_sig_parse(&r, &s, input, inputlen)) {
            ecdsa_signature_save(sig, &r, &s);
            return 1;
        } else {
            memset(sig, 0, sizeof(*sig));
            return 0;
        }
        */
}

pub fn ecdsa_signature_parse_compact(
        ctx:     *const Secp256k1Context,
        sig:     *mut Secp256k1EcdsaSignature,
        input64: *const u8) -> i32 {
    
    todo!();
        /*
            scalar r, s;
        int ret = 1;
        int overflow = 0;

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(sig != NULL);
        ARG_CHECK(input64 != NULL);

        scalar_set_b32(&r, &input64[0], &overflow);
        ret &= !overflow;
        scalar_set_b32(&s, &input64[32], &overflow);
        ret &= !overflow;
        if (ret) {
            ecdsa_signature_save(sig, &r, &s);
        } else {
            memset(sig, 0, sizeof(*sig));
        }
        return ret;
        */
}

pub fn ecdsa_signature_serialize_der(
        ctx:       *const Secp256k1Context,
        output:    *mut u8,
        outputlen: *mut usize,
        sig:       *const Secp256k1EcdsaSignature) -> i32 {
    
    todo!();
        /*
            scalar r, s;

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(output != NULL);
        ARG_CHECK(outputlen != NULL);
        ARG_CHECK(sig != NULL);

        ecdsa_signature_load(ctx, &r, &s, sig);
        return ecdsa_sig_serialize(output, outputlen, &r, &s);
        */
}

pub fn ecdsa_signature_serialize_compact(
        ctx:      *const Secp256k1Context,
        output64: *mut u8,
        sig:      *const Secp256k1EcdsaSignature) -> i32 {
    
    todo!();
        /*
            scalar r, s;

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(output64 != NULL);
        ARG_CHECK(sig != NULL);

        ecdsa_signature_load(ctx, &r, &s, sig);
        scalar_get_b32(&output64[0], &r);
        scalar_get_b32(&output64[32], &s);
        return 1;
        */
}

pub fn ecdsa_signature_normalize(
        ctx:    *const Secp256k1Context,
        sigout: *mut Secp256k1EcdsaSignature,
        sigin:  *const Secp256k1EcdsaSignature) -> i32 {
    
    todo!();
        /*
            scalar r, s;
        int ret = 0;

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(sigin != NULL);

        ecdsa_signature_load(ctx, &r, &s, sigin);
        ret = scalar_is_high(&s);
        if (sigout != NULL) {
            if (ret) {
                scalar_negate(&s, &s);
            }
            ecdsa_signature_save(sigout, &r, &s);
        }

        return ret;
        */
}

pub fn ecdsa_verify(
        ctx:       *const Secp256k1Context,
        sig:       *const Secp256k1EcdsaSignature,
        msghash32: *const u8,
        pubkey:    *const PubKey) -> i32 {
    
    todo!();
        /*
            ge q;
        scalar r, s;
        scalar m;
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(ecmult_context_is_built(&ctx->ecmult_ctx));
        ARG_CHECK(msghash32 != NULL);
        ARG_CHECK(sig != NULL);
        ARG_CHECK(pubkey != NULL);

        scalar_set_b32(&m, msghash32, NULL);
        ecdsa_signature_load(ctx, &r, &s, sig);
        return (!scalar_is_high(&s) &&
                pubkey_load(ctx, &q, pubkey) &&
                ecdsa_sig_verify(&ctx->ecmult_ctx, &r, &s, &q, &m));
        */
}

/**
  | Check that the sig has a low R value and
  | will be less than 71 bytes
  |
  */
pub fn sig_has_lowr(sig: *const Secp256k1EcdsaSignature) -> bool {
    
    todo!();
        /*
            unsigned char compact_sig[64];
        secp256k1_ecdsa_signature_serialize_compact(secp256k1_context_sign, compact_sig, sig);

        // In DER serialization, all values are interpreted as big-endian, signed integers. The highest bit in the integer indicates
        // its signed-ness; 0 is positive, 1 is negative. When the value is interpreted as a negative integer, it must be converted
        // to a positive value by prepending a 0x00 byte so that the highest bit is 0. We can avoid this prepending by ensuring that
        // our highest bit is always 0, and thus we must check that the first byte is less than 0x80.
        return compact_sig[0] < 0x80;
        */
}

pub fn ecdsa_sign(
        ctx:       *const Secp256k1Context,
        signature: *mut Secp256k1EcdsaSignature,
        msghash32: *const u8,
        seckey:    *const u8,
        noncefp:   NonceFunction,
        noncedata: *const c_void) -> i32 {
    
    todo!();
        /*
            scalar r, s;
        int ret;
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(ecmult_gen_context_is_built(&ctx->ecmult_gen_ctx));
        ARG_CHECK(msghash32 != NULL);
        ARG_CHECK(signature != NULL);
        ARG_CHECK(seckey != NULL);

        ret = ecdsa_sign_inner(ctx, &r, &s, NULL, msghash32, seckey, noncefp, noncedata);
        ecdsa_signature_save(signature, &r, &s);
        return ret;
        */
}
