// ---------------- [ File: bitcoinsecp256k1-eccontext/src/get_verify_context.rs ]
crate::ix!();

/**
  | Access to the internal secp256k1 context
  | used for verification. Only intended
  | to be used by key.cpp.
  |
  */
pub fn get_verify_context() -> *const Secp256k1Context {
    unsafe {
        secp256k1_context_verify
    }
}

#[cfg(test)]
mod get_verify_context_api_contract_suite {
    use super::*;

    #[traced_test]
    fn get_verify_context_is_stable_across_multiple_calls() {
        let p1 = get_verify_context();
        let p2 = get_verify_context();

        tracing::debug!(p1 = ?p1, p2 = ?p2, "get_verify_context returned pointers");
        assert_eq!(p1, p2);

        if p1.is_null() {
            tracing::warn!("verification context pointer is NULL (may be uninitialized in this configuration)");
        } else {
            tracing::info!(p1 = ?p1, "verification context pointer is non-NULL");
        }
    }
}
