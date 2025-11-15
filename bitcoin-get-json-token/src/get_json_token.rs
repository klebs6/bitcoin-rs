// ---------------- [ File: bitcoin-get-json-token/src/get_json_token.rs ]
crate::ix!();

#[instrument(level = "trace", skip(token_val, consumed))]
pub fn get_json_token(
    token_val: &mut String,
    consumed:  &mut u32,
    raw:       *const u8,
    end:       *const u8,
) -> JTokenType {
    unsafe {
        token_val.clear();
        *consumed = 0;

        /* ---------- 1 / 5  skip leading whitespace / NUL ---------- */
        let mut p     = skip_ws_nul(raw, end);
        let raw_start = p;

        // number of bytes skipped before seeing the token
        let skipped = (raw_start as usize - raw as usize) as u32;
        if skipped > 0 {
            trace!(
                skipped,
                "get_json_token: skipped leading JSON whitespace / NUL padding"
            );
        }

        /* ---------- 2 / 5  try structural ---------- */
        if let Some((tok, after, n)) = lex_structural(p, end) {
            *consumed = n; // count only the token itself, not leading whitespace
            trace!(
                ?tok,
                consumed = *consumed,
                "get_json_token: structural token recognised"
            );
            return tok;
        }

        /* ---------- 3 / 5  try keyword ---------- */
        if let Some((tok, after, n)) = lex_keyword(p, end) {
            *consumed = n;
            trace!(
                ?tok,
                consumed = *consumed,
                "get_json_token: keyword token recognised"
            );
            return tok;
        }

        /* ---------- 4 / 5  try number ---------- */
        if let Some((tok, after, n)) = lex_number(token_val, p, end) {
            *consumed = n;
            trace!(
                ?tok,
                consumed = *consumed,
                value = %token_val,
                "get_json_token: number token recognised"
            );
            return tok;
        }

        /* ---------- 5 / 5  try string ---------- */
        if let Some((tok, after, n)) = lex_string(token_val, p, end) {
            *consumed = n;
            trace!(
                ?tok,
                consumed = *consumed,
                value = %token_val,
                "get_json_token: string token recognised"
            );
            return tok;
        }

        /* ---------- no match ⇒ error / eof ---------- */
        if p >= end {
            trace!("get_json_token: reached end of input ⇒ JTOK_NONE");
            JTokenType::JTOK_NONE
        } else {
            trace!(byte = *p, "get_json_token: unrecognised input ⇒ JTOK_ERR");
            JTokenType::JTOK_ERR
        }
    }

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
