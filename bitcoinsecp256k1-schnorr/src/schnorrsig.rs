/*!
  | This module implements a variant of
  | Schnorr signatures compliant with
  | 
  | Bitcoin Improvement Proposal 340 "Schnorr
  | Signatures for secp256k1" 
  | (https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki).
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/include/schnorrsig.h]

/** 
 | A pointer to a function to deterministically
 | generate a nonce.
 |
 |  Same as nonce function with the
 |  exception of accepting an additional pubkey
 |  argument and not requiring an attempt
 |  argument. The pubkey argument can protect
 |  signature schemes with key-prefixed challenge
 |  hash inputs against reusing the nonce when
 |  signing with the wrong precomputed pubkey.
 |
 |  Returns: 1 if a nonce was successfully
 |           generated. 0 will cause signing to
 |           return an error.
 |
 |  Out:  nonce32: pointer to a 32-byte array to
 |                 be filled by the function
 |
 |  In:       msg: the message being verified. Is
 |                 NULL if and only if msglen is
 |                 0.
 |
 |         msglen: the length of the message
 |
 |          key32: pointer to a 32-byte secret key
 |                 (will not be NULL)
 |
 |     xonly_pk32: the 32-byte serialized xonly
 |                 pubkey corresponding to key32
 |                 (will not be NULL)
 |
 |           algo: pointer to an array describing
 |                 the signature algorithm (will
 |                 not be NULL)
 |
 |        algolen: the length of the algo array
 |
 |           data: arbitrary data pointer that is
 |           passed through
 |
 |  Except for test cases, this function should
 |  compute some cryptographic hash of the
 |  message, the key, the pubkey, the algorithm
 |  description, and data.
 */
pub type NonceFunctionHardened = fn(
    nonce32:    *mut u8,
    msg:        *const u8,
    msglen:     usize,
    key32:      *const u8,
    xonly_pk32: *const u8,
    algo:       *const u8,
    algolen:    usize,
    data:       *mut c_void
) -> i32;

/**
  | An implementation of the nonce generation
  | function as defined in Bitcoin
  | 
  | Improvement Proposal 340 "Schnorr
  | Signatures for secp256k1" (https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki).
  | 
  | If a data pointer is passed, it is assumed
  | to be a pointer to 32 bytes of auxiliary
  | random data as defined in BIP-340. If
  | the data pointer is NULL, the nonce derivation
  | procedure follows BIP-340 by setting
  | the auxiliary random data to zero. The
  | algo argument must be non-NULL, otherwise
  | the function will fail and return 0.
  | The hash will be tagged with algo.
  | 
  | Therefore, to create BIP-340 compliant
  | signatures, algo must be set to "BIP0340/nonce"
  | and algolen to 13.
  |
  */
lazy_static!{
    /*
    extern const nonce_function_hardened nonce_function_bip340;
    */
}

/** 
 | Data structure that contains additional
 | arguments for schnorrsig_sign_custom.
 |
 | A schnorrsig_extraparams structure object can
 | be initialized correctly by setting it to
 | SCHNORRSIG_EXTRAPARAMS_INIT.
 |
 |  Members:
 |
 |      magic: set to SCHNORRSIG_EXTRAPARAMS_MAGIC
 |             at initialization and has no other
 |             function than making sure the
 |             object is initialized.
 |
 |    noncefp: pointer to a nonce generation
 |             function. If NULL,
 |             nonce_function_bip340 is used
 |
 |      ndata: pointer to arbitrary data used by
 |             the nonce generation function (can
 |             be NULL). If it is non-NULL and
 |             nonce_function_bip340 is used, then
 |             ndata must be a pointer to 32-byte
 |             auxiliary randomness as per
 |             BIP-340.
 */
pub struct SchnorrSigExtraParams {
    magic:   [u8; 4],
    noncefp: NonceFunctionHardened,
    ndata:   *mut c_void,
}

macro_rules! schnorrsig_extraparams_init {
    () => {
        /*
                {
            SCHNORRSIG_EXTRAPARAMS_MAGIC,
            NULL,
            NULL
        }
        */
    }
}

/** 
 | Create a Schnorr signature.
 |
 |  Does _not_ strictly follow BIP-340 because it
 |  does not verify the resulting
 |  signature. Instead, you can manually use
 |  schnorrsig_verify and abort if it fails.
 |
 |  This function only signs 32-byte messages. If
 |  you have messages of a different size (or the
 |  same size but without a context-specific tag
 |  prefix), it is recommended to create a 32-byte
 |  message hash with tagged_sha256 and then sign
 |  the hash. Tagged hashing allows providing an
 |  context-specific tag for domain
 |  separation. This prevents signatures from
 |  being valid in multiple contexts by accident.
 |
 |  Returns 1 on success, 0 on failure.
 |
 |  Args:    ctx: pointer to a context object,
 |                initialized for signing (cannot
 |                be NULL)
 |
 |  Out:   sig64: pointer to a 64-byte array to
 |                store the serialized signature
 |                (cannot be NULL)
 |
 |  In:    msg32: the 32-byte message being signed
 |                (cannot be NULL)
 |
 |       keypair: pointer to an initialized
 |                keypair (cannot be NULL)
 |
 |    aux_rand32: 32 bytes of fresh
 |                randomness. While recommended to
 |                provide this, it is only
 |                supplemental to security and can
 |                be NULL. See BIP-340 "Default
 |                Signing" for a full explanation
 |                of this argument and for
 |                guidance if randomness is
 |                expensive.
 */
lazy_static!{
    /*
    int schnorrsig_sign(
        const context* ctx,
        unsigned char *sig64,
        const unsigned char *msg32,
        const keypair *keypair,
        unsigned char *aux_rand32
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3) ARG_NONNULL(4);
    */
}

/** 
 | Create a Schnorr signature with a more flexible
 | API.
 |
 |  Same arguments as schnorrsig_sign except that
 |  it allows signing variable length messages and
 |  accepts a pointer to an extraparams object
 |  that allows customizing signing by passing
 |  additional arguments.
 |
 |  Creates the same signatures as schnorrsig_sign
 |  if msglen is 32 and the extraparams.ndata is
 |  the same as aux_rand32.
 |
 |  In:     msg: the message being signed. Can
 |               only be NULL if msglen is 0.
 |
 |       msglen: length of the message
 |
 |  extraparams: pointer to a extraparams object
 |               (can be NULL)
 */
lazy_static!{
    /*
    int schnorrsig_sign_custom(
        const context* ctx,
        unsigned char *sig64,
        const unsigned char *msg,
        size_t msglen,
        const keypair *keypair,
        schnorrsig_extraparams *extraparams
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(5);
    */
}

/** 
 | Verify a Schnorr signature.
 |
 |  Returns: 1: correct signature
 |
 |           0: incorrect signature
 |
 |  Args:    ctx: a secp256k1 context object,
 |                initialized for verification.
 |
 |  In:    sig64: pointer to the 64-byte signature
 |                to verify (cannot be NULL)
 |
 |           msg: the message being verified. Can
 |                only be NULL if msglen is 0.
 |
 |        msglen: length of the message
 |
 |        pubkey: pointer to an x-only public key
 |                to verify with (cannot be NULL)
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int schnorrsig_verify(
        const context* ctx,
        const unsigned char *sig64,
        const unsigned char *msg,
        size_t msglen,
        const xonly_pubkey *pubkey
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(5);
    */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/modules/schnorrsig/main_impl.h]

/**
  | Initializes SHA256 with fixed midstate.
  | This midstate was computed by applying
  | 
  | SHA256 to SHA256("BIP0340/nonce")||SHA256("BIP0340/nonce").
  |
  */
pub fn nonce_function_bip340_sha256_tagged(sha: *mut Sha256)  {
    
    todo!();
        /*
            sha256_initialize(sha);
        sha->s[0] = 0x46615b35ul;
        sha->s[1] = 0xf4bfbff7ul;
        sha->s[2] = 0x9f8dc671ul;
        sha->s[3] = 0x83627ab3ul;
        sha->s[4] = 0x60217180ul;
        sha->s[5] = 0x57358661ul;
        sha->s[6] = 0x21a29e54ul;
        sha->s[7] = 0x68b07b4cul;

        sha->bytes = 64;
        */
}

/**
  | Initializes SHA256 with fixed midstate.
  | This midstate was computed by applying
  | 
  | SHA256 to SHA256("BIP0340/aux")||SHA256("BIP0340/aux").
  |
  */
pub fn nonce_function_bip340_sha256_tagged_aux(sha: *mut Sha256)  {
    
    todo!();
        /*
            sha256_initialize(sha);
        sha->s[0] = 0x24dd3219ul;
        sha->s[1] = 0x4eba7e70ul;
        sha->s[2] = 0xca0fabb9ul;
        sha->s[3] = 0x0fa3166dul;
        sha->s[4] = 0x3afbe4b1ul;
        sha->s[5] = 0x4c44df97ul;
        sha->s[6] = 0x4aac2739ul;
        sha->s[7] = 0x249e850aul;

        sha->bytes = 64;
        */
}

/**
  | algo argument for nonce_function_bip340
  | to derive the nonce exactly as stated
  | in BIP-340 by using the correct tagged
  | hash function.
  |
  | NOTE: in c++ this was 13 bytes long
  */
pub const BIP340_ALGO: &'static str = "BIP0340/nonce";

pub const SCHNORRSIG_EXTRAPARAMS_MAGIC: [u8; 4] = [ 0xda, 0x6f, 0xb3, 0x8c ];

