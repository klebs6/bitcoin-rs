// ---------------- [ File: bitcoin-string/src/decode.rs ]
crate::ix!();

/// Map a Base32 character to its 5‑bit value.
#[inline]
fn decode_base32_val(b: u8) -> Option<u8> {
    match b {
        b'a'..=b'z' => Some(b - b'a'),
        b'A'..=b'Z' => Some(b - b'A'),
        b'2'..=b'7' => Some(b - b'2' + 26),
        _ => None,
    }
}

/// Decode Base32 into raw bytes (RFC 4648); works on NUL‑terminated C strings.
///
/// This is the low‑level worker used by the higher‑level helpers. It
/// mirrors the upstream behaviour: it accepts padded input, treats any
/// non‑alphabet character as the start of the padding run, and enforces
/// the standard length/padding rules.
pub fn decode_base32_bytes(
    p: *const u8,
    pf_invalid: Option<*mut bool>,
) -> Vec<u8> {
    trace!("decode_base32_bytes: start");

    // SAFETY: caller guarantees `p` is a valid C‑string or NULL.
    let bytes = unsafe {
        if p.is_null() {
            if let Some(ptr) = pf_invalid {
                *ptr = true;
            }
            return Vec::new();
        }
        CStr::from_ptr(p as *const std::os::raw::c_char).to_bytes()
    };

    /* first segment: collect 5‑bit symbols */
    let mut symbols = Vec::<u8>::with_capacity(bytes.len());
    let mut idx = 0;
    for &b in bytes {
        match decode_base32_val(b) {
            Some(v) => {
                symbols.push(v);
                idx += 1;
            }
            None => break,
        }
    }

    /* 5 → 8 */
    let mut ret = Vec::<u8>::with_capacity((symbols.len() * 5) / 8);
    let mut valid =
        convert_bits::<5, 8, false, _, _>(symbols.into_iter(), |c| ret.push(c));

    /* padding check */
    let pad_slice = &bytes[idx..];
    for &b in pad_slice {
        if b != b'=' {
            valid = false;
            break;
        }
    }
    valid &= bytes.len() % 8 == 0 && pad_slice.len() < 8;

    if let Some(ptr) = pf_invalid {
        unsafe { *ptr = !valid };
    }
    debug!("decode_base32_bytes: valid={valid}");
    ret
}

/// Decode a Rust `&str` that is padded RFC4648 Base32 into raw bytes.
///
/// This is a safer, byte‑oriented alternative that does **not** attempt
/// UTF‑8 decoding; callers that need text can interpret the bytes
/// themselves. It preserves and reports validity via `pf_invalid`.
pub fn decode_base32_bytes_strict(
    s: &str,
    pf_invalid: Option<*mut bool>,
) -> Vec<u8> {
    trace!("decode_base32_bytes_strict: start");

    // Reject interior NUL up‑front to mirror the C‑string semantics.
    let embedded_nul = s.as_bytes().contains(&0);

    // Prepare NUL‑terminated copy and call the C‑string worker.
    let mut buf: Vec<u8> = Vec::with_capacity(s.len() + 1);
    buf.extend_from_slice(s.as_bytes());
    buf.push(0);

    let mut low_invalid = false;
    let raw = decode_base32_bytes(buf.as_ptr(), Some(&mut low_invalid as *mut bool));

    let invalid = embedded_nul || low_invalid;
    if let Some(ptr) = pf_invalid {
        unsafe { *ptr = invalid };
    }

    debug!(len = raw.len(), invalid, "decode_base32_bytes_strict: done");
    raw
}

/// Variant used by Tor/I2P code paths: lower‑case alphabet, no '=', and
/// non‑zero tail bits rejected. This mirrors the custom decoders we had
/// in `set_tor` and `set_i2p`, but centralises the behaviour here.
///
/// Input is a Rust `&str` and we operate directly on raw bytes without
/// any UTF‑8 post‑processing.
pub fn decode_base32_bytes_nopad_lower(
    s: &str,
    pf_invalid: Option<*mut bool>,
) -> Vec<u8> {
    trace!(len = s.len(), "decode_base32_bytes_nopad_lower: start");

    let mut acc: u32 = 0;
    let mut bits: u32 = 0;
    let mut out: Vec<u8> = Vec::with_capacity((s.len() * 5 + 7) / 8);
    let mut invalid = false;

    for (i, ch) in s.chars().enumerate() {
        let c = ch.to_ascii_lowercase() as u8;
        let val = match c {
            b'a'..=b'z' => (c - b'a') as u32,
            b'2'..=b'7' => (c - b'2') as u32 + 26,
            _ => {
                invalid = true;
                warn!(index = i, ch = c as u32, "Invalid base32 character (nopad-lower)");
                break;
            }
        };

        acc = (acc << 5) | val;
        bits += 5;

        while bits >= 8 {
            let byte = ((acc >> (bits - 8)) & 0xFF) as u8;
            out.push(byte);
            bits -= 8;
        }
    }

    if !invalid && bits > 0 {
        // For unpadded base32, leftover bits must be zero.
        let mask = (1u32 << bits) - 1;
        if (acc & mask) != 0 {
            invalid = true;
            warn!(remaining_bits = bits, "Non‑zero leftover bits in base32 tail (nopad-lower)");
        }
    }

    if let Some(ptr) = pf_invalid {
        unsafe { *ptr = invalid };
    }

    if invalid {
        out.clear();
    }

    debug!(decoded_len = out.len(), invalid, "decode_base32_bytes_nopad_lower: done");
    out
}

