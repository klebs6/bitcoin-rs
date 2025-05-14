// ---------------- [ File: bitcoin-secp256k1/src/secp256k1.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/include/secp256k1.h]

/**
  | All flags' lower 8 bits indicate what
  | they're for. Do not use directly.
  |
  */
pub const FLAGS_TYPE_MASK:        usize = ((1 << 8) - 1);
pub const FLAGS_TYPE_CONTEXT:     usize = (1 << 0);
pub const FLAGS_TYPE_COMPRESSION: usize = (1 << 1);

/**
  | The higher bits contain the actual data.
  | Do not use directly.
  |
  */
pub const FLAGS_BIT_CONTEXT_VERIFY:     usize = (1 << 8);
pub const FLAGS_BIT_CONTEXT_SIGN:       usize = (1 << 9);
pub const FLAGS_BIT_CONTEXT_DECLASSIFY: usize = (1 << 10);
pub const FLAGS_BIT_COMPRESSION:        usize = (1 << 8);

/**
  | Flags to pass to context_create, context_preallocated_size,
  | and context_preallocated_create.
  |
  */
pub const CONTEXT_VERIFY:     usize = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_VERIFY;
pub const CONTEXT_SIGN:       usize = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_SIGN;
pub const CONTEXT_DECLASSIFY: usize = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_DECLASSIFY;
pub const CONTEXT_NONE:       usize = FLAGS_TYPE_CONTEXT;

/**
  | Flag to pass to ec_pubkey_serialize.
  |
  */
pub const EC_COMPRESSED:   usize = (FLAGS_TYPE_COMPRESSION | FLAGS_BIT_COMPRESSION);
pub const EC_UNCOMPRESSED: usize = (FLAGS_TYPE_COMPRESSION);

/**
  | Prefix byte used to tag various encoded
  | curvepoints for specific purposes
  |
  */
pub const TAG_PUBKEY_EVEN:         usize = 0x02;
pub const TAG_PUBKEY_ODD:          usize = 0x03;
pub const TAG_PUBKEY_UNCOMPRESSED: usize = 0x04;
pub const TAG_PUBKEY_HYBRID_EVEN:  usize = 0x06;
pub const TAG_PUBKEY_HYBRID_ODD:   usize = 0x07;

/**
  | A simple secp256k1 context object with
  | no precomputed tables. These are useful
  | for type serialization/parsing functions
  | which require a context object to maintain
  | 
  | API consistency, but currently do not
  | require expensive precomputations
  | or dynamic allocations.
  |
  */
lazy_static!{
    /*
    extern const context *context_no_precomp;
    */
}

/**
  | Copy a secp256k1 context object (into
  | dynamically allocated memory).
  | 
  | This function uses malloc to allocate
  | memory. It is guaranteed that malloc
  | is called at most once for every call
  | of this function. If you need to avoid
  | dynamic memory allocation entirely,
  | see the functions in preallocated.h.
  | 
  | Returns: a newly created context object.
  | 
  | Args: ctx: an existing context to copy
  | (cannot be NULL)
  |
  */
lazy_static!{
    /*
    context* context_clone(
        const context* ctx
    ) ARG_NONNULL(1) WARN_UNUSED_RESULT;
    */
}

/** 
  | Set a callback function to be called when an
  | illegal argument is passed to an API call. It
  | will only trigger for violations that are
  | mentioned explicitly in the header.
  |
  |  The philosophy is that these shouldn't be
  |  dealt with through a specific return value, as
  |  calling code should not have branches to deal
  |  with the case that this code itself is broken.
  |
  |  On the other hand, during debug stage, one
  |  would want to be informed about such mistakes,
  |  and the default (crashing) may be inadvisable.
  |
  |  When this callback is triggered, the API
  |  function called is guaranteed not to cause
  |  a crash, though its return value and output
  |  arguments are undefined.
  |
  |  When this function has not been called (or
  |  called with fn==NULL), then the default
  |  handler will be used. The library provides
  |  a default handler which writes the message to
  |  stderr and calls abort. This default handler
  |  can be replaced at link time if the
  |  preprocessor macro
  |  USE_EXTERNAL_DEFAULT_CALLBACKS is defined,
  |  which is the case if the build has been
  |  configured with
  |  --enable-external-default-callbacks. Then the
  |  following two symbols must be provided to link
  |  against:
  |
  |   - c_void default_illegal_callback_fn(const
  |   char* message, c_void* data);
  |
  |   - c_void default_error_callback_fn(const char*
  |   message, c_void* data);
  |
  |  The library can call these default handlers
  |  even before a proper callback data pointer
  |  could have been set using
  |  context_set_illegal_callback or
  |  context_set_error_callback, e.g., when the
  |  creation of a context fails. In this case, the
  |  corresponding default handler will be called
  |  with the data pointer argument set to NULL.
  |
  |  Args: ctx:  an existing context object (cannot
  |              be NULL)
  |
  |  In:   fun:  a pointer to a function to call
  |              when an illegal argument is passed
  |              to the API, taking a message and
  |              an opaque pointer.  (NULL restores
  |              the default handler.)
  |
  |        data: the opaque pointer to pass to fun
  |              above.
  |
  |  See also context_set_error_callback.
  */
lazy_static!{
    /*
    c_void context_set_illegal_callback(
        context* ctx,
        c_void (*fun)(const char* message, c_void* data),
        const c_void* data
    ) ARG_NONNULL(1);
    */
}

/** 
 | Set a callback function to be called when an
 | internal consistency check fails. The default
 | is crashing.
 |
 | This can only trigger in case of a hardware
 | failure, miscompilation, memory corruption,
 | serious bug in the library, or other error
 | would can otherwise result in undefined
 | behaviour. It will not trigger due to mere
 | incorrect usage of the API (see
 | context_set_illegal_callback for that). 
 |
 | After this callback returns, anything may
 | happen, including crashing.
 |
 |  Args: ctx:  an existing context object (cannot
 |  be NULL)
 |
 |  In:   fun:  a pointer to a function to call
 |              when an internal error occurs,
 |              taking a message and an opaque
 |              pointer (NULL restores the default
 |              handler, see
 |              context_set_illegal_callback for
 |              details).
 |
 |        data: the opaque pointer to pass to fun
 |        above.
 |
 |  See also context_set_illegal_callback.
 */
