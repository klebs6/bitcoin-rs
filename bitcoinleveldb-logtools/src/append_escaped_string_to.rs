// ---------------- [ File: bitcoinleveldb-logtools/src/append_escaped_string_to.rs ]
crate::ix!();

/**
  | Append a human-readable printout of
  | "value" to *str.
  | 
  | Escapes any non-printable characters
  | found in "value".
  |
  */
pub fn append_escaped_string_to(str_: *mut String, value: &Slice) {
    trace!(
        "append_escaped_string_to: value_size={}",
        value.size()
    );

    if str_.is_null() {
        error!("append_escaped_string_to: received null String pointer");
        return;
    }

    unsafe {
        let s: &mut String = &mut *str_;
        let data_ptr = value.data();

        if data_ptr.is_null() {
            debug!(
                "append_escaped_string_to: Slice has null outer data pointer; nothing to append"
            );
            return;
        }

        let inner_ptr = *data_ptr;
        if inner_ptr.is_null() {
            debug!(
                "append_escaped_string_to: Slice has null inner data pointer; nothing to append"
            );
            return;
        }

        let len = *value.size();
        let bytes = core::slice::from_raw_parts(inner_ptr, len);

        for &b in bytes {
            let c = b as char;
            if c >= ' ' && c <= '~' {
                s.push(c);
            } else {
                use core::fmt::Write as _;
                let _ = write!(s, "\\x{:02x}", b);
            }
        }
    }
}

#[cfg(test)]
mod append_escaped_string_to_spec {
    use super::*;

    fn slice_from_bytes(bytes: &[u8]) -> Slice {
        Slice::from_ptr_len(bytes.as_ptr(), bytes.len())
    }

    #[traced_test]
    fn append_escaped_preserves_printable_ascii() {
        let bytes = b"Hello, World! 123 ~";
        let slice = slice_from_bytes(bytes);
        let mut output = String::new();

        info!("append_escaped_preserves_printable_ascii: starting test");
        append_escaped_string_to(&mut output as *mut String, &slice);

        info!("append_escaped_preserves_printable_ascii: output={}", output);
        assert_eq!(output, String::from_utf8_lossy(bytes));
    }

    #[traced_test]
    fn append_escaped_converts_non_printable_to_hex() {
        let bytes: [u8; 6] = [0x00, b'A', 0x1F, b' ', 0x7F, 0xFF];
        let slice = slice_from_bytes(&bytes);
        let mut output = String::new();

        info!(
            "append_escaped_converts_non_printable_to_hex: input={:?}",
            &bytes
        );
        append_escaped_string_to(&mut output as *mut String, &slice);

        info!(
            "append_escaped_converts_non_printable_to_hex: escaped_output={}",
            output
        );
        assert_eq!(output, "\\x00A\\x1f \\x7f\\xff");
    }

    #[traced_test]
    fn append_escaped_with_empty_slice_leaves_string_unchanged() {
        let empty: [u8; 0] = [];
        let slice = slice_from_bytes(&empty);
        let mut output = String::from("prefix");

        info!(
            "append_escaped_with_empty_slice_leaves_string_unchanged: initial_output={}",
            output
        );
        append_escaped_string_to(&mut output as *mut String, &slice);

        info!(
            "append_escaped_with_empty_slice_leaves_string_unchanged: final_output={}",
            output
        );
        assert_eq!(output, "prefix");
    }

    #[traced_test]
    fn append_escaped_appends_to_existing_content() {
        let bytes = b"XYZ";
        let slice = slice_from_bytes(bytes);
        let mut output = String::from("prefix:");

        info!(
            "append_escaped_appends_to_existing_content: initial_output={}",
            output
        );
        append_escaped_string_to(&mut output as *mut String, &slice);

        info!(
            "append_escaped_appends_to_existing_content: final_output={}",
            output
        );
        assert_eq!(output, "prefix:XYZ");
    }

    #[traced_test]
    fn append_escaped_handles_null_destination_pointer() {
        let bytes = b"ignored";
        let slice = slice_from_bytes(bytes);

        info!("append_escaped_handles_null_destination_pointer: invoking with null pointer");
        append_escaped_string_to(core::ptr::null_mut::<String>(), &slice);
    }
}
