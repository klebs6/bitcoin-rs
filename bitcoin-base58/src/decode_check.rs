// ---------------- [ File: bitcoin-base58/src/decode_check.rs ]
crate::ix!();

/// Return the **first 4 bytes** of `SHA256(SHA256(payload))` in digest order
/// (no endianness flips, no integer reinterpretation).
#[inline]
pub fn checksum4_sha256d(payload: &[u8]) -> [u8; 4] {
    trace!(
        payload_len = payload.len(),
        "Base58Check checksum: computing SHA256d and taking first 4 bytes"
    );
    // Use the bitcoin-hash crate's one-pass double‑SHA256 hasher directly to
    // avoid any u256 endianness concerns.
    let mut hasher = Hash256::default();
    hasher.write(payload);

    let mut digest = [0u8; Hash256::OUTPUT_SIZE];
    hasher.finalize(&mut digest); // digest = SHA256(SHA256(payload))

    let out = [digest[0], digest[1], digest[2], digest[3]];
    debug!(checksum = %hex::encode(out), "Base58Check checksum computed");
    out

}

/// Decode a base58-encoded string (psz) that includes a checksum into a byte
/// vector (vchRet). Return `true` if decoding is successful.
pub unsafe fn decode_base_58check_raw(
    mut psz:     *const u8,
    vch_ret:     &mut Vec<u8>,
    max_ret_len: i32
) -> bool {
    let budget = if max_ret_len > i32::MAX - 4 { i32::MAX } else { max_ret_len + 4 };
    debug!(max_ret_len, budget, "decode_base_58check_raw: starting");

    if !decode_base58_raw(psz, vch_ret, budget) {
        warn!("decode_base_58check_raw: underlying Base‑58 decode failed");
        vch_ret.clear();
        return false;
    }
    if vch_ret.len() < 4 {
        warn!(decoded_len = vch_ret.len(), "decode_base_58check_raw: insufficient length for checksum");
        vch_ret.clear();
        return false;
    }

    let payload_len = vch_ret.len() - 4;
    trace!(payload_len, total = vch_ret.len(), "decode_base_58check_raw: verifying checksum");
    let expected = checksum4_sha256d(&vch_ret[..payload_len]);

    if vch_ret[payload_len..] != expected {
        error!(
            found = %hex::encode(&vch_ret[payload_len..]),
            expected = %hex::encode(expected),
            "decode_base_58check_raw: checksum mismatch"
        );
        vch_ret.clear();
        return false;
    }

    vch_ret.truncate(payload_len);
    info!(payload_len, "decode_base_58check_raw: success");
    true

}

/// Decode a base58-encoded string (str) that includes a checksum into a byte
/// vector (vchRet). Return `true` if decoding is successful.
pub fn decode_base_58check(input: &str, vch_ret: &mut Vec<u8>, max_ret_len: usize) -> bool {
    debug!(input_len = input.len(), max_ret_len, "decode_base_58check: starting");
    if max_ret_len > usize::MAX - 4 {
        warn!("decode_base_58check: max_ret_len overflow guard triggered");
        return false;
    }
    if !decode_base58(input, vch_ret, max_ret_len + 4) {
        warn!("decode_base_58check: underlying Base‑58 decode failed");
        vch_ret.clear();
        return false;
    }
    if vch_ret.len() < 4 {
        warn!(decoded_len = vch_ret.len(), "decode_base_58check: insufficient length for checksum");
        vch_ret.clear();
        return false;
    }

    let payload_len = vch_ret.len() - 4;
    trace!(payload_len, total = vch_ret.len(), "decode_base_58check: verifying checksum");
    let expected = checksum4_sha256d(&vch_ret[..payload_len]);

    if vch_ret[payload_len..] != expected {
        error!(
            found = %hex::encode(&vch_ret[payload_len..]),
            expected = %hex::encode(expected),
            "decode_base_58check: checksum mismatch"
        );
        vch_ret.clear();
        return false;
    }

    vch_ret.truncate(payload_len);
    info!(payload_len, "decode_base_58check: success");
    true

}

#[cfg(test)]
mod decode_check_spec {
    use super::*;

    /// Round‑trip using **Base‑58‑check** with random payloads.
    #[traced_test]
    fn base58check_roundtrip_randomised() {
        // Simple LCG for deterministic pseudo‑random bytes
        let mut state = 0x1234_5678_9ABC_DEF0u64;
        for case in 0..128 {
            state = state.wrapping_mul(6364136223846793005u64).wrapping_add(1);
            let len = (state & 0x1F) as usize + 1; // 1‑32 bytes
            let mut data = (0..len).map(|i| (state >> ((i * 7) % 56)) as u8).collect::<Vec<_>>();

            let encoded = encode_base_58check(&data);
            trace!(case, len, ?encoded, "encoded random payload");

            let mut decoded = Vec::new();
            assert!(
                decode_base_58check(&encoded, &mut decoded, ((len + 4) as i32).try_into().unwrap()),
                "case {case}: decode failed"
            );
            assert_eq!(decoded, data, "case {case}: round‑trip mismatch");
        }
    }

    /// Corruption in **checksum** must be detected.
    #[traced_test]
    fn detects_bad_checksum() {
        let payload = b"bitcoin";
        let mut encoded = encode_base_58check(payload);
        // Flip one character that is guaranteed to stay in the alphabet.
        encoded.pop();
        encoded.push('2'); // replace final char
        let mut sink = Vec::new();
        assert!(
            !decode_base_58check(&encoded, &mut sink, 128),
            "corrupted checksum was not detected"
        );
    }
}