lazy_static!{
    /*
    c_void context_set_error_callback(
        context* ctx,
        c_void (*fun)(const char* message, c_void* data),
        const c_void* data
    ) ARG_NONNULL(1);
    */
}

/** 
 | Create a secp256k1 scratch space object.
 |
 |  Returns: a newly created scratch space.
 |
 |  Args: ctx:  an existing context object (cannot
 |              be NULL)
 |
 |  In:   size: amount of memory to be available
 |              as scratch space. Some extra (<100
 |              bytes) will be allocated for extra
 |              accounting.
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT scratch_space* scratch_space_create(
        const context* ctx,
        size_t size
    ) ARG_NONNULL(1);
    */
}

/** 
 | Destroy a secp256k1 scratch space.
 |
 |  The pointer may not be used afterwards.
 |
 |  Args:       ctx: a secp256k1 context object.
 |
 |          scratch: space to destroy
 */
lazy_static!{
    /*
    c_void scratch_space_destroy(
        const context* ctx,
        scratch_space* scratch
    ) ARG_NONNULL(1);
    */
}

/** 
 | Parse a variable-length public key into the
 | pubkey object.
 |
 |  Returns: 1 if the public key was fully valid.
 |
 |           0 if the public key could not be
 |           parsed or is invalid.
 |
 |  Args: ctx:      a secp256k1 context object.
 |
 |  Out:  pubkey:   pointer to a pubkey object. If
 |                  1 is returned, it is set to
 |                  a parsed version of input. If
 |                  not, its value is undefined.
 |
 |  In:   input:    pointer to a serialized public
 |                  key
 |
 |        inputlen: length of the array pointed to
 |                  by input
 |
 |  This function supports parsing compressed (33
 |  bytes, header byte 0x02 or 0x03), uncompressed
 |  (65 bytes, header byte 0x04), or hybrid (65
 |  bytes, header byte 0x06 or 0x07) format public
 |  keys.
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_pubkey_parse(
        const context* ctx,
        pubkey* pubkey,
        const unsigned char *input,
        size_t inputlen
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Serialize a pubkey object into a serialized
 | byte sequence.
 |
 |  Returns: 1 always.
 |
 |  Args:   ctx:        a secp256k1 context object.
 |
 |  Out:    output:     a pointer to a 65-byte (if
 |                      compressed==0) or 33-byte
 |                      (if compressed==1) byte
 |                      array to place the
 |                      serialized key in.
 |
 |  In/Out: outputlen:  a pointer to an integer
 |                      which is initially set to
 |                      the size of output, and is
 |                      overwritten with the
 |                      written size.
 |
 |  In:     pubkey:     a pointer to a pubkey
 |                      containing an initialized
 |                      public key.
 |
 |          flags:      EC_COMPRESSED if
 |                      serialization should be in
 |                      compressed format,
 |                      otherwise EC_UNCOMPRESSED.
 */
lazy_static!{
    /*
    int ec_pubkey_serialize(
        const context* ctx,
        unsigned char *output,
        size_t *outputlen,
        const pubkey* pubkey,
        unsigned int flags
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3) ARG_NONNULL(4);
    */
}

/** 
 | Compare two public keys using lexicographic (of
 | compressed serialization) order
 |
 |  Returns: <0 if the first public key is less
 |              than the second
 |
 |           >0 if the first public key is greater
 |              than the second
 |
 |           0 if the two public keys are equal
 |
 |  Args: ctx:      a secp256k1 context object.
 |
 |  In:   pubkey1:  first public key to compare
 |
 |        pubkey2:  second public key to compare
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_pubkey_cmp(
        const context* ctx,
        const pubkey* pubkey1,
        const pubkey* pubkey2
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Parse an ECDSA signature in compact (64 bytes)
 | format.
 |
 |  Returns: 1 when the signature could be parsed,
 |  0 otherwise.
 |
 |  Args: ctx:      a secp256k1 context object
 |
 |  Out:  sig:      a pointer to a signature
 |                  object
 |
 |  In:   input64:  a pointer to the 64-byte array
 |                  to parse
 |
 |  The signature must consist of a 32-byte big
 |  endian R value, followed by a 32-byte big
 |  endian S value. If R or S fall outside of
 |  [0..order-1], the encoding is invalid. R and
 |  S with value 0 are allowed in the encoding.
 |
 |  After the call, sig will always be
 |  initialized. If parsing failed or R or S are
 |  zero, the resulting sig value is guaranteed to
 |  fail validation for any message and public
 |  key.
 */
