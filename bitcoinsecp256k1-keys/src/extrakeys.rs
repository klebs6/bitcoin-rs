crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/include/extrakeys.h]

/**
  | Opaque data structure that holds a parsed
  | and valid "x-only" public key.
  | 
  | An x-only pubkey encodes a point whose
  | Y coordinate is even. It is serialized
  | using only its X coordinate (32 bytes).
  | See BIP-340 for more information about
  | x-only pubkeys.
  | 
  | The exact representation of data inside
  | is implementation defined and not guaranteed
  | to be portable between different platforms
  | or versions. It is however guaranteed
  | to be 64 bytes in size, and can be safely
  | copied/moved.
  | 
  | If you need to convert to a format suitable
  | for storage, transmission, use use
  | xonly_pubkey_serialize and xonly_pubkey_parse.
  | To compare keys, use xonly_pubkey_cmp.
  |
  */
pub struct XOnlyPubKey {
    data: [u8; 64],
}

/**
  | Opaque data structure that holds a keypair
  | consisting of a secret and a public key.
  | 
  | The exact representation of data inside
  | is implementation defined and not guaranteed
  | to be portable between different platforms
  | or versions. It is however guaranteed
  | to be 96 bytes in size, and can be safely
  | copied/moved.
  |
  */
pub struct KeyPair {
    data: [u8; 96],
}

/** 
 | Parse a 32-byte sequence into a xonly_pubkey
 | object.
 |
 |  Returns: 1 if the public key was fully valid.
 |
 |           0 if the public key could not be
 |           parsed or is invalid.
 |
 |  Args:   ctx: a secp256k1 context object
 |               (cannot be NULL).
 |
 |  Out: pubkey: pointer to a pubkey object. If
 |               1 is returned, it is set to
 |               a parsed version of input. If
 |               not, it's set to an invalid
 |               value.  (cannot be NULL).
 |
 |  In: input32: pointer to a serialized
 |               xonly_pubkey (cannot be NULL)
 */
