// ---------------- [ File: bitcoin-string/src/sanitize.rs ]
crate::ix!();

/// Remove unsafe chars. Safe chars chosen to allow simple messages/URLs/email addresses, but
/// avoid anything even possibly remotely dangerous like & or >
/// 
/// ----------- @param[in] str
/// 
/// The string to sanitize ---------- @param[in] rule
/// 
/// The set of safe chars to choose (default: least restrictive)
/// 
/// ----------- @return
/// 
/// A new string without unsafe chars
/// 
/// ------------------------------------- @note
///
/// Remove unsafe characters from `str_` according to the chosen `rule`.
///
/// The implementation follows the original C++ control‑flow literally:
/// iterate over every character and copy it to the result only if it occurs
/// in the selected entry of `SAFE_CHARS`.
#[tracing::instrument(level = "trace", skip(str_, rule))]
pub fn sanitize_string(str_: &str, rule: Option<i32>) -> String {
    let mut selected_rule = rule.unwrap_or(SafeChars::SAFE_CHARS_DEFAULT as i32);

    // Clamp to a known rule; the C++ code silently assumed callers were well‑behaved,
    // but we add a warning and fall back to the default if we receive nonsense.
    if selected_rule < 0 || selected_rule as usize >= SAFE_CHARS.len() {
        warn!(
            selected_rule,
            "invalid SafeChars value supplied – falling back to SAFE_CHARS_DEFAULT"
        );
        selected_rule = SafeChars::SAFE_CHARS_DEFAULT as i32;
    }

    let safe_chars = &SAFE_CHARS[selected_rule as usize];
    let mut result = String::with_capacity(str_.len());

    for ch in str_.chars() {
        if safe_chars.contains(ch) {
            result.push(ch);
        } else {
            trace!(unsafe_char = %ch, "filtered out unsafe character");
        }
    }

    result
}

#[cfg(test)]
mod sanitize_string_tests {
    use super::*;

    /// All printable ASCII characters, used to stress‑test every rule.
    fn printable_ascii() -> String {
        (0x20u8..0x7fu8).map(char::from).collect()
    }

    #[traced_test]
    fn default_rule_filters_out_angle_brackets_and_ampersand() {
        let input = "Hello<World>&123";
        let expected = "HelloWorld123";
        let output = sanitize_string(input, None);
        assert_eq!(output, expected, "DEFAULT rule did not remove disallowed chars");
    }

    #[traced_test]
    fn ua_comment_rule_subset_is_enforced() {
        let input = "UA: foo/1.0 (bar) <baz>";
        let output = sanitize_string(input, Some(SafeChars::SAFE_CHARS_UA_COMMENT as i32));

        for ch in output.chars() {
            assert!(
                SAFE_CHARS[SafeChars::SAFE_CHARS_UA_COMMENT as usize].contains(ch),
                "UA_COMMENT rule leaked disallowed char: {ch}"
            );
        }
        assert!(!output.contains('<') && !output.contains('>'));
    }

    #[traced_test]
    fn filename_rule_only_keeps_filename_safe_chars() {
        let input = "filename<>/?:*|\".txt";
        let expected = "filename.txt";
        let output = sanitize_string(input, Some(SafeChars::SAFE_CHARS_FILENAME as i32));
        assert_eq!(output, expected);
    }

    #[traced_test]
    fn uri_rule_permits_rfc3986_set() {
        let input = "http://example.com/path?query=value#frag<>";
        let output = sanitize_string(input, Some(SafeChars::SAFE_CHARS_URI as i32));

        for ch in output.chars() {
            assert!(
                SAFE_CHARS[SafeChars::SAFE_CHARS_URI as usize].contains(ch),
                "URI rule leaked disallowed char: {ch}"
            );
        }
        assert!(!output.contains('<') && !output.contains('>'));
    }

    #[traced_test]
    fn invalid_rule_falls_back_to_default() {
        let input = "<>";
        let output = sanitize_string(input, Some(1234)); // nonsensical rule
        // DEFAULT does **not** allow angle brackets, so result must be empty.
        assert!(output.is_empty());
    }

    #[traced_test]
    fn empty_input_yields_empty_output() {
        assert!(sanitize_string("", None).is_empty());
    }

    #[traced_test]
    fn exhaustive_ascii_pass_for_each_rule() {
        let ascii = printable_ascii();

        for (idx, safe) in SAFE_CHARS.iter().enumerate() {
            let filtered = sanitize_string(&ascii, Some(idx as i32));
            for ch in filtered.chars() {
                assert!(
                    safe.contains(ch),
                    "rule {idx} leaked disallowed char: {ch}"
                );
            }
        }
    }
}