lazy_static!{
    /*
    int ecdsa_signature_parse_compact(
        const context* ctx,
        ecdsa_signature* sig,
        const unsigned char *input64
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Parse a DER ECDSA signature.
 |
 |  Returns: 1 when the signature could be parsed,
 |           0 otherwise.
 |
 |  Args: ctx:      a secp256k1 context object
 |
 |  Out:  sig:      a pointer to a signature object
 |
 |  In:   input:    a pointer to the signature to
 |                  be parsed
 |
 |        inputlen: the length of the array
 |                  pointed to be input
 |
 |  This function will accept any valid DER
 |  encoded signature, even if the encoded numbers
 |  are out of range.
 |
 |  After the call, sig will always be
 |  initialized. If parsing failed or the encoded
 |  numbers are out of range, signature validation
 |  with it is guaranteed to fail for every
 |  message and public key.
 */
lazy_static!{
    /*
    int ecdsa_signature_parse_der(
        const context* ctx,
        ecdsa_signature* sig,
        const unsigned char *input,
        size_t inputlen
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Serialize an ECDSA signature in DER format.
 |
 |  Returns: 1 if enough space was available to
 |  serialize, 0 otherwise
 |
 |  Args:   ctx:       a secp256k1 context object
 |
 |  Out:    output:    a pointer to an array to
 |                     store the DER serialization
 |
 |  In/Out: outputlen: a pointer to a length
 |                     integer. Initially, this
 |                     integer should be set to
 |                     the length of output. After
 |                     the call it will be set to
 |                     the length of the
 |                     serialization (even if
 |                     0 was returned).
 |
 |  In:     sig:       a pointer to an initialized
 |                     signature object
 */
lazy_static!{
    /*
    int ecdsa_signature_serialize_der(
        const context* ctx,
        unsigned char *output,
        size_t *outputlen,
        const ecdsa_signature* sig
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3) ARG_NONNULL(4);
    */
}

/** 
 | Serialize an ECDSA signature in compact (64
 | byte) format.
 |
 |  Returns: 1
 |
 |  Args:   ctx:       a secp256k1 context object
 |
 |  Out:    output64:  a pointer to a 64-byte
 |                     array to store the compact
 |                     serialization
 |
 |  In:     sig:       a pointer to an initialized
 |                     signature object
 |
 |  See ecdsa_signature_parse_compact for details
 |  about the encoding.
 */
lazy_static!{
    /*
    int ecdsa_signature_serialize_compact(
        const context* ctx,
        unsigned char *output64,
        const ecdsa_signature* sig
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Verify an ECDSA signature.
 |
 |  Returns: 1: correct signature
 |
 |           0: incorrect or unparseable signature
 |
 |  Args:    ctx:       a secp256k1 context
 |                      object, initialized for
 |                      verification.
 |
 |  In:      sig:       the signature being
 |                      verified (cannot be NULL)
 |
 |           msghash32: the 32-byte message hash
 |                      being verified (cannot be
 |                      NULL).  The verifier must
 |                      make sure to apply
 |                      a cryptographic hash
 |                      function to the message by
 |                      itself and not accept an
 |                      msghash32 value
 |                      directly. Otherwise, it
 |                      would be easy to
 |                      create a "valid" signature
 |                      without knowledge of the
 |                      secret key. 
 |
 |                      See also
 |                      https://bitcoin.stackexchange.com/a/81116/35586
 |                      for more background on
 |                      this topic.
 |
 |           pubkey:    pointer to an initialized
 |                      public key to verify with (cannot be
 |                      NULL)
 |
 | To avoid accepting malleable signatures, only
 | ECDSA signatures in lower-S form are accepted.
 |
 | If you need to accept ECDSA signatures from
 | sources that do not obey this rule, apply
 | ecdsa_signature_normalize to the signature
 | prior to validation, but be aware that doing so
 | results in malleable signatures.
 |
 | For details, see the comments for that
 | function.
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ecdsa_verify(
        const context* ctx,
        const ecdsa_signature *sig,
        const unsigned char *msghash32,
        const pubkey *pubkey
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3) ARG_NONNULL(4);
    */
}

/** 
 | Convert a signature to a normalized lower-S
 | form.
 |
 |  Returns: 1 if sigin was not normalized, 0 if
 |  it already was.
 |
 |  Args: ctx:    a secp256k1 context object
 |
 |  Out:  sigout: a pointer to a signature to fill
 |                with the normalized form, or
 |                copy if the input was already
 |                normalized. (can be NULL if
 |                you're only interested in
 |                whether the input was already
 |                normalized).
 |
 |  In:   sigin:  a pointer to a signature to
 |                check/normalize (cannot be NULL,
 |                can be identical to sigout)
 |
 |  With ECDSA a third-party can forge a second
 |  distinct signature of the same message, given
 |  a single initial signature, but without
 |  knowing the key. This is done by negating the
 |  S value modulo the order of the curve,
 |  'flipping' the sign of the random point
 |  R which is not included in the signature.
 |
 |  Forgery of the same message isn't universally
 |  problematic, but in systems where message
 |  malleability or uniqueness of signatures is
 |  important this can cause issues. This forgery
 |  can be blocked by all verifiers forcing
 |  signers to use a normalized form.
 |
 |  The lower-S form reduces the size of
 |  signatures slightly on average when variable
 |  length encodings (such as DER) are used and is
 |  cheap to verify, making it a good
 |  choice. Security of always using lower-S is
 |  assured because anyone can trivially modify
 |  a signature after the fact to enforce this
 |  property anyway.
 |
 |  The lower S value is always between 0x1 and
 |  0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0,
 |  inclusive.
 |
 |  No other forms of ECDSA malleability are known
 |  and none seem likely, but there is no formal
 |  proof that ECDSA, even with this additional
 |  restriction, is free of other
 |  malleability. Commonly used serialization
 |  schemes will also accept various non-unique
 |  encodings, so care should be taken when this
 |  property is required for an application.
 |
 |  The ecdsa_sign function will by default create
 |  signatures in the lower-S form, and
 |  ecdsa_verify will not accept others. In case
 |  signatures come from a system that cannot
 |  enforce this property,
 |  ecdsa_signature_normalize must be called
 |  before verification.
 */
lazy_static!{
    /*
    int ecdsa_signature_normalize(
        const context* ctx,
        ecdsa_signature *sigout,
        const ecdsa_signature *sigin
    ) ARG_NONNULL(1) ARG_NONNULL(3);
    */
}

/**
  | An implementation of RFC6979 (using
  | HMAC-SHA256) as nonce generation function.
  | 
  | If a data pointer is passed, it is assumed
  | to be a pointer to 32 bytes of extra entropy.
  |
  */
lazy_static!{
    /*
    extern const nonce_function nonce_function_rfc6979;
    */
}

/**
  | A default safe nonce generation function
  | (currently equal to nonce_function_rfc6979).
  |
  */
lazy_static!{
    /*
    extern const nonce_function nonce_function_default;
    */
}

/** 
 | Create an ECDSA signature.
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
 |                      secret key (cannot be NULL)
 |
 |           noncefp:   pointer to a nonce
 |                      generation function. If
 |                      NULL,
 |                      nonce_function_default is
 |                      used
 |
 |           ndata:     pointer to arbitrary data
 |                      used by the nonce
 |                      generation function (can
 |                      be NULL)
 |
 | The created signature is always in lower-S
 | form. See ecdsa_signature_normalize for more
 | details.
 */
lazy_static!{
    /*
    int ecdsa_sign(
        const context* ctx,
        ecdsa_signature *sig,
        const unsigned char *msghash32,
        const unsigned char *seckey,
        nonce_function noncefp,
        const c_void *ndata
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3) ARG_NONNULL(4);
    */
}

/** 
 | Verify an ECDSA secret key.
 |
 | A secret key is valid if it is not 0 and less
 | than the secp256k1 curve order when
 | interpreted as an integer (most significant
 | byte first). The probability of choosing
 | a 32-byte string uniformly at random which is
 | an invalid secret key is negligible.
 |
 |  Returns: 1: secret key is valid
 |           0: secret key is invalid
 |
 |  Args:    ctx: pointer to a context object
 |                (cannot be NULL)
 |
 |  In:      seckey: pointer to a 32-byte secret
 |                   key (cannot be NULL)
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_seckey_verify(
        const context* ctx,
        const unsigned char *seckey
    ) ARG_NONNULL(1) ARG_NONNULL(2);
    */
}

/** 
 | Compute the public key for a secret key.
 |
 |  Returns: 1: secret was valid, public key
 |              stores
 |
 |           0: secret was invalid, try again
 |
 |  Args:   ctx:        pointer to a context
 |                      object, initialized for
 |                      signing (cannot be NULL)
 |
 |  Out:    pubkey:     pointer to the created
 |                      public key (cannot be
 |                      NULL)
 |
 |  In:     seckey:     pointer to a 32-byte
 |                      secret key (cannot be
 |                      NULL)
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_pubkey_create(
        const context* ctx,
        pubkey *pubkey,
        const unsigned char *seckey
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Negates a secret key in place.
 |
 |  Returns: 0 if the given secret key is invalid
 |           according to
 |           ec_seckey_verify. 1 otherwise
 |
 |  Args:   ctx:    pointer to a context object
 |
 |  In/Out: seckey: pointer to the 32-byte secret
 |                  key to be negated. If the
 |                  secret key is invalid
 |                  according to ec_seckey_verify,
 |                  this function returns 0 and
 |                  seckey will be set to some
 |                  unspecified value. (cannot be
 |                  NULL)
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_seckey_negate(
        const context* ctx,
        unsigned char *seckey
    ) ARG_NONNULL(1) ARG_NONNULL(2);
    */
}

