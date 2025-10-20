// ---------------- [ File: bitcoin-sock/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{sock}
x!{io_error_is_permanent}
x!{interface}
x!{network_error_string}
x!{is_connected}
x!{recv_until_terminator}
x!{send_complete}
x!{wait}
x!{close_socket}
x!{compat}
x!{lifetime}
x!{last_socket_error}
x!{compute_bounded_wait}

#[cfg(test)]
pub(crate) mod test_serial {
    use once_cell::sync::Lazy;
    use parking_lot::Mutex;

    /// Global lock to serialize socket-using tests that would otherwise be flaky
    /// due to process-global FD reuse.
    pub static FD_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
}

/// Helper macro for tests that should not run concurrently with other socket tests.
#[cfg(test)]
#[macro_export]
macro_rules! serialize_fds {
    () => {
        let _fd_guard = $crate::test_serial::FD_LOCK.lock(); // non-poisoning
    };
}
