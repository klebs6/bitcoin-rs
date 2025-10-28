// ---------------- [ File: bitcoin-base58/src/encode_check.rs ]
crate::ix!();

/// Encode a byte span into a base58-encoded string, **including checksum** (double SHA‑256).
pub fn encode_base_58check(input: &[u8]) -> String {
    trace!(payload_len = input.len(), "encode_base_58check: starting");
    let mut vch: Vec<u8> = Vec::with_capacity(input.len() + 4);
    vch.extend_from_slice(input);

    // Canonical checksum: first 4 **big‑endian** bytes of SHA256(SHA256(payload))
    let chk = checksum4_sha256d(input);
    vch.extend_from_slice(&chk);

    let out = encode_base58(&vch);
    info!(encoded_len = out.len(), "encode_base_58check: success");
    out

}

#[cfg(test)]
mod encode_check_spec {
    use super::*;

    /// Leading‑zero semantics for **checked** encoding.
    #[traced_test]
    fn leading_zeros_retained_with_checksum() {
        let payload = b"\0\0hello";
        let encoded = encode_base_58check(payload);
        info!(?encoded, "encoded payload with leading zeros");
        assert!(
            encoded.starts_with("11"),
            "each leading zero byte must map to a ‘1’ in Base‑58"
        );
        let mut decoded = Vec::new();
        assert!(
            decode_base_58check(&encoded, &mut decoded, 64),
            "decoding failed"
        );
        assert_eq!(decoded, payload);
    }

    /// `encode_base_58check` must append **four checksum bytes**.
    #[traced_test]
    fn output_length_is_payload_plus_checksum() {
        let payload = b"cat";
        let encoded = encode_base_58check(payload);
        let mut decoded = Vec::<u8>::new();
        assert!(decode_base_58check(&encoded, &mut decoded, 16));
        assert_eq!(decoded, payload);
    }
}
