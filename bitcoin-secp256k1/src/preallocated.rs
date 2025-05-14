// ---------------- [ File: bitcoin-secp256k1/src/preallocated.rs ]
/*!
  | The module provided by this header file
  | is intended for settings in which it
  | is not possible or desirable to rely
  | on dynamic memory allocation. It provides
  | functions for creating, cloning, and
  | destroying secp256k1 context objects
  | in a contiguous fixed-size block of
  | memory provided by the caller.
  | 
  | Secp256k1Context objects created by functions
  | in this module can be used like contexts
  | objects created by functions in secp256k1.h,
  | i.e., they can be passed to any API function
  | that expects a context object (see secp256k1.h
  | for details). The only exception is
  | that context objects created by functions
  | in this module must be destroyed using
  | context_preallocated_destroy
  | (in this module) instead of context_destroy
  | (in secp256k1.h).
  | 
  | It is guaranteed that functions in this
  | module will not call malloc or its friends
  | realloc, calloc, and free.
  |
  */

crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/secp256k1/include/preallocated.h]

/**
  | Determine the memory size of a secp256k1
  | context object to be created in caller-provided
  | memory.
  | 
  | The purpose of this function is to determine
  | how much memory must be provided to context_preallocated_create.
  | 
  | Returns: the required size of the caller-provided
  | memory block
  | 
  | In: flags: which parts of the context
  | to initialize.
  |
  */
lazy_static!{
    /*
    API size_t context_preallocated_size(
        unsigned int flags
    ) WARN_UNUSED_RESULT;
    */
}

/** 
  | Create a secp256k1 context object in
  | caller-provided memory.
  |
  | The caller must provide a pointer to
  | a rewritable contiguous block of memory of
  | size at least context_preallocated_size(flags)
  | bytes, suitably aligned to hold an object of
  | any type.
  |
  | The block of memory is exclusively owned by
  | the created context object during the lifetime
  | of this context object, which begins with the
  | call to this function and ends when a call to
  | context_preallocated_destroy (which destroys
  | the context object again) returns. During the
  | lifetime of the context object, the caller is
  | obligated not to access this block of memory,
  | i.e., the caller may not read or write the
  | memory, e.g., by copying the memory contents
  | to a different location or trying to create
  | a second context object in the memory. In
  | simpler words, the prealloc pointer (or any
  | pointer derived from it) should not be used
  | during the lifetime of the context object.
  |
  |  Returns: a newly created context object.
  |
  |  In:      prealloc: a pointer to a rewritable
  |                     contiguous block of memory
  |                     of size at least
  |                     context_preallocated_size(flags)
  |                     bytes, as detailed above
  |                     (cannot be NULL)
  |
  |           flags:    which parts of the context
  |           to initialize.
  |
  |  See also context_randomize (in secp256k1.h)
  |  and context_preallocated_destroy.
  */
lazy_static!{
    /*
    context* context_preallocated_create(
        c_void* prealloc,
        unsigned int flags
    ) ARG_NONNULL(1) WARN_UNUSED_RESULT;
    */
}

/** 
 | Determine the memory size of a secp256k1
 | context object to be copied into
 | caller-provided memory.
 |
 |  Returns: the required size of the
 |           caller-provided memory block.
 |
 |  In:      ctx: an existing context to copy
 |           (cannot be NULL)
 */
lazy_static!{
    /*
    size_t context_preallocated_clone_size(
        const context* ctx
    ) ARG_NONNULL(1) WARN_UNUSED_RESULT;
    */
}

/** 
 | Copy a secp256k1 context object into
 | caller-provided memory.
 |
 | The caller must provide a pointer to
 | a rewritable contiguous block of memory of
 | size at least context_preallocated_size(flags)
 | bytes, suitably aligned to hold an object of
 | any type.
 |
 | The block of memory is exclusively owned by
 | the created context object during the lifetime
 | of this context object, see the description of
 | context_preallocated_create for details.
 |
 |  Returns: a newly created context object.
 |
 |  Args:    ctx:      an existing context to copy
 |                     (cannot be NULL)
 |
 |  In:      prealloc: a pointer to a rewritable
 |                     contiguous block of memory
 |                     of size at least
 |                     context_preallocated_size(flags)
 |                     bytes, as detailed above
 |                     (cannot be NULL)
 */
lazy_static!{
    /*
    context* context_preallocated_clone(
        const context* ctx,
        c_void* prealloc
    ) ARG_NONNULL(1) ARG_NONNULL(2) WARN_UNUSED_RESULT;
    */
}

/** 
 | Destroy a secp256k1 context object that has
 | been created in caller-provided memory.
 |
 |  The context pointer may not be used
 |  afterwards.
 |
 |  The context to destroy must have been created
 |  using context_preallocated_create or
 |  context_preallocated_clone.  If the context
 |  has instead been created using context_create
 |  or context_clone, the behaviour is
 |  undefined. In that case, context_destroy must
 |  be used instead.
 |
 |  If required, it is the responsibility of the
 |  caller to deallocate the block of memory
 |  properly after this function returns, e.g., by
 |  calling free on the preallocated pointer given
 |  to context_preallocated_create or
 |  context_preallocated_clone.
 |
 |  Args:   ctx: an existing context to destroy,
 |               constructed using
 |               context_preallocated_create or
 |               context_preallocated_clone (cannot be NULL)
 */
pub fn context_preallocated_destroy(ctx: *mut Secp256k1Context)  {
    
    todo!();
        /*
        
        */
}
