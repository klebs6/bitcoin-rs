// ---------------- [ File: bitcoinsecp256k1-ec/src/context.rs ]
/*!
  | Unless explicitly stated all pointer
  | arguments must not be NULL.
  | 
  | The following rules specify the order
  | of arguments in API calls:
  | 
  | -1. Context pointers go first, followed
  | by output arguments, combined output/input
  | arguments, and finally input-only
  | arguments.
  | 
  | -2. Array lengths always immediately
  | follow the argument whose length they
  | describe, even if this violates rule
  | 1.
  | 
  | -3. Within the OUT/OUTIN/IN groups,
  | pointers to data that is typically generated
  | later go first. This means: signatures,
  | public nonces, secret nonces, messages,
  | public keys, secret keys, tweaks.
  | 
  | -4. Arguments that are not data pointers
  | go last, from more complex to less complex:
  | function pointers, algorithm names,
  | messages, c_void pointers, counts, flags,
  | booleans.
  | 
  | -5. Opaque data pointers follow the
  | function pointer they are to be passed
  | to.
  |
  */

crate::ix!();

/**
  | Opaque data structure that holds context
  | information (precomputed tables etc.).
  | 
  | The purpose of context structures is
  | to cache large precomputed data tables
  | that are expensive to construct, and
  | also to maintain the randomization
  | data for blinding.
  | 
  | Do not create a new context object for
  | each operation, as construction is
  | far slower than all other API calls (~100
  | times slower than an ECDSA verification).
  | 
  | A constructed context can safely be
  | used from multiple threads simultaneously,
  | but API calls that take a non-const pointer
  | to a context need exclusive access to
  | it. In particular this is the case for
  | context_destroy, context_preallocated_destroy,
  | and context_randomize.
  | 
  | Regarding randomization, either do
  | it once at creation time (in which case
  | you do not need any locking for the other
  | calls), or use a read-write lock.
  |
  */
pub struct Secp256k1Context {
    ecmult_ctx:       EcMultContext,
    ecmult_gen_ctx:   EcMultGenContext,
    illegal_callback: Callback,
    error_callback:   Callback,
    declassify:       i32,
}

pub const CONTEXT_NO_PRECOMP: Secp256k1Context = Secp256k1Context {
    ecmult_ctx:       EcMultContext::new(),
    ecmult_gen_ctx:   EcMultGenContext::new(),
    illegal_callback: Callback {
        fn_:  default_illegal_callback_fn,
        data: null_mut(),
    },
    error_callback:   Callback {
        fn_:  default_error_callback_fn,
        data: null_mut(),
    },
    declassify:       0,
};

pub fn context_preallocated_size(flags: u32) -> usize {
    
    todo!();
        /*
            size_t ret = ROUND_TO_ALIGN(sizeof(context));
        /* A return value of 0 is reserved as an indicator for errors when we call this function internally. */
        VERIFY_CHECK(ret != 0);

        if (EXPECT((flags & FLAGS_TYPE_MASK) != FLAGS_TYPE_CONTEXT, 0)) {
                callback_call(&default_illegal_callback,
                                        "Invalid flags");
                return 0;
        }

        if (flags & FLAGS_BIT_CONTEXT_SIGN) {
            ret += ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE;
        }
        if (flags & FLAGS_BIT_CONTEXT_VERIFY) {
            ret += ECMULT_CONTEXT_PREALLOCATED_SIZE;
        }
        return ret;
        */
}

pub fn context_preallocated_clone_size(ctx: *const Secp256k1Context) -> usize {
    
    todo!();
        /*
            size_t ret = ROUND_TO_ALIGN(sizeof(context));
        VERIFY_CHECK(ctx != NULL);
        if (ecmult_gen_context_is_built(&ctx->ecmult_gen_ctx)) {
            ret += ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE;
        }
        if (ecmult_context_is_built(&ctx->ecmult_ctx)) {
            ret += ECMULT_CONTEXT_PREALLOCATED_SIZE;
        }
        return ret;
        */
}

pub fn context_preallocated_create(
        prealloc: *mut c_void,
        flags:    u32) -> *mut Secp256k1Context {
    
    todo!();
        /*
            c_void* const base = prealloc;
        size_t prealloc_size;
        context* ret;

        if (!selftest()) {
            callback_call(&default_error_callback, "self test failed");
        }

        prealloc_size = context_preallocated_size(flags);
        if (prealloc_size == 0) {
            return NULL;
        }
        VERIFY_CHECK(prealloc != NULL);
        ret = (context*)manual_alloc(&prealloc, sizeof(context), base, prealloc_size);
        ret->illegal_callback = default_illegal_callback;
        ret->error_callback = default_error_callback;

        ecmult_context_init(&ret->ecmult_ctx);
        ecmult_gen_context_init(&ret->ecmult_gen_ctx);

        /* Flags have been checked by context_preallocated_size. */
        VERIFY_CHECK((flags & FLAGS_TYPE_MASK) == FLAGS_TYPE_CONTEXT);
        if (flags & FLAGS_BIT_CONTEXT_SIGN) {
            ecmult_gen_context_build(&ret->ecmult_gen_ctx, &prealloc);
        }
        if (flags & FLAGS_BIT_CONTEXT_VERIFY) {
            ecmult_context_build(&ret->ecmult_ctx, &prealloc);
        }
        ret->declassify = !!(flags & FLAGS_BIT_CONTEXT_DECLASSIFY);

        return (context*) ret;
        */
}

