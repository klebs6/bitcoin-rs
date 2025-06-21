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
}
