// ---------------- [ File: bitcoin-base58/src/encode_check.rs ]
crate::ix!();

/**
  | Encode a byte span into a base58-encoded
  | string, including checksum
  |
  */
pub fn encode_base_58check(input: &[u8]) -> String {
    
    // add 4-byte hash check to the end
    let mut vch: Vec::<u8> = input.to_vec();

    let hash: u256 = hash1(&vch);

    for byte in &hash.blob().data()[0..4] {
        vch.push(*byte);
    }

    encode_base58(&vch)
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