lazy_static!{
    /*
    API WARN_UNUSED_RESULT int xonly_pubkey_parse(
        const context* ctx,
        xonly_pubkey* pubkey,
        const unsigned char *input32
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Serialize an xonly_pubkey object into a 32-byte
 | sequence.
 |
 |  Returns: 1 always.
 |
 |  Args:     ctx: a secp256k1 context object
 |                 (cannot be NULL).
 |
 |  Out: output32: a pointer to a 32-byte array to
 |                 place the serialized key in
 |                 (cannot be NULL).
 |
 |  In:    pubkey: a pointer to a xonly_pubkey
 |                 containing an initialized
 |                 public key (cannot be NULL).
 */
lazy_static!{
    /*
    API int xonly_pubkey_serialize(
        const context* ctx,
        unsigned char *output32,
        const xonly_pubkey* pubkey
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Compare two x-only public keys using
 | lexicographic order
 |
 |  Returns: <0 if the first public key is less
 |              than the second
 |
 |           >0 if the first public key is greater
 |           than the second
 |
 |           0 if the two public keys are equal
 |
 |  Args: ctx:      a secp256k1 context object.
 |  In:   pubkey1:  first public key to compare
 |        pubkey2:  second public key to compare
 */
lazy_static!{
    /*
    API int xonly_pubkey_cmp(
        const context* ctx,
        const xonly_pubkey* pk1,
        const xonly_pubkey* pk2
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Converts a pubkey into a xonly_pubkey.
 |
 |  Returns: 1 if the public key was successfully
 |           converted
 |
 |           0 otherwise
 |
 |  Args:         ctx: pointer to a context object
 |                     (cannot be NULL)
 |
 |  Out: xonly_pubkey: pointer to an x-only public
 |                     key object for placing the
 |                     converted public key
 |                     (cannot be NULL)
 |
 |          pk_parity: pointer to an integer that
 |                     will be set to 1 if the
 |                     point encoded by
 |                     xonly_pubkey is the
 |                     negation of the pubkey and
 |                     set to 0 otherwise. (can be
 |                     NULL)
 |
 |  In:        pubkey: pointer to a public key
 |                     that is converted (cannot
 |                     be NULL)
 |
 */
lazy_static!{
    /*
    API WARN_UNUSED_RESULT int xonly_pubkey_from_pubkey(
        const context* ctx,
        xonly_pubkey *xonly_pubkey,
        int *pk_parity,
        const pubkey *pubkey
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(4);
    */
}

/** 
 | Tweak an x-only public key by adding the
 | generator multiplied with tweak32 to it.
 |
 | Note that the resulting point can not in
 | general be represented by an x-only pubkey
 | because it may have an odd
 | Y coordinate. Instead, the output_pubkey is
 | a normal pubkey.
 |
 |  Returns: 0 if the arguments are invalid or the
 |           resulting public key would be invalid
 |           (only when the tweak is the negation
 |           of the corresponding secret
 |           key). 1 otherwise.
 |
 |  Args:           ctx: pointer to a context
 |                       object initialized for
 |                       verification (cannot be
 |                       NULL)
 |
 |  Out:  output_pubkey: pointer to a public key
 |                       to store the result. Will
 |                       be set to an invalid
 |                       value if this function
 |                       returns 0 (cannot be
 |                       NULL)
 |
 |  In: internal_pubkey: pointer to an x-only
 |                       pubkey to apply the tweak
 |                       to.  (cannot be NULL).
 |
 |              tweak32: pointer to a 32-byte
 |                       tweak. If the tweak is
 |                       invalid according to
 |                       ec_seckey_verify, this
 |                       function returns 0. For
 |                       uniformly random 32-byte
 |                       arrays the chance of
 |                       being invalid is
 |                       negligible (around 1 in
 |                       2^128) (cannot be NULL).
 */
lazy_static!{
    /*
    API WARN_UNUSED_RESULT int xonly_pubkey_tweak_add(
        const context* ctx,
        pubkey *output_pubkey,
        const xonly_pubkey *internal_pubkey,
        const unsigned char *tweak32
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3) ARG_NONNULL(4);
    */
}

/** 
 | Checks that a tweaked pubkey is the result of
 | calling xonly_pubkey_tweak_add with
 | internal_pubkey and tweak32.
 |
 |  The tweaked pubkey is represented by its
 |  32-byte x-only serialization and its
 |  pk_parity, which can both be obtained by
 |  converting the result of tweak_add to
 |  a xonly_pubkey.
 |
 |  Note that this alone does _not_ verify that
 |  the tweaked pubkey is a commitment. If the
 |  tweak is not chosen in a specific way, the
 |  tweaked pubkey can easily be the result of
 |  a different internal_pubkey and tweak.
 |
 |  Returns: 0 if the arguments are invalid or the
 |           tweaked pubkey is not the result of
 |           tweaking the internal_pubkey with
 |           tweak32. 1 otherwise.
 |
 |  Args:            ctx: pointer to a context
 |                        object initialized for
 |                        verification (cannot be
 |                        NULL)
 |
 |  In: tweaked_pubkey32: pointer to a serialized
 |                        xonly_pubkey (cannot be
 |                        NULL)
 |
 |     tweaked_pk_parity: the parity of the
 |                        tweaked pubkey (whose
 |                        serialization is passed
 |                        in as tweaked_pubkey32). 
 |                        This must match the
 |                        pk_parity value that is
 |                        returned when calling
 |                        xonly_pubkey with the
 |                        tweaked pubkey, or this
 |                        function will fail.
 |
 |       internal_pubkey: pointer to an x-only
 |                        public key object to
 |                        apply the tweak to
 |                        (cannot be NULL)
 |
 |               tweak32: pointer to a 32-byte
 |               tweak (cannot be NULL)
 |
 */
lazy_static!{
    /*
    API WARN_UNUSED_RESULT int xonly_pubkey_tweak_add_check(
        const context* ctx,
        const unsigned char *tweaked_pubkey32,
        int tweaked_pk_parity,
        const xonly_pubkey *internal_pubkey,
        const unsigned char *tweak32
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(4) ARG_NONNULL(5);
    */
}

/** 
 | Compute the keypair for a secret key.
 |
 |  Returns: 1: secret was valid, keypair is ready
 |              to use
 |
 |           0: secret was invalid, try again with
 |              a different secret
 |
 |  Args:    ctx: pointer to a context object,
 |                initialized for signing (cannot
 |                be NULL)
 |
 |  Out: keypair: pointer to the created keypair
 |                (cannot be NULL)
 |
 |  In:   seckey: pointer to a 32-byte secret key
 |                (cannot be NULL)
 */
lazy_static!{
    /*
    API WARN_UNUSED_RESULT int keypair_create(
        const context* ctx,
        keypair *keypair,
        const unsigned char *seckey
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Get the secret key from a keypair.
 |
 |  Returns: 0 if the arguments are
 |           invalid. 1 otherwise.
 |
 |  Args:   ctx: pointer to a context object
 |               (cannot be NULL)
 |
 |  Out: seckey: pointer to a 32-byte buffer for
 |               the secret key (cannot be NULL)
 |
 |  In: keypair: pointer to a keypair (cannot be
 |               NULL)
 */
lazy_static!{
    /*
    API WARN_UNUSED_RESULT int keypair_sec(
        const context* ctx,
        unsigned char *seckey,
        const keypair *keypair
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Get the public key from a keypair.
 |
 |  Returns: 0 if the arguments are
 |             invalid. 1 otherwise.
 |
 |  Args:    ctx: pointer to a context object
 |                (cannot be NULL)
 |
 |  Out: pubkey: pointer to a pubkey object. If
 |               1 is returned, it is set to the
 |               keypair public key. If not, it's
 |               set to an invalid value.  (cannot
 |               be NULL)
 |
 |  In: keypair: pointer to a keypair (cannot be
 |               NULL)
 |
 */
lazy_static!{
    /*
    API WARN_UNUSED_RESULT int keypair_pub(
        const context* ctx,
        pubkey *pubkey,
        const keypair *keypair
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Get the x-only public key from a keypair.
 |
 |  This is the same as calling keypair_pub and
 |  then xonly_pubkey_from_pubkey.
 |
 |  Returns: 0 if the arguments are
 |             invalid. 1 otherwise.
 |
 |  Args:   ctx: pointer to a context object
 |               (cannot be NULL)
 |
 |  Out: pubkey: pointer to an xonly_pubkey
 |               object. If 1 is returned, it is
 |               set to the keypair public key
 |               after converting it to an
 |               xonly_pubkey. If not, it's set to
 |               an invalid value (cannot be
 |               NULL).
 |
 |    pk_parity: pointer to an integer that will
 |               be set to the pk_parity argument
 |               of xonly_pubkey_from_pubkey (can
 |               be NULL).
 |
 |  In: keypair: pointer to a keypair (cannot be
 |               NULL)
 |
 */
lazy_static!{
    /*
    API WARN_UNUSED_RESULT int keypair_xonly_pub(
        const context* ctx,
        xonly_pubkey *pubkey,
        int *pk_parity,
        const keypair *keypair
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(4);
    */
}

/** 
 | Tweak a keypair by adding tweak32 to the secret
 | key and updating the public key accordingly.
 |
 |  Calling this function and then keypair_pub
 |  results in the same public key as calling
 |  keypair_xonly_pub and then
 |  xonly_pubkey_tweak_add.
 |
 |  Returns: 0 if the arguments are invalid or the
 |           resulting keypair would be invalid
 |           (only when the tweak is the negation
 |           of the keypair's secret
 |           key). 1 otherwise.
 |
 |  Args:       ctx: pointer to a context object
 |                   initialized for verification
 |                   (cannot be NULL)
 |
 |  In/Out: keypair: pointer to a keypair to apply
 |                   the tweak to. Will be set to
 |                   an invalid value if this
 |                   function returns 0 (cannot be
 |                   NULL).
 |
 |  In:     tweak32: pointer to a 32-byte
 |                   tweak. If the tweak is
 |                   invalid according to
 |                   ec_seckey_verify, this
 |                   function returns 0. For
 |                   uniformly random 32-byte
 |                   arrays the chance of being
 |                   invalid is negligible (around
 |                   1 in 2^128) (cannot be NULL).
 |
 */
lazy_static!{
    /*
    API WARN_UNUSED_RESULT int keypair_xonly_tweak_add(
        const context* ctx,
        keypair *keypair,
        const unsigned char *tweak32
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/modules/extrakeys/main_impl.h]

#[inline] pub fn xonly_pubkey_load(
        ctx:    *const Secp256k1Context,
        ge:     *mut Ge,
        pubkey: *const XOnlyPubKey) -> i32 {
    
    todo!();
        /*
            return pubkey_load(ctx, ge, (const pubkey *) pubkey);
        */
}

#[inline] pub fn xonly_pubkey_save(
        pubkey: *mut XOnlyPubKey,
        ge:     *mut Ge)  {
    
    todo!();
        /*
            pubkey_save((pubkey *) pubkey, ge);
        */
}

pub fn xonly_pubkey_parse(
        ctx:     *const Secp256k1Context,
        pubkey:  *mut XOnlyPubKey,
        input32: *const u8) -> i32 {
    
    todo!();
        /*
            ge pk;
        fe x;

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(pubkey != NULL);
        memset(pubkey, 0, sizeof(*pubkey));
        ARG_CHECK(input32 != NULL);

        if (!fe_set_b32(&x, input32)) {
            return 0;
        }
        if (!ge_set_xo_var(&pk, &x, 0)) {
            return 0;
        }
        if (!ge_is_in_correct_subgroup(&pk)) {
            return 0;
        }
        xonly_pubkey_save(pubkey, &pk);
        return 1;
        */
}

pub fn xonly_pubkey_serialize(
        ctx:      *const Secp256k1Context,
        output32: *mut u8,
        pubkey:   *const XOnlyPubKey) -> i32 {
    
    todo!();
        /*
            ge pk;

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(output32 != NULL);
        memset(output32, 0, 32);
        ARG_CHECK(pubkey != NULL);

        if (!xonly_pubkey_load(ctx, &pk, pubkey)) {
            return 0;
        }
        fe_get_b32(output32, &pk.x);
        return 1;
        */
}

pub fn xonly_pubkey_cmp(
        ctx: *const Secp256k1Context,
        pk0: *const XOnlyPubKey,
        pk1: *const XOnlyPubKey) -> i32 {
    
    todo!();
        /*
            unsigned char out[2][32];
        const xonly_pubkey* pk[2];
        int i;

        VERIFY_CHECK(ctx != NULL);
        pk[0] = pk0; pk[1] = pk1;
        for (i = 0; i < 2; i++) {
            /* If the public key is NULL or invalid, xonly_pubkey_serialize will
             * call the illegal_callback and return 0. In that case we will
             * serialize the key as all zeros which is less than any valid public
             * key. This results in consistent comparisons even if NULL or invalid
             * pubkeys are involved and prevents edge cases such as sorting
             * algorithms that use this function and do not terminate as a
             * result. */
            if (!xonly_pubkey_serialize(ctx, out[i], pk[i])) {
                /* Note that xonly_pubkey_serialize should already set the output to
                 * zero in that case, but it's not guaranteed by the API, we can't
                 * test it and writing a VERIFY_CHECK is more complex than
                 * explicitly memsetting (again). */
                memset(out[i], 0, sizeof(out[i]));
            }
        }
        return memcmp_var(out[0], out[1], sizeof(out[1]));
        */
}

/**
  | Keeps a group element as is if it has an
  | even Y and otherwise negates it. y_parity
  | is set to 0 in the former case and to 1 in
  | the latter case.
  | 
  | Requires that the coordinates of r are
  | normalized.
  |
  */
pub fn extrakeys_ge_even_y(r: *mut Ge) -> i32 {
    
    todo!();
        /*
            int y_parity = 0;
        VERIFY_CHECK(!ge_is_infinity(r));

        if (fe_is_odd(&r->y)) {
            fe_negate(&r->y, &r->y, 1);
            y_parity = 1;
        }
        return y_parity;
        */
}

pub fn xonly_pubkey_from_pubkey(
        ctx:          *const Secp256k1Context,
        xonly_pubkey: *mut XOnlyPubKey,
        pk_parity:    *mut i32,
        pubkey:       *const PubKey) -> i32 {
    
    todo!();
        /*
            ge pk;
        int tmp;

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(xonly_pubkey != NULL);
        ARG_CHECK(pubkey != NULL);

        if (!pubkey_load(ctx, &pk, pubkey)) {
            return 0;
        }
        tmp = extrakeys_ge_even_y(&pk);
        if (pk_parity != NULL) {
            *pk_parity = tmp;
        }
        xonly_pubkey_save(xonly_pubkey, &pk);
        return 1;
        */
}

pub fn xonly_pubkey_tweak_add(
        ctx:             *const Secp256k1Context,
        output_pubkey:   *mut PubKey,
        internal_pubkey: *const XOnlyPubKey,
        tweak32:         *const u8) -> i32 {
    
    todo!();
        /*
            ge pk;

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(output_pubkey != NULL);
        memset(output_pubkey, 0, sizeof(*output_pubkey));
        ARG_CHECK(ecmult_context_is_built(&ctx->ecmult_ctx));
        ARG_CHECK(internal_pubkey != NULL);
        ARG_CHECK(tweak32 != NULL);

        if (!xonly_pubkey_load(ctx, &pk, internal_pubkey)
            || !ec_pubkey_tweak_add_helper(&ctx->ecmult_ctx, &pk, tweak32)) {
            return 0;
        }
        pubkey_save(output_pubkey, &pk);
        return 1;
        */
}

pub fn xonly_pubkey_tweak_add_check(
        ctx:               *const Secp256k1Context,
        tweaked_pubkey32:  *const u8,
        tweaked_pk_parity: i32,
        internal_pubkey:   *const XOnlyPubKey,
        tweak32:           *const u8) -> i32 {
    
    todo!();
        /*
        ge pk;
        unsigned char pk_expected32[32];

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(ecmult_context_is_built(&ctx->ecmult_ctx));
        ARG_CHECK(internal_pubkey != NULL);
        ARG_CHECK(tweaked_pubkey32 != NULL);
        ARG_CHECK(tweak32 != NULL);

        if (!xonly_pubkey_load(ctx, &pk, internal_pubkey)
            || !ec_pubkey_tweak_add_helper(&ctx->ecmult_ctx, &pk, tweak32)) {
            return 0;
        }
        fe_normalize_var(&pk.x);
        fe_normalize_var(&pk.y);
        fe_get_b32(pk_expected32, &pk.x);

        return memcmp_var(&pk_expected32, tweaked_pubkey32, 32) == 0
                && fe_is_odd(&pk.y) == tweaked_pk_parity;
        */
}

pub fn keypair_save(
        keypair: *mut KeyPair,
        sk:      *const Scalar,
        pk:      *mut Ge)  {
    
    todo!();
        /*
        scalar_get_b32(&keypair->data[0], sk);
        pubkey_save((pubkey *)&keypair->data[32], pk);
        */
}

pub fn keypair_seckey_load(
        ctx:     *const Secp256k1Context,
        sk:      *mut Scalar,
        keypair: *const KeyPair) -> i32 {
    
    todo!();
        /*
        int ret;

        ret = scalar_set_b32_seckey(sk, &keypair->data[0]);
        /* We can declassify ret here because sk is only zero if a keypair function failed (which zeroes the keypair) and its return value is ignored. */
        declassify(ctx, &ret, sizeof(ret));
        ARG_CHECK(ret);
        return ret;
        */
}

/**
  | Load a keypair into pk and sk (if non-NULL).
  | This function declassifies pk and ARG_CHECKs
  | that the keypair is not invalid. It always
  | initializes sk and pk with dummy values.
  |
  */
pub fn keypair_load(
        ctx:     *const Secp256k1Context,
        sk:      *mut Scalar,
        pk:      *mut Ge,
        keypair: *const KeyPair) -> i32 {
    
    todo!();
        /*
        int ret;
        const pubkey *pubkey = (const pubkey *)&keypair->data[32];

        /* Need to declassify the pubkey because pubkey_load ARG_CHECKs if it's
         * invalid. */
        declassify(ctx, pubkey, sizeof(*pubkey));
        ret = pubkey_load(ctx, pk, pubkey);
        if (sk != NULL) {
            ret = ret && keypair_seckey_load(ctx, sk, keypair);
        }
        if (!ret) {
            *pk = ge_const_g;
            if (sk != NULL) {
                *sk = scalar_one;
            }
        }
        return ret;
        */
}

pub fn keypair_create(
        ctx:      *const Secp256k1Context,
        keypair:  *mut KeyPair,
        seckey32: *const u8) -> i32 {
    
    todo!();
        /*
        scalar sk;
        ge pk;
        int ret = 0;
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(keypair != NULL);
        memset(keypair, 0, sizeof(*keypair));
        ARG_CHECK(ecmult_gen_context_is_built(&ctx->ecmult_gen_ctx));
        ARG_CHECK(seckey32 != NULL);

        ret = ec_pubkey_create_helper(&ctx->ecmult_gen_ctx, &sk, &pk, seckey32);
        keypair_save(keypair, &sk, &pk);
        memczero(keypair, sizeof(*keypair), !ret);

        scalar_clear(&sk);
        return ret;
        */
}

pub fn keypair_sec(
        ctx:     *const Secp256k1Context,
        seckey:  *mut u8,
        keypair: *const KeyPair) -> i32 {
    
    todo!();
        /*
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(seckey != NULL);
        memset(seckey, 0, 32);
        ARG_CHECK(keypair != NULL);

        memcpy(seckey, &keypair->data[0], 32);
        return 1;
        */
}

pub fn keypair_pub(
        ctx:     *const Secp256k1Context,
        pubkey:  *mut PubKey,
        keypair: *const KeyPair) -> i32 {
    
    todo!();
        /*
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(pubkey != NULL);
        memset(pubkey, 0, sizeof(*pubkey));
        ARG_CHECK(keypair != NULL);

        memcpy(pubkey->data, &keypair->data[32], sizeof(*pubkey));
        return 1;
        */
}

pub fn keypair_xonly_pub(
        ctx:       *const Secp256k1Context,
        pubkey:    *mut XOnlyPubKey,
        pk_parity: *mut i32,
        keypair:   *const KeyPair) -> i32 {
    
    todo!();
        /*
        ge pk;
        int tmp;

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(pubkey != NULL);
        memset(pubkey, 0, sizeof(*pubkey));
        ARG_CHECK(keypair != NULL);

        if (!keypair_load(ctx, NULL, &pk, keypair)) {
            return 0;
        }
        tmp = extrakeys_ge_even_y(&pk);
        if (pk_parity != NULL) {
            *pk_parity = tmp;
        }
        xonly_pubkey_save(pubkey, &pk);

        return 1;
        */
}

pub fn keypair_xonly_tweak_add(
        ctx:     *const Secp256k1Context,
        keypair: *mut KeyPair,
        tweak32: *const u8) -> i32 {
    
    todo!();
        /*
        ge pk;
        scalar sk;
        int y_parity;
        int ret;

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(ecmult_context_is_built(&ctx->ecmult_ctx));
        ARG_CHECK(keypair != NULL);
        ARG_CHECK(tweak32 != NULL);

        ret = keypair_load(ctx, &sk, &pk, keypair);
        memset(keypair, 0, sizeof(*keypair));

        y_parity = extrakeys_ge_even_y(&pk);
        if (y_parity == 1) {
            scalar_negate(&sk, &sk);
        }

        ret &= ec_seckey_tweak_add_helper(&sk, tweak32);
        ret &= ec_pubkey_tweak_add_helper(&ctx->ecmult_ctx, &pk, tweak32);

        declassify(ctx, &ret, sizeof(ret));
        if (ret) {
            keypair_save(keypair, &sk, &pk);
        }

        scalar_clear(&sk);
        return ret;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/modules/extrakeys/tests_impl.h]

pub fn api_test_context(
        flags:  i32,
        ecount: *mut i32) -> *mut Secp256k1Context {
    
    todo!();
        /*
        context *ctx0 = context_create(flags);
        context_set_error_callback(ctx0, counting_illegal_callback_fn, ecount);
        context_set_illegal_callback(ctx0, counting_illegal_callback_fn, ecount);
        return ctx0;
        */
}

pub fn test_xonly_pubkey()  {
    
    todo!();
        /*
        pubkey pk;
        xonly_pubkey xonly_pk, xonly_pk_tmp;
        ge pk1;
        ge pk2;
        fe y;
        unsigned char sk[32];
        unsigned char xy_sk[32];
        unsigned char buf32[32];
        unsigned char ones32[32];
        unsigned char zeros64[64] = { 0 };
        int pk_parity;
        int i;

        int ecount;
        context *none = api_test_context(CONTEXT_NONE, &ecount);
        context *sign = api_test_context(CONTEXT_SIGN, &ecount);
        context *verify = api_test_context(CONTEXT_VERIFY, &ecount);

        testrand256(sk);
        memset(ones32, 0xFF, 32);
        testrand256(xy_sk);
        CHECK(ec_pubkey_create(sign, &pk, sk) == 1);
        CHECK(xonly_pubkey_from_pubkey(none, &xonly_pk, &pk_parity, &pk) == 1);

        /* Test xonly_pubkey_from_pubkey */
        ecount = 0;
        CHECK(xonly_pubkey_from_pubkey(none, &xonly_pk, &pk_parity, &pk) == 1);
        CHECK(xonly_pubkey_from_pubkey(sign, &xonly_pk, &pk_parity, &pk) == 1);
        CHECK(xonly_pubkey_from_pubkey(verify, &xonly_pk, &pk_parity, &pk) == 1);
        CHECK(xonly_pubkey_from_pubkey(none, NULL, &pk_parity, &pk) == 0);
        CHECK(ecount == 1);
        CHECK(xonly_pubkey_from_pubkey(none, &xonly_pk, NULL, &pk) == 1);
        CHECK(xonly_pubkey_from_pubkey(none, &xonly_pk, &pk_parity, NULL) == 0);
        CHECK(ecount == 2);
        memset(&pk, 0, sizeof(pk));
        CHECK(xonly_pubkey_from_pubkey(none, &xonly_pk, &pk_parity, &pk) == 0);
        CHECK(ecount == 3);

        /* Choose a secret key such that the resulting pubkey and xonly_pubkey match. */
        memset(sk, 0, sizeof(sk));
        sk[0] = 1;
        CHECK(ec_pubkey_create(ctx, &pk, sk) == 1);
        CHECK(xonly_pubkey_from_pubkey(ctx, &xonly_pk, &pk_parity, &pk) == 1);
        CHECK(memcmp_var(&pk, &xonly_pk, sizeof(pk)) == 0);
        CHECK(pk_parity == 0);

        /* Choose a secret key such that pubkey and xonly_pubkey are each others
         * negation. */
        sk[0] = 2;
        CHECK(ec_pubkey_create(ctx, &pk, sk) == 1);
        CHECK(xonly_pubkey_from_pubkey(ctx, &xonly_pk, &pk_parity, &pk) == 1);
        CHECK(memcmp_var(&xonly_pk, &pk, sizeof(xonly_pk)) != 0);
        CHECK(pk_parity == 1);
        pubkey_load(ctx, &pk1, &pk);
        pubkey_load(ctx, &pk2, (pubkey *) &xonly_pk);
        CHECK(fe_equal(&pk1.x, &pk2.x) == 1);
        fe_negate(&y, &pk2.y, 1);
        CHECK(fe_equal(&pk1.y, &y) == 1);

        /* Test xonly_pubkey_serialize and xonly_pubkey_parse */
        ecount = 0;
        CHECK(xonly_pubkey_serialize(none, NULL, &xonly_pk) == 0);
        CHECK(ecount == 1);
        CHECK(xonly_pubkey_serialize(none, buf32, NULL) == 0);
        CHECK(memcmp_var(buf32, zeros64, 32) == 0);
        CHECK(ecount == 2);
        {
            /* A pubkey filled with 0s will fail to serialize due to pubkey_load
             * special casing. */
            xonly_pubkey pk_tmp;
            memset(&pk_tmp, 0, sizeof(pk_tmp));
            CHECK(xonly_pubkey_serialize(none, buf32, &pk_tmp) == 0);
        }
        /* pubkey_load called illegal callback */
        CHECK(ecount == 3);

        CHECK(xonly_pubkey_serialize(none, buf32, &xonly_pk) == 1);
        ecount = 0;
        CHECK(xonly_pubkey_parse(none, NULL, buf32) == 0);
        CHECK(ecount == 1);
        CHECK(xonly_pubkey_parse(none, &xonly_pk, NULL) == 0);
        CHECK(ecount == 2);

        /* Serialization and parse roundtrip */
        CHECK(xonly_pubkey_from_pubkey(none, &xonly_pk, NULL, &pk) == 1);
        CHECK(xonly_pubkey_serialize(ctx, buf32, &xonly_pk) == 1);
        CHECK(xonly_pubkey_parse(ctx, &xonly_pk_tmp, buf32) == 1);
        CHECK(memcmp_var(&xonly_pk, &xonly_pk_tmp, sizeof(xonly_pk)) == 0);

        /* Test parsing invalid field elements */
        memset(&xonly_pk, 1, sizeof(xonly_pk));
        /* Overflowing field element */
        CHECK(xonly_pubkey_parse(none, &xonly_pk, ones32) == 0);
        CHECK(memcmp_var(&xonly_pk, zeros64, sizeof(xonly_pk)) == 0);
        memset(&xonly_pk, 1, sizeof(xonly_pk));
        /* There's no point with x-coordinate 0 on secp256k1 */
        CHECK(xonly_pubkey_parse(none, &xonly_pk, zeros64) == 0);
        CHECK(memcmp_var(&xonly_pk, zeros64, sizeof(xonly_pk)) == 0);
        /* If a random 32-byte string can not be parsed with ec_pubkey_parse
         * (because interpreted as X coordinate it does not correspond to a point on
         * the curve) then xonly_pubkey_parse should fail as well. */
        for (i = 0; i < count; i++) {
            unsigned char rand33[33];
            testrand256(&rand33[1]);
            rand33[0] = TAG_PUBKEY_EVEN;
            if (!ec_pubkey_parse(ctx, &pk, rand33, 33)) {
                memset(&xonly_pk, 1, sizeof(xonly_pk));
                CHECK(xonly_pubkey_parse(ctx, &xonly_pk, &rand33[1]) == 0);
                CHECK(memcmp_var(&xonly_pk, zeros64, sizeof(xonly_pk)) == 0);
            } else {
                CHECK(xonly_pubkey_parse(ctx, &xonly_pk, &rand33[1]) == 1);
            }
        }
        CHECK(ecount == 2);

        context_destroy(none);
        context_destroy(sign);
        context_destroy(verify);
        */
}

pub fn test_xonly_pubkey_comparison()  {
    
    todo!();
        /*
        unsigned char pk1_ser[32] = {
            0x58, 0x84, 0xb3, 0xa2, 0x4b, 0x97, 0x37, 0x88, 0x92, 0x38, 0xa6, 0x26, 0x62, 0x52, 0x35, 0x11,
            0xd0, 0x9a, 0xa1, 0x1b, 0x80, 0x0b, 0x5e, 0x93, 0x80, 0x26, 0x11, 0xef, 0x67, 0x4b, 0xd9, 0x23
        };
        const unsigned char pk2_ser[32] = {
            0xde, 0x36, 0x0e, 0x87, 0x59, 0x8f, 0x3c, 0x01, 0x36, 0x2a, 0x2a, 0xb8, 0xc6, 0xf4, 0x5e, 0x4d,
            0xb2, 0xc2, 0xd5, 0x03, 0xa7, 0xf9, 0xf1, 0x4f, 0xa8, 0xfa, 0x95, 0xa8, 0xe9, 0x69, 0x76, 0x1c
        };
        xonly_pubkey pk1;
        xonly_pubkey pk2;
        int ecount = 0;
        context *none = api_test_context(CONTEXT_NONE, &ecount);

        CHECK(xonly_pubkey_parse(none, &pk1, pk1_ser) == 1);
        CHECK(xonly_pubkey_parse(none, &pk2, pk2_ser) == 1);

        CHECK(xonly_pubkey_cmp(none, NULL, &pk2) < 0);
        CHECK(ecount == 1);
        CHECK(xonly_pubkey_cmp(none, &pk1, NULL) > 0);
        CHECK(ecount == 2);
        CHECK(xonly_pubkey_cmp(none, &pk1, &pk2) < 0);
        CHECK(xonly_pubkey_cmp(none, &pk2, &pk1) > 0);
        CHECK(xonly_pubkey_cmp(none, &pk1, &pk1) == 0);
        CHECK(xonly_pubkey_cmp(none, &pk2, &pk2) == 0);
        CHECK(ecount == 2);
        memset(&pk1, 0, sizeof(pk1)); /* illegal pubkey */
        CHECK(xonly_pubkey_cmp(none, &pk1, &pk2) < 0);
        CHECK(ecount == 3);
        CHECK(xonly_pubkey_cmp(none, &pk1, &pk1) == 0);
        CHECK(ecount == 5);
        CHECK(xonly_pubkey_cmp(none, &pk2, &pk1) > 0);
        CHECK(ecount == 6);

        context_destroy(none);
        */
}

pub fn test_xonly_pubkey_tweak()  {
    
    todo!();
        /*
            unsigned char zeros64[64] = { 0 };
        unsigned char overflows[32];
        unsigned char sk[32];
        pubkey internal_pk;
        xonly_pubkey internal_xonly_pk;
        pubkey output_pk;
        int pk_parity;
        unsigned char tweak[32];
        int i;

        int ecount;
        context *none = api_test_context(CONTEXT_NONE, &ecount);
        context *sign = api_test_context(CONTEXT_SIGN, &ecount);
        context *verify = api_test_context(CONTEXT_VERIFY, &ecount);

        memset(overflows, 0xff, sizeof(overflows));
        testrand256(tweak);
        testrand256(sk);
        CHECK(ec_pubkey_create(ctx, &internal_pk, sk) == 1);
        CHECK(xonly_pubkey_from_pubkey(none, &internal_xonly_pk, &pk_parity, &internal_pk) == 1);

        ecount = 0;
        CHECK(xonly_pubkey_tweak_add(none, &output_pk, &internal_xonly_pk, tweak) == 0);
        CHECK(ecount == 1);
        CHECK(xonly_pubkey_tweak_add(sign, &output_pk, &internal_xonly_pk, tweak) == 0);
        CHECK(ecount == 2);
        CHECK(xonly_pubkey_tweak_add(verify, &output_pk, &internal_xonly_pk, tweak) == 1);
        CHECK(xonly_pubkey_tweak_add(verify, NULL, &internal_xonly_pk, tweak) == 0);
        CHECK(ecount == 3);
        CHECK(xonly_pubkey_tweak_add(verify, &output_pk, NULL, tweak) == 0);
        CHECK(ecount == 4);
        /* NULL internal_xonly_pk zeroes the output_pk */
        CHECK(memcmp_var(&output_pk, zeros64, sizeof(output_pk)) == 0);
        CHECK(xonly_pubkey_tweak_add(verify, &output_pk, &internal_xonly_pk, NULL) == 0);
        CHECK(ecount == 5);
        /* NULL tweak zeroes the output_pk */
        CHECK(memcmp_var(&output_pk, zeros64, sizeof(output_pk)) == 0);

        /* Invalid tweak zeroes the output_pk */
        CHECK(xonly_pubkey_tweak_add(verify, &output_pk, &internal_xonly_pk, overflows) == 0);
        CHECK(memcmp_var(&output_pk, zeros64, sizeof(output_pk))  == 0);

        /* A zero tweak is fine */
        CHECK(xonly_pubkey_tweak_add(verify, &output_pk, &internal_xonly_pk, zeros64) == 1);

        /* Fails if the resulting key was infinity */
        for (i = 0; i < count; i++) {
            scalar scalar_tweak;
            /* Because sk may be negated before adding, we need to try with tweak =
             * sk as well as tweak = -sk. */
            scalar_set_b32(&scalar_tweak, sk, NULL);
            scalar_negate(&scalar_tweak, &scalar_tweak);
            scalar_get_b32(tweak, &scalar_tweak);
            CHECK((xonly_pubkey_tweak_add(verify, &output_pk, &internal_xonly_pk, sk) == 0)
                  || (xonly_pubkey_tweak_add(verify, &output_pk, &internal_xonly_pk, tweak) == 0));
            CHECK(memcmp_var(&output_pk, zeros64, sizeof(output_pk)) == 0);
        }

        /* Invalid pk with a valid tweak */
        memset(&internal_xonly_pk, 0, sizeof(internal_xonly_pk));
        testrand256(tweak);
        ecount = 0;
        CHECK(xonly_pubkey_tweak_add(verify, &output_pk, &internal_xonly_pk, tweak) == 0);
        CHECK(ecount == 1);
        CHECK(memcmp_var(&output_pk, zeros64, sizeof(output_pk))  == 0);

        context_destroy(none);
        context_destroy(sign);
        context_destroy(verify);
        */
}

pub fn test_xonly_pubkey_tweak_check()  {
    
    todo!();
        /*
        unsigned char zeros64[64] = { 0 };
        unsigned char overflows[32];
        unsigned char sk[32];
        pubkey internal_pk;
        xonly_pubkey internal_xonly_pk;
        pubkey output_pk;
        xonly_pubkey output_xonly_pk;
        unsigned char output_pk32[32];
        unsigned char buf32[32];
        int pk_parity;
        unsigned char tweak[32];

        int ecount;
        context *none = api_test_context(CONTEXT_NONE, &ecount);
        context *sign = api_test_context(CONTEXT_SIGN, &ecount);
        context *verify = api_test_context(CONTEXT_VERIFY, &ecount);

        memset(overflows, 0xff, sizeof(overflows));
        testrand256(tweak);
        testrand256(sk);
        CHECK(ec_pubkey_create(ctx, &internal_pk, sk) == 1);
        CHECK(xonly_pubkey_from_pubkey(none, &internal_xonly_pk, &pk_parity, &internal_pk) == 1);

        ecount = 0;
        CHECK(xonly_pubkey_tweak_add(verify, &output_pk, &internal_xonly_pk, tweak) == 1);
        CHECK(xonly_pubkey_from_pubkey(verify, &output_xonly_pk, &pk_parity, &output_pk) == 1);
        CHECK(xonly_pubkey_serialize(ctx, buf32, &output_xonly_pk) == 1);
        CHECK(xonly_pubkey_tweak_add_check(none, buf32, pk_parity, &internal_xonly_pk, tweak) == 0);
        CHECK(ecount == 1);
        CHECK(xonly_pubkey_tweak_add_check(sign, buf32, pk_parity, &internal_xonly_pk, tweak) == 0);
        CHECK(ecount == 2);
        CHECK(xonly_pubkey_tweak_add_check(verify, buf32, pk_parity, &internal_xonly_pk, tweak) == 1);
        CHECK(xonly_pubkey_tweak_add_check(verify, NULL, pk_parity, &internal_xonly_pk, tweak) == 0);
        CHECK(ecount == 3);
        /* invalid pk_parity value */
        CHECK(xonly_pubkey_tweak_add_check(verify, buf32, 2, &internal_xonly_pk, tweak) == 0);
        CHECK(ecount == 3);
        CHECK(xonly_pubkey_tweak_add_check(verify, buf32, pk_parity, NULL, tweak) == 0);
        CHECK(ecount == 4);
        CHECK(xonly_pubkey_tweak_add_check(verify, buf32, pk_parity, &internal_xonly_pk, NULL) == 0);
        CHECK(ecount == 5);

        memset(tweak, 1, sizeof(tweak));
        CHECK(xonly_pubkey_from_pubkey(ctx, &internal_xonly_pk, NULL, &internal_pk) == 1);
        CHECK(xonly_pubkey_tweak_add(ctx, &output_pk, &internal_xonly_pk, tweak) == 1);
        CHECK(xonly_pubkey_from_pubkey(ctx, &output_xonly_pk, &pk_parity, &output_pk) == 1);
        CHECK(xonly_pubkey_serialize(ctx, output_pk32, &output_xonly_pk) == 1);
        CHECK(xonly_pubkey_tweak_add_check(ctx, output_pk32, pk_parity, &internal_xonly_pk, tweak) == 1);

        /* Wrong pk_parity */
        CHECK(xonly_pubkey_tweak_add_check(ctx, output_pk32, !pk_parity, &internal_xonly_pk, tweak) == 0);
        /* Wrong public key */
        CHECK(xonly_pubkey_serialize(ctx, buf32, &internal_xonly_pk) == 1);
        CHECK(xonly_pubkey_tweak_add_check(ctx, buf32, pk_parity, &internal_xonly_pk, tweak) == 0);

        /* Overflowing tweak not allowed */
        CHECK(xonly_pubkey_tweak_add_check(ctx, output_pk32, pk_parity, &internal_xonly_pk, overflows) == 0);
        CHECK(xonly_pubkey_tweak_add(ctx, &output_pk, &internal_xonly_pk, overflows) == 0);
        CHECK(memcmp_var(&output_pk, zeros64, sizeof(output_pk)) == 0);
        CHECK(ecount == 5);

        context_destroy(none);
        context_destroy(sign);
        context_destroy(verify);
        */
}

/**
  | Starts with an initial pubkey and recursively
  | creates N_PUBKEYS - 1 additional pubkeys
  | by calling tweak_add. Then verifies
  | every tweak starting from the last pubkey.
  |
  */
pub fn test_xonly_pubkey_tweak_recursive()  {

    pub const N_PUBKEYS: usize = 32;
    
    todo!();
        /*
            unsigned char sk[32];
        pubkey pk[N_PUBKEYS];
        unsigned char pk_serialized[32];
        unsigned char tweak[N_PUBKEYS - 1][32];
        int i;

        testrand256(sk);
        CHECK(ec_pubkey_create(ctx, &pk[0], sk) == 1);
        /* Add tweaks */
        for (i = 0; i < N_PUBKEYS - 1; i++) {
            xonly_pubkey xonly_pk;
            memset(tweak[i], i + 1, sizeof(tweak[i]));
            CHECK(xonly_pubkey_from_pubkey(ctx, &xonly_pk, NULL, &pk[i]) == 1);
            CHECK(xonly_pubkey_tweak_add(ctx, &pk[i + 1], &xonly_pk, tweak[i]) == 1);
        }

        /* Verify tweaks */
        for (i = N_PUBKEYS - 1; i > 0; i--) {
            xonly_pubkey xonly_pk;
            int pk_parity;
            CHECK(xonly_pubkey_from_pubkey(ctx, &xonly_pk, &pk_parity, &pk[i]) == 1);
            CHECK(xonly_pubkey_serialize(ctx, pk_serialized, &xonly_pk) == 1);
            CHECK(xonly_pubkey_from_pubkey(ctx, &xonly_pk, NULL, &pk[i - 1]) == 1);
            CHECK(xonly_pubkey_tweak_add_check(ctx, pk_serialized, pk_parity, &xonly_pk, tweak[i - 1]) == 1);
        }
        */
}

pub fn test_keypair()  {
    
    todo!();
        /*
            unsigned char sk[32];
        unsigned char sk_tmp[32];
        unsigned char zeros96[96] = { 0 };
        unsigned char overflows[32];
        keypair keypair;
        pubkey pk, pk_tmp;
        xonly_pubkey xonly_pk, xonly_pk_tmp;
        int pk_parity, pk_parity_tmp;
        int ecount;
        context *none = api_test_context(CONTEXT_NONE, &ecount);
        context *sign = api_test_context(CONTEXT_SIGN, &ecount);
        context *verify = api_test_context(CONTEXT_VERIFY, &ecount);

        CHECK(sizeof(zeros96) == sizeof(keypair));
        memset(overflows, 0xFF, sizeof(overflows));

        /* Test keypair_create */
        ecount = 0;
        testrand256(sk);
        CHECK(keypair_create(none, &keypair, sk) == 0);
        CHECK(memcmp_var(zeros96, &keypair, sizeof(keypair)) == 0);
        CHECK(ecount == 1);
        CHECK(keypair_create(verify, &keypair, sk) == 0);
        CHECK(memcmp_var(zeros96, &keypair, sizeof(keypair)) == 0);
        CHECK(ecount == 2);
        CHECK(keypair_create(sign, &keypair, sk) == 1);
        CHECK(keypair_create(sign, NULL, sk) == 0);
        CHECK(ecount == 3);
        CHECK(keypair_create(sign, &keypair, NULL) == 0);
        CHECK(memcmp_var(zeros96, &keypair, sizeof(keypair)) == 0);
        CHECK(ecount == 4);

        /* Invalid secret key */
        CHECK(keypair_create(sign, &keypair, zeros96) == 0);
        CHECK(memcmp_var(zeros96, &keypair, sizeof(keypair)) == 0);
        CHECK(keypair_create(sign, &keypair, overflows) == 0);
        CHECK(memcmp_var(zeros96, &keypair, sizeof(keypair)) == 0);

        /* Test keypair_pub */
        ecount = 0;
        testrand256(sk);
        CHECK(keypair_create(ctx, &keypair, sk) == 1);
        CHECK(keypair_pub(none, &pk, &keypair) == 1);
        CHECK(keypair_pub(none, NULL, &keypair) == 0);
        CHECK(ecount == 1);
        CHECK(keypair_pub(none, &pk, NULL) == 0);
        CHECK(ecount == 2);
        CHECK(memcmp_var(zeros96, &pk, sizeof(pk)) == 0);

        /* Using an invalid keypair is fine for keypair_pub */
        memset(&keypair, 0, sizeof(keypair));
        CHECK(keypair_pub(none, &pk, &keypair) == 1);
        CHECK(memcmp_var(zeros96, &pk, sizeof(pk)) == 0);

        /* keypair holds the same pubkey as pubkey_create */
        CHECK(ec_pubkey_create(sign, &pk, sk) == 1);
        CHECK(keypair_create(sign, &keypair, sk) == 1);
        CHECK(keypair_pub(none, &pk_tmp, &keypair) == 1);
        CHECK(memcmp_var(&pk, &pk_tmp, sizeof(pk)) == 0);

        /** Test keypair_xonly_pub **/
        ecount = 0;
        testrand256(sk);
        CHECK(keypair_create(ctx, &keypair, sk) == 1);
        CHECK(keypair_xonly_pub(none, &xonly_pk, &pk_parity, &keypair) == 1);
        CHECK(keypair_xonly_pub(none, NULL, &pk_parity, &keypair) == 0);
        CHECK(ecount == 1);
        CHECK(keypair_xonly_pub(none, &xonly_pk, NULL, &keypair) == 1);
        CHECK(keypair_xonly_pub(none, &xonly_pk, &pk_parity, NULL) == 0);
        CHECK(ecount == 2);
        CHECK(memcmp_var(zeros96, &xonly_pk, sizeof(xonly_pk)) == 0);
        /* Using an invalid keypair will set the xonly_pk to 0 (first reset
         * xonly_pk). */
        CHECK(keypair_xonly_pub(none, &xonly_pk, &pk_parity, &keypair) == 1);
        memset(&keypair, 0, sizeof(keypair));
        CHECK(keypair_xonly_pub(none, &xonly_pk, &pk_parity, &keypair) == 0);
        CHECK(memcmp_var(zeros96, &xonly_pk, sizeof(xonly_pk)) == 0);
        CHECK(ecount == 3);

        /** keypair holds the same xonly pubkey as pubkey_create **/
        CHECK(ec_pubkey_create(sign, &pk, sk) == 1);
        CHECK(xonly_pubkey_from_pubkey(none, &xonly_pk, &pk_parity, &pk) == 1);
        CHECK(keypair_create(sign, &keypair, sk) == 1);
        CHECK(keypair_xonly_pub(none, &xonly_pk_tmp, &pk_parity_tmp, &keypair) == 1);
        CHECK(memcmp_var(&xonly_pk, &xonly_pk_tmp, sizeof(pk)) == 0);
        CHECK(pk_parity == pk_parity_tmp);

        /* Test keypair_seckey */
        ecount = 0;
        testrand256(sk);
        CHECK(keypair_create(ctx, &keypair, sk) == 1);
        CHECK(keypair_sec(none, sk_tmp, &keypair) == 1);
        CHECK(keypair_sec(none, NULL, &keypair) == 0);
        CHECK(ecount == 1);
        CHECK(keypair_sec(none, sk_tmp, NULL) == 0);
        CHECK(ecount == 2);
        CHECK(memcmp_var(zeros96, sk_tmp, sizeof(sk_tmp)) == 0);

        /* keypair returns the same seckey it got */
        CHECK(keypair_create(sign, &keypair, sk) == 1);
        CHECK(keypair_sec(none, sk_tmp, &keypair) == 1);
        CHECK(memcmp_var(sk, sk_tmp, sizeof(sk_tmp)) == 0);

        /* Using an invalid keypair is fine for keypair_seckey */
        memset(&keypair, 0, sizeof(keypair));
        CHECK(keypair_sec(none, sk_tmp, &keypair) == 1);
        CHECK(memcmp_var(zeros96, sk_tmp, sizeof(sk_tmp)) == 0);

        context_destroy(none);
        context_destroy(sign);
        context_destroy(verify);
        */
}

pub fn test_keypair_add()  {
    
    todo!();
        /*
        unsigned char sk[32];
        keypair keypair;
        unsigned char overflows[32];
        unsigned char zeros96[96] = { 0 };
        unsigned char tweak[32];
        int i;
        int ecount = 0;
        context *none = api_test_context(CONTEXT_NONE, &ecount);
        context *sign = api_test_context(CONTEXT_SIGN, &ecount);
        context *verify = api_test_context(CONTEXT_VERIFY, &ecount);

        CHECK(sizeof(zeros96) == sizeof(keypair));
        testrand256(sk);
        testrand256(tweak);
        memset(overflows, 0xFF, 32);
        CHECK(keypair_create(ctx, &keypair, sk) == 1);

        CHECK(keypair_xonly_tweak_add(none, &keypair, tweak) == 0);
        CHECK(ecount == 1);
        CHECK(keypair_xonly_tweak_add(sign, &keypair, tweak) == 0);
        CHECK(ecount == 2);
        CHECK(keypair_xonly_tweak_add(verify, &keypair, tweak) == 1);
        CHECK(keypair_xonly_tweak_add(verify, NULL, tweak) == 0);
        CHECK(ecount == 3);
        CHECK(keypair_xonly_tweak_add(verify, &keypair, NULL) == 0);
        CHECK(ecount == 4);
        /* This does not set the keypair to zeroes */
        CHECK(memcmp_var(&keypair, zeros96, sizeof(keypair)) != 0);

        /* Invalid tweak zeroes the keypair */
        CHECK(keypair_create(ctx, &keypair, sk) == 1);
        CHECK(keypair_xonly_tweak_add(ctx, &keypair, overflows) == 0);
        CHECK(memcmp_var(&keypair, zeros96, sizeof(keypair))  == 0);

        /* A zero tweak is fine */
        CHECK(keypair_create(ctx, &keypair, sk) == 1);
        CHECK(keypair_xonly_tweak_add(ctx, &keypair, zeros96) == 1);

        /* Fails if the resulting keypair was (sk=0, pk=infinity) */
        for (i = 0; i < count; i++) {
            scalar scalar_tweak;
            keypair keypair_tmp;
            testrand256(sk);
            CHECK(keypair_create(ctx, &keypair, sk) == 1);
            memcpy(&keypair_tmp, &keypair, sizeof(keypair));
            /* Because sk may be negated before adding, we need to try with tweak =
             * sk as well as tweak = -sk. */
            scalar_set_b32(&scalar_tweak, sk, NULL);
            scalar_negate(&scalar_tweak, &scalar_tweak);
            scalar_get_b32(tweak, &scalar_tweak);
            CHECK((keypair_xonly_tweak_add(ctx, &keypair, sk) == 0)
                  || (keypair_xonly_tweak_add(ctx, &keypair_tmp, tweak) == 0));
            CHECK(memcmp_var(&keypair, zeros96, sizeof(keypair)) == 0
                  || memcmp_var(&keypair_tmp, zeros96, sizeof(keypair_tmp)) == 0);
        }

        /* Invalid keypair with a valid tweak */
        memset(&keypair, 0, sizeof(keypair));
        testrand256(tweak);
        ecount = 0;
        CHECK(keypair_xonly_tweak_add(verify, &keypair, tweak) == 0);
        CHECK(ecount == 1);
        CHECK(memcmp_var(&keypair, zeros96, sizeof(keypair))  == 0);
        /* Only seckey part of keypair invalid */
        CHECK(keypair_create(ctx, &keypair, sk) == 1);
        memset(&keypair, 0, 32);
        CHECK(keypair_xonly_tweak_add(verify, &keypair, tweak) == 0);
        CHECK(ecount == 2);
        /* Only pubkey part of keypair invalid */
        CHECK(keypair_create(ctx, &keypair, sk) == 1);
        memset(&keypair.data[32], 0, 64);
        CHECK(keypair_xonly_tweak_add(verify, &keypair, tweak) == 0);
        CHECK(ecount == 3);

        /* Check that the keypair_tweak_add implementation is correct */
        CHECK(keypair_create(ctx, &keypair, sk) == 1);
        for (i = 0; i < count; i++) {
            xonly_pubkey internal_pk;
            xonly_pubkey output_pk;
            pubkey output_pk_xy;
            pubkey output_pk_expected;
            unsigned char pk32[32];
            unsigned char sk32[32];
            int pk_parity;

            testrand256(tweak);
            CHECK(keypair_xonly_pub(ctx, &internal_pk, NULL, &keypair) == 1);
            CHECK(keypair_xonly_tweak_add(ctx, &keypair, tweak) == 1);
            CHECK(keypair_xonly_pub(ctx, &output_pk, &pk_parity, &keypair) == 1);

            /* Check that it passes xonly_pubkey_tweak_add_check */
            CHECK(xonly_pubkey_serialize(ctx, pk32, &output_pk) == 1);
            CHECK(xonly_pubkey_tweak_add_check(ctx, pk32, pk_parity, &internal_pk, tweak) == 1);

            /* Check that the resulting pubkey matches xonly_pubkey_tweak_add */
            CHECK(keypair_pub(ctx, &output_pk_xy, &keypair) == 1);
            CHECK(xonly_pubkey_tweak_add(ctx, &output_pk_expected, &internal_pk, tweak) == 1);
            CHECK(memcmp_var(&output_pk_xy, &output_pk_expected, sizeof(output_pk_xy)) == 0);

            /* Check that the secret key in the keypair is tweaked correctly */
            CHECK(keypair_sec(none, sk32, &keypair) == 1);
            CHECK(ec_pubkey_create(ctx, &output_pk_expected, sk32) == 1);
            CHECK(memcmp_var(&output_pk_xy, &output_pk_expected, sizeof(output_pk_xy)) == 0);
        }
        context_destroy(none);
        context_destroy(sign);
        context_destroy(verify);
        */
}

pub fn run_extrakeys_tests()  {
    
    todo!();
        /*
            /* xonly key test cases */
        test_xonly_pubkey();
        test_xonly_pubkey_tweak();
        test_xonly_pubkey_tweak_check();
        test_xonly_pubkey_tweak_recursive();
        test_xonly_pubkey_comparison();

        /* keypair tests */
        test_keypair();
        test_keypair_add();
        */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/modules/extrakeys/tests_exhaustive_impl.h]

pub fn test_exhaustive_extrakeys(
        ctx:   *const Secp256k1Context,
        group: *const Ge)  {
    
    todo!();
        /*
            keypair keypair[EXHAUSTIVE_TEST_ORDER - 1];
        pubkey pubkey[EXHAUSTIVE_TEST_ORDER - 1];
        xonly_pubkey xonly_pubkey[EXHAUSTIVE_TEST_ORDER - 1];
        int parities[EXHAUSTIVE_TEST_ORDER - 1];
        unsigned char xonly_pubkey_bytes[EXHAUSTIVE_TEST_ORDER - 1][32];
        int i;

        for (i = 1; i < EXHAUSTIVE_TEST_ORDER; i++) {
            fe fe;
            scalar scalar_i;
            unsigned char buf[33];
            int parity;

            scalar_set_int(&scalar_i, i);
            scalar_get_b32(buf, &scalar_i);

            /* Construct pubkey and keypair. */
            CHECK(keypair_create(ctx, &keypair[i - 1], buf));
            CHECK(ec_pubkey_create(ctx, &pubkey[i - 1], buf));

            /* Construct serialized xonly_pubkey from keypair. */
            CHECK(keypair_xonly_pub(ctx, &xonly_pubkey[i - 1], &parities[i - 1], &keypair[i - 1]));
            CHECK(xonly_pubkey_serialize(ctx, xonly_pubkey_bytes[i - 1], &xonly_pubkey[i - 1]));

            /* Parse the xonly_pubkey back and verify it matches the previously serialized value. */
            CHECK(xonly_pubkey_parse(ctx, &xonly_pubkey[i - 1], xonly_pubkey_bytes[i - 1]));
            CHECK(xonly_pubkey_serialize(ctx, buf, &xonly_pubkey[i - 1]));
            CHECK(memcmp_var(xonly_pubkey_bytes[i - 1], buf, 32) == 0);

            /* Construct the xonly_pubkey from the pubkey, and verify it matches the same. */
            CHECK(xonly_pubkey_from_pubkey(ctx, &xonly_pubkey[i - 1], &parity, &pubkey[i - 1]));
            CHECK(parity == parities[i - 1]);
            CHECK(xonly_pubkey_serialize(ctx, buf, &xonly_pubkey[i - 1]));
            CHECK(memcmp_var(xonly_pubkey_bytes[i - 1], buf, 32) == 0);

            /* Compare the xonly_pubkey bytes against the precomputed group. */
            fe_set_b32(&fe, xonly_pubkey_bytes[i - 1]);
            CHECK(fe_equal_var(&fe, &group[i].x));

            /* Check the parity against the precomputed group. */
            fe = group[i].y;
            fe_normalize_var(&fe);
            CHECK(fe_is_odd(&fe) == parities[i - 1]);

            /* Verify that the higher half is identical to the lower half mirrored. */
            if (i > EXHAUSTIVE_TEST_ORDER / 2) {
                CHECK(memcmp_var(xonly_pubkey_bytes[i - 1], xonly_pubkey_bytes[EXHAUSTIVE_TEST_ORDER - i - 1], 32) == 0);
                CHECK(parities[i - 1] == 1 - parities[EXHAUSTIVE_TEST_ORDER - i - 1]);
            }
        }

        /* TODO: keypair/xonly_pubkey tweak tests */
        */
}
