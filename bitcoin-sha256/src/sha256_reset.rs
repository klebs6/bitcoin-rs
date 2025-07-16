// ---------------- [ File: bitcoin-sha256/src/sha256_reset.rs ]
crate::ix!();

impl Sha256 {

    /// Reset the context to its IV, zeroing buffer & counters.
    #[inline]
    pub fn reset(&mut self) -> &mut Self {
        *self.bytes_mut() = 0;
        self.buf_mut().fill(0);
        unsafe { sha256_initialize(self.s_mut().as_mut_ptr()) };
        trace!(target: "sha256", "Sha256::reset: context re‑initialised");
        self
    }
}

// -----------------------------------------------------------------------------
// Tests: reset() behavioural contract
// -----------------------------------------------------------------------------
#[cfg(test)]
mod sha256_reset_contract_validation {
    use super::*;
    use std::io::Write;

    /// After writing data then calling `reset`, the context must be
    /// indistinguishable from a freshly‑constructed one.
    #[traced_test]
    fn reset_restores_iv_and_clears_state() {
        let mut ctx = Sha256::new();
        ctx.write_all(b"some data").unwrap();

        assert_ne!(*ctx.bytes(), 0, "pre-condition failed: no bytes written");

        ctx.reset(); // ── SUT ────────────────────────────────────────────────

        // A pristine context for comparison.
        let pristine = Sha256::new();

        assert_eq!(ctx.s(),  pristine.s(),  "state words differ after reset()");
        assert_eq!(ctx.buf(), pristine.buf(), "buffer not zeroed by reset()");
        assert_eq!(*ctx.bytes(), 0,           "byte counter not cleared");
    }

    /// A context should remain fully usable after `reset`, producing the
    /// same digest as a brand‑new context fed identical data.
    #[traced_test]
    fn reset_allows_full_reuse_without_side_effects() {
        const MSG: &[u8] = b"hello-after-reset";

        // Path 1: write → reset → write again
        let mut ctx_reuse = Sha256::new();
        ctx_reuse.write_all(b"throw-away").unwrap();
        ctx_reuse.reset();
        ctx_reuse.write_all(MSG).unwrap();
        let mut digest_reuse = [0u8; SHA256_OUTPUT_SIZE];
        ctx_reuse.finalize(&mut digest_reuse);

        // Path 2: fresh context, single write
        let mut ctx_fresh = Sha256::new();
        ctx_fresh.write_all(MSG).unwrap();
        let mut digest_fresh = [0u8; SHA256_OUTPUT_SIZE];
        ctx_fresh.finalize(&mut digest_fresh);

        assert_eq!(
            digest_reuse, digest_fresh,
            "digest mismatch: context reuse after reset() is broken"
        );
    }
}
