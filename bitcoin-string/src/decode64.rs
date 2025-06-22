// ---------------- [ File: bitcoin-string/src/decode64.rs ]
crate::ix!();

/// Map a Base64 character to its 6‑bit value.
#[inline]
fn decode_base64_val(b: u8) -> Option<u8> {
    match b {
        b'A'..=b'Z' => Some(b - b'A'),
        b'a'..=b'z' => Some(b - b'a' + 26),
        b'0'..=b'9' => Some(b - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

/// Decode Base64 into raw bytes (RFC 4648); works on NUL‑terminated C strings.
pub fn decode_base64_bytes(
    p: *const u8,
    pf_invalid: Option<*mut bool>,
) -> Vec<u8> {
    trace!("decode_base64_bytes: start");

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

    let mut symbols = Vec::<u8>::with_capacity(bytes.len());
    let mut idx = 0;
    for &b in bytes {
        match decode_base64_val(b) {
            Some(v) => {
                symbols.push(v);
                idx += 1;
            }
            None => break,
        }
    }

    let mut ret = Vec::<u8>::with_capacity((symbols.len() * 3) / 4);
    let mut valid =
        convert_bits::<6, 8, false, _, _>(symbols.into_iter(), |c| ret.push(c));

    /* padding check */
    let pad_slice = &bytes[idx..];
    for &b in pad_slice {
        if b != b'=' {
            valid = false;
            break;
        }
    }
    valid &= bytes.len() % 4 == 0 && pad_slice.len() < 4;

    if let Some(ptr) = pf_invalid {
        unsafe { *ptr = !valid };
    }
    debug!("decode_base64_bytes: valid={valid}");
    ret
}

/// Decode Base64 returning a UTF‑8 `String`.
pub fn decode_base64(
    s: &str,
    pf_invalid: Option<&mut bool>,
) -> String {
    trace!("decode_base64: start");

    let embedded_nul = s.as_bytes().contains(&0);

    /* prepare NUL‑terminated copy */
    let mut buf: Vec<u8> = Vec::with_capacity(s.len() + 1);
    buf.extend_from_slice(s.as_bytes());
    buf.push(0);

    /* run the core decoder */
    let mut local_invalid = false;
    let raw = decode_base64_bytes(
        buf.as_ptr(),
        Some(&mut local_invalid as *mut bool),
    );

    let invalid = embedded_nul || local_invalid;
    if let Some(flag) = pf_invalid {
        *flag = invalid;
    }

    String::from_utf8(raw).unwrap_or_default()
}
