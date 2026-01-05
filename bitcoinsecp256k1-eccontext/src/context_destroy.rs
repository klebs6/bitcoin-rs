// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_destroy.rs ]
crate::ix!();

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
pub fn context_destroy(ctx: *mut Secp256k1Context) {
    unsafe {
        if !ctx.is_null() {
            context_preallocated_destroy(ctx);
            libc::free(ctx as *mut c_void);
        }
    }
}