pub fn nonce_function_bip340(
        nonce32:    *mut u8,
        msg:        *const u8,
        msglen:     usize,
        key32:      *const u8,
        xonly_pk32: *const u8,
        algo:       *const u8,
        algolen:    usize,
        data:       *mut c_void) -> i32 {
    
    todo!();
        /*
            sha256 sha;
        unsigned char masked_key[32];
        int i;

        if (algo == NULL) {
            return 0;
        }

        if (data != NULL) {
            nonce_function_bip340_sha256_tagged_aux(&sha);
            sha256_write(&sha, data, 32);
            sha256_finalize(&sha, masked_key);
            for (i = 0; i < 32; i++) {
                masked_key[i] ^= key32[i];
            }
        }

        /* Tag the hash with algo which is important to avoid nonce reuse across
         * algorithms. If this nonce function is used in BIP-340 signing as defined
         * in the spec, an optimized tagging implementation is used. */
        if (algolen == sizeof(bip340_algo)
                && memcmp_var(algo, bip340_algo, algolen) == 0) {
            nonce_function_bip340_sha256_tagged(&sha);
        } else {
            sha256_initialize_tagged(&sha, algo, algolen);
        }

        /* Hash (masked-)key||pk||msg using the tagged hash as per the spec */
        if (data != NULL) {
            sha256_write(&sha, masked_key, 32);
        } else {
            sha256_write(&sha, key32, 32);
        }
        sha256_write(&sha, xonly_pk32, 32);
        sha256_write(&sha, msg, msglen);
        sha256_finalize(&sha, nonce32);
        return 1;
        */
}

pub const NONCE_FUNCTION_BIP340: NonceFunctionHardened = nonce_function_bip340;

/**
  | Initializes SHA256 with fixed midstate.
  | This midstate was computed by applying
  | 
  | SHA256 to SHA256("BIP0340/challenge")||SHA256("BIP0340/challenge").
  |
  */
pub fn schnorrsig_sha256_tagged(sha: *mut Sha256)  {
    
    todo!();
        /*
            sha256_initialize(sha);
        sha->s[0] = 0x9cecba11ul;
        sha->s[1] = 0x23925381ul;
        sha->s[2] = 0x11679112ul;
        sha->s[3] = 0xd1627e0ful;
        sha->s[4] = 0x97c87550ul;
        sha->s[5] = 0x003cc765ul;
        sha->s[6] = 0x90f61164ul;
        sha->s[7] = 0x33e9b66aul;
        sha->bytes = 64;
        */
}

pub fn schnorrsig_challenge(
        e:        *mut Scalar,
        r32:      *const u8,
        msg:      *const u8,
        msglen:   usize,
        pubkey32: *const u8)  {
    
    todo!();
        /*
            unsigned char buf[32];
        sha256 sha;

        /* tagged hash(r.x, pk.x, msg) */
        schnorrsig_sha256_tagged(&sha);
        sha256_write(&sha, r32, 32);
        sha256_write(&sha, pubkey32, 32);
        sha256_write(&sha, msg, msglen);
        sha256_finalize(&sha, buf);
        /* Set scalar e to the challenge hash modulo the curve order as per
         * BIP340. */
        scalar_set_b32(e, buf, NULL);
        */
}

pub fn schnorrsig_sign_internal(
        ctx:     *const Secp256k1Context,
        sig64:   *mut u8,
        msg:     *const u8,
        msglen:  usize,
        keypair: *const KeyPair,
        noncefp: NonceFunctionHardened,
        ndata:   *mut c_void) -> i32 {
    
    todo!();
        /*
            scalar sk;
        scalar e;
        scalar k;
        gej rj;
        ge pk;
        ge r;
        unsigned char buf[32] = { 0 };
        unsigned char pk_buf[32];
        unsigned char seckey[32];
        int ret = 1;

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(ecmult_gen_context_is_built(&ctx->ecmult_gen_ctx));
        ARG_CHECK(sig64 != NULL);
        ARG_CHECK(msg != NULL || msglen == 0);
        ARG_CHECK(keypair != NULL);

        if (noncefp == NULL) {
            noncefp = nonce_function_bip340;
        }

        ret &= keypair_load(ctx, &sk, &pk, keypair);
        /* Because we are signing for a x-only pubkey, the secret key is negated
         * before signing if the point corresponding to the secret key does not
         * have an even Y. */
        if (fe_is_odd(&pk.y)) {
            scalar_negate(&sk, &sk);
        }

        scalar_get_b32(seckey, &sk);
        fe_get_b32(pk_buf, &pk.x);
        ret &= !!noncefp(buf, msg, msglen, seckey, pk_buf, bip340_algo, sizeof(bip340_algo), ndata);
        scalar_set_b32(&k, buf, NULL);
        ret &= !scalar_is_zero(&k);
        scalar_cmov(&k, &scalar_one, !ret);

        ecmult_gen(&ctx->ecmult_gen_ctx, &rj, &k);
        ge_set_gej(&r, &rj);

        /* We declassify r to allow using it as a branch point. This is fine
         * because r is not a secret. */
        declassify(ctx, &r, sizeof(r));
        fe_normalize_var(&r.y);
        if (fe_is_odd(&r.y)) {
            scalar_negate(&k, &k);
        }
        fe_normalize_var(&r.x);
        fe_get_b32(&sig64[0], &r.x);

        schnorrsig_challenge(&e, &sig64[0], msg, msglen, pk_buf);
        scalar_mul(&e, &e, &sk);
        scalar_add(&e, &e, &k);
        scalar_get_b32(&sig64[32], &e);

        memczero(sig64, 64, !ret);
        scalar_clear(&k);
        scalar_clear(&sk);
        memset(seckey, 0, sizeof(seckey));

        return ret;
        */
}

pub fn schnorrsig_sign(
        ctx:        *const Secp256k1Context,
        sig64:      *mut u8,
        msg32:      *const u8,
        keypair:    *const KeyPair,
        aux_rand32: *mut u8) -> i32 {
    
    todo!();
        /*
            return schnorrsig_sign_internal(ctx, sig64, msg32, 32, keypair, nonce_function_bip340, aux_rand32);
        */
}

pub fn schnorrsig_sign_custom(
        ctx:         *const Secp256k1Context,
        sig64:       *mut u8,
        msg:         *const u8,
        msglen:      usize,
        keypair:     *const KeyPair,
        extraparams: *mut SchnorrSigExtraParams) -> i32 {
    
    todo!();
        /*
            nonce_function_hardened noncefp = NULL;
        c_void *ndata = NULL;
        VERIFY_CHECK(ctx != NULL);

        if (extraparams != NULL) {
            ARG_CHECK(memcmp_var(extraparams->magic,
                                           schnorrsig_extraparams_magic,
                                           sizeof(extraparams->magic)) == 0);
            noncefp = extraparams->noncefp;
            ndata = extraparams->ndata;
        }
        return schnorrsig_sign_internal(ctx, sig64, msg, msglen, keypair, noncefp, ndata);
        */
}

pub fn schnorrsig_verify(
        ctx:    *const Secp256k1Context,
        sig64:  *const u8,
        msg:    *const u8,
        msglen: usize,
        pubkey: *const XOnlyPubKey) -> i32 {
    
    todo!();
        /*
            scalar s;
        scalar e;
        gej rj;
        ge pk;
        gej pkj;
        fe rx;
        ge r;
        unsigned char buf[32];
        int overflow;

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(ecmult_context_is_built(&ctx->ecmult_ctx));
        ARG_CHECK(sig64 != NULL);
        ARG_CHECK(msg != NULL || msglen == 0);
        ARG_CHECK(pubkey != NULL);

        if (!fe_set_b32(&rx, &sig64[0])) {
            return 0;
        }

        scalar_set_b32(&s, &sig64[32], &overflow);
        if (overflow) {
            return 0;
        }

        if (!xonly_pubkey_load(ctx, &pk, pubkey)) {
            return 0;
        }

        /* Compute e. */
        fe_get_b32(buf, &pk.x);
        schnorrsig_challenge(&e, &sig64[0], msg, msglen, buf);

        /* Compute rj =  s*G + (-e)*pkj */
        scalar_negate(&e, &e);
        gej_set_ge(&pkj, &pk);
        ecmult(&ctx->ecmult_ctx, &rj, &pkj, &e, &s);

        ge_set_gej_var(&r, &rj);
        if (ge_is_infinity(&r)) {
            return 0;
        }

        fe_normalize_var(&r.y);
        return !fe_is_odd(&r.y) &&
               fe_equal_var(&rx, &r.x);
        */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/modules/schnorrsig/tests_impl.h]

/**
  | Checks that a bit flip in the n_flip-th
  | argument (that has n_bytes many bytes)
  | changes the hash function
  |
  */
pub fn nonce_function_bip340_bitflip(
        args:    *mut *mut u8,
        n_flip:  usize,
        n_bytes: usize,
        msglen:  usize,
        algolen: usize)  {
    
    todo!();
        /*
            unsigned char nonces[2][32];
        CHECK(nonce_function_bip340(nonces[0], args[0], msglen, args[1], args[2], args[3], algolen, args[4]) == 1);
        testrand_flip(args[n_flip], n_bytes);
        CHECK(nonce_function_bip340(nonces[1], args[0], msglen, args[1], args[2], args[3], algolen, args[4]) == 1);
        CHECK(memcmp_var(nonces[0], nonces[1], 32) != 0);
        */
}

/**
  | Tests for the equality of two sha256
  | structs. This function only produces
  | a correct result if an integer multiple
  | of 64 many bytes have been written into
  | the hash functions.
  |
  */
pub fn test_sha256_eq(
        sha1: *const Sha256,
        sha2: *const Sha256)  {
    
    todo!();
        /*
            /* Is buffer fully consumed? */
        CHECK((sha1->bytes & 0x3F) == 0);

        CHECK(sha1->bytes == sha2->bytes);
        CHECK(memcmp_var(sha1->s, sha2->s, sizeof(sha1->s)) == 0);
        */
}

