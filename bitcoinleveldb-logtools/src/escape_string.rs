// ---------------- [ File: bitcoinleveldb-logtools/src/escape_string.rs ]
crate::ix!();

/**
  | Return a human-readable version of "value".
  |
  | Escapes any non-printable characters found in
  | "value".
  */
pub fn escape_string(value: &Slice) -> String {
    trace!("escape_string: value_size={}", value.size());
    let mut r = String::new();
    append_escaped_string_to(&mut r as *mut String, value);
    r
}

#[cfg(test)]
mod escape_string_spec {
    use super::*;

    fn slice_from_bytes(bytes: &[u8]) -> Slice {
        Slice::from_ptr_len(bytes.as_ptr(), bytes.len())
    }

    #[traced_test]
    fn escape_string_round_trips_printable_ascii() {
        let bytes = b"LevelDB logtools escape";
        let slice = slice_from_bytes(bytes);

        info!(
            "escape_string_round_trips_printable_ascii: input='{}'",
            String::from_utf8_lossy(bytes)
        );

        let escaped = escape_string(&slice);

        info!(
            "escape_string_round_trips_printable_ascii: escaped='{}'",
            escaped
        );
        assert_eq!(escaped, String::from_utf8_lossy(bytes));
    }

    #[traced_test]
    fn escape_string_escapes_non_printable_bytes() {
        let bytes: [u8; 4] = [0x00, b'A', 0xFF, b'\n'];
        let slice = slice_from_bytes(&bytes);

        info!(
            "escape_string_escapes_non_printable_bytes: input={:?}",
            &bytes
        );

        let escaped = escape_string(&slice);

        info!(
            "escape_string_escapes_non_printable_bytes: escaped='{}'",
            escaped
        );
        assert_eq!(escaped, "\\x00A\\xff\\x0a");
    }

    #[traced_test]
    fn escape_string_handles_empty_slice() {
        let empty: [u8; 0] = [];
        let slice = slice_from_bytes(&empty);

        info!("escape_string_handles_empty_slice: starting");
        let escaped = escape_string(&slice);

        info!(
            "escape_string_handles_empty_slice: escaped='{}'",
            escaped
        );
        assert!(escaped.is_empty());
    }

    #[traced_test]
    fn escape_string_is_consistent_with_append_escaped_string_to() {
        let data = b"\x01Hello\x7F";
        let slice = slice_from_bytes(data);

        info!(
            "escape_string_is_consistent_with_append_escaped_string_to: input={:?}",
            &data
        );

        let escaped_direct = escape_string(&slice);

        let mut manual = String::new();
        append_escaped_string_to(&mut manual as *mut String, &slice);

        info!(
            "escape_string_is_consistent_with_append_escaped_string_to: escaped_direct='{}' manual='{}'",
            escaped_direct,
            manual
        );

        assert_eq!(escaped_direct, manual);
    }
}
