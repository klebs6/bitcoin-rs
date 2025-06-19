// ---------------- [ File: bitcoin-sha1/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{sha1}
x!{sha1_round}
x!{sha1_transform}
x!{endian}

//──────────────────────────────────────────────────────────────────────────────
//  Conformance tests – verify bit‑for‑bit parity with a reference SHA‑1
//──────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod sha1_equivalence {
    use super::*;
    use rand::{rngs::StdRng, RngCore, SeedableRng};
    use ::sha1::{Digest, Sha1 as RefSha1};
    use tracing::info;

    /// Compute SHA‑1 with the reference (RustCrypto) implementation.
    fn digest_with_reference(input: &[u8]) -> [u8; SHA1_OUTPUT_SIZE] {
        let mut ref_hasher = RefSha1::new();
        ref_hasher.update(input);
        let mut out = [0u8; SHA1_OUTPUT_SIZE];
        out.copy_from_slice(&ref_hasher.finalize());
        out
    }

    /// Compute SHA‑1 with our port.
    fn digest_with_ours(input: &[u8]) -> [u8; SHA1_OUTPUT_SIZE] {
        let mut h = Sha1::new();
        h.write(input.as_ptr(), input.len());
        let mut out = [0u8; SHA1_OUTPUT_SIZE];
        h.finalize(&mut out);
        out
    }

    #[traced_test]
    fn empty_string_matches_reference() {
        let expected = digest_with_reference(b"");
        let got = digest_with_ours(b"");
        assert_eq!(expected, got, "digest mismatch for empty string");
        info!("digest(empty) = {}", hex::encode(got));
    }

    #[traced_test]
    fn known_vectors_match_reference() {
        let vectors: &[&[u8]] = &[
            b"abc",
            b"The quick brown fox jumps over the lazy dog",
            b"The quick brown fox jumps over the lazy dog.",
        ];
        for &msg in vectors {
            let expected = digest_with_reference(msg);
            let got = digest_with_ours(msg);
            assert_eq!(expected, got, "digest mismatch for {:?}", msg);
        }
    }

    #[traced_test]
    fn random_inputs_match_reference() {
        let mut rng = StdRng::seed_from_u64(0xDEAD_BEEF_DEAD_BEEF);
        for _ in 0..1_000 {
            let len = (rng.next_u32() % 2048) as usize; // up to 2 KiB
            let mut buf = vec![0u8; len];
            rng.fill_bytes(&mut buf);
            let expected = digest_with_reference(&buf);
            let got = digest_with_ours(&buf);
            assert_eq!(expected, got, "digest mismatch for random len={}", len);
        }
    }
}
