// ---------------- [ File: bitcoinleveldbt-util/src/cstring_label_to_owned_string.rs ]
crate::ix!();

/// Invariant: converts a NUL-terminated label pointer into an owned Rust string without mutating
/// the pointed-to bytes.
pub fn dbtest_c_string_label_to_owned_string(label: *const u8) -> String {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::dbtest",
        label = "dbtest.c_string_label_to_owned_string.enter",
        label_is_null = label.is_null()
    );

    let out = match label.is_null() {
        true => "(null)".to_string(),
        false => unsafe {
            CStr::from_ptr(label as *const c_char)
                .to_string_lossy()
                .into_owned()
        },
    };

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::dbtest",
        label = "dbtest.c_string_label_to_owned_string.exit",
        out_len = out.len()
    );

    out
}
