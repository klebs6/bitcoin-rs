// ---------------- [ File: bitcoin-hash/src/hash_256.rs ]
crate::ix!();

/**
  | A hasher class for Bitcoin's 256-bit
  | hash (double SHA-256).
  |
  */
#[derive(Default)]
pub struct Hash256 {
    sha: Sha256,
}

impl Hash256 {
    pub const OUTPUT_SIZE: usize = SHA256_OUTPUT_SIZE;

    #[instrument(level = "trace", skip(self, input))]
    pub fn write(&mut self, input: &[u8]) -> &mut Self {
        self.sha.write(input);
        self
    }

    #[instrument(level = "trace", skip(self, output))]
    pub fn finalize(&mut self, output: &mut [u8; Hash256::OUTPUT_SIZE]) {
        debug_assert_eq!(output.len(), Hash256::OUTPUT_SIZE);

        let mut buf = [0u8; SHA256_OUTPUT_SIZE];
        self.sha.finalize(&mut buf);

        self.sha.reset();
        self.sha.write(&buf);
        self.sha.finalize(output);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn reset(&mut self) -> &mut Self {
        self.sha.reset();
        self
    }
}

// ---------------- [ File: bitcoin-hash/src/hash_256.rs ] (new test module)
#[cfg(test)]
mod hash256_spec {
    use super::*;

    /// Helper that returns the reference double‑SHA‑256 of the `payload`.
    fn reference_double_sha(payload: &[u8]) -> [u8; 32] {
        let mut first = [0u8; 32];
        let mut sha = Sha256::new();
        sha.write(payload);
        sha.finalize(&mut first);

        sha.reset();
        sha.write(&first);
        let mut second = [0u8; 32];
        sha.finalize(&mut second);
        second
    }

    #[traced_test]
    fn finalize_of_empty_matches_reference() {
        let mut hasher = Hash256::default();
        let mut out = [0u8; Hash256::OUTPUT_SIZE];
        hasher.finalize(&mut out);

        assert_eq!(out, reference_double_sha(&[]));
    }

    #[traced_test]
    fn write_then_finalize_matches_reference() {
        let payload = b"bitcoin-core";
        let mut hasher = Hash256::default();
        hasher.write(payload);
        let mut out = [0u8; Hash256::OUTPUT_SIZE];
        hasher.finalize(&mut out);

        assert_eq!(out, reference_double_sha(payload));
    }

    #[traced_test]
    fn reset_clears_internal_state() {
        let payload = b"x";
        let mut hasher = Hash256::default();
        hasher.write(payload);
        let mut buf1 = [0u8; Hash256::OUTPUT_SIZE];
        hasher.finalize(&mut buf1);

        hasher.reset().write(payload);
        let mut buf2 = [0u8; Hash256::OUTPUT_SIZE];
        hasher.finalize(&mut buf2);

        assert_eq!(buf1, buf2);
    }
}
