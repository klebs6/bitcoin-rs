crate::ix!();

#[cfg(target_os = "macos")]
#[inline]
pub fn c_stdout() -> *mut libc::FILE {
    // The libc crate doesn't export __stdoutp, so we define it ourselves:
    extern "C" {
        #[link_name = "__stdoutp"]
        static STDOUTP: *mut libc::FILE;
    }
    unsafe { STDOUTP }
}

#[cfg(not(target_os = "macos"))]
#[inline]
pub fn c_stdout() -> *mut libc::FILE {
    unsafe { libc::stdout }
}

#[cfg(test)]
mod c_stdout_tests {
    use super::*;

    /// Test that `c_stdout()` returns a valid pointer on each platform.
    #[traced_test]
    #[serial]
    fn test_c_stdout_pointer_non_null() {
        info!("Testing c_stdout pointer is not null for this platform.");

        let ptr = c_stdout();
        assert!(!ptr.is_null(), "c_stdout() must never return a null pointer.");

        trace!("test_c_stdout_pointer_non_null passed.");
    }
}