pub fn run_nonce_function_bip340_tests()  {
    
    todo!();
        /*
            unsigned char tag[13] = "BIP0340/nonce";
        unsigned char aux_tag[11] = "BIP0340/aux";
        unsigned char algo[13] = "BIP0340/nonce";
        size_t algolen = sizeof(algo);
        sha256 sha;
        sha256 sha_optimized;
        unsigned char nonce[32];
        unsigned char msg[32];
        size_t msglen = sizeof(msg);
        unsigned char key[32];
        unsigned char pk[32];
        unsigned char aux_rand[32];
        unsigned char *args[5];
        int i;

        /* Check that hash initialized by
         * nonce_function_bip340_sha256_tagged has the expected
         * state. */
        sha256_initialize_tagged(&sha, tag, sizeof(tag));
        nonce_function_bip340_sha256_tagged(&sha_optimized);
        test_sha256_eq(&sha, &sha_optimized);

       /* Check that hash initialized by
        * nonce_function_bip340_sha256_tagged_aux has the expected
        * state. */
        sha256_initialize_tagged(&sha, aux_tag, sizeof(aux_tag));
        nonce_function_bip340_sha256_tagged_aux(&sha_optimized);
        test_sha256_eq(&sha, &sha_optimized);

        testrand256(msg);
        testrand256(key);
        testrand256(pk);
        testrand256(aux_rand);

        /* Check that a bitflip in an argument results in different nonces. */
        args[0] = msg;
        args[1] = key;
        args[2] = pk;
        args[3] = algo;
        args[4] = aux_rand;
        for (i = 0; i < count; i++) {
            nonce_function_bip340_bitflip(args, 0, 32, msglen, algolen);
            nonce_function_bip340_bitflip(args, 1, 32, msglen, algolen);
            nonce_function_bip340_bitflip(args, 2, 32, msglen, algolen);
            /* Flip algo special case "BIP0340/nonce" */
            nonce_function_bip340_bitflip(args, 3, algolen, msglen, algolen);
            /* Flip algo again */
            nonce_function_bip340_bitflip(args, 3, algolen, msglen, algolen);
            nonce_function_bip340_bitflip(args, 4, 32, msglen, algolen);
        }

        /* NULL algo is disallowed */
        CHECK(nonce_function_bip340(nonce, msg, msglen, key, pk, NULL, 0, NULL) == 0);
        CHECK(nonce_function_bip340(nonce, msg, msglen, key, pk, algo, algolen, NULL) == 1);
        /* Other algo is fine */
        rfc6979_hmac_sha256_generate(&test_rng, algo, algolen);
        CHECK(nonce_function_bip340(nonce, msg, msglen, key, pk, algo, algolen, NULL) == 1);

        for (i = 0; i < count; i++) {
            unsigned char nonce2[32];
            uint32_t offset = testrand_int(msglen - 1);
            size_t msglen_tmp = (msglen + offset) % msglen;
            size_t algolen_tmp;

            /* Different msglen gives different nonce */
            CHECK(nonce_function_bip340(nonce2, msg, msglen_tmp, key, pk, algo, algolen, NULL) == 1);
            CHECK(memcmp_var(nonce, nonce2, 32) != 0);

            /* Different algolen gives different nonce */
            offset = testrand_int(algolen - 1);
            algolen_tmp = (algolen + offset) % algolen;
            CHECK(nonce_function_bip340(nonce2, msg, msglen, key, pk, algo, algolen_tmp, NULL) == 1);
            CHECK(memcmp_var(nonce, nonce2, 32) != 0);
        }

        /* NULL aux_rand argument is allowed. */
        CHECK(nonce_function_bip340(nonce, msg, msglen, key, pk, algo, algolen, NULL) == 1);
        */
}

pub fn test_schnorrsig_api()  {
    
    todo!();
        /*
            unsigned char sk1[32];
        unsigned char sk2[32];
        unsigned char sk3[32];
        unsigned char msg[32];
        keypair keypairs[3];
        keypair invalid_keypair = {{ 0 }};
        xonly_pubkey pk[3];
        xonly_pubkey zero_pk;
        unsigned char sig[64];
        schnorrsig_extraparams extraparams = SCHNORRSIG_EXTRAPARAMS_INIT;
        schnorrsig_extraparams invalid_extraparams = {{ 0 }, NULL, NULL};

        /** setup **/
        context *none = context_create(CONTEXT_NONE);
        context *sign = context_create(CONTEXT_SIGN);
        context *vrfy = context_create(CONTEXT_VERIFY);
        context *both = context_create(CONTEXT_SIGN | CONTEXT_VERIFY);
        int ecount;

        context_set_error_callback(none, counting_illegal_callback_fn, &ecount);
        context_set_error_callback(sign, counting_illegal_callback_fn, &ecount);
        context_set_error_callback(vrfy, counting_illegal_callback_fn, &ecount);
        context_set_error_callback(both, counting_illegal_callback_fn, &ecount);
        context_set_illegal_callback(none, counting_illegal_callback_fn, &ecount);
        context_set_illegal_callback(sign, counting_illegal_callback_fn, &ecount);
        context_set_illegal_callback(vrfy, counting_illegal_callback_fn, &ecount);
        context_set_illegal_callback(both, counting_illegal_callback_fn, &ecount);

        testrand256(sk1);
        testrand256(sk2);
        testrand256(sk3);
        testrand256(msg);
        CHECK(keypair_create(ctx, &keypairs[0], sk1) == 1);
        CHECK(keypair_create(ctx, &keypairs[1], sk2) == 1);
        CHECK(keypair_create(ctx, &keypairs[2], sk3) == 1);
        CHECK(keypair_xonly_pub(ctx, &pk[0], NULL, &keypairs[0]) == 1);
        CHECK(keypair_xonly_pub(ctx, &pk[1], NULL, &keypairs[1]) == 1);
        CHECK(keypair_xonly_pub(ctx, &pk[2], NULL, &keypairs[2]) == 1);
        memset(&zero_pk, 0, sizeof(zero_pk));

        /** main test body **/
        ecount = 0;
        CHECK(schnorrsig_sign(none, sig, msg, &keypairs[0], NULL) == 0);
        CHECK(ecount == 1);
        CHECK(schnorrsig_sign(vrfy, sig, msg, &keypairs[0], NULL) == 0);
        CHECK(ecount == 2);
        CHECK(schnorrsig_sign(sign, sig, msg, &keypairs[0], NULL) == 1);
        CHECK(ecount == 2);
        CHECK(schnorrsig_sign(sign, NULL, msg, &keypairs[0], NULL) == 0);
        CHECK(ecount == 3);
        CHECK(schnorrsig_sign(sign, sig, NULL, &keypairs[0], NULL) == 0);
        CHECK(ecount == 4);
        CHECK(schnorrsig_sign(sign, sig, msg, NULL, NULL) == 0);
        CHECK(ecount == 5);
        CHECK(schnorrsig_sign(sign, sig, msg, &invalid_keypair, NULL) == 0);
        CHECK(ecount == 6);

        ecount = 0;
        CHECK(schnorrsig_sign_custom(none, sig, msg, sizeof(msg), &keypairs[0], &extraparams) == 0);
        CHECK(ecount == 1);
        CHECK(schnorrsig_sign_custom(vrfy, sig, msg, sizeof(msg), &keypairs[0], &extraparams) == 0);
        CHECK(ecount == 2);
        CHECK(schnorrsig_sign_custom(sign, sig, msg, sizeof(msg), &keypairs[0], &extraparams) == 1);
        CHECK(ecount == 2);
        CHECK(schnorrsig_sign_custom(sign, NULL, msg, sizeof(msg), &keypairs[0], &extraparams) == 0);
        CHECK(ecount == 3);
        CHECK(schnorrsig_sign_custom(sign, sig, NULL, sizeof(msg), &keypairs[0], &extraparams) == 0);
        CHECK(ecount == 4);
        CHECK(schnorrsig_sign_custom(sign, sig, NULL, 0, &keypairs[0], &extraparams) == 1);
        CHECK(ecount == 4);
        CHECK(schnorrsig_sign_custom(sign, sig, msg, sizeof(msg), NULL, &extraparams) == 0);
        CHECK(ecount == 5);
        CHECK(schnorrsig_sign_custom(sign, sig, msg, sizeof(msg), &invalid_keypair, &extraparams) == 0);
        CHECK(ecount == 6);
        CHECK(schnorrsig_sign_custom(sign, sig, msg, sizeof(msg), &keypairs[0], NULL) == 1);
        CHECK(ecount == 6);
        CHECK(schnorrsig_sign_custom(sign, sig, msg, sizeof(msg), &keypairs[0], &invalid_extraparams) == 0);
        CHECK(ecount == 7);

        ecount = 0;
        CHECK(schnorrsig_sign(sign, sig, msg, &keypairs[0], NULL) == 1);
        CHECK(schnorrsig_verify(none, sig, msg, sizeof(msg), &pk[0]) == 0);
        CHECK(ecount == 1);
        CHECK(schnorrsig_verify(sign, sig, msg, sizeof(msg), &pk[0]) == 0);
        CHECK(ecount == 2);
        CHECK(schnorrsig_verify(vrfy, sig, msg, sizeof(msg), &pk[0]) == 1);
        CHECK(ecount == 2);
        CHECK(schnorrsig_verify(vrfy, NULL, msg, sizeof(msg), &pk[0]) == 0);
        CHECK(ecount == 3);
        CHECK(schnorrsig_verify(vrfy, sig, NULL, sizeof(msg), &pk[0]) == 0);
        CHECK(ecount == 4);
        CHECK(schnorrsig_verify(vrfy, sig, NULL, 0, &pk[0]) == 0);
        CHECK(ecount == 4);
        CHECK(schnorrsig_verify(vrfy, sig, msg, sizeof(msg), NULL) == 0);
        CHECK(ecount == 5);
        CHECK(schnorrsig_verify(vrfy, sig, msg, sizeof(msg), &zero_pk) == 0);
        CHECK(ecount == 6);

        context_destroy(none);
        context_destroy(sign);
        context_destroy(vrfy);
        context_destroy(both);
        */
}

/**
  | Checks that hash initialized by schnorrsig_sha256_tagged
  | has the expected state.
  |
  */
pub fn test_schnorrsig_sha256_tagged()  {
    
    todo!();
        /*
            unsigned char tag[17] = "BIP0340/challenge";
        sha256 sha;
        sha256 sha_optimized;

        sha256_initialize_tagged(&sha, (unsigned char *) tag, sizeof(tag));
        schnorrsig_sha256_tagged(&sha_optimized);
        test_sha256_eq(&sha, &sha_optimized);
        */
}

/**
  | Helper function for schnorrsig_bip_vectors
  | 
  | Signs the message and checks that it's
  | the same as expected_sig.
  |
  */
