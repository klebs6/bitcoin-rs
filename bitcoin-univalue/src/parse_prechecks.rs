// ---------------- [ File: bitcoin-univalue/src/parse_prechecks.rs ]
crate::ix!();

/// Strict pre‑scan for numeric / string parsing.
/// Rejects empty input, padding whitespace, and
/// embedded NUL bytes (RFC 7159 §2).
#[instrument(level = "trace", skip_all)]
pub fn parse_prechecks(x: &str) -> bool {
    if x.is_empty() {
        trace!(input = x, "rejected: empty string");
        return false;
    }
    let first = x.as_bytes()[0] as i32;
    let last  = x.as_bytes()[x.len() - 1] as i32;
    if json_isspace(first) || json_isspace(last) {
        trace!(input = x, "rejected: padded with whitespace");
        return false;
    }
    if x.bytes().any(|b| b == 0) {
        trace!(input = x, "rejected: embedded NUL");
        return false;
    }

    trace!(input = x, "parse_prechecks accepted");
    true
}

#[cfg(test)]
mod parse_prechecks_spec {
    use super::*;

    #[traced_test]
    fn accepts_clean_input() {
        assert!(parse_prechecks("123"));
        assert!(parse_prechecks("-42.0"));
    }

    #[traced_test]
    fn rejects_empty() {
        assert!(!parse_prechecks(""));
    }

    #[traced_test]
    fn rejects_leading_or_trailing_ws() {
        assert!(!parse_prechecks(" 1"));
        assert!(!parse_prechecks("1 "));
        assert!(!parse_prechecks("\t1"));
        assert!(!parse_prechecks("1\n"));
    }

    #[traced_test]
    fn rejects_embedded_nul() {
        let mut s = String::from("12");
        s.push('\0');
        s.push('3');
        assert!(!parse_prechecks(&s));
    }
}
