// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_create.rs ]
crate::ix!();

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
    unsafe {
        let prealloc_size: usize = context_preallocated_size(flags);
        let ctx: *mut Secp256k1Context =
            checked_malloc(&*default_error_callback, prealloc_size) as *mut Secp256k1Context;

        if expect!(
            context_preallocated_create(ctx as *mut c_void, flags).is_null(),
            0
        ) {
            libc::free(ctx as *mut c_void);
            return core::ptr::null_mut();
        }

        ctx
    }
}