/**
  | Same as ec_seckey_negate, but DEPRECATED.
  | Will be removed in future versions.
  |
  */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_privkey_negate(
        const context* ctx,
        unsigned char *seckey
    ) ARG_NONNULL(1) ARG_NONNULL(2);
    */
}

/** 
 | Negates a public key in place.
 |
 |  Returns: 1 always
 |
 |  Args:   ctx:        pointer to a context
 |                      object
 |
 |  In/Out: pubkey:     pointer to the public key
 |                      to be negated (cannot be
 |                      NULL)
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_pubkey_negate(
        const context* ctx,
        pubkey *pubkey
    ) ARG_NONNULL(1) ARG_NONNULL(2);
    */
}

/** 
 | Tweak a secret key by adding tweak to it.
 |
 |  Returns: 0 if the arguments are invalid or the
 |           resulting secret key would be invalid
 |           (only when the tweak is the negation
 |           of the secret key). 1 otherwise.
 |
 |  Args:    ctx:   pointer to a context object
 |  (cannot be NULL).
 |
 |  In/Out: seckey: pointer to a 32-byte secret
 |                  key. If the secret key is
 |                  invalid according to
 |                  ec_seckey_verify, this
 |                  function returns 0. seckey
 |                  will be set to some
 |                  unspecified value if this
 |                  function returns 0. (cannot be
 |                  NULL)
 |
 |  In:    tweak32: pointer to a 32-byte tweak. If
 |                  the tweak is invalid according
 |                  to ec_seckey_verify, this
 |                  function returns 0. For
 |                  uniformly random 32-byte
 |                  arrays the chance of being
 |                  invalid is negligible (around
 |                  1 in 2^128) (cannot be NULL).
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_seckey_tweak_add(
        const context* ctx,
        unsigned char *seckey,
        const unsigned char *tweak32
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/**
  | Same as ec_seckey_tweak_add, but DEPRECATED.
  | Will be removed in future versions.
  |
  */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_privkey_tweak_add(
        const context* ctx,
        unsigned char *seckey,
        const unsigned char *tweak32
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Tweak a public key by adding tweak times the
 | generator to it.
 |
 |  Returns: 0 if the arguments are invalid or the
 |           resulting public key would be invalid
 |           (only when the tweak is the negation
 |           of the corresponding secret
 |           key). 1 otherwise.
 |
 |  Args:    ctx:   pointer to a context object
 |                  initialized for validation
 |                  (cannot be NULL).
 |
 |  In/Out: pubkey: pointer to a public key
 |                  object. pubkey will be set to
 |                  an invalid value if this
 |                  function returns 0 (cannot be
 |                  NULL).
 |
 |  In:    tweak32: pointer to a 32-byte tweak. If
 |                  the tweak is invalid according
 |                  to ec_seckey_verify, this
 |                  function returns 0. For
 |                  uniformly random 32-byte
 |                  arrays the chance of being
 |                  invalid is negligible (around
 |                  1 in 2^128) (cannot be NULL).
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_pubkey_tweak_add(
        const context* ctx,
        pubkey *pubkey,
        const unsigned char *tweak32
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Tweak a secret key by multiplying it by
 | a tweak.
 |
 |  Returns: 0 if the arguments are
 |           invalid. 1 otherwise.
 |
 |  Args:   ctx:    pointer to a context object
 |                  (cannot be NULL).
 |
 |  In/Out: seckey: pointer to a 32-byte secret
 |                  key. If the secret key is
 |                  invalid according to
 |                  ec_seckey_verify, this
 |                  function returns 0. seckey
 |                  will be set to some
 |                  unspecified value if this
 |                  function returns 0. (cannot be
 |                  NULL)
 |
 |  In:    tweak32: pointer to a 32-byte tweak. If
 |                  the tweak is invalid according
 |                  to ec_seckey_verify, this
 |                  function returns 0. For
 |                  uniformly random 32-byte
 |                  arrays the chance of being
 |                  invalid is negligible (around
 |                  1 in 2^128) (cannot be NULL).
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_seckey_tweak_mul(
        const context* ctx,
        unsigned char *seckey,
        const unsigned char *tweak32
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/**
  | Same as ec_seckey_tweak_mul, but DEPRECATED.
  | Will be removed in future versions.
  |
  */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_privkey_tweak_mul(
        const context* ctx,
        unsigned char *seckey,
        const unsigned char *tweak32
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Tweak a public key by multiplying it by a tweak
 | value.
 |
 |  Returns: 0 if the arguments are
 |           invalid. 1 otherwise.
 |
 |  Args:    ctx:   pointer to a context object
 |                  initialized for validation
 |                  (cannot be NULL).
 |
 |  In/Out: pubkey: pointer to a public key
 |                  object. pubkey will be set to
 |                  an invalid value if this
 |                  function returns 0 (cannot be
 |                  NULL).
 |
 |  In:    tweak32: pointer to a 32-byte tweak. If
 |                  the tweak is invalid according
 |                  to ec_seckey_verify, this
 |                  function returns 0. For
 |                  uniformly random 32-byte
 |                  arrays the chance of being
 |                  invalid is negligible (around
 |                  1 in 2^128) (cannot be NULL).
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_pubkey_tweak_mul(
        const context* ctx,
        pubkey *pubkey,
        const unsigned char *tweak32
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Updates the context randomization to protect
 | against side-channel leakage.
 |
 |  Returns: 1: randomization successfully updated
 |              or nothing to randomize
 |
 |           0: error
 |
 |  Args:    ctx:       pointer to a context
 |                      object (cannot be NULL)
 |
 |  In:      seed32:    pointer to a 32-byte
 |                      random seed (NULL resets
 |                      to initial state)
 |
 | While secp256k1 code is written to be
 | constant-time no matter what secret values are,
 | it's possible that a future compiler may output
 | code which isn't, and also that the CPU may not
 | emit the same radio frequencies or draw the
 | same amount power for all values.
 |
 | This function provides a seed which is combined
 | into the blinding value: that blinding value is
 | added before each multiplication (and removed
 | afterwards) so that it does not affect function
 | results, but shields against attacks which rely
 | on any input-dependent behaviour.
 |
 | This function has currently an effect only on
 | contexts initialized for signing because
 | randomization is currently used only for
 | signing. However, this is not guaranteed and
 | may change in the future. It is safe to call
 | this function on contexts not initialized for
 | signing; then it will have no effect and return
 | 1.
 |
 | You should call this after context_create or
 | context_clone (and context_preallocated_create
 | or context_clone, resp.), and you may call this
 | repeatedly afterwards.
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int context_randomize(
        context* ctx,
        const unsigned char *seed32
    ) ARG_NONNULL(1);
    */
}

/** 
 | Add a number of public keys together.
 |
 |  Returns: 1: the sum of the public keys is valid.
 |           0: the sum of the public keys is not valid.
 |
 |  Args:   ctx:        pointer to a context object
 |
 |  Out:    out:        pointer to a public key
 |                      object for placing the
 |                      resulting public key
 |                      (cannot be NULL)
 |
 |  In:     ins:        pointer to array of
 |                      pointers to public keys
 |                      (cannot be NULL)
 |
 |          n:          the number of public keys
 |                      to add together (must be
 |                      at least 1)
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_pubkey_combine(
        const context* ctx,
        pubkey *out,
        const pubkey * const * ins,
        size_t n
    ) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

/** 
 | Compute a tagged hash as defined in BIP-340.
 |
 |  This is useful for creating a message hash and
 |  achieving domain separation through an
 |  application-specific tag. This function
 |  returns
 |  SHA256(SHA256(tag)||SHA256(tag)||msg). Therefore,
 |  tagged hash implementations optimized for
 |  a specific tag can precompute the SHA256 state
 |  after hashing the tag hashes.
 |
 |  Returns 0 if the arguments are invalid and
 |          1 otherwise.
 |
 |  Args:    ctx: pointer to a context object
 |
 |  Out:  hash32: pointer to a 32-byte array to
 |                store the resulting hash
 |
 |  In:      tag: pointer to an array containing
 |                the tag
 |
 |        taglen: length of the tag array
 |
 |           msg: pointer to an array containing
 |                the message
 |
 |        msglen: length of the message array
 */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int tagged_sha256(
        const context* ctx,
        unsigned char *hash32,
        const unsigned char *tag,
        size_t taglen,
        const unsigned char *msg,
        size_t msglen
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3) ARG_NONNULL(5);
    */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/secp256k1.c]

macro_rules! arg_check {
    ($cond:ident) => {
        /*
                do { 
            if (EXPECT(!(cond), 0)) { 
                callback_call(&ctx->illegal_callback, #cond); 
                return 0; 
            } 
        } while(0)
        */
    }
}

macro_rules! arg_check_no_return {
    ($cond:ident) => {
        /*
                do { 
            if (EXPECT(!(cond), 0)) { 
                callback_call(&ctx->illegal_callback, #cond); 
            } 
        } while(0)
        */
    }
}

pub fn scratch_space_create(
        ctx:      *const Secp256k1Context,
        max_size: usize) -> *mut Scratch {
    
    todo!();
        /*
            VERIFY_CHECK(ctx != NULL);
        return scratch_create(&ctx->error_callback, max_size);
        */
}

pub fn scratch_space_destroy(
        ctx:     *const Secp256k1Context,
        scratch: *mut Scratch)  {
    
    todo!();
        /*
            VERIFY_CHECK(ctx != NULL);
        scratch_destroy(&ctx->error_callback, scratch);
        */
}

/**
  | Mark memory as no-longer-secret for
  | the purpose of analysing constant-time
  | behaviour of the software. This is setup
  | for use with valgrind but could be substituted
  | with the appropriate instrumentation
  | for other analysis tools.
  |
  */
#[inline] pub fn declassify(
        ctx: *const Secp256k1Context,
        p:   *const c_void,
        len: usize)  {
    
    todo!();
        /*
            #if defined(VALGRIND)
        if (EXPECT(ctx->declassify,0)) VALGRIND_MAKE_MEM_DEFINED(p, len);
    #else
        (c_void)ctx;
        (c_void)p;
        (c_void)len;
    #endif
        */
}

pub fn pubkey_load(
        ctx:    *const Secp256k1Context,
        ge:     *mut Ge,
        pubkey: *const PubKey) -> i32 {
    
    todo!();
        /*
            if (sizeof(ge_storage) == 64) {
            /* When the ge_storage type is exactly 64 byte, use its
             * representation inside pubkey, as conversion is very fast.
             * Note that pubkey_save must use the same representation. */
            ge_storage s;
            memcpy(&s, &pubkey->data[0], sizeof(s));
            ge_from_storage(ge, &s);
        } else {
            /* Otherwise, fall back to 32-byte big endian for X and Y. */
            fe x, y;
            fe_set_b32(&x, pubkey->data);
            fe_set_b32(&y, pubkey->data + 32);
            ge_set_xy(ge, &x, &y);
        }
        ARG_CHECK(!fe_is_zero(&ge->x));
        return 1;
        */
}

pub fn pubkey_save(
        pubkey: *mut PubKey,
        ge:     *mut Ge)  {
    
    todo!();
        /*
            if (sizeof(ge_storage) == 64) {
            ge_storage s;
            ge_to_storage(&s, ge);
            memcpy(&pubkey->data[0], &s, sizeof(s));
        } else {
            VERIFY_CHECK(!ge_is_infinity(ge));
            fe_normalize_var(&ge->x);
            fe_normalize_var(&ge->y);
            fe_get_b32(pubkey->data, &ge->x);
            fe_get_b32(pubkey->data + 32, &ge->y);
        }
        */
}

pub fn ec_pubkey_parse(
        ctx:      *const Secp256k1Context,
        pubkey:   *mut PubKey,
        input:    *const u8,
        inputlen: usize) -> i32 {
    
    todo!();
        /*
            ge Q;

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(pubkey != NULL);
        memset(pubkey, 0, sizeof(*pubkey));
        ARG_CHECK(input != NULL);
        if (!eckey_pubkey_parse(&Q, input, inputlen)) {
            return 0;
        }
        if (!ge_is_in_correct_subgroup(&Q)) {
            return 0;
        }
        pubkey_save(pubkey, &Q);
        ge_clear(&Q);
        return 1;
        */
}

pub fn ec_pubkey_serialize(
        ctx:       *const Secp256k1Context,
        output:    *mut u8,
        outputlen: *mut usize,
        pubkey:    *const PubKey,
        flags:     u32) -> i32 {
    
    todo!();
        /*
            ge Q;
        size_t len;
        int ret = 0;

        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(outputlen != NULL);
        ARG_CHECK(*outputlen >= ((flags & FLAGS_BIT_COMPRESSION) ? 33u : 65u));
        len = *outputlen;
        *outputlen = 0;
        ARG_CHECK(output != NULL);
        memset(output, 0, len);
        ARG_CHECK(pubkey != NULL);
        ARG_CHECK((flags & FLAGS_TYPE_MASK) == FLAGS_TYPE_COMPRESSION);
        if (pubkey_load(ctx, &Q, pubkey)) {
            ret = eckey_pubkey_serialize(&Q, output, &len, flags & FLAGS_BIT_COMPRESSION);
            if (ret) {
                *outputlen = len;
            }
        }
        return ret;
        */
}

pub fn ec_pubkey_cmp(
        ctx:     *const Secp256k1Context,
        pubkey0: *const PubKey,
        pubkey1: *const PubKey) -> i32 {
    
    todo!();
        /*
            unsigned char out[2][33];
        const pubkey* pk[2];
        int i;

        VERIFY_CHECK(ctx != NULL);
        pk[0] = pubkey0; pk[1] = pubkey1;
        for (i = 0; i < 2; i++) {
            size_t out_size = sizeof(out[i]);
            /* If the public key is NULL or invalid, ec_pubkey_serialize will call
             * the illegal_callback and return 0. In that case we will serialize the
             * key as all zeros which is less than any valid public key. This
             * results in consistent comparisons even if NULL or invalid pubkeys are
             * involved and prevents edge cases such as sorting algorithms that use
             * this function and do not terminate as a result. */
            if (!ec_pubkey_serialize(ctx, out[i], &out_size, pk[i], EC_COMPRESSED)) {
                /* Note that ec_pubkey_serialize should already set the output to
                 * zero in that case, but it's not guaranteed by the API, we can't
                 * test it and writing a VERIFY_CHECK is more complex than
                 * explicitly memsetting (again). */
                memset(out[i], 0, sizeof(out[i]));
            }
        }
        return memcmp_var(out[0], out[1], sizeof(out[0]));
        */
}

