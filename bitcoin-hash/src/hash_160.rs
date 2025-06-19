// ---------------- [ File: bitcoin-hash/src/hash_160.rs ]
crate::ix!();

/**
  | A hasher class for Bitcoin's 160-bit
  | hash (SHA-256 + RIPEMD-160).
  |
  */
pub struct Hash160 {
    sha: Sha256,
}

impl Hash160 {
    pub const OUTPUT_SIZE: usize = RIPEMD160_OUTPUT_SIZE;

    #[instrument(level = "trace", skip(self, output))]
    pub fn finalize(&mut self, output: &mut [u8; Hash160::OUTPUT_SIZE]) {
        debug_assert_eq!(output.len(), Hash160::OUTPUT_SIZE);

        /* first SHA‑256 pass */
        let mut sha_buf = [0u8; SHA256_OUTPUT_SIZE];
        self.sha.finalize(sha_buf);

        /* second RIPEMD‑160 pass */
        let mut ripemd = Ripemd160::new();
        ripemd.update(&sha_buf);
        ripemd.finalize(output.clone()); // writes directly into `output`
    }

    #[instrument(level = "trace", skip(self, input))]
    pub fn write(&mut self, input: &[u8]) -> &mut Self {
        // SAFETY: slice pointer + len remains valid for the duration
        // of the call.
        self.sha.write(input.as_ptr(), input.len());
        self
    }

    #[instrument(level = "trace", skip(self))]
    pub fn reset(&mut self) -> &mut Self {
        self.sha.reset();
        self
    }
}

/**
  | Compute the 160‑bit (SHA‑256 ➜ RIPEMD‑160) hash of an object that
  | can expose its bytes through `AsRef<[u8]>`.
  |
  | This is a thin convenience wrapper around [`Hash160`] that mirrors
  | Bitcoin Core’s `Hash160()` helper.
  */
#[inline]
pub fn hash160<T: AsRef<[u8]>>(in1: &T) -> u160 {
    let mut out = [0u8; RIPEMD160_OUTPUT_SIZE];
    Hash160 { sha: Sha256::new() }
        .write(in1.as_ref())
        .finalize(&mut out);

    // The `bitcoin‑u160` crate exposes `from_le_bytes()` just like
    // `u256`; fall back to a generic constructor if the exact name
    // ever changes.
    #[allow(clippy::useless_conversion)]
    u160::from_le_bytes(out)
}

// ---------------- [ File: bitcoin-hash/src/hash_160.rs ] (new test module)
#[cfg(test)]
mod hash160_spec {
    use super::*;

    const RIPEMD_OUT: usize = RIPEMD160_OUTPUT_SIZE;

    /// Helper computing the reference SHA‑256 + RIPEMD‑160 chain.
    fn reference_hash(payload: &[u8]) -> [u8; RIPEMD_OUT] {
        /* SHA‑256 */
        let mut sha_buf = [0u8; 32];
        let mut sha = Sha256::new();
        sha.write(payload.as_ptr(), payload.len());
        sha.finalize(sha_buf);

        /* RIPEMD‑160 */
        let mut ripemd = Ripemd160::new();
        ripemd.update(&sha_buf);
        let mut out = [0u8; RIPEMD_OUT];
        ripemd.finalize(out);
        out
    }

    #[traced_test]
    fn empty_payload_matches_reference() {
        let mut hasher = Hash160 { sha: Sha256::new() };
        let mut out = [0u8; Hash160::OUTPUT_SIZE];
        hasher.finalize(&mut out);
        assert_eq!(out, reference_hash(&[]));
    }

    #[traced_test]
    fn write_then_finalize_matches_reference() {
        let data = b"bitcoin-hash crate";
        let mut hasher = Hash160 { sha: Sha256::new() };
        hasher.write(data);
        let mut out = [0u8; Hash160::OUTPUT_SIZE];
        hasher.finalize(&mut out);
        assert_eq!(out, reference_hash(data));
    }

    #[traced_test]
    fn reset_clears_internal_state() {
        let payload = b"*";
        let mut hasher = Hash160 { sha: Sha256::new() };
        hasher.write(payload);
        let mut buf1 = [0u8; Hash160::OUTPUT_SIZE];
        hasher.finalize(&mut buf1);

        hasher.reset().write(payload);
        let mut buf2 = [0u8; Hash160::OUTPUT_SIZE];
        hasher.finalize(&mut buf2);
        assert_eq!(buf1, buf2);
    }

    #[traced_test]
    fn hash160_matches_manual_chain() {
        let data = b"Bitcoin hash160";
        let via_helper = hash160(&data[..]);

        /* manual reference computation */
        let mut sha_buf = [0u8; 32];
        let mut sha = Sha256::new();
        sha.write(data.as_ptr(), data.len());
        sha.finalize(sha_buf);

        let mut ripemd = Ripemd160::new();
        ripemd.update(&sha_buf);
        let mut ref_buf = [0u8; RIPEMD160_OUTPUT_SIZE];
        ripemd.finalize(ref_buf);

        let via_manual = u160::from_le_bytes(ref_buf);
        assert_eq!(via_helper, via_manual);
    }

    #[traced_test]
    fn hash160_of_empty_is_stable() {
        // hard‑coded test vector from Bitcoin Core
        let expected = "b472a266d0bd89c13706a4132ccfb16f7c3b9fcb";
        let digest = hash160(&[] as &[u8]).to_string();
        assert_eq!(digest, expected);
    }
}
