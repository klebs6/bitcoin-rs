crate::ix!();

/// Emulate the C++ function `RemovePrefix(source_file, "./")`.
pub fn remove_prefix(s: &str, prefix: &str) -> String {
    if s.starts_with(prefix) {
        s[prefix.len()..].to_string()
    } else {
        s.to_string()
    }
}

/// A minimal imitation of `GetTimeMicros()` in the C++ code,
/// returning the current Unix time in microseconds.
pub fn get_time_micros() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    now.as_micros() as i64
}

#[inline]
pub unsafe fn file_write_str(msg: &str, fp: *mut libc::FILE) {
    // Removed the `error!()` call to avoid re‐entrant logging deadlock if `fp` is null.
    if fp.is_null() {
        return;
    }
    libc::fwrite(
        msg.as_ptr() as *const libc::c_void,
        1,
        msg.len(),
        fp
    );
}