#[inline] pub fn buffer_append(
        buf:    *mut u8,
        offset: *mut u32,
        data:   *const c_void,
        len:    u32)  {
    
    todo!();
        /*
            memcpy(buf + *offset, data, len);
        *offset += len;
        */
}

pub fn nonce_function_rfc6979(
        nonce32: *mut u8,
        msg32:   *const u8,
        key32:   *const u8,
        algo16:  *const u8,
        data:    *mut c_void,
        counter: u32) -> i32 {
    
    todo!();
        /*
            unsigned char keydata[112];
       unsigned int offset = 0;
       rfc6979_hmac_sha256 rng;
       unsigned int i;
       /* We feed a byte array to the PRNG as input, consisting of:
        * - the private key (32 bytes) and message (32 bytes), see RFC 6979 3.2d.
        * - optionally 32 extra bytes of data, see RFC 6979 3.6 Additional Data.
        * - optionally 16 extra bytes with the algorithm name.
        * Because the arguments have distinct fixed lengths it is not possible for
        *  different argument mixtures to emulate each other and result in the same
        *  nonces.
        */
       buffer_append(keydata, &offset, key32, 32);
       buffer_append(keydata, &offset, msg32, 32);
       if (data != NULL) {
           buffer_append(keydata, &offset, data, 32);
       }
       if (algo16 != NULL) {
           buffer_append(keydata, &offset, algo16, 16);
       }
       rfc6979_hmac_sha256_initialize(&rng, keydata, offset);
       memset(keydata, 0, sizeof(keydata));
       for (i = 0; i <= counter; i++) {
           rfc6979_hmac_sha256_generate(&rng, nonce32, 32);
       }
       rfc6979_hmac_sha256_finalize(&rng);
       return 1;
        */
}

