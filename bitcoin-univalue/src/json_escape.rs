// ---------------- [ File: bitcoin-univalue/src/json_escape.rs ]
crate::ix!();

/// Escape a raw string according to the
/// Bitcoin‑Core Univalue rules (identical
/// to RFC 7159 JSON string escaping).
///
/// Uses the pre‑computed `escapes` lookup
/// table for performance.
#[instrument(level = "trace", skip_all)]
pub fn json_escape(in_s: &str) -> String {
    let mut out = String::with_capacity(in_s.len() * 2);

    for &byte in in_s.as_bytes() {
        if let Some(esc) = escapes[byte as usize] {
            out.push_str(esc);
        } else {
            out.push(byte as char);
        }
    }

    out
}

#[cfg(test)]
mod json_escape_spec {
    use super::*;

    #[traced_test]
    fn escapes_control_and_quote() {
        let original = "\"\n\\";
        let escaped  = json_escape(original);
        assert_eq!(escaped, "\\\"\\n\\\\");
    }

    #[traced_test]
    fn passes_through_regular_chars() {
        let original = "hello world";
        assert_eq!(json_escape(original), original);
    }
}
