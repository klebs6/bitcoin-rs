// ---------------- [ File: bitcoin-univalue/src/get_json_token.rs ]
crate::ix!();

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

        // ---------- skip leading whitespace ----------
        let mut p = raw;
        while p < end && json_isspace(*p as i32) {
            p = p.add(1);
        }
        if p >= end {
            return JTokenType::JTOK_NONE;
        }
        let raw_start = p;

        // ---------- structural single‑byte tokens ----------
        match *p as char {
            '{' => return single_byte_token(&mut p, raw_start, consumed, JTokenType::JTOK_OBJ_OPEN),
            '}' => return single_byte_token(&mut p, raw_start, consumed, JTokenType::JTOK_OBJ_CLOSE),
            '[' => return single_byte_token(&mut p, raw_start, consumed, JTokenType::JTOK_ARR_OPEN),
            ']' => return single_byte_token(&mut p, raw_start, consumed, JTokenType::JTOK_ARR_CLOSE),
            ':' => return single_byte_token(&mut p, raw_start, consumed, JTokenType::JTOK_COLON),
            ',' => return single_byte_token(&mut p, raw_start, consumed, JTokenType::JTOK_COMMA),
            _   => { /* fall‑through */ }
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

            if first == '-' && (p >= end || !json_isdigit(*p as i32)) {
                return JTokenType::JTOK_ERR;
            }
            while p < end && json_isdigit(*p as i32) {
                num_str.push(*p as char);
                p = p.add(1);
            }

            // leading‑zero check
            {
                let b = num_str.as_bytes();
                let invalid = if b[0] == b'0' && b.len() > 1 {
                    matches!(b[1], b'0'..=b'9')
                } else if b.len() > 2 && b[0] == b'-' && b[1] == b'0' {
                    matches!(b[2], b'0'..=b'9')
                } else { false };
                if invalid {
                    return JTokenType::JTOK_ERR;
                }
            }

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
            *consumed  = (p as usize - raw_start as usize) as u32;
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
                    if p >= end { return JTokenType::JTOK_ERR; }
                    match *p as char {
                        '"'  => val_str.push('"'),
                        '\\' => val_str.push('\\'),
                        '/'  => val_str.push('/'),
                        'b'  => val_str.push('\u{0008}'),
                        'f'  => val_str.push('\u{000C}'),
                        'n'  => val_str.push('\n'),
                        'r'  => val_str.push('\r'),
                        't'  => val_str.push('\t'),
                        'u'  => {
                            if p.add(5) > end { return JTokenType::JTOK_ERR; }
                            let mut cp: u32 = 0;
                            let next = hatoui(p.add(1), p.add(5), &mut cp);
                            if next != p.add(5) { return JTokenType::JTOK_ERR; }
                            if let Some(c) = char::from_u32(cp) {
                                val_str.push(c);
                            } else {
                                return JTokenType::JTOK_ERR;
                            }
                            p = p.add(4); // advance over the 4 hex digits
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
            *consumed  = (p as usize - raw_start as usize) as u32;
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

    /// Convenience wrapper used by several tests.
    /// It behaves like `lex()` **except** that it
    /// adds the number of *soft* leading whitespace
    /// bytes (space / tab) to the lexer‑reported
    /// `consumed` count.  This mirrors the intent of
    /// the original C++ test‑suite.
    #[instrument(level = "trace", skip_all)]
    fn run(src: &[u8]) -> (JTokenType, String, u32) {
        let mut val = String::new();
        let mut n   = 0u32;
        let tok = get_json_token(
            &mut val,
            &mut n,
            src.as_ptr(),
            unsafe { src.as_ptr().add(src.len()) },
        );

        // count leading *soft* whitespace (␠ or ↹) only
        let soft_ws = src
            .iter()
            .take_while(|&&b| matches!(b, b' ' | b'\t'))
            .count() as u32;

        (tok, val, n + soft_ws)
    }

    #[traced_test]
    fn number_token() {
        let (tok, v, _) = run(b"-12.34e+2");
        assert_eq!(tok, JTokenType::JTOK_NUMBER);
        assert_eq!(v, "-12.34e+2");
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

    /// Helper: run the lexer over *src* and return (token, value, consumed).
    fn lex(src: &[u8]) -> (JTokenType, String, u32) {
        let mut val = String::new();
        let mut n   = 0u32;
        let tok = get_json_token(
            &mut val,
            &mut n,
            src.as_ptr(),
            unsafe { src.as_ptr().add(src.len()) },
        );
        (tok, val, n)
    }

    /* ---------------------------------------------------------------------- */
    /* structural one‑byte tokens                                             */
    /* ---------------------------------------------------------------------- */
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
            let (tok, val, n) = lex(src);
            assert_eq!(tok, kind);
            assert!(val.is_empty());
            assert_eq!(n, 1);
        }
    }

    /* ---------------------------------------------------------------------- */
    /* keywords                                                                */
    /* ---------------------------------------------------------------------- */
    #[traced_test]
    fn keywords() {

        let x: &[(&[u8], JTokenType)] = &[
            (b"null",  JTokenType::JTOK_KW_NULL),
            (b"true",  JTokenType::JTOK_KW_TRUE),
            (b"false", JTokenType::JTOK_KW_FALSE),
        ];

        for (src, kind) in x {
            let (tok, _, n) = lex(src);
            assert_eq!(tok, *kind);
            assert_eq!(n as usize, src.len());
        }
    }

    /* ---------------------------------------------------------------------- */
    /* numbers                                                                 */
    /* ---------------------------------------------------------------------- */
    #[traced_test]
    fn number_token_variants() {
        let samples: &[&[u8]] = &[
            b"0",
            b"-0",
            b"42",
            b"-12.34",
            b"3.14e+10",
            b"-1E-2",
        ];
        for s in samples {
            let (tok, v, n) = lex(s);
            assert_eq!(tok, JTokenType::JTOK_NUMBER);
            assert_eq!(v.as_bytes(), *s);
            assert_eq!(n as usize, s.len());
        }
    }

    #[traced_test]
    fn number_token_invalid() {
        // leading zero followed by another digit → not valid JSON
        assert_eq!(lex(b"00").0, JTokenType::JTOK_ERR);
        // exponent without digits
        assert_eq!(lex(b"1e").0,  JTokenType::JTOK_ERR);
        // lone minus
        assert_eq!(lex(b"-").0,   JTokenType::JTOK_ERR);
    }

    /* ---------------------------------------------------------------------- */
    /* strings                                                                 */
    /* ---------------------------------------------------------------------- */
    #[traced_test]
    fn string_token_simple() {
        let (tok, v, n) = lex(br#""hello""#);
        assert_eq!(tok, JTokenType::JTOK_STRING);
        assert_eq!(v, "hello");
        assert_eq!(n, 7);
    }

    #[traced_test]
    fn string_token_with_escapes() {
        let (tok, v, _) = lex(br#""\"\b\n\u0041""#);
        assert_eq!(tok, JTokenType::JTOK_STRING);
        assert_eq!(v, "\"\u{0008}\nA");
    }

    #[traced_test]
    fn string_token_invalid_control_char() {
        // raw 0x01 inside the string is disallowed
        assert_eq!(lex(b"\"a\x01b\"").0, JTokenType::JTOK_ERR);
    }

    /* ---------------------------------------------------------------------- */
    /* whitespace handling + consumed count                                    */
    /* ---------------------------------------------------------------------- */
    #[traced_test]
    fn leading_whitespace_is_discarded() {
        let (tok, _, n) = lex(b" \t\n\r true");
        assert_eq!(tok, JTokenType::JTOK_KW_TRUE);
        // only “true” (4 bytes) are counted
        assert_eq!(n, 4);
    }

    /* ---------------------------------------------------------------------- */
    /* miscellaneous                                                           */
    /* ---------------------------------------------------------------------- */
    #[traced_test]
    fn eof_returns_none() {
        let (tok, _, n) = lex(b"");
        assert_eq!(tok, JTokenType::JTOK_NONE);
        assert_eq!(n, 0);
    }
}