pub const NONCE_FUNCTION_RFC6979: NonceFunction = nonce_function_rfc6979;
pub const NONCE_FUNCTION_DEFAULT: NonceFunction = nonce_function_rfc6979;

pub fn ecdsa_sign_inner(
        ctx:       *const Secp256k1Context,
        r:         *mut Scalar,
        s:         *mut Scalar,
        recid:     *mut i32,
        msg32:     *const u8,
        seckey:    *const u8,
        noncefp:   NonceFunction,
        noncedata: *const c_void) -> i32 {
    
    todo!();
        /*
            scalar sec, non, msg;
        int ret = 0;
        int is_sec_valid;
        unsigned char nonce32[32];
        unsigned int count = 0;
        /* Default initialization here is important so we won't pass uninit values to the cmov in the end */
        *r = scalar_zero;
        *s = scalar_zero;
        if (recid) {
            *recid = 0;
        }
        if (noncefp == NULL) {
            noncefp = nonce_function_default;
        }

        /* Fail if the secret key is invalid. */
        is_sec_valid = scalar_set_b32_seckey(&sec, seckey);
        scalar_cmov(&sec, &scalar_one, !is_sec_valid);
        scalar_set_b32(&msg, msg32, NULL);
        while (1) {
            int is_nonce_valid;
            ret = !!noncefp(nonce32, msg32, seckey, NULL, (c_void*)noncedata, count);
            if (!ret) {
                break;
            }
            is_nonce_valid = scalar_set_b32_seckey(&non, nonce32);
            /* The nonce is still secret here, but it being invalid is is less likely than 1:2^255. */
            declassify(ctx, &is_nonce_valid, sizeof(is_nonce_valid));
            if (is_nonce_valid) {
                ret = ecdsa_sig_sign(&ctx->ecmult_gen_ctx, r, s, &sec, &msg, &non, recid);
                /* The final signature is no longer a secret, nor is the fact that we were successful or not. */
                declassify(ctx, &ret, sizeof(ret));
                if (ret) {
                    break;
                }
            }
            count++;
        }
        /* We don't want to declassify is_sec_valid and therefore the range of
         * seckey. As a result is_sec_valid is included in ret only after ret was
         * used as a branching variable. */
        ret &= is_sec_valid;
        memset(nonce32, 0, 32);
        scalar_clear(&msg);
        scalar_clear(&non);
        scalar_clear(&sec);
        scalar_cmov(r, &scalar_zero, !ret);
        scalar_cmov(s, &scalar_zero, !ret);
        if (recid) {
            const int zero = 0;
            int_cmov(recid, &zero, !ret);
        }
        return ret;
        */
}