pub fn test_schnorrsig_bip_vectors_check_signing(
        sk:            *const u8,
        pk_serialized: *const u8,
        aux_rand:      *mut u8,
        msg32:         *const u8,
        expected_sig:  *const u8)  {
    
    todo!();
        /*
            unsigned char sig[64];
        keypair keypair;
        xonly_pubkey pk, pk_expected;

        CHECK(keypair_create(ctx, &keypair, sk));
        CHECK(schnorrsig_sign(ctx, sig, msg32, &keypair, aux_rand));
        CHECK(memcmp_var(sig, expected_sig, 64) == 0);

        CHECK(xonly_pubkey_parse(ctx, &pk_expected, pk_serialized));
        CHECK(keypair_xonly_pub(ctx, &pk, NULL, &keypair));
        CHECK(memcmp_var(&pk, &pk_expected, sizeof(pk)) == 0);
        CHECK(schnorrsig_verify(ctx, sig, msg32, 32, &pk));
        */
}

/**
  | Helper function for schnorrsig_bip_vectors
  | 
  | Checks that both verify and verify_batch
  | (TODO) return the same value as expected.
  |
  */
pub fn test_schnorrsig_bip_vectors_check_verify(
        pk_serialized: *const u8,
        msg32:         *const u8,
        sig:           *const u8,
        expected:      i32)  {
    
    todo!();
        /*
            xonly_pubkey pk;

        CHECK(xonly_pubkey_parse(ctx, &pk, pk_serialized));
        CHECK(expected == schnorrsig_verify(ctx, sig, msg32, 32, &pk));
        */
}

/**
  | Test vectors according to BIP-340 ("Schnorr
  | Signatures for secp256k1"). See https://github.com/bitcoin/bips/blob/master/bip-0340/test-vectors.csv.
  |
  */
