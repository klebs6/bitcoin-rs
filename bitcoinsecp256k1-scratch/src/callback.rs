// ---------------- [ File: bitcoinsecp256k1-ec/src/callback.rs ]
crate::ix!();

#[cfg(not(feature="secp256k1-use-external-default-callbacks"))]
pub fn default_illegal_callback_fn(
        str_: *const u8,
        data: *mut c_void)  {

    unsafe {
        let _ = data;

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        let stderr_file: *mut libc::FILE =
            bitcoin_imports::__stderrp as *mut libc::FILE;

        #[cfg(not(any(target_os = "macos", target_os = "ios")))]
        let stderr_file: *mut libc::FILE = libc::stderr;

        libc::fprintf(
            stderr_file,
            b"[libsecp256k1] illegal argument: %s\n\0".as_ptr() as *const libc::c_char,
            str_ as *const libc::c_char,
        );
        libc::abort();
    }
}

#[cfg(not(feature="secp256k1-use-external-default-callbacks"))]
pub fn default_error_callback_fn(
        str_: *const u8,
        data: *mut c_void)  {

    unsafe {
        let _ = data;

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        let stderr_file: *mut libc::FILE =
            bitcoin_imports::__stderrp as *mut libc::FILE;

        #[cfg(not(any(target_os = "macos", target_os = "ios")))]
        let stderr_file: *mut libc::FILE = libc::stderr;

        libc::fprintf(
            stderr_file,
            b"[libsecp256k1] internal consistency check failed: %s\n\0".as_ptr() as *const libc::c_char,
            str_ as *const libc::c_char,
        );
        libc::abort();
    }
}

#[cfg(feature="secp256k1-use-external-default-callbacks")]
pub fn default_illegal_callback_fn(
        str_: *const u8,
        data: *mut c_void)  {

    let _ = str_;
    let _ = data;
}

#[cfg(feature="secp256k1-use-external-default-callbacks")]
pub fn default_error_callback_fn(
        str_: *const u8,
        data: *mut c_void)  {

    let _ = str_;
    let _ = data;
}

lazy_static!{
    pub static ref default_illegal_callback: Callback = Callback::new(
        default_illegal_callback_fn,
        core::ptr::null(),
    );
}

lazy_static!{
    pub static ref default_error_callback: Callback = Callback::new(
        default_error_callback_fn,
        core::ptr::null(),
    );
}