pub fn ec_seckey_verify(
        ctx:    *const Secp256k1Context,
        seckey: *const u8) -> i32 {
    
    todo!();
        /*
            scalar sec;
        int ret;
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(seckey != NULL);

        ret = scalar_set_b32_seckey(&sec, seckey);
        scalar_clear(&sec);
        return ret;
        */
}

pub fn ec_pubkey_create_helper(
        ecmult_gen_ctx: *const EcMultGenContext,
        seckey_scalar:  *mut Scalar,
        p:              *mut Ge,
        seckey:         *const u8) -> i32 {
    
    todo!();
        /*
            gej pj;
        int ret;

        ret = scalar_set_b32_seckey(seckey_scalar, seckey);
        scalar_cmov(seckey_scalar, &scalar_one, !ret);

        ecmult_gen(ecmult_gen_ctx, &pj, seckey_scalar);
        ge_set_gej(p, &pj);
        return ret;
        */
}

pub fn ec_pubkey_create(
        ctx:    *const Secp256k1Context,
        pubkey: *mut PubKey,
        seckey: *const u8) -> i32 {
    
    todo!();
        /*
            ge p;
        scalar seckey_scalar;
        int ret = 0;
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(pubkey != NULL);
        memset(pubkey, 0, sizeof(*pubkey));
        ARG_CHECK(ecmult_gen_context_is_built(&ctx->ecmult_gen_ctx));
        ARG_CHECK(seckey != NULL);

        ret = ec_pubkey_create_helper(&ctx->ecmult_gen_ctx, &seckey_scalar, &p, seckey);
        pubkey_save(pubkey, &p);
        memczero(pubkey, sizeof(*pubkey), !ret);

        scalar_clear(&seckey_scalar);
        return ret;
        */
}

pub fn ec_seckey_negate(
        ctx:    *const Secp256k1Context,
        seckey: *mut u8) -> i32 {
    
    todo!();
        /*
            scalar sec;
        int ret = 0;
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(seckey != NULL);

        ret = scalar_set_b32_seckey(&sec, seckey);
        scalar_cmov(&sec, &scalar_zero, !ret);
        scalar_negate(&sec, &sec);
        scalar_get_b32(seckey, &sec);

        scalar_clear(&sec);
        return ret;
        */
}

pub fn ec_privkey_negate(
        ctx:    *const Secp256k1Context,
        seckey: *mut u8) -> i32 {
    
    todo!();
        /*
            return ec_seckey_negate(ctx, seckey);
        */
}

pub fn ec_pubkey_negate(
        ctx:    *const Secp256k1Context,
        pubkey: *mut PubKey) -> i32 {
    
    todo!();
        /*
            int ret = 0;
        ge p;
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(pubkey != NULL);

        ret = pubkey_load(ctx, &p, pubkey);
        memset(pubkey, 0, sizeof(*pubkey));
        if (ret) {
            ge_neg(&p, &p);
            pubkey_save(pubkey, &p);
        }
        return ret;
        */
}

pub fn ec_seckey_tweak_add_helper(
        sec:     *mut Scalar,
        tweak32: *const u8) -> i32 {
    
    todo!();
        /*
            scalar term;
        int overflow = 0;
        int ret = 0;

        scalar_set_b32(&term, tweak32, &overflow);
        ret = (!overflow) & eckey_privkey_tweak_add(sec, &term);
        scalar_clear(&term);
        return ret;
        */
}