pub fn test_schnorrsig_bip_vectors()  {
    
    todo!();
        /*
            {
            /* Test vector 0 */
            const unsigned char sk[32] = {
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03
            };
            const unsigned char pk[32] = {
                0xF9, 0x30, 0x8A, 0x01, 0x92, 0x58, 0xC3, 0x10,
                0x49, 0x34, 0x4F, 0x85, 0xF8, 0x9D, 0x52, 0x29,
                0xB5, 0x31, 0xC8, 0x45, 0x83, 0x6F, 0x99, 0xB0,
                0x86, 0x01, 0xF1, 0x13, 0xBC, 0xE0, 0x36, 0xF9
            };
            unsigned char aux_rand[32] = {
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
            };
            const unsigned char msg[32] = {
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
            };
            const unsigned char sig[64] = {
                0xE9, 0x07, 0x83, 0x1F, 0x80, 0x84, 0x8D, 0x10,
                0x69, 0xA5, 0x37, 0x1B, 0x40, 0x24, 0x10, 0x36,
                0x4B, 0xDF, 0x1C, 0x5F, 0x83, 0x07, 0xB0, 0x08,
                0x4C, 0x55, 0xF1, 0xCE, 0x2D, 0xCA, 0x82, 0x15,
                0x25, 0xF6, 0x6A, 0x4A, 0x85, 0xEA, 0x8B, 0x71,
                0xE4, 0x82, 0xA7, 0x4F, 0x38, 0x2D, 0x2C, 0xE5,
                0xEB, 0xEE, 0xE8, 0xFD, 0xB2, 0x17, 0x2F, 0x47,
                0x7D, 0xF4, 0x90, 0x0D, 0x31, 0x05, 0x36, 0xC0
            };
            test_schnorrsig_bip_vectors_check_signing(sk, pk, aux_rand, msg, sig);
            test_schnorrsig_bip_vectors_check_verify(pk, msg, sig, 1);
        }
        {
            /* Test vector 1 */
            const unsigned char sk[32] = {
                0xB7, 0xE1, 0x51, 0x62, 0x8A, 0xED, 0x2A, 0x6A,
                0xBF, 0x71, 0x58, 0x80, 0x9C, 0xF4, 0xF3, 0xC7,
                0x62, 0xE7, 0x16, 0x0F, 0x38, 0xB4, 0xDA, 0x56,
                0xA7, 0x84, 0xD9, 0x04, 0x51, 0x90, 0xCF, 0xEF
            };
            const unsigned char pk[32] = {
                0xDF, 0xF1, 0xD7, 0x7F, 0x2A, 0x67, 0x1C, 0x5F,
                0x36, 0x18, 0x37, 0x26, 0xDB, 0x23, 0x41, 0xBE,
                0x58, 0xFE, 0xAE, 0x1D, 0xA2, 0xDE, 0xCE, 0xD8,
                0x43, 0x24, 0x0F, 0x7B, 0x50, 0x2B, 0xA6, 0x59
            };
            unsigned char aux_rand[32] = {
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
            };
            const unsigned char msg[32] = {
                0x24, 0x3F, 0x6A, 0x88, 0x85, 0xA3, 0x08, 0xD3,
                0x13, 0x19, 0x8A, 0x2E, 0x03, 0x70, 0x73, 0x44,
                0xA4, 0x09, 0x38, 0x22, 0x29, 0x9F, 0x31, 0xD0,
                0x08, 0x2E, 0xFA, 0x98, 0xEC, 0x4E, 0x6C, 0x89
            };
            const unsigned char sig[64] = {
                0x68, 0x96, 0xBD, 0x60, 0xEE, 0xAE, 0x29, 0x6D,
                0xB4, 0x8A, 0x22, 0x9F, 0xF7, 0x1D, 0xFE, 0x07,
                0x1B, 0xDE, 0x41, 0x3E, 0x6D, 0x43, 0xF9, 0x17,
                0xDC, 0x8D, 0xCF, 0x8C, 0x78, 0xDE, 0x33, 0x41,
                0x89, 0x06, 0xD1, 0x1A, 0xC9, 0x76, 0xAB, 0xCC,
                0xB2, 0x0B, 0x09, 0x12, 0x92, 0xBF, 0xF4, 0xEA,
                0x89, 0x7E, 0xFC, 0xB6, 0x39, 0xEA, 0x87, 0x1C,
                0xFA, 0x95, 0xF6, 0xDE, 0x33, 0x9E, 0x4B, 0x0A
            };
            test_schnorrsig_bip_vectors_check_signing(sk, pk, aux_rand, msg, sig);
            test_schnorrsig_bip_vectors_check_verify(pk, msg, sig, 1);
        }
        {
            /* Test vector 2 */
            const unsigned char sk[32] = {
                0xC9, 0x0F, 0xDA, 0xA2, 0x21, 0x68, 0xC2, 0x34,
                0xC4, 0xC6, 0x62, 0x8B, 0x80, 0xDC, 0x1C, 0xD1,
                0x29, 0x02, 0x4E, 0x08, 0x8A, 0x67, 0xCC, 0x74,
                0x02, 0x0B, 0xBE, 0xA6, 0x3B, 0x14, 0xE5, 0xC9
            };
            const unsigned char pk[32] = {
                0xDD, 0x30, 0x8A, 0xFE, 0xC5, 0x77, 0x7E, 0x13,
                0x12, 0x1F, 0xA7, 0x2B, 0x9C, 0xC1, 0xB7, 0xCC,
                0x01, 0x39, 0x71, 0x53, 0x09, 0xB0, 0x86, 0xC9,
                0x60, 0xE1, 0x8F, 0xD9, 0x69, 0x77, 0x4E, 0xB8
            };
            unsigned char aux_rand[32] = {
                0xC8, 0x7A, 0xA5, 0x38, 0x24, 0xB4, 0xD7, 0xAE,
                0x2E, 0xB0, 0x35, 0xA2, 0xB5, 0xBB, 0xBC, 0xCC,
                0x08, 0x0E, 0x76, 0xCD, 0xC6, 0xD1, 0x69, 0x2C,
                0x4B, 0x0B, 0x62, 0xD7, 0x98, 0xE6, 0xD9, 0x06
            };
            const unsigned char msg[32] = {
                0x7E, 0x2D, 0x58, 0xD8, 0xB3, 0xBC, 0xDF, 0x1A,
                0xBA, 0xDE, 0xC7, 0x82, 0x90, 0x54, 0xF9, 0x0D,
                0xDA, 0x98, 0x05, 0xAA, 0xB5, 0x6C, 0x77, 0x33,
                0x30, 0x24, 0xB9, 0xD0, 0xA5, 0x08, 0xB7, 0x5C
            };
            const unsigned char sig[64] = {
                0x58, 0x31, 0xAA, 0xEE, 0xD7, 0xB4, 0x4B, 0xB7,
                0x4E, 0x5E, 0xAB, 0x94, 0xBA, 0x9D, 0x42, 0x94,
                0xC4, 0x9B, 0xCF, 0x2A, 0x60, 0x72, 0x8D, 0x8B,
                0x4C, 0x20, 0x0F, 0x50, 0xDD, 0x31, 0x3C, 0x1B,
                0xAB, 0x74, 0x58, 0x79, 0xA5, 0xAD, 0x95, 0x4A,
                0x72, 0xC4, 0x5A, 0x91, 0xC3, 0xA5, 0x1D, 0x3C,
                0x7A, 0xDE, 0xA9, 0x8D, 0x82, 0xF8, 0x48, 0x1E,
                0x0E, 0x1E, 0x03, 0x67, 0x4A, 0x6F, 0x3F, 0xB7
            };
            test_schnorrsig_bip_vectors_check_signing(sk, pk, aux_rand, msg, sig);
            test_schnorrsig_bip_vectors_check_verify(pk, msg, sig, 1);
        }
        {
            /* Test vector 3 */
            const unsigned char sk[32] = {
                0x0B, 0x43, 0x2B, 0x26, 0x77, 0x93, 0x73, 0x81,
                0xAE, 0xF0, 0x5B, 0xB0, 0x2A, 0x66, 0xEC, 0xD0,
                0x12, 0x77, 0x30, 0x62, 0xCF, 0x3F, 0xA2, 0x54,
                0x9E, 0x44, 0xF5, 0x8E, 0xD2, 0x40, 0x17, 0x10
            };
            const unsigned char pk[32] = {
                0x25, 0xD1, 0xDF, 0xF9, 0x51, 0x05, 0xF5, 0x25,
                0x3C, 0x40, 0x22, 0xF6, 0x28, 0xA9, 0x96, 0xAD,
                0x3A, 0x0D, 0x95, 0xFB, 0xF2, 0x1D, 0x46, 0x8A,
                0x1B, 0x33, 0xF8, 0xC1, 0x60, 0xD8, 0xF5, 0x17
            };
            unsigned char aux_rand[32] = {
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
            };
            const unsigned char msg[32] = {
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
            };
            const unsigned char sig[64] = {
                0x7E, 0xB0, 0x50, 0x97, 0x57, 0xE2, 0x46, 0xF1,
                0x94, 0x49, 0x88, 0x56, 0x51, 0x61, 0x1C, 0xB9,
                0x65, 0xEC, 0xC1, 0xA1, 0x87, 0xDD, 0x51, 0xB6,
                0x4F, 0xDA, 0x1E, 0xDC, 0x96, 0x37, 0xD5, 0xEC,
                0x97, 0x58, 0x2B, 0x9C, 0xB1, 0x3D, 0xB3, 0x93,
                0x37, 0x05, 0xB3, 0x2B, 0xA9, 0x82, 0xAF, 0x5A,
                0xF2, 0x5F, 0xD7, 0x88, 0x81, 0xEB, 0xB3, 0x27,
                0x71, 0xFC, 0x59, 0x22, 0xEF, 0xC6, 0x6E, 0xA3
            };
            test_schnorrsig_bip_vectors_check_signing(sk, pk, aux_rand, msg, sig);
            test_schnorrsig_bip_vectors_check_verify(pk, msg, sig, 1);
        }
        {
            /* Test vector 4 */
            const unsigned char pk[32] = {
                0xD6, 0x9C, 0x35, 0x09, 0xBB, 0x99, 0xE4, 0x12,
                0xE6, 0x8B, 0x0F, 0xE8, 0x54, 0x4E, 0x72, 0x83,
                0x7D, 0xFA, 0x30, 0x74, 0x6D, 0x8B, 0xE2, 0xAA,
                0x65, 0x97, 0x5F, 0x29, 0xD2, 0x2D, 0xC7, 0xB9
            };
            const unsigned char msg[32] = {
                0x4D, 0xF3, 0xC3, 0xF6, 0x8F, 0xCC, 0x83, 0xB2,
                0x7E, 0x9D, 0x42, 0xC9, 0x04, 0x31, 0xA7, 0x24,
                0x99, 0xF1, 0x78, 0x75, 0xC8, 0x1A, 0x59, 0x9B,
                0x56, 0x6C, 0x98, 0x89, 0xB9, 0x69, 0x67, 0x03
            };
            const unsigned char sig[64] = {
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x3B, 0x78, 0xCE, 0x56, 0x3F,
                0x89, 0xA0, 0xED, 0x94, 0x14, 0xF5, 0xAA, 0x28,
                0xAD, 0x0D, 0x96, 0xD6, 0x79, 0x5F, 0x9C, 0x63,
                0x76, 0xAF, 0xB1, 0x54, 0x8A, 0xF6, 0x03, 0xB3,
                0xEB, 0x45, 0xC9, 0xF8, 0x20, 0x7D, 0xEE, 0x10,
                0x60, 0xCB, 0x71, 0xC0, 0x4E, 0x80, 0xF5, 0x93,
                0x06, 0x0B, 0x07, 0xD2, 0x83, 0x08, 0xD7, 0xF4
            };
            test_schnorrsig_bip_vectors_check_verify(pk, msg, sig, 1);
        }
        {
            /* Test vector 5 */
            const unsigned char pk[32] = {
                0xEE, 0xFD, 0xEA, 0x4C, 0xDB, 0x67, 0x77, 0x50,
                0xA4, 0x20, 0xFE, 0xE8, 0x07, 0xEA, 0xCF, 0x21,
                0xEB, 0x98, 0x98, 0xAE, 0x79, 0xB9, 0x76, 0x87,
                0x66, 0xE4, 0xFA, 0xA0, 0x4A, 0x2D, 0x4A, 0x34
            };
            xonly_pubkey pk_parsed;
            /* No need to check the signature of the test vector as parsing the pubkey already fails */
            CHECK(!xonly_pubkey_parse(ctx, &pk_parsed, pk));
        }
        {
            /* Test vector 6 */
            const unsigned char pk[32] = {
                0xDF, 0xF1, 0xD7, 0x7F, 0x2A, 0x67, 0x1C, 0x5F,
                0x36, 0x18, 0x37, 0x26, 0xDB, 0x23, 0x41, 0xBE,
                0x58, 0xFE, 0xAE, 0x1D, 0xA2, 0xDE, 0xCE, 0xD8,
                0x43, 0x24, 0x0F, 0x7B, 0x50, 0x2B, 0xA6, 0x59
            };
            const unsigned char msg[32] = {
                0x24, 0x3F, 0x6A, 0x88, 0x85, 0xA3, 0x08, 0xD3,
                0x13, 0x19, 0x8A, 0x2E, 0x03, 0x70, 0x73, 0x44,
                0xA4, 0x09, 0x38, 0x22, 0x29, 0x9F, 0x31, 0xD0,
                0x08, 0x2E, 0xFA, 0x98, 0xEC, 0x4E, 0x6C, 0x89
            };
            const unsigned char sig[64] = {
                0xFF, 0xF9, 0x7B, 0xD5, 0x75, 0x5E, 0xEE, 0xA4,
                0x20, 0x45, 0x3A, 0x14, 0x35, 0x52, 0x35, 0xD3,
                0x82, 0xF6, 0x47, 0x2F, 0x85, 0x68, 0xA1, 0x8B,
                0x2F, 0x05, 0x7A, 0x14, 0x60, 0x29, 0x75, 0x56,
                0x3C, 0xC2, 0x79, 0x44, 0x64, 0x0A, 0xC6, 0x07,
                0xCD, 0x10, 0x7A, 0xE1, 0x09, 0x23, 0xD9, 0xEF,
                0x7A, 0x73, 0xC6, 0x43, 0xE1, 0x66, 0xBE, 0x5E,
                0xBE, 0xAF, 0xA3, 0x4B, 0x1A, 0xC5, 0x53, 0xE2
            };
            test_schnorrsig_bip_vectors_check_verify(pk, msg, sig, 0);
        }
        {
            /* Test vector 7 */
            const unsigned char pk[32] = {
                0xDF, 0xF1, 0xD7, 0x7F, 0x2A, 0x67, 0x1C, 0x5F,
                0x36, 0x18, 0x37, 0x26, 0xDB, 0x23, 0x41, 0xBE,
                0x58, 0xFE, 0xAE, 0x1D, 0xA2, 0xDE, 0xCE, 0xD8,
                0x43, 0x24, 0x0F, 0x7B, 0x50, 0x2B, 0xA6, 0x59
            };
            const unsigned char msg[32] = {
                0x24, 0x3F, 0x6A, 0x88, 0x85, 0xA3, 0x08, 0xD3,
                0x13, 0x19, 0x8A, 0x2E, 0x03, 0x70, 0x73, 0x44,
                0xA4, 0x09, 0x38, 0x22, 0x29, 0x9F, 0x31, 0xD0,
                0x08, 0x2E, 0xFA, 0x98, 0xEC, 0x4E, 0x6C, 0x89
            };
            const unsigned char sig[64] = {
                0x1F, 0xA6, 0x2E, 0x33, 0x1E, 0xDB, 0xC2, 0x1C,
                0x39, 0x47, 0x92, 0xD2, 0xAB, 0x11, 0x00, 0xA7,
                0xB4, 0x32, 0xB0, 0x13, 0xDF, 0x3F, 0x6F, 0xF4,
                0xF9, 0x9F, 0xCB, 0x33, 0xE0, 0xE1, 0x51, 0x5F,
                0x28, 0x89, 0x0B, 0x3E, 0xDB, 0x6E, 0x71, 0x89,
                0xB6, 0x30, 0x44, 0x8B, 0x51, 0x5C, 0xE4, 0xF8,
                0x62, 0x2A, 0x95, 0x4C, 0xFE, 0x54, 0x57, 0x35,
                0xAA, 0xEA, 0x51, 0x34, 0xFC, 0xCD, 0xB2, 0xBD
            };
            test_schnorrsig_bip_vectors_check_verify(pk, msg, sig, 0);
        }
        {
            /* Test vector 8 */
            const unsigned char pk[32] = {
                0xDF, 0xF1, 0xD7, 0x7F, 0x2A, 0x67, 0x1C, 0x5F,
                0x36, 0x18, 0x37, 0x26, 0xDB, 0x23, 0x41, 0xBE,
                0x58, 0xFE, 0xAE, 0x1D, 0xA2, 0xDE, 0xCE, 0xD8,
                0x43, 0x24, 0x0F, 0x7B, 0x50, 0x2B, 0xA6, 0x59
            };
            const unsigned char msg[32] = {
                0x24, 0x3F, 0x6A, 0x88, 0x85, 0xA3, 0x08, 0xD3,
                0x13, 0x19, 0x8A, 0x2E, 0x03, 0x70, 0x73, 0x44,
                0xA4, 0x09, 0x38, 0x22, 0x29, 0x9F, 0x31, 0xD0,
                0x08, 0x2E, 0xFA, 0x98, 0xEC, 0x4E, 0x6C, 0x89
            };
            const unsigned char sig[64] = {
                0x6C, 0xFF, 0x5C, 0x3B, 0xA8, 0x6C, 0x69, 0xEA,
                0x4B, 0x73, 0x76, 0xF3, 0x1A, 0x9B, 0xCB, 0x4F,
                0x74, 0xC1, 0x97, 0x60, 0x89, 0xB2, 0xD9, 0x96,
                0x3D, 0xA2, 0xE5, 0x54, 0x3E, 0x17, 0x77, 0x69,
                0x96, 0x17, 0x64, 0xB3, 0xAA, 0x9B, 0x2F, 0xFC,
                0xB6, 0xEF, 0x94, 0x7B, 0x68, 0x87, 0xA2, 0x26,
                0xE8, 0xD7, 0xC9, 0x3E, 0x00, 0xC5, 0xED, 0x0C,
                0x18, 0x34, 0xFF, 0x0D, 0x0C, 0x2E, 0x6D, 0xA6
            };
            test_schnorrsig_bip_vectors_check_verify(pk, msg, sig, 0);
        }
        {
            /* Test vector 9 */
            const unsigned char pk[32] = {
                0xDF, 0xF1, 0xD7, 0x7F, 0x2A, 0x67, 0x1C, 0x5F,
                0x36, 0x18, 0x37, 0x26, 0xDB, 0x23, 0x41, 0xBE,
                0x58, 0xFE, 0xAE, 0x1D, 0xA2, 0xDE, 0xCE, 0xD8,
                0x43, 0x24, 0x0F, 0x7B, 0x50, 0x2B, 0xA6, 0x59
            };
            const unsigned char msg[32] = {
                0x24, 0x3F, 0x6A, 0x88, 0x85, 0xA3, 0x08, 0xD3,
                0x13, 0x19, 0x8A, 0x2E, 0x03, 0x70, 0x73, 0x44,
                0xA4, 0x09, 0x38, 0x22, 0x29, 0x9F, 0x31, 0xD0,
                0x08, 0x2E, 0xFA, 0x98, 0xEC, 0x4E, 0x6C, 0x89
            };
            const unsigned char sig[64] = {
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x12, 0x3D, 0xDA, 0x83, 0x28, 0xAF, 0x9C, 0x23,
                0xA9, 0x4C, 0x1F, 0xEE, 0xCF, 0xD1, 0x23, 0xBA,
                0x4F, 0xB7, 0x34, 0x76, 0xF0, 0xD5, 0x94, 0xDC,
                0xB6, 0x5C, 0x64, 0x25, 0xBD, 0x18, 0x60, 0x51
            };
            test_schnorrsig_bip_vectors_check_verify(pk, msg, sig, 0);
        }
        {
            /* Test vector 10 */
            const unsigned char pk[32] = {
                0xDF, 0xF1, 0xD7, 0x7F, 0x2A, 0x67, 0x1C, 0x5F,
                0x36, 0x18, 0x37, 0x26, 0xDB, 0x23, 0x41, 0xBE,
                0x58, 0xFE, 0xAE, 0x1D, 0xA2, 0xDE, 0xCE, 0xD8,
                0x43, 0x24, 0x0F, 0x7B, 0x50, 0x2B, 0xA6, 0x59
            };
            const unsigned char msg[32] = {
                0x24, 0x3F, 0x6A, 0x88, 0x85, 0xA3, 0x08, 0xD3,
                0x13, 0x19, 0x8A, 0x2E, 0x03, 0x70, 0x73, 0x44,
                0xA4, 0x09, 0x38, 0x22, 0x29, 0x9F, 0x31, 0xD0,
                0x08, 0x2E, 0xFA, 0x98, 0xEC, 0x4E, 0x6C, 0x89
            };
            const unsigned char sig[64] = {
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
                0x76, 0x15, 0xFB, 0xAF, 0x5A, 0xE2, 0x88, 0x64,
                0x01, 0x3C, 0x09, 0x97, 0x42, 0xDE, 0xAD, 0xB4,
                0xDB, 0xA8, 0x7F, 0x11, 0xAC, 0x67, 0x54, 0xF9,
                0x37, 0x80, 0xD5, 0xA1, 0x83, 0x7C, 0xF1, 0x97
            };
            test_schnorrsig_bip_vectors_check_verify(pk, msg, sig, 0);
        }
        {
            /* Test vector 11 */
            const unsigned char pk[32] = {
                0xDF, 0xF1, 0xD7, 0x7F, 0x2A, 0x67, 0x1C, 0x5F,
                0x36, 0x18, 0x37, 0x26, 0xDB, 0x23, 0x41, 0xBE,
                0x58, 0xFE, 0xAE, 0x1D, 0xA2, 0xDE, 0xCE, 0xD8,
                0x43, 0x24, 0x0F, 0x7B, 0x50, 0x2B, 0xA6, 0x59
            };
            const unsigned char msg[32] = {
                0x24, 0x3F, 0x6A, 0x88, 0x85, 0xA3, 0x08, 0xD3,
                0x13, 0x19, 0x8A, 0x2E, 0x03, 0x70, 0x73, 0x44,
                0xA4, 0x09, 0x38, 0x22, 0x29, 0x9F, 0x31, 0xD0,
                0x08, 0x2E, 0xFA, 0x98, 0xEC, 0x4E, 0x6C, 0x89
            };
            const unsigned char sig[64] = {
                0x4A, 0x29, 0x8D, 0xAC, 0xAE, 0x57, 0x39, 0x5A,
                0x15, 0xD0, 0x79, 0x5D, 0xDB, 0xFD, 0x1D, 0xCB,
                0x56, 0x4D, 0xA8, 0x2B, 0x0F, 0x26, 0x9B, 0xC7,
                0x0A, 0x74, 0xF8, 0x22, 0x04, 0x29, 0xBA, 0x1D,
                0x69, 0xE8, 0x9B, 0x4C, 0x55, 0x64, 0xD0, 0x03,
                0x49, 0x10, 0x6B, 0x84, 0x97, 0x78, 0x5D, 0xD7,
                0xD1, 0xD7, 0x13, 0xA8, 0xAE, 0x82, 0xB3, 0x2F,
                0xA7, 0x9D, 0x5F, 0x7F, 0xC4, 0x07, 0xD3, 0x9B
            };
            test_schnorrsig_bip_vectors_check_verify(pk, msg, sig, 0);
        }
        {
            /* Test vector 12 */
            const unsigned char pk[32] = {
                0xDF, 0xF1, 0xD7, 0x7F, 0x2A, 0x67, 0x1C, 0x5F,
                0x36, 0x18, 0x37, 0x26, 0xDB, 0x23, 0x41, 0xBE,
                0x58, 0xFE, 0xAE, 0x1D, 0xA2, 0xDE, 0xCE, 0xD8,
                0x43, 0x24, 0x0F, 0x7B, 0x50, 0x2B, 0xA6, 0x59
            };
            const unsigned char msg[32] = {
                0x24, 0x3F, 0x6A, 0x88, 0x85, 0xA3, 0x08, 0xD3,
                0x13, 0x19, 0x8A, 0x2E, 0x03, 0x70, 0x73, 0x44,
                0xA4, 0x09, 0x38, 0x22, 0x29, 0x9F, 0x31, 0xD0,
                0x08, 0x2E, 0xFA, 0x98, 0xEC, 0x4E, 0x6C, 0x89
            };
            const unsigned char sig[64] = {
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x2F,
                0x69, 0xE8, 0x9B, 0x4C, 0x55, 0x64, 0xD0, 0x03,
                0x49, 0x10, 0x6B, 0x84, 0x97, 0x78, 0x5D, 0xD7,
                0xD1, 0xD7, 0x13, 0xA8, 0xAE, 0x82, 0xB3, 0x2F,
                0xA7, 0x9D, 0x5F, 0x7F, 0xC4, 0x07, 0xD3, 0x9B
            };
            test_schnorrsig_bip_vectors_check_verify(pk, msg, sig, 0);
        }
        {
            /* Test vector 13 */
            const unsigned char pk[32] = {
                0xDF, 0xF1, 0xD7, 0x7F, 0x2A, 0x67, 0x1C, 0x5F,
                0x36, 0x18, 0x37, 0x26, 0xDB, 0x23, 0x41, 0xBE,
                0x58, 0xFE, 0xAE, 0x1D, 0xA2, 0xDE, 0xCE, 0xD8,
                0x43, 0x24, 0x0F, 0x7B, 0x50, 0x2B, 0xA6, 0x59
            };
            const unsigned char msg[32] = {
                0x24, 0x3F, 0x6A, 0x88, 0x85, 0xA3, 0x08, 0xD3,
                0x13, 0x19, 0x8A, 0x2E, 0x03, 0x70, 0x73, 0x44,
                0xA4, 0x09, 0x38, 0x22, 0x29, 0x9F, 0x31, 0xD0,
                0x08, 0x2E, 0xFA, 0x98, 0xEC, 0x4E, 0x6C, 0x89
            };
            const unsigned char sig[64] = {
                0x6C, 0xFF, 0x5C, 0x3B, 0xA8, 0x6C, 0x69, 0xEA,
                0x4B, 0x73, 0x76, 0xF3, 0x1A, 0x9B, 0xCB, 0x4F,
                0x74, 0xC1, 0x97, 0x60, 0x89, 0xB2, 0xD9, 0x96,
                0x3D, 0xA2, 0xE5, 0x54, 0x3E, 0x17, 0x77, 0x69,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
                0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B,
                0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x41
            };
            test_schnorrsig_bip_vectors_check_verify(pk, msg, sig, 0);
        }
        {
            /* Test vector 14 */
            const unsigned char pk[32] = {
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x30
            };
            xonly_pubkey pk_parsed;
            /* No need to check the signature of the test vector as parsing the pubkey already fails */
            CHECK(!xonly_pubkey_parse(ctx, &pk_parsed, pk));
        }
        */
}

