// ---------------- [ File: bitcoin-string/src/hex.rs ]
crate::ix!();

pub const util_hexdigit: [i8; 256] = [ 
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    0,1,2,3,4,5,6,7,8,9,-1,-1,-1,-1,-1,-1,
    -1,0xa,0xb,0xc,0xd,0xe,0xf,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,0xa,0xb,0xc,0xd,0xe,0xf,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 
];

#[inline]
pub fn hex_digit(c: u8) -> i8 {
    util_hexdigit[c as usize]
}

/// Returns `true` iff every char is hex and the length is even.
pub fn is_hex(s: &str) -> bool {
    if s.is_empty() || s.len() % 2 != 0 {
        return false;
    }
    s.bytes().all(|b| hex_digit(b) >= 0)
}

/// Returns `true` iff `s` is a (possibly “0x”‑prefixed) hex literal.
pub fn is_hex_number(mut s: &str) -> bool {
    if s.starts_with("0x") || s.starts_with("0X") {
        s = &s[2..];
    }
    !s.is_empty() && s.bytes().all(|b| hex_digit(b) >= 0)
}

/// Parse a hexadecimal dump (ignores ASCII whitespace).
pub fn parse_hex(psz: &str) -> Vec<u8> {
    use crate::check::is_space;

    let mut out = Vec::<u8>::with_capacity(psz.len() / 2);
    let bytes = psz.as_bytes();
    let mut idx = 0;

    while idx < bytes.len() {
        /* skip whitespace */
        while idx < bytes.len() && is_space(bytes[idx]) {
            idx += 1;
        }
        if idx + 2 > bytes.len() {
            break;
        }
        let high = hex_digit(bytes[idx]);
        let low = hex_digit(bytes[idx + 1]);
        if high < 0 || low < 0 {
            break;
        }
        out.push(((high as u8) << 4) | (low as u8));
        idx += 2;
    }
    trace!("parse_hex: parsed {} bytes", out.len());
    out
}

/// Lower‑case hex string from raw bytes.
pub fn hex_str(data: &[u8]) -> String {
    const HEXMAP: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(data.len() * 2);
    for &b in data {
        out.push(HEXMAP[(b >> 4) as usize] as char);
        out.push(HEXMAP[(b & 0xF) as usize] as char);
    }
    out
}

#[cfg(test)]
mod tests_hex_utils {
    use super::*;

    #[traced_test]
    fn digit_mapping_roundtrip() {
        for b in 0u8..=255 {
            let d = hex_digit(b);
            if d >= 0 {
                assert_eq!(
                    hex_digit((d as u8) + if d < 10 { b'0' } else { b'a' - 10 }),
                    d
                );
            }
        }
    }

    #[traced_test]
    fn is_hex_and_number_checks() {
        assert!(is_hex("deadBEEF00"));
        assert!(!is_hex("xyz"));

        assert!(is_hex_number("0xdead"));
        assert!(is_hex_number("DEAD"));
        assert!(!is_hex_number("0x"));
        assert!(!is_hex_number("g0"));
    }

    #[traced_test]
    fn parse_and_print_roundtrip() {
        let src = "0123456789abcdef";
        let bytes = parse_hex(src);
        assert_eq!(hex_str(&bytes), src);
    }

    #[traced_test]
    fn ignores_ascii_whitespace() {
        let txt = "  0a 0b\n0c\r0d\t0e";
        let bytes = parse_hex(txt);
        assert_eq!(bytes, [0x0A, 0x0B, 0x0C, 0x0D, 0x0E]);
    }
}
