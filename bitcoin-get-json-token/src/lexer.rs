crate::ix!();

/// Try to lex a single‑byte structural token.  On success returns
/// `(token, ptr_after, consumed)`.
#[inline]
#[instrument(level = "trace", skip_all)]
pub unsafe fn lex_structural(
    p: *const u8,
    end: *const u8,
) -> Option<(JTokenType, *const u8, u32)> {
    if p >= end {
        return None;
    }
    let t = match *p as char {
        '{' => JTokenType::JTOK_OBJ_OPEN,
        '}' => JTokenType::JTOK_OBJ_CLOSE,
        '[' => JTokenType::JTOK_ARR_OPEN,
        ']' => JTokenType::JTOK_ARR_CLOSE,
        ':' => JTokenType::JTOK_COLON,
        ',' => JTokenType::JTOK_COMMA,
        _   => return None,
    };
    let after = p.add(1);
    Some((t, after, 1))
}

/// Try to lex one of the three JSON keywords (`null / true / false`).
#[inline]
#[instrument(level = "trace", skip_all)]
pub unsafe fn lex_keyword(
    p: *const u8,
    end: *const u8,
) -> Option<(JTokenType, *const u8, u32)> {
    if p.add(4) <= end {
        let lit4 = std::slice::from_raw_parts(p, 4);
        if lit4 == b"null" {
            return Some((
                JTokenType::JTOK_KW_NULL,
                p.add(4),
                4,
            ));
        }
        if lit4 == b"true" {
            return Some((
                JTokenType::JTOK_KW_TRUE,
                p.add(4),
                4,
            ));
        }
    }
    if p.add(5) <= end && std::slice::from_raw_parts(p, 5) == b"false" {
        return Some((
            JTokenType::JTOK_KW_FALSE,
            p.add(5),
            5,
        ));
    }
    None
}

/// Try to lex a JSON number.  Returns `(token, after, consumed, literal)`.
#[inline]
#[instrument(level = "trace", skip(token_val))]
pub unsafe fn lex_number(
    token_val: &mut String,
    p: *const u8,
    end: *const u8,
) -> Option<(JTokenType, *const u8, u32)> {
    use std::ptr::copy_nonoverlapping;

    if p >= end || !matches!(*p as char, '-' | '0'..='9') {
        return None;
    }
    let start = p;
    let mut cur = p;

    /* --------‑‑ first char already validated ‑‑-------- */
    cur = cur.add(1);

    /* --------‑‑ digits before dot ‑‑-------- */
    while cur < end && json_isdigit(*cur as i32) {
        cur = cur.add(1);
    }

    /* ---- leading‑zero check (same as previous impl) ---- */
    {
        let len = (cur as usize - start as usize) as usize;
        let slice = std::slice::from_raw_parts(start, len);
        if (slice[0] == b'0' && len > 1 && slice[1].is_ascii_digit())
            || (slice[0] == b'-' && len > 2 && slice[1] == b'0' && slice[2].is_ascii_digit())
        {
            return None;
        }
    }

    /* --------‑‑ fraction ‑‑-------- */
    if cur < end && *cur as char == '.' {
        cur = cur.add(1);
        if cur >= end || !json_isdigit(*cur as i32) {
            return None;
        }
        while cur < end && json_isdigit(*cur as i32) {
            cur = cur.add(1);
        }
    }

    /* --------‑‑ exponent ‑‑-------- */
    if cur < end && matches!(*cur as char, 'e' | 'E') {
        cur = cur.add(1);
        if cur < end && matches!(*cur as char, '+' | '-') {
            cur = cur.add(1);
        }
        if cur >= end || !json_isdigit(*cur as i32) {
            return None;
        }
        while cur < end && json_isdigit(*cur as i32) {
            cur = cur.add(1);
        }
    }

    /* --------‑‑ copy literal into token_val ‑‑-------- */
    let len = (cur as usize - start as usize) as usize;
    token_val.clear();
    token_val.reserve(len);
    token_val.as_mut_vec().resize(len, 0);
    copy_nonoverlapping(start, token_val.as_mut_ptr(), len);
    token_val.as_mut_vec().set_len(len);

    let consumed = bytes_consumed(start, cur);
    trace!(value = %token_val, consumed, "number token");
    Some((JTokenType::JTOK_NUMBER, cur, consumed))
}

/// Try to lex a JSON string.  Returns `(token, after, consumed, literal)`.
#[inline]
#[instrument(level = "trace", skip(token_val))]
pub unsafe fn lex_string(
    token_val: &mut String,
    p: *const u8,
    end: *const u8,
) -> Option<(JTokenType, *const u8, u32)> {
    if p >= end || *p as char != '"' {
        return None;
    }
    let start = p;
    let mut cur = p.add(1);            // after opening quote
    token_val.clear();

    while cur < end {
        let ch = *cur;
        if ch < 0x20 {
            return None;
        }
        if ch == b'"' {
            cur = cur.add(1);          // position after closing quote
            break;
        }
        if ch == b'\\' {
            /*  Re‑use existing escape handling by delegating to the
                original (now private) code – moved into its own helper
                for reuse.                                                */
            if let Some((parsed, after)) = parse_string_escape(cur, end) {
                token_val.push_str(&parsed);
                cur = after;
                continue;
            } else {
                return None;
            }
        }
        token_val.push(ch as char);
        cur = cur.add(1);
    }

    if cur > end {
        return None;                   // unterminated
    }

    let consumed = bytes_consumed(start, cur);
    trace!(value = %token_val, consumed, "string token");
    Some((JTokenType::JTOK_STRING, cur, consumed))
}