/// Decode padded Base32 into a UTF‑8 string.
///
/// This is the legacy string‑oriented API which keeps its semantics:
/// it decodes to bytes and then attempts `String::from_utf8`, returning
/// the empty string on failure. For binary payloads (like Tor/I2P) the
/// byte‑level helpers above should be preferred.
pub fn decode_base32(
    s: &str,
    pf_invalid: Option<*mut bool>,
) -> String {
    trace!("decode_base32: start");

    // Detect interior NULs up‑front so we can mirror the upstream
    // behaviour that treats them as a hard failure.
    let embedded_nul = s.as_bytes().contains(&0);

    /* prepare NUL‑terminated copy */
    let mut buf: Vec<u8> = Vec::with_capacity(s.len() + 1);
    buf.extend_from_slice(s.as_bytes());
    buf.push(0);

    /* run the core decoder while capturing its validity flag */
    let mut local_invalid = false;
    let raw = decode_base32_bytes(
        buf.as_ptr(),
        Some(&mut local_invalid as *mut bool),
    );

    /* combine flags: either the low‑level decoder found an error
       *or* we discovered interior NUL bytes in the Rust slice       */
    let invalid = embedded_nul || local_invalid;
    if let Some(ptr) = pf_invalid {
        // SAFETY: caller opted‑in by supplying a pointer.
        unsafe { *ptr = invalid };
    }

    String::from_utf8(raw).unwrap_or_default()
}

#[cfg(test)]
mod tests_base32_64_decoding {
    use super::*;

    /// Exhaustive round‑trip Base32 for short payload lengths (0‑7 bytes)
    /// to exercise every padding branch.
    #[traced_test]
    fn base32_roundtrip_padding_edges() {
        for len in 0u8..=7 {
            let data: Vec<u8> = (0..len).collect();
            let encoded = crate::encode::encode_base32_bytes(&data, Some(true));
            let decoded = decode_base32(&encoded, None);
            assert_eq!(decoded.as_bytes(), data.as_slice(), "len = {len}");
        }
        info!("Base32 round‑trip over padding edges verified");
    }

    /// Exhaustive round‑trip Base64 for all 3‑byte alignment cases (0‑2 residual bytes).
    #[traced_test]
    fn base64_roundtrip_padding_edges() {
        for len in 0u8..=2 {
            let data: Vec<u8> = (0..len).collect();
            let encoded = crate::encode::encode_base64_bytes(&data);
            let decoded = decode_base64(&encoded, None);
            assert_eq!(decoded.as_bytes(), data.as_slice(), "len = {len}");
        }
        info!("Base64 round‑trip over padding edges verified");
    }

    /// Verify `pf_invalid` flag is set when illegal characters appear.
    #[traced_test]
    fn invalid_base32_sets_flag() {
        let mut invalid = false;
        let _ = decode_base32("#$%^", Some(&mut invalid as *mut bool));
        assert!(invalid, "invalid flag must be set");
    }

    /// Verify `pf_invalid` flag is set when illegal padding appears.
    #[traced_test]
    fn invalid_base64_sets_flag() {
        let mut invalid = false;
        let _ = decode_base64("A===", Some(&mut invalid));
        assert!(invalid, "invalid flag must be set");
    }

    #[traced_test]
    fn nopad_lower_decoder_matches_encode_helper() {
        // Check that encode_base32_bytes(..., pad=false) round‑trips through
        // decode_base32_bytes_nopad_lower for some random-ish payload sizes.
        for len in [0usize, 1, 2, 5, 10, 31, 32, 35].iter().copied() {
            let data: Vec<u8> = (0..len as u8).map(|v| v ^ 0xA5).collect();
            let encoded = crate::encode::encode_base32_bytes(&data, Some(false));
            let mut invalid = false;
            let decoded = decode_base32_bytes_nopad_lower(&encoded, Some(&mut invalid as *mut bool));
            assert!(!invalid, "nopad-lower decode should be valid for len={len}");
            assert_eq!(decoded, data);
        }
    }
}