/**
  | Nonce function that returns constant
  | 0
  |
  */
pub fn nonce_function_failing(
        nonce32:    *mut u8,
        msg:        *const u8,
        msglen:     usize,
        key32:      *const u8,
        xonly_pk32: *const u8,
        algo:       *const u8,
        algolen:    usize,
        data:       *mut c_void) -> i32 {
    
    todo!();
        /*
            (c_void) msg;
        (c_void) msglen;
        (c_void) key32;
        (c_void) xonly_pk32;
        (c_void) algo;
        (c_void) algolen;
        (c_void) data;
        (c_void) nonce32;
        return 0;
        */
}

/**
  | Nonce function that sets nonce to 0
  |
  */
pub fn nonce_function_0(
        nonce32:    *mut u8,
        msg:        *const u8,
        msglen:     usize,
        key32:      *const u8,
        xonly_pk32: *const u8,
        algo:       *const u8,
        algolen:    usize,
        data:       *mut c_void) -> i32 {
    
    todo!();
        /*
            (c_void) msg;
        (c_void) msglen;
        (c_void) key32;
        (c_void) xonly_pk32;
        (c_void) algo;
        (c_void) algolen;
        (c_void) data;

        memset(nonce32, 0, 32);
        return 1;
        */
}

/**
  | Nonce function that sets nonce to 0xFF...0xFF
  |
  */
