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
