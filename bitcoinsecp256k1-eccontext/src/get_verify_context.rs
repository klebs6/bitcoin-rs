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