pub fn nonce_function_overflowing(
        nonce32:    *mut u8,
        msg:        *const u8,
        msglen:     usize,
        key32:      *const u8,
        xonly_pk32: *const u8,
        algo:       *const u8,
        algolen:    usize,
        data:       *mut c_void) -> i32 {
    
    todo!();
        /*
            (c_void) msg;
        (c_void) msglen;
        (c_void) key32;
        (c_void) xonly_pk32;
        (c_void) algo;
        (c_void) algolen;
        (c_void) data;

        memset(nonce32, 0xFF, 32);
        return 1;
        */
}

pub fn test_schnorrsig_sign()  {
    
    todo!();
        /*
            unsigned char sk[32];
        xonly_pubkey pk;
        keypair keypair;
        const unsigned char msg[32] = "this is a msg for a schnorrsig..";
        unsigned char sig[64];
        unsigned char sig2[64];
        unsigned char zeros64[64] = { 0 };
        schnorrsig_extraparams extraparams = SCHNORRSIG_EXTRAPARAMS_INIT;
        unsigned char aux_rand[32];

        testrand256(sk);
        testrand256(aux_rand);
        CHECK(keypair_create(ctx, &keypair, sk));
        CHECK(keypair_xonly_pub(ctx, &pk, NULL, &keypair));
        CHECK(schnorrsig_sign(ctx, sig, msg, &keypair, NULL) == 1);
        CHECK(schnorrsig_verify(ctx, sig, msg, sizeof(msg), &pk));

        /* Test different nonce functions */
        CHECK(schnorrsig_sign_custom(ctx, sig, msg, sizeof(msg), &keypair, &extraparams) == 1);
        CHECK(schnorrsig_verify(ctx, sig, msg, sizeof(msg), &pk));
        memset(sig, 1, sizeof(sig));
        extraparams.noncefp = nonce_function_failing;
        CHECK(schnorrsig_sign_custom(ctx, sig, msg, sizeof(msg), &keypair, &extraparams) == 0);
        CHECK(memcmp_var(sig, zeros64, sizeof(sig)) == 0);
        memset(&sig, 1, sizeof(sig));
        extraparams.noncefp = nonce_function_0;
        CHECK(schnorrsig_sign_custom(ctx, sig, msg, sizeof(msg), &keypair, &extraparams) == 0);
        CHECK(memcmp_var(sig, zeros64, sizeof(sig)) == 0);
        memset(&sig, 1, sizeof(sig));
        extraparams.noncefp = nonce_function_overflowing;
        CHECK(schnorrsig_sign_custom(ctx, sig, msg, sizeof(msg), &keypair, &extraparams) == 1);
        CHECK(schnorrsig_verify(ctx, sig, msg, sizeof(msg), &pk));

        /* When using the default nonce function, schnorrsig_sign_custom produces
         * the same result as schnorrsig_sign with aux_rand = extraparams.ndata */
        extraparams.noncefp = NULL;
        extraparams.ndata = aux_rand;
        CHECK(schnorrsig_sign_custom(ctx, sig, msg, sizeof(msg), &keypair, &extraparams) == 1);
        CHECK(schnorrsig_sign(ctx, sig2, msg, &keypair, extraparams.ndata) == 1);
        CHECK(memcmp_var(sig, sig2, sizeof(sig)) == 0);
        */
}

/**
  | Creates N_SIGS valid signatures and
  | verifies them with verify and verify_batch
  | (TODO). Then flips some bits and checks
  | that verification now fails.
  |
  */
pub fn test_schnorrsig_sign_verify()  {

    pub const N_SIGS: usize = 3;
    
    todo!();
        /*
            unsigned char sk[32];
        unsigned char msg[N_SIGS][32];
        unsigned char sig[N_SIGS][64];
        size_t i;
        keypair keypair;
        xonly_pubkey pk;
        scalar s;

        testrand256(sk);
        CHECK(keypair_create(ctx, &keypair, sk));
        CHECK(keypair_xonly_pub(ctx, &pk, NULL, &keypair));

        for (i = 0; i < N_SIGS; i++) {
            testrand256(msg[i]);
            CHECK(schnorrsig_sign(ctx, sig[i], msg[i], &keypair, NULL));
            CHECK(schnorrsig_verify(ctx, sig[i], msg[i], sizeof(msg[i]), &pk));
        }

        {
            /* Flip a few bits in the signature and in the message and check that
             * verify and verify_batch (TODO) fail */
            size_t sig_idx = testrand_int(N_SIGS);
            size_t byte_idx = testrand_int(32);
            unsigned char xorbyte = testrand_int(254)+1;
            sig[sig_idx][byte_idx] ^= xorbyte;
            CHECK(!schnorrsig_verify(ctx, sig[sig_idx], msg[sig_idx], sizeof(msg[sig_idx]), &pk));
            sig[sig_idx][byte_idx] ^= xorbyte;

            byte_idx = testrand_int(32);
            sig[sig_idx][32+byte_idx] ^= xorbyte;
            CHECK(!schnorrsig_verify(ctx, sig[sig_idx], msg[sig_idx], sizeof(msg[sig_idx]), &pk));
            sig[sig_idx][32+byte_idx] ^= xorbyte;

            byte_idx = testrand_int(32);
            msg[sig_idx][byte_idx] ^= xorbyte;
            CHECK(!schnorrsig_verify(ctx, sig[sig_idx], msg[sig_idx], sizeof(msg[sig_idx]), &pk));
            msg[sig_idx][byte_idx] ^= xorbyte;

            /* Check that above bitflips have been reversed correctly */
            CHECK(schnorrsig_verify(ctx, sig[sig_idx], msg[sig_idx], sizeof(msg[sig_idx]), &pk));
        }

        /* Test overflowing s */
        CHECK(schnorrsig_sign(ctx, sig[0], msg[0], &keypair, NULL));
        CHECK(schnorrsig_verify(ctx, sig[0], msg[0], sizeof(msg[0]), &pk));
        memset(&sig[0][32], 0xFF, 32);
        CHECK(!schnorrsig_verify(ctx, sig[0], msg[0], sizeof(msg[0]), &pk));

        /* Test negative s */
        CHECK(schnorrsig_sign(ctx, sig[0], msg[0], &keypair, NULL));
        CHECK(schnorrsig_verify(ctx, sig[0], msg[0], sizeof(msg[0]), &pk));
        scalar_set_b32(&s, &sig[0][32], NULL);
        scalar_negate(&s, &s);
        scalar_get_b32(&sig[0][32], &s);
        CHECK(!schnorrsig_verify(ctx, sig[0], msg[0], sizeof(msg[0]), &pk));

        /* The empty message can be signed & verified */
        CHECK(schnorrsig_sign_custom(ctx, sig[0], NULL, 0, &keypair, NULL) == 1);
        CHECK(schnorrsig_verify(ctx, sig[0], NULL, 0, &pk) == 1);

        {
            /* Test varying message lengths */
            unsigned char msg_large[32 * 8];
            uint32_t msglen  = testrand_int(sizeof(msg_large));
            for (i = 0; i < sizeof(msg_large); i += 32) {
                testrand256(&msg_large[i]);
            }
            CHECK(schnorrsig_sign_custom(ctx, sig[0], msg_large, msglen, &keypair, NULL) == 1);
            CHECK(schnorrsig_verify(ctx, sig[0], msg_large, msglen, &pk) == 1);
            /* Verification for a random wrong message length fails */
            msglen = (msglen + (sizeof(msg_large) - 1)) % sizeof(msg_large);
            CHECK(schnorrsig_verify(ctx, sig[0], msg_large, msglen, &pk) == 0);
        }
        */
}

pub fn test_schnorrsig_taproot()  {
    
    todo!();
        /*
            unsigned char sk[32];
        keypair keypair;
        xonly_pubkey internal_pk;
        unsigned char internal_pk_bytes[32];
        xonly_pubkey output_pk;
        unsigned char output_pk_bytes[32];
        unsigned char tweak[32];
        int pk_parity;
        unsigned char msg[32];
        unsigned char sig[64];

        /* Create output key */
        testrand256(sk);
        CHECK(keypair_create(ctx, &keypair, sk) == 1);
        CHECK(keypair_xonly_pub(ctx, &internal_pk, NULL, &keypair) == 1);
        /* In actual taproot the tweak would be hash of internal_pk */
        CHECK(xonly_pubkey_serialize(ctx, tweak, &internal_pk) == 1);
        CHECK(keypair_xonly_tweak_add(ctx, &keypair, tweak) == 1);
        CHECK(keypair_xonly_pub(ctx, &output_pk, &pk_parity, &keypair) == 1);
        CHECK(xonly_pubkey_serialize(ctx, output_pk_bytes, &output_pk) == 1);

        /* Key spend */
        testrand256(msg);
        CHECK(schnorrsig_sign(ctx, sig, msg, &keypair, NULL) == 1);
        /* Verify key spend */
        CHECK(xonly_pubkey_parse(ctx, &output_pk, output_pk_bytes) == 1);
        CHECK(schnorrsig_verify(ctx, sig, msg, sizeof(msg), &output_pk) == 1);

        /* Script spend */
        CHECK(xonly_pubkey_serialize(ctx, internal_pk_bytes, &internal_pk) == 1);
        /* Verify script spend */
        CHECK(xonly_pubkey_parse(ctx, &internal_pk, internal_pk_bytes) == 1);
        CHECK(xonly_pubkey_tweak_add_check(ctx, output_pk_bytes, pk_parity, &internal_pk, tweak) == 1);
        */
}

