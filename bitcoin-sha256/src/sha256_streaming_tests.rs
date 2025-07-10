crate::ix!();

#[cfg(test)]
mod sha256_streaming_tests {
    use super::*;
    use hex_literal::hex; // dev‑dependency, small & ubiquitous

    /// Convenience to compute a digest in one shot.
    fn digest_one_shot(data: &[u8]) -> [u8; SHA256_OUTPUT_SIZE] {
        let mut ctx = Sha256::new();
        ctx.write_all(data).expect("in‑memory write cannot fail");
        let mut out = [0u8; SHA256_OUTPUT_SIZE];
        ctx.finalize(&mut out);
        out
    }

    /// SHA‑256("") from FIPS 180‑4 §7.3 – single empty message.
    const DIGEST_EMPTY: [u8; 32] =
        hex!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");

    /// SHA‑256("abc") from FIPS 180‑4 test vectors.
    const DIGEST_ABC: [u8; 32] =
        hex!("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");

    #[traced_test]
    fn empty_string_matches_reference() {
        assert_eq!(digest_one_shot(b""), DIGEST_EMPTY);
    }

    #[traced_test]
    fn abc_string_matches_reference() {
        assert_eq!(digest_one_shot(b"abc"), DIGEST_ABC);
    }

    #[traced_test]
    fn chunked_vs_single_write_identical() {
        let data = b"The quick brown fox jumps over the lazy dog";
        /* one‑shot */
        let one_shot = digest_one_shot(data);

        /* chunked 7 + 13 + rest */
        let mut ctx = Sha256::new();
        ctx.write_all(&data[..7]).unwrap();
        ctx.write_all(&data[7..20]).unwrap();
        ctx.write_all(&data[20..]).unwrap();
        let mut chunked = [0u8; SHA256_OUTPUT_SIZE];
        ctx.finalize(&mut chunked);

        assert_eq!(one_shot, chunked, "streaming API must be chunk‑invariant");
    }

    #[traced_test]
    fn write_from_iterator_yields_same_digest() {
        let data = b"iterator feed test";
        let iter = Box::new(data.to_vec().into_iter());
        let mut ctx = Sha256::new();
        ctx.write_from_iterator(iter, data.len());
        let mut out = [0u8; SHA256_OUTPUT_SIZE];
        ctx.finalize(&mut out);

        assert_eq!(out, digest_one_shot(data));
    }
}