/**
  | Create a secp256k1 context object (in
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
  | In: flags: which parts of the context
  | to initialize.
  | 
  | See also context_randomize.
  |
  */
pub fn context_create(flags: u32) -> *mut Secp256k1Context {
    
    todo!();
        /*
            size_t const prealloc_size = context_preallocated_size(flags);
        context* ctx = (context*)checked_malloc(&default_error_callback, prealloc_size);
        if (EXPECT(context_preallocated_create(ctx, flags) == NULL, 0)) {
            free(ctx);
            return NULL;
        }

        return ctx;
        */
}

pub fn context_preallocated_clone(
        ctx:      *const Secp256k1Context,
        prealloc: *mut c_void) -> *mut Secp256k1Context {
    
    todo!();
        /*
            size_t prealloc_size;
        context* ret;
        VERIFY_CHECK(ctx != NULL);
        ARG_CHECK(prealloc != NULL);

        prealloc_size = context_preallocated_clone_size(ctx);
        ret = (context*)prealloc;
        memcpy(ret, ctx, prealloc_size);
        ecmult_gen_context_finalize_memcpy(&ret->ecmult_gen_ctx, &ctx->ecmult_gen_ctx);
        ecmult_context_finalize_memcpy(&ret->ecmult_ctx, &ctx->ecmult_ctx);
        return ret;
        */
}

pub fn context_clone(ctx: *const Secp256k1Context) -> *mut Secp256k1Context {
    
    todo!();
        /*
            context* ret;
        size_t prealloc_size;

        VERIFY_CHECK(ctx != NULL);
        prealloc_size = context_preallocated_clone_size(ctx);
        ret = (context*)checked_malloc(&ctx->error_callback, prealloc_size);
        ret = context_preallocated_clone(ctx, ret);
        return ret;
        */
}

pub fn context_preallocated_destroy(ctx: *mut Secp256k1Context)  {
    
    todo!();
        /*
            ARG_CHECK_NO_RETURN(ctx != context_no_precomp);
        if (ctx != NULL) {
            ecmult_context_clear(&ctx->ecmult_ctx);
            ecmult_gen_context_clear(&ctx->ecmult_gen_ctx);
        }
        */
}

/**
  | Destroy a secp256k1 context object
  | (created in dynamically allocated
  | memory).
  | 
  | The context pointer may not be used afterwards.
  | 
  | The context to destroy must have been
  | created using context_create or context_clone.
  | If the context has instead been created
  | using context_preallocated_create
  | or context_preallocated_clone, the
  | behaviour is undefined. In that case,
  | context_preallocated_destroy must
  | be used instead.
  | 
  | Args: ctx: an existing context to destroy,
  |            constructed using context_create
  |            or context_clone
  |
  */
pub fn context_destroy(ctx: *mut Secp256k1Context)  {
    
    todo!();
        /*
            if (ctx != NULL) {
            context_preallocated_destroy(ctx);
            free(ctx);
        }
        */
}

pub fn context_set_illegal_callback(
        ctx:  *mut Secp256k1Context,
        fun:  fn(message: *const u8, data: *mut c_void) -> c_void,
        data: *const c_void)  {
    
    todo!();
        /*
            ARG_CHECK_NO_RETURN(ctx != context_no_precomp);
        if (fun == NULL) {
            fun = default_illegal_callback_fn;
        }
        ctx->illegal_callback.fn = fun;
        ctx->illegal_callback.data = data;
        */
}

pub fn context_set_error_callback(
        ctx:  *mut Secp256k1Context,
        fun:  fn(message: *const u8, data: *mut c_void) -> c_void,
        data: *const c_void)  {
    
    todo!();
        /*
            ARG_CHECK_NO_RETURN(ctx != context_no_precomp);
        if (fun == NULL) {
            fun = default_error_callback_fn;
        }
        ctx->error_callback.fn = fun;
        ctx->error_callback.data = data;
        */
}

pub fn context_randomize(
        ctx:    *mut Secp256k1Context,
        seed32: *const u8) -> i32 {
    
    todo!();
        /*
            VERIFY_CHECK(ctx != NULL);
        if (ecmult_gen_context_is_built(&ctx->ecmult_gen_ctx)) {
            ecmult_gen_blind(&ctx->ecmult_gen_ctx, seed32);
        }
        return 1;
        */
}

/**
  | Access to the internal secp256k1 context
  | used for verification. Only intended
  | to be used by key.cpp.
  |
  */
pub fn get_verify_context() -> *const Secp256k1Context {
    
    todo!();
        /*
            return secp256k1_context_verify;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/basic-config.h]
#[cfg(USE_BASIC_CONFIG)] pub const ECMULT_WINDOW_SIZE:   usize = 15;
#[cfg(USE_BASIC_CONFIG)] pub const ECMULT_GEN_PREC_BITS: usize = 4;
