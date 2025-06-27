crate::ix!();

#[cfg(unix)]
#[inline]
pub fn io_error_is_permanent(err: i32) -> bool {
    use libc::{EAGAIN, EINTR, EWOULDBLOCK, EINPROGRESS};

    // Errors considered *transient* by the original C++ implementation.
    // Anything else is treated as permanent.
    let permanent = !(err == EAGAIN
        || err == EINTR
        || err == EWOULDBLOCK
        || err == EINPROGRESS);

    trace!(
        error_code = err,
        permanent,
        "Checked if I/O error is permanent (unix)"
    );
    permanent
}

#[cfg(windows)]
#[inline]
pub fn io_error_is_permanent(err: i32) -> bool {
    use winapi::um::winsock2::{WSAEAGAIN, WSAEINTR, WSAEWOULDBLOCK, WSAEINPROGRESS};

    let permanent = !(err == WSAEAGAIN
        || err == WSAEINTR
        || err == WSAEWOULDBLOCK
        || err == WSAEINPROGRESS);

    trace!(
        error_code = err,
        permanent,
        "Checked if I/O error is permanent (windows)"
    );
    permanent
}

// -----------------------------------------------------------------------------
// Specification
// -----------------------------------------------------------------------------
#[cfg(test)]
mod io_error_is_permanent_spec {
    use super::*;

    #[traced_test]
    fn recognises_transient_and_permanent_errors() {
        #[cfg(unix)]
        {
            use libc::{
                EAGAIN, EINTR, EWOULDBLOCK, EINPROGRESS, ECONNREFUSED, ECONNRESET, ETIMEDOUT,
            };

            // Exact set from the reference implementation.
            for code in [EAGAIN, EINTR, EWOULDBLOCK, EINPROGRESS] {
                assert!(
                    !io_error_is_permanent(code),
                    "expected {code} to be non‑permanent"
                );
            }

            // Representative permanent errors.
            for code in [ECONNREFUSED, ECONNRESET, ETIMEDOUT] {
                assert!(
                    io_error_is_permanent(code),
                    "expected {code} to be permanent"
                );
            }
        }

        #[cfg(windows)]
        {
            use winapi::um::winsock2::{
                WSAEAGAIN, WSAEINTR, WSAEWOULDBLOCK, WSAEINPROGRESS, WSAECONNREFUSED,
                WSAECONNRESET, WSAETIMEDOUT,
            };

            for code in [WSAEAGAIN, WSAEINTR, WSAEWOULDBLOCK, WSAEINPROGRESS] {
                assert!(
                    !io_error_is_permanent(code),
                    "expected {code} to be non‑permanent"
                );
            }

            for code in [WSAECONNREFUSED, WSAECONNRESET, WSAETIMEDOUT] {
                assert!(
                    io_error_is_permanent(code),
                    "expected {code} to be permanent"
                );
            }
        }

        info!("All io_error_is_permanent contract checks passed");
    }
}
