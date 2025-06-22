// ---------------- [ File: bitcoin-univalue/src/indent_str.rs ]
crate::ix!();

/// Append `pretty_indent * indent_level` space
/// characters to `s`.
///
/// Used by the pretty‑printer in `UniValue::write`.
#[instrument(level = "trace", skip(s))]
pub fn indent_str(pretty_indent: u32, indent_level: u32, s: &mut String) {
    if pretty_indent == 0 || indent_level == 0 {
        return;
    }
    // Calculated once to avoid re‑allocation churn.
    let count = (pretty_indent * indent_level) as usize;
    s.reserve(count);
    for _ in 0..count {
        s.push(' ');
    }
}

#[cfg(test)]
mod indent_str_spec {
    use super::*;
    use traced_test::traced_test;

    #[traced_test]
    fn adds_expected_number_of_spaces() {
        let mut buf = String::new();
        indent_str(4, 3, &mut buf); // 12 spaces
        assert_eq!(buf.len(), 12);
        assert!(buf.chars().all(|c| c == ' '));
    }

    #[traced_test]
    fn no_op_when_pretty_indent_is_zero() {
        let mut buf = String::from("x");
        indent_str(0, 5, &mut buf);
        assert_eq!(buf, "x");
    }
}

