// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context.rs ]
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
    illegal_callback: Callback::new(default_illegal_callback_fn,null_mut()),
    error_callback:   Callback::new(default_error_callback_fn,null_mut()),
    declassify:       0,
};

#[cfg(test)]
mod secp256k1_context_type_contract_suite {
    use super::*;

    fn assert_send_sync<T: Send + Sync>() {}

    #[traced_test]
    fn secp256k1_context_is_send_and_sync_for_shared_read_use() {
        tracing::info!("asserting Send + Sync bounds for Secp256k1Context");
        assert_send_sync::<Secp256k1Context>();
        tracing::debug!("Send + Sync bounds satisfied");
    }

    #[traced_test]
    fn context_no_precomp_clone_size_matches_base_preallocated_size() {
        tracing::trace!("computing clone size for CONTEXT_NO_PRECOMP");
        let ctx_ptr: *const Secp256k1Context = &CONTEXT_NO_PRECOMP as *const Secp256k1Context;

        let clone_size = context_preallocated_clone_size(ctx_ptr);
        let base_size = context_preallocated_size(FLAGS_TYPE_CONTEXT);

        tracing::debug!(clone_size, base_size, "computed sizes");
        assert_eq!(clone_size, base_size);
    }
}
