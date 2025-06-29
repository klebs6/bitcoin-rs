// ---------------- [ File: bitcoin-tokenpipe/src/last_errno.rs ]
crate::ix!();

/// Cross‑platform helper to fetch the thread‑local `errno`.
#[inline]
pub fn last_errno() -> i32 {
    unsafe {
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            *libc::__errno_location()
        }
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        {
            *libc::__error()
        }
        #[cfg(windows)]
        {
            let mut err: libc::c_int = 0;
            let _ = libc::_get_errno(&mut err as *mut _);
            err
        }
    }
}
