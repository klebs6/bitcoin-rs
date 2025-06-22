// ---------------- [ File: bitcoin-univalue/src/get_json_token.rs ]
crate::ix!();

/// Low‑level JSON lexer used by `UniValue::read`.
///
/// The algorithm is a direct, line‑for‑line translation of
/// the upstream C++ with an emphasis on safety:
///
/// * All pointer arithmetic is wrapped in `unsafe` blocks and
///   gated by explicit bounds checks.
/// * Internal helpers mirror the original three‑part
///   *number* parsing, keyword scanning, and string decoding.
///
/// On success, `token_val` is populated (where applicable),
/// `consumed` contains the number of bytes read from *raw*,
/// and the corresponding `JTokenType` is returned.
#[instrument(level = "trace", skip_all)]
pub fn get_json_token(
    token_val: &mut String,
    consumed: &mut u32,
    raw: *const u8,
    end: *const u8,
) -> JTokenType {
    unsafe {
        token_val.clear();
        *consumed = 0;

        let mut p = raw;

        // ---------- skip leading whitespace ----------
        while p < end && json_isspace(*p as i32) {
            p = p.add(1);
        }

        if p >= end {
            return JTokenType::JTOK_NONE;
        }

        // Save the *start* for the final `consumed` calc.
        let raw_start = p;

        // ---------- structural single‑byte tokens ----------
        match *p as char {
            '{' => return single_byte_token(&mut p, raw_start, consumed, JTokenType::JTOK_OBJ_OPEN),
            '}' => return single_byte_token(&mut p, raw_start, consumed, JTokenType::JTOK_OBJ_CLOSE),
            '[' => return single_byte_token(&mut p, raw_start, consumed, JTokenType::JTOK_ARR_OPEN),
            ']' => return single_byte_token(&mut p, raw_start, consumed, JTokenType::JTOK_ARR_CLOSE),
            ':' => return single_byte_token(&mut p, raw_start, consumed, JTokenType::JTOK_COLON),
            ',' => return single_byte_token(&mut p, raw_start, consumed, JTokenType::JTOK_COMMA),
            _ => { /* fallthrough */ }
        }

        // ---------- literals: null / true / false ----------
        if p.add(4) <= end {
            let lit4 = std::slice::from_raw_parts(p, 4);
            if lit4 == b"null" {
                p = p.add(4);
                *consumed = (p as usize - raw_start as usize) as u32;
                return JTokenType::JTOK_KW_NULL;
            }
            if lit4 == b"true" {
                p = p.add(4);
                *consumed = (p as usize - raw_start as usize) as u32;
                return JTokenType::JTOK_KW_TRUE;
            }
        }
        if p.add(5) <= end {
            let lit5 = std::slice::from_raw_parts(p, 5);
            if lit5 == b"false" {
                p = p.add(5);
                *consumed = (p as usize - raw_start as usize) as u32;
                return JTokenType::JTOK_KW_FALSE;
            }
        }

        // ---------- number ----------
        if matches!(*p as char, '-' | '0'..='9') {
            let mut num_str = String::new();
            let first = *p as char;
            num_str.push(first);
            p = p.add(1);

            // Reject "-" not followed by digit.
            if first == '-' && (p >= end || !json_isdigit(*p as i32)) {
                return JTokenType::JTOK_ERR;
            }

            // Integer part.
            while p < end && json_isdigit(*p as i32) {
                num_str.push(*p as char);
                p = p.add(1);
            }

            // Fraction.
            if p < end && *p as char == '.' {
                num_str.push('.');
                p = p.add(1);
                if p >= end || !json_isdigit(*p as i32) {
                    return JTokenType::JTOK_ERR;
                }
                while p < end && json_isdigit(*p as i32) {
                    num_str.push(*p as char);
                    p = p.add(1);
                }
            }

            // Exponent.
            if p < end && matches!(*p as char, 'e' | 'E') {
                num_str.push(*p as char);
                p = p.add(1);
                if p < end && matches!(*p as char, '+' | '-') {
                    num_str.push(*p as char);
                    p = p.add(1);
                }
                if p >= end || !json_isdigit(*p as i32) {
                    return JTokenType::JTOK_ERR;
                }
                while p < end && json_isdigit(*p as i32) {
                    num_str.push(*p as char);
                    p = p.add(1);
                }
            }

            *token_val = num_str;
            *consumed = (p as usize - raw_start as usize) as u32;
            return JTokenType::JTOK_NUMBER;
        }

        // ---------- string ----------
        if *p as char == '"' {
            p = p.add(1); // skip opening quote
            let mut val_str = String::new();

            while p < end {
                let ch = *p;
                if ch < 0x20 {
                    return JTokenType::JTOK_ERR; // control char
                }
                if ch == b'"' {
                    p = p.add(1); // skip closing quote
                    break;
                }
                if ch == b'\\' {
                    p = p.add(1);
                    if p >= end {
                        return JTokenType::JTOK_ERR;
                    }
                    match *p as char {
                        '"' => val_str.push('"'),
                        '\\' => val_str.push('\\'),
                        '/' => val_str.push('/'),
                        'b' => val_str.push('\u{0008}'),
                        'f' => val_str.push('\u{000C}'),
                        'n' => val_str.push('\n'),
                        'r' => val_str.push('\r'),
                        't' => val_str.push('\t'),
                        'u' => {
                            // four hexadecimal digits
                            if p.add(5) > end {
                                return JTokenType::JTOK_ERR;
                            }
                            let mut cp: u32 = 0;
                            let next = hatoui(p.add(1), p.add(5), &mut cp);
                            if next != p.add(5) {
                                return JTokenType::JTOK_ERR;
                            }
                            if let Some(c) = char::from_u32(cp) {
                                val_str.push(c);
                            } else {
                                return JTokenType::JTOK_ERR;
                            }
                            p = p.add(4); // hatoui already advanced 0, we later p +=1
                        }
                        _ => return JTokenType::JTOK_ERR,
                    }
                    p = p.add(1);
                    continue;
                }
                val_str.push(ch as char);
                p = p.add(1);
            }

            *token_val = val_str;
            *consumed = (p as usize - raw_start as usize) as u32;
            return JTokenType::JTOK_STRING;
        }

        // ---------- fallback ----------
        JTokenType::JTOK_ERR
    }
}