pub fn run_schnorrsig_tests()  {
    
    todo!();
        /*
            int i;
        run_nonce_function_bip340_tests();

        test_schnorrsig_api();
        test_schnorrsig_sha256_tagged();
        test_schnorrsig_bip_vectors();
        for (i = 0; i < count; i++) {
            test_schnorrsig_sign();
            test_schnorrsig_sign_verify();
        }
        test_schnorrsig_taproot();
        */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/modules/schnorrsig/tests_exhaustive_impl.h]

lazy_static!{
    /*
    static const unsigned char invalid_pubkey_bytes[][32] = {
        /* 0 */
        {
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        },
        /* 2 */
        {
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2
        },
        /* order */
        {
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ((EXHAUSTIVE_TEST_ORDER + 0UL) >> 24) & 0xFF,
            ((EXHAUSTIVE_TEST_ORDER + 0UL) >> 16) & 0xFF,
            ((EXHAUSTIVE_TEST_ORDER + 0UL) >> 8) & 0xFF,
            (EXHAUSTIVE_TEST_ORDER + 0UL) & 0xFF
        },
        /* order + 1 */
        {
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ((EXHAUSTIVE_TEST_ORDER + 1UL) >> 24) & 0xFF,
            ((EXHAUSTIVE_TEST_ORDER + 1UL) >> 16) & 0xFF,
            ((EXHAUSTIVE_TEST_ORDER + 1UL) >> 8) & 0xFF,
            (EXHAUSTIVE_TEST_ORDER + 1UL) & 0xFF
        },
        /* field size */
        {
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x2F
        },
        /* field size + 1 (note that 1 is legal) */
        {
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x30
        },
        /* 2^256 - 1 */
        {
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
        }
    };
    */
}

macro_rules! NUM_INVALID_KEYS {
    () => {
        /*
                (sizeof(invalid_pubkey_bytes) / sizeof(invalid_pubkey_bytes[0]))
        */
    }
}

pub fn hardened_nonce_function_smallint(
        nonce32:    *mut u8,
        msg:        *const u8,
        msglen:     usize,
        key32:      *const u8,
        xonly_pk32: *const u8,
        algo:       *const u8,
        algolen:    usize,
        data:       *mut c_void) -> i32 {
    
    todo!();
        /*
            scalar s;
        int *idata = data;
        (c_void)msg;
        (c_void)msglen;
        (c_void)key32;
        (c_void)xonly_pk32;
        (c_void)algo;
        (c_void)algolen;
        scalar_set_int(&s, *idata);
        scalar_get_b32(nonce32, &s);
        return 1;
        */
}

pub fn test_exhaustive_schnorrsig_verify(
        ctx:                *const Secp256k1Context,
        pubkeys:            *const XOnlyPubKey,
        xonly_pubkey_bytes: [*mut u8; 32],
        parities:           *const i32)  {
    
    todo!();
        /*
            int d;
        uint64_t iter = 0;
        /* Iterate over the possible public keys to verify against (through their corresponding DL d). */
        for (d = 1; d <= EXHAUSTIVE_TEST_ORDER / 2; ++d) {
            int actual_d;
            unsigned k;
            unsigned char pk32[32];
            memcpy(pk32, xonly_pubkey_bytes[d - 1], 32);
            actual_d = parities[d - 1] ? EXHAUSTIVE_TEST_ORDER - d : d;
            /* Iterate over the possible valid first 32 bytes in the signature, through their corresponding DL k.
               Values above EXHAUSTIVE_TEST_ORDER/2 refer to the entries in invalid_pubkey_bytes. */
            for (k = 1; k <= EXHAUSTIVE_TEST_ORDER / 2 + NUM_INVALID_KEYS; ++k) {
                unsigned char sig64[64];
                int actual_k = -1;
                int e_done[EXHAUSTIVE_TEST_ORDER] = {0};
                int e_count_done = 0;
                if (skip_section(&iter)) continue;
                if (k <= EXHAUSTIVE_TEST_ORDER / 2) {
                    memcpy(sig64, xonly_pubkey_bytes[k - 1], 32);
                    actual_k = parities[k - 1] ? EXHAUSTIVE_TEST_ORDER - k : k;
                } else {
                    memcpy(sig64, invalid_pubkey_bytes[k - 1 - EXHAUSTIVE_TEST_ORDER / 2], 32);
                }
                /* Randomly generate messages until all challenges have been hit. */
                while (e_count_done < EXHAUSTIVE_TEST_ORDER) {
                    scalar e;
                    unsigned char msg32[32];
                    testrand256(msg32);
                    schnorrsig_challenge(&e, sig64, msg32, sizeof(msg32), pk32);
                    /* Only do work if we hit a challenge we haven't tried before. */
                    if (!e_done[e]) {
                        /* Iterate over the possible valid last 32 bytes in the signature.
                           0..order=that s value; order+1=random bytes */
                        int count_valid = 0, s;
                        for (s = 0; s <= EXHAUSTIVE_TEST_ORDER + 1; ++s) {
                            int expect_valid, valid;
                            if (s <= EXHAUSTIVE_TEST_ORDER) {
                                scalar s_s;
                                scalar_set_int(&s_s, s);
                                scalar_get_b32(sig64 + 32, &s_s);
                                expect_valid = actual_k != -1 && s != EXHAUSTIVE_TEST_ORDER &&
                                               (s_s == (actual_k + actual_d * e) % EXHAUSTIVE_TEST_ORDER);
                            } else {
                                testrand256(sig64 + 32);
                                expect_valid = 0;
                            }
                            valid = schnorrsig_verify(ctx, sig64, msg32, sizeof(msg32), &pubkeys[d - 1]);
                            CHECK(valid == expect_valid);
                            count_valid += valid;
                        }
                        /* Exactly one s value must verify, unless R is illegal. */
                        CHECK(count_valid == (actual_k != -1));
                        /* Don't retry other messages that result in the same challenge. */
                        e_done[e] = 1;
                        ++e_count_done;
                    }
                }
            }
        }
        */
}

pub fn test_exhaustive_schnorrsig_sign(
        ctx:                *const Secp256k1Context,
        xonly_pubkey_bytes: [*mut u8; 32],
        keypairs:           *const KeyPair,
        parities:           *const i32)  {
    
    todo!();
        /*
            int d, k;
        uint64_t iter = 0;
        schnorrsig_extraparams extraparams = SCHNORRSIG_EXTRAPARAMS_INIT;

        /* Loop over keys. */
        for (d = 1; d < EXHAUSTIVE_TEST_ORDER; ++d) {
            int actual_d = d;
            if (parities[d - 1]) actual_d = EXHAUSTIVE_TEST_ORDER - d;
            /* Loop over nonces. */
            for (k = 1; k < EXHAUSTIVE_TEST_ORDER; ++k) {
                int e_done[EXHAUSTIVE_TEST_ORDER] = {0};
                int e_count_done = 0;
                unsigned char msg32[32];
                unsigned char sig64[64];
                int actual_k = k;
                if (skip_section(&iter)) continue;
                extraparams.noncefp = hardened_nonce_function_smallint;
                extraparams.ndata = &k;
                if (parities[k - 1]) actual_k = EXHAUSTIVE_TEST_ORDER - k;
                /* Generate random messages until all challenges have been tried. */
                while (e_count_done < EXHAUSTIVE_TEST_ORDER) {
                    scalar e;
                    testrand256(msg32);
                    schnorrsig_challenge(&e, xonly_pubkey_bytes[k - 1], msg32, sizeof(msg32), xonly_pubkey_bytes[d - 1]);
                    /* Only do work if we hit a challenge we haven't tried before. */
                    if (!e_done[e]) {
                        scalar expected_s = (actual_k + e * actual_d) % EXHAUSTIVE_TEST_ORDER;
                        unsigned char expected_s_bytes[32];
                        scalar_get_b32(expected_s_bytes, &expected_s);
                        /* Invoke the real function to construct a signature. */
                        CHECK(schnorrsig_sign_custom(ctx, sig64, msg32, sizeof(msg32), &keypairs[d - 1], &extraparams));
                        /* The first 32 bytes must match the xonly pubkey for the specified k. */
                        CHECK(memcmp_var(sig64, xonly_pubkey_bytes[k - 1], 32) == 0);
                        /* The last 32 bytes must match the expected s value. */
                        CHECK(memcmp_var(sig64 + 32, expected_s_bytes, 32) == 0);
                        /* Don't retry other messages that result in the same challenge. */
                        e_done[e] = 1;
                        ++e_count_done;
                    }
                }
            }
        }
        */
}

pub fn test_exhaustive_schnorrsig(ctx: *const Secp256k1Context)  {
    
    todo!();
        /*
            keypair keypair[EXHAUSTIVE_TEST_ORDER - 1];
        xonly_pubkey xonly_pubkey[EXHAUSTIVE_TEST_ORDER - 1];
        int parity[EXHAUSTIVE_TEST_ORDER - 1];
        unsigned char xonly_pubkey_bytes[EXHAUSTIVE_TEST_ORDER - 1][32];
        unsigned i;

        /* Verify that all invalid_pubkey_bytes are actually invalid. */
        for (i = 0; i < NUM_INVALID_KEYS; ++i) {
            xonly_pubkey pk;
            CHECK(!xonly_pubkey_parse(ctx, &pk, invalid_pubkey_bytes[i]));
        }

        /* Construct keypairs and xonly-pubkeys for the entire group. */
        for (i = 1; i < EXHAUSTIVE_TEST_ORDER; ++i) {
            scalar scalar_i;
            unsigned char buf[32];
            scalar_set_int(&scalar_i, i);
            scalar_get_b32(buf, &scalar_i);
            CHECK(keypair_create(ctx, &keypair[i - 1], buf));
            CHECK(keypair_xonly_pub(ctx, &xonly_pubkey[i - 1], &parity[i - 1], &keypair[i - 1]));
            CHECK(xonly_pubkey_serialize(ctx, xonly_pubkey_bytes[i - 1], &xonly_pubkey[i - 1]));
        }

        test_exhaustive_schnorrsig_sign(ctx, xonly_pubkey_bytes, keypair, parity);
        test_exhaustive_schnorrsig_verify(ctx, xonly_pubkey, xonly_pubkey_bytes, parity);
        */
}
