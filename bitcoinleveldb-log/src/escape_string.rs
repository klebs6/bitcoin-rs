// ---------------- [ File: bitcoinleveldb-log/src/escape_string.rs ]
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
