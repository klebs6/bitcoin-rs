// ---------------- [ File: bitcoin-string/src/encode.rs ]
crate::ix!();

/// Encode bytes to Base64 (RFC 4648, with `=` padding).
pub fn encode_base64_bytes(input: &[u8]) -> String {
    const TABLE: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    trace!("encode_base64_bytes: len={}", input.len());

    let mut out = String::with_capacity(((input.len() + 2) / 3) * 4);
    convert_bits::<8, 6, true, _, _>(input.iter().copied(), |v| {
        out.push(TABLE[v as usize] as char)
    });
    while out.len() % 4 != 0 {
        out.push('=');
    }
    out
}

/// Encode a UTF‑8 string to Base64.
pub fn encode_base64(s: &String) -> String {
    encode_base64_bytes(s.as_bytes())
}

/// Encode bytes to Base32 (lower‑case RFC 4648 alphabet).
///
/// When `pad` is `true`, output is padded with `=` to a multiple of 8.
pub fn encode_base32_bytes(input: &[u8], pad: Option<bool>) -> String {
    const TABLE: &[u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";
    let pad = pad.unwrap_or(true);
    trace!(
        "encode_base32_bytes: len={}, pad={}",
        input.len(),
        pad
    );

    let mut out = String::with_capacity(((input.len() + 4) / 5) * 8);
    convert_bits::<8, 5, true, _, _>(input.iter().copied(), |v| {
        out.push(TABLE[v as usize] as char)
    });
    if pad {
        while out.len() % 8 != 0 {
            out.push('=');
        }
    }
    out
}

/// Base32 encode.
/// 
/// If `pad` is true, then the output will be padded with '=' so that its length is a multiple of 8.
///
/// Encode arbitrary bytes to Base32.
pub fn encode_base32(input: &[u8], pad: Option<bool>) -> String {
    encode_base32_bytes(input, pad)
}

#[cfg(test)]
mod tests_base32_64_encoding {
    use super::*;
    use tracing::debug;

    /// Full ASCII payload round‑trip through Base64.
    #[traced_test]
    fn ascii_roundtrip_base64() {
        let ascii: Vec<u8> = (0x20u8..=0x7Eu8).collect();
        let encoded = encode_base64_bytes(&ascii);
        let decoded = crate::decode_base64(&encoded, None);   // <-- corrected path
        assert_eq!(decoded.as_bytes(), ascii.as_slice());
        debug!("ASCII Base64 round‑trip OK");
    }

    /// Full ASCII payload round‑trip through Base32.
    #[traced_test]
    fn ascii_roundtrip_base32() {
        let ascii: Vec<u8> = (0x20u8..=0x7Eu8).collect();
        let encoded = encode_base32_bytes(&ascii, Some(true));
        let decoded = crate::decode::decode_base32(&encoded, None);
        assert_eq!(decoded.as_bytes(), ascii.as_slice());
        debug!("ASCII Base32 round‑trip OK");
    }
}
