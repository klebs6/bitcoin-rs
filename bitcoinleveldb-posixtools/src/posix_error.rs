// ---------------- [ File: bitcoinleveldb-posixtools/src/posix_error.rs ]
crate::ix!();

pub fn posix_error(context: &String, error_number: i32) -> crate::Status {
    use std::ffi::CStr;

    trace!(
        context = %context,
        error_number,
        "posix_error: building Status from POSIX errno"
    );

    let message = unsafe {
        let ptr = libc::strerror(error_number);
        if ptr.is_null() {
            // Fallback when strerror returns null, which should be rare.
            "<unknown-posix-error>".to_string()
        } else {
            CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        }
    };

    let ctx_slice = Slice::from(context);
    let msg_slice = Slice::from(&message);

    let status = if error_number == libc::ENOENT {
        Status::not_found(&ctx_slice, Some(&msg_slice))
    } else {
        Status::io_error(&ctx_slice, Some(&msg_slice))
    };

    debug!(
        context = %context,
        error_number,
        status_str = %status.to_string(),
        "posix_error: created status"
    );

    status
}
