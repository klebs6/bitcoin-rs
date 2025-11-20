// ---------------- [ File: bitcoinleveldb-log/src/append_escaped_string_to.rs ]
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
                "append_escaped_string_to: Slice has null data pointer; nothing to append"
            );
            return;
        }

        let len = value.size();
        let bytes = std::slice::from_raw_parts(data_ptr, len);

        for &b in bytes {
            let c = b as char;
            if c >= ' ' && c <= '~' {
                s.push(c);
            } else {
                use std::fmt::Write as _;
                let _ = write!(s, "\\x{:02x}", b);
            }
        }
    }
}
