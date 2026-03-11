// ---------------- [ File: bitcoinleveldb-dbtest/src/custom_number_comparator_parse_bracketed_integer.rs ]
crate::ix!();

/// Invariant: successful parses recover exactly the bracketed integer value accepted by the
/// custom comparator tests, including hexadecimal forms with `0x`/`0X`.
///
/// Precondition: `x` is intended to be a key of the form `"[<integer>]"`.
/// Postcondition: returns the integer encoded inside the brackets, or panics through the
/// surrounding test assertions on malformed input.
pub fn dbtest_custom_number_comparator_parse_bracketed_integer(
    x: &Slice,
) -> i32 {
    tracing::trace!(
        target: "bitcoinleveldb_dbtest::fixture",
        label = "dbtest_custom_number_comparator_parse_bracketed_integer.entry",
        input_len = x.size()
    );

    let text = x.to_string();
    let bytes = text.as_bytes();

    assert!(bytes.len() >= 2);
    assert_eq!(bytes[0], b'[');
    assert_eq!(bytes[bytes.len() - 1], b']');

    let body = &text[1..text.len() - 1];

    let value = if body.starts_with("-0x") || body.starts_with("-0X") {
        let magnitude = match i32::from_str_radix(&body[3..], 16) {
            Ok(v) => v,
            Err(_) => {
                assert!(false);
                0
            }
        };
        -magnitude
    } else if body.starts_with("0x") || body.starts_with("0X") {
        match i32::from_str_radix(&body[2..], 16) {
            Ok(v) => v,
            Err(_) => {
                assert!(false);
                0
            }
        }
    } else {
        match body.parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                assert!(false);
                0
            }
        }
    };

    tracing::trace!(
        target: "bitcoinleveldb_dbtest::fixture",
        label = "dbtest_custom_number_comparator_parse_bracketed_integer.exit",
        value = value
    );

    value
}
