// ---------------- [ File: bitcoinsecp256k1-eccontext/src/flags.rs ]
crate::ix!();

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
