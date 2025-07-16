// ---------------- [ File: bitcoin-sha256/src/sha256_initialize_tagged.rs ]
crate::ix!();

/**
  | Initializes a sha256 struct and writes
  | the 64 byte string
  | SHA256(tag)||SHA256(tag) into it.
  |
  */
pub unsafe fn sha256_initialize_tagged(
    hash:   *mut Sha256,
    tag:    *const u8,
    taglen: usize
) {
    let mut buf: [u8; 32] = [0; 32];

    sha256_initialize((*hash).s_mut().as_mut_ptr());
    sha256_write(hash, tag, taglen);
    sha256_finalize(hash, buf.as_mut_ptr());

    sha256_initialize((*hash).s_mut().as_mut_ptr());
    sha256_write(hash, buf.as_mut_ptr(), 32);
    sha256_write(hash, buf.as_mut_ptr(), 32);
}

// -----------------------------------------------------------------------------
// Tests: sha256_initialize_tagged()
// -----------------------------------------------------------------------------
#[cfg(test)]
mod sha256_initialize_tagged_contract_validation {
    use super::*;
    use std::{io::Write, ptr::null};
    use hex_literal::hex;

    /// Helper: SHA256(tag) via the safe API.
    fn sha256_once(tag: &[u8]) -> [u8; 32] {
        let mut out = [0u8; 32];
        let mut ctx = Sha256::new();
        ctx.write_all(tag).unwrap();
        ctx.finalize(&mut out);
        out
    }

    /// For any tag *T* and message *M*,
    /// `initialize_tagged(T)` ‖ write(M) ≡ SHA256(SHA256(T)‖SHA256(T)‖M)
    #[traced_test]
    fn tagged_initialisation_matches_explicit_prefix_path() {
        const TAG: &[u8] = b"example-tag";
        const MSG: &[u8] = b"hello-tagged-world";

        // ── Reference: explicit prefix path ─────────────────────────────────
        let tag_hash = sha256_once(TAG);
        let mut ref_ctx = Sha256::new();
        ref_ctx.write_all(&tag_hash).unwrap();
        ref_ctx.write_all(&tag_hash).unwrap();
        ref_ctx.write_all(MSG).unwrap();
        let mut digest_ref = [0u8; 32];
        ref_ctx.finalize(&mut digest_ref);

        // ── SUT: use sha256_initialize_tagged --------------------------------
        let mut ctx_tagged = Sha256::new();
        unsafe {
            sha256_initialize_tagged(
                &mut ctx_tagged,
                TAG.as_ptr(),
                TAG.len(),
            );
        }
        ctx_tagged.write_all(MSG).unwrap();
        let mut digest_tagged = [0u8; 32];
        ctx_tagged.finalize(&mut digest_tagged);

        assert_eq!(
            digest_tagged, digest_ref,
            "tagged initialisation produced wrong digest"
        );
    }

    /// Empty tag is legal and should behave as if the prefix is
    /// SHA256("")‖SHA256("").
    #[traced_test]
    fn empty_tag_behaviour_is_consistent() {
        const MSG: &[u8] = b"data-with-empty-tag";

        let tag_hash = sha256_once(b"");

        // Reference digest
        let mut ref_ctx = Sha256::new();
        ref_ctx.write_all(&tag_hash).unwrap();
        ref_ctx.write_all(&tag_hash).unwrap();
        ref_ctx.write_all(MSG).unwrap();
        let mut digest_ref = [0u8; 32];
        ref_ctx.finalize(&mut digest_ref);

        // SUT digest
        let mut ctx = Sha256::new();
        unsafe { sha256_initialize_tagged(&mut ctx, null(), 0) };
        ctx.write_all(MSG).unwrap();
        let mut digest_sut = [0u8; 32];
        ctx.finalize(&mut digest_sut);

        assert_eq!(digest_sut, digest_ref, "empty-tag initialisation mismatch");
    }
}