#[cfg(test)]
mod lexer_subroutine_spec {
    use super::*;

    /* -------------------------------------------------------------- */
    /* skip_ws_nul                                                    */
    /* -------------------------------------------------------------- */
    #[traced_test]
    fn skip_ws_nul_advances_past_all_ws_and_nul() {
        unsafe {
            let buf = b"\0 \t\n\rABC";
            let p0  = buf.as_ptr();
            let end = p0.add(buf.len());
            let p1  = skip_ws_nul(p0, end);
            assert_eq!(*p1 as char, 'A');
        }
    }

    /* -------------------------------------------------------------- */
    /* structural tokens                                              */
    /* -------------------------------------------------------------- */
    #[traced_test]
    fn structural_tokens_roundtrip() {
        unsafe {
            let cases: &[(&[u8],JTokenType)] = &[
                (b"{", JTokenType::JTOK_OBJ_OPEN),
                (b"}", JTokenType::JTOK_OBJ_CLOSE),
                (b"[", JTokenType::JTOK_ARR_OPEN),
                (b"]", JTokenType::JTOK_ARR_CLOSE),
                (b":", JTokenType::JTOK_COLON),
                (b",", JTokenType::JTOK_COMMA),
            ];
            for (buf, kind) in cases {
                let p   = buf.as_ptr();
                let end = p.add(buf.len());
                let (tok, after, n) = lex_structural(p, end).unwrap();
                assert_eq!(tok, *kind);
                assert_eq!(n, 1);
                assert_eq!(after, end);
            }
        }
    }

    /* -------------------------------------------------------------- */
    /* keyword tokens                                                 */
    /* -------------------------------------------------------------- */
    #[traced_test]
    fn keyword_tokens_roundtrip() {
        unsafe {
            let cases: &[(&[u8],JTokenType)] = &[
                (b"null",  JTokenType::JTOK_KW_NULL),
                (b"true",  JTokenType::JTOK_KW_TRUE),
                (b"false", JTokenType::JTOK_KW_FALSE),
            ];
            for (buf, kind) in cases {
                let p   = buf.as_ptr();
                let end = p.add(buf.len());
                let (tok, after, n) = lex_keyword(p, end).unwrap();
                assert_eq!(tok, *kind);
                assert_eq!(n as usize, buf.len());
                assert_eq!(after, end);
            }
            // negative
            assert!(lex_keyword(b"nul".as_ptr(), b"nul".as_ptr().add(3)).is_none());
        }
    }

    /* -------------------------------------------------------------- */
    /* number tokens                                                  */
    /* -------------------------------------------------------------- */
    #[traced_test]
    fn number_tokens_valid_and_invalid() {
        unsafe {
            let valid: &[&[u8]] = &[
                b"0", b"-0", b"42", b"-12.34", b"3.14e+10", b"-1E-2",
            ];
            for &src in valid {
                let mut val = String::new();
                let (tok, after, n) = lex_number(
                    &mut val,
                    src.as_ptr(),
                    src.as_ptr().add(src.len()),
                ).unwrap();
                assert_eq!(tok, JTokenType::JTOK_NUMBER);
                assert_eq!(val.as_bytes(), src);
                assert_eq!(n as usize, src.len());
                assert_eq!(after, src.as_ptr().add(src.len()));
            }
            let invalid: &[&[u8]] = &[b"00", b"-", b"1e", b"-01"];
            for &src in invalid {
                let mut val = String::new();
                assert!(lex_number(
                    &mut val,
                    src.as_ptr(),
                    src.as_ptr().add(src.len())
                ).is_none());
            }
        }
    }

    /* -------------------------------------------------------------- */
    /* string tokens                                                  */
    /* -------------------------------------------------------------- */
    #[traced_test]
    fn string_tokens_simple_and_escapes() {
        unsafe {
            // simple
            let mut val = String::new();
            let src = br#""hello""#;
            let (tok, after, n) = lex_string(
                &mut val,
                src.as_ptr(),
                src.as_ptr().add(src.len()),
            ).unwrap();
            assert_eq!(tok, JTokenType::JTOK_STRING);
            assert_eq!(val, "hello");
            assert_eq!(n, 7);
            assert_eq!(after, src.as_ptr().add(src.len()));

            // escapes
            let esc = br#""\"\b\n\u0041""#;
            let (tok2, _, _) = lex_string(
                &mut val,
                esc.as_ptr(),
                esc.as_ptr().add(esc.len()),
            ).unwrap();
            assert_eq!(tok2, JTokenType::JTOK_STRING);
            assert_eq!(val, "\"\u{0008}\nA");

            // invalid control
            assert!(lex_string(
                &mut val,
                b"\"a\x01b\"".as_ptr(),
                b"\"a\x01b\"".as_ptr().add(5)
            ).is_none());
        }
    }

    /* -------------------------------------------------------------- */
    /* orchestrator smoke‑test                                        */
    /* -------------------------------------------------------------- */
    #[traced_test]
    fn orchestrator_handles_padding() {
        let json = b"  {}\n  ";
        let mut val = String::new();
        let mut n   = 0u32;
        let tok = super::get_json_token(
            &mut val,
            &mut n,
            json.as_ptr(),
            unsafe { json.as_ptr().add(json.len()) },
        );
        assert_eq!(tok, JTokenType::JTOK_OBJ_OPEN);     // “{”
        assert_eq!(n, 1);
    }
}