pub fn ec_seckey_tweak_add(
        ctx:     *const Secp256k1Context,
        seckey:  *mut u8,
        tweak32: *const u8) -> i32 {
    
    todo!();
        /*
            scalar sec;
        int ret = 0;
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(seckey != NULL);
        ARG_CHECK(tweak32 != NULL);

        ret = scalar_set_b32_seckey(&sec, seckey);
        ret &= ec_seckey_tweak_add_helper(&sec, tweak32);
        scalar_cmov(&sec, &scalar_zero, !ret);
        scalar_get_b32(seckey, &sec);

        scalar_clear(&sec);
        return ret;
        */
}

pub fn ec_privkey_tweak_add(
        ctx:     *const Secp256k1Context,
        seckey:  *mut u8,
        tweak32: *const u8) -> i32 {
    
    todo!();
        /*
            return ec_seckey_tweak_add(ctx, seckey, tweak32);
        */
}

pub fn ec_pubkey_tweak_add_helper(
        ecmult_ctx: *const EcMultContext,
        p:          *mut Ge,
        tweak32:    *const u8) -> i32 {
    
    todo!();
        /*
            scalar term;
        int overflow = 0;
        scalar_set_b32(&term, tweak32, &overflow);
        return !overflow && eckey_pubkey_tweak_add(ecmult_ctx, p, &term);
        */
}

pub fn ec_pubkey_tweak_add(
        ctx:     *const Secp256k1Context,
        pubkey:  *mut PubKey,
        tweak32: *const u8) -> i32 {
    
    todo!();
        /*
            ge p;
        int ret = 0;
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(ecmult_context_is_built(&ctx->ecmult_ctx));
        ARG_CHECK(pubkey != NULL);
        ARG_CHECK(tweak32 != NULL);

        ret = pubkey_load(ctx, &p, pubkey);
        memset(pubkey, 0, sizeof(*pubkey));
        ret = ret && ec_pubkey_tweak_add_helper(&ctx->ecmult_ctx, &p, tweak32);
        if (ret) {
            pubkey_save(pubkey, &p);
        }

        return ret;
        */
}

pub fn ec_seckey_tweak_mul(
        ctx:     *const Secp256k1Context,
        seckey:  *mut u8,
        tweak32: *const u8) -> i32 {
    
    todo!();
        /*
            scalar factor;
        scalar sec;
        int ret = 0;
        int overflow = 0;
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(seckey != NULL);
        ARG_CHECK(tweak32 != NULL);

        scalar_set_b32(&factor, tweak32, &overflow);
        ret = scalar_set_b32_seckey(&sec, seckey);
        ret &= (!overflow) & eckey_privkey_tweak_mul(&sec, &factor);
        scalar_cmov(&sec, &scalar_zero, !ret);
        scalar_get_b32(seckey, &sec);

        scalar_clear(&sec);
        scalar_clear(&factor);
        return ret;
        */
}

pub fn ec_privkey_tweak_mul(
        ctx:     *const Secp256k1Context,
        seckey:  *mut u8,
        tweak32: *const u8) -> i32 {
    
    todo!();
        /*
            return ec_seckey_tweak_mul(ctx, seckey, tweak32);
        */
}

pub fn ec_pubkey_tweak_mul(
        ctx:     *const Secp256k1Context,
        pubkey:  *mut PubKey,
        tweak32: *const u8) -> i32 {
    
    todo!();
        /*
            ge p;
        scalar factor;
        int ret = 0;
        int overflow = 0;
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(ecmult_context_is_built(&ctx->ecmult_ctx));
        ARG_CHECK(pubkey != NULL);
        ARG_CHECK(tweak32 != NULL);

        scalar_set_b32(&factor, tweak32, &overflow);
        ret = !overflow && pubkey_load(ctx, &p, pubkey);
        memset(pubkey, 0, sizeof(*pubkey));
        if (ret) {
            if (eckey_pubkey_tweak_mul(&ctx->ecmult_ctx, &p, &factor)) {
                pubkey_save(pubkey, &p);
            } else {
                ret = 0;
            }
        }

        return ret;
        */
}

pub fn ec_pubkey_combine(
        ctx:       *const Secp256k1Context,
        pubnonce:  *mut PubKey,
        pubnonces: *const *const PubKey,
        n:         usize) -> i32 {
    
    todo!();
        /*
            size_t i;
        gej Qj;
        ge Q;

        ARG_CHECK(pubnonce != NULL);
        memset(pubnonce, 0, sizeof(*pubnonce));
        ARG_CHECK(n >= 1);
        ARG_CHECK(pubnonces != NULL);

        gej_set_infinity(&Qj);

        for (i = 0; i < n; i++) {
            pubkey_load(ctx, &Q, pubnonces[i]);
            gej_add_ge(&Qj, &Qj, &Q);
        }
        if (gej_is_infinity(&Qj)) {
            return 0;
        }
        ge_set_gej(&Q, &Qj);
        pubkey_save(pubnonce, &Q);
        return 1;
        */
}

pub fn tagged_sha256(
        ctx:    *const Secp256k1Context,
        hash32: *mut u8,
        tag:    *const u8,
        taglen: usize,
        msg:    *const u8,
        msglen: usize) -> i32 {
    
    todo!();
        /*
            sha256 sha;
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(hash32 != NULL);
        ARG_CHECK(tag != NULL);
        ARG_CHECK(msg != NULL);

        sha256_initialize_tagged(&sha, tag, taglen);
        sha256_write(&sha, msg, msglen);
        sha256_finalize(&sha, hash32);
        return 1;
        */
}

lazy_static!{
    /*
    #ifdef ENABLE_MODULE_ECDH
    # include "modules/ecdh/main_impl.h"
    #endif

    #ifdef ENABLE_MODULE_RECOVERY
    # include "modules/recovery/main_impl.h"
    #endif

    #ifdef ENABLE_MODULE_EXTRAKEYS
    # include "modules/extrakeys/main_impl.h"
    #endif

    #ifdef ENABLE_MODULE_SCHNORRSIG
    # include "modules/schnorrsig/main_impl.h"
    #endif
    */
}