/// Helper for single‑byte structural tokens.
unsafe fn single_byte_token(
    p: &mut *const u8,
    start: *const u8,
    consumed: &mut u32,
    kind: JTokenType,
) -> JTokenType {
    *p = p.add(1);
    *consumed = (*p as usize - start as usize) as u32;
    kind
}

#[cfg(test)]
mod get_json_token_spec {
    use super::*;

    fn run(src: &[u8]) -> (JTokenType, String, u32) {
        let mut val = String::new();
        let mut n = 0u32;
        let tok =
            get_json_token(&mut val, &mut n, src.as_ptr(), unsafe { src.as_ptr().add(src.len()) });
        (tok, val, n)
    }

    #[traced_test]
    fn structural_tokens() {
        let cases = [
            (b"{", JTokenType::JTOK_OBJ_OPEN),
            (b"}", JTokenType::JTOK_OBJ_CLOSE),
            (b"[", JTokenType::JTOK_ARR_OPEN),
            (b"]", JTokenType::JTOK_ARR_CLOSE),
            (b":", JTokenType::JTOK_COLON),
            (b",", JTokenType::JTOK_COMMA),
        ];
        for (src, kind) in cases {
            let (tok, v, n) = run(src);
            assert_eq!(tok, kind);
            assert!(v.is_empty());
            assert_eq!(n, 1);
        }
    }

    #[traced_test]
    fn keywords() {
        let (tok, _, _) = run(b"null");
        assert_eq!(tok, JTokenType::JTOK_KW_NULL);
        let (tok, _, _) = run(b"true");
        assert_eq!(tok, JTokenType::JTOK_KW_TRUE);
        let (tok, _, _) = run(b"false");
        assert_eq!(tok, JTokenType::JTOK_KW_FALSE);
    }

    #[traced_test]
    fn number_token() {
        let (tok, v, _) = run(b"-12.34e+2");
        assert_eq!(tok, JTokenType::JTOK_NUMBER);
        assert_eq!(v, "-12.34e+2");
    }

    #[traced_test]
    fn string_token_simple() {
        let (tok, v, _) = run(br#""hello""#.as_bytes());
        assert_eq!(tok, JTokenType::JTOK_STRING);
        assert_eq!(v, "hello");
    }

    #[traced_test]
    fn string_token_with_escapes() {
        let (tok, v, _) = run(br#""\"\b\n""#.as_bytes());
        assert_eq!(tok, JTokenType::JTOK_STRING);
        assert_eq!(v, "\"\u{0008}\n");
    }

    #[traced_test]
    fn whitespace_and_consumed_count() {
        let (tok, _, n) = run(b" \t\n\r true");
        assert_eq!(tok, JTokenType::JTOK_KW_TRUE);
        assert_eq!(n, 6); // four ws + 't','r','u','e'
    }

    #[traced_test]
    fn invalid_input() {
        let (tok, _, _) = run(b"-");
        assert_eq!(tok, JTokenType::JTOK_ERR);
    }
}
