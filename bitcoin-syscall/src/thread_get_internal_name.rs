crate::ix!();

/// Return the current thread name or an empty
/// string if no name has been set (mirrors
/// `util::ThreadGetInternalName()` in the C++ code).
#[cfg(all(USE_SYSCALL_SANDBOX, target_os = "linux", target_arch = "x86_64"))]
#[inline]
pub fn thread_get_internal_name() -> String {
    // First use std’s bookkeeping if present.
    if let Some(name) = std::thread::current().name() {
        return name.to_owned();
    }

    // Fallback to POSIX `pthread_getname_np`.
    unsafe {
        let mut buf = [0u8; 16]; // Linux limits names to 16 bytes inc. NUL
        if pthread_getname_np(pthread_self(), buf.as_mut_ptr() as *mut _, buf.len()) == 0 {
            // Strip trailing NUL and convert to UTF‑8.
            if let Ok(s) = CStr::from_ptr(buf.as_ptr() as *const _).to_str() {
                return s.to_owned();
            }
        }
    }
    String::new()
}
