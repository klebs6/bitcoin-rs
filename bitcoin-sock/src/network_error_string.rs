// ---------------- [ File: bitcoin-sock/src/network_error_string.rs ]
crate::ix!();

use std::ffi::CStr;

#[cfg(not(target_os = "windows"))]
pub fn network_error_string(err: i32) -> String {
    const BUF_CAP: usize = 256;
    let mut buf = [0u8; BUF_CAP];

    // Two incompatible strerror_r variants exist; handle both.
    #[cfg(any(
        target_env = "gnu",     // glibc returns `char *`
        target_env = "uclibc"
    ))]
    let c_ptr = unsafe { libc::strerror_r(err, buf.as_mut_ptr() as *mut _, BUF_CAP) };

    #[cfg(not(any(
        target_env = "gnu",
        target_env = "uclibc"
    )))]
    let ret_val = unsafe { libc::strerror_r(err, buf.as_mut_ptr() as *mut _, BUF_CAP) };

    #[cfg(any(
        target_env = "gnu",
        target_env = "uclibc"
    ))]
    let c_str = unsafe { CStr::from_ptr(c_ptr) };

    #[cfg(not(any(
        target_env = "gnu",
        target_env = "uclibc"
    )))]
    let c_str = {
        if ret_val != 0 {
            // Unknown error – empty buffer.
            &CStr::from_bytes_with_nul(b"\0").unwrap()
        } else {
            unsafe { CStr::from_ptr(buf.as_ptr() as *const _) }
        }
    };

    let msg = c_str.to_string_lossy();
    let full = format!("{msg} ({err})");
    debug!(err, %full, "network_error_string (unix)");
    full
}

/// Return a printable description for the given network error code.
///
/// Mirrors the C++ reference exactly: _“`%s (%d)`”_ where the first
/// field is the platform’s error text and the second is the integer
/// error value.
#[cfg(target_os = "windows")]
pub fn network_error_string(err: i32) -> String {
    use std::ptr;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::{
        winbase::{
            FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS,
            FORMAT_MESSAGE_MAX_WIDTH_MASK,
        },
        winnt::{LANG_NEUTRAL, SUBLANG_DEFAULT},
    };

    const BUF_CAP: usize = 256;
    let mut buf: [u16; BUF_CAP] = [0; BUF_CAP];

    // SAFETY: `FormatMessageW` writes at most BUF_CAP elements.
    let len: DWORD = unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM
                | FORMAT_MESSAGE_IGNORE_INSERTS
                | FORMAT_MESSAGE_MAX_WIDTH_MASK,
            ptr::null(),
            err as DWORD,
            (LANG_NEUTRAL as DWORD) << 10 | (SUBLANG_DEFAULT as DWORD),
            buf.as_mut_ptr(),
            BUF_CAP as DWORD,
            ptr::null_mut(),
        )
    };

    let msg = if len != 0 {
        // Trim trailing CR/LF added by Windows.
        let slice = &buf[..len as usize];
        let mut s = String::from_utf16_lossy(slice);
        while s.ends_with(['\r', '\n']) {
            s.pop();
        }
        s
    } else {
        String::from("Unknown error")
    };

    let full = format!("{msg} ({err})");
    trace!(err, %full, "network_error_string (windows)");
    full
}

// -----------------------------------------------------------------------------
// Specification
// -----------------------------------------------------------------------------
#[cfg(test)]
mod network_error_string_spec {
    use super::*;

    #[traced_test]
    fn formats_known_error() {
        serialize_fds!(); // <— add this
        #[cfg(unix)]
        {
            let text = network_error_string(libc::EBADF);
            // Should embed both message and code.
            assert!(text.contains("(9)"), "expected numeric code in output");
            assert!(
                text.to_ascii_lowercase().contains("bad file"),
                "expected human text in output"
            );
        }

        #[cfg(target_os = "windows")]
        {
            use winapi::shared::winerror::WSAEINVAL;
            let text = network_error_string(WSAEINVAL as i32);
            assert!(text.contains(&format!("({})", WSAEINVAL)));
            assert!(
                text.to_ascii_lowercase().contains("invalid"),
                "expected human description"
            );
        }
    }
}
