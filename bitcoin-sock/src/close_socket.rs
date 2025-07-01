// ---------------- [ File: bitcoin-sock/src/close_socket.rs ]
crate::ix!();

/**
  | Close socket and set `h_socket` to `INVALID_SOCKET`.
  |
  | Mirrors the original C++ logic while adding rich tracing.
  */
pub fn close_socket(h_socket: &mut CSocket) -> bool {
    if *h_socket == INVALID_SOCKET {
        return false;
    }

    // --- perform the actual close -------------------------------------------------------------
    #[cfg(target_os = "windows")]
    let ret: libc::c_int =
        unsafe { winapi::um::winsock2::closesocket(*h_socket as winapi::um::winsock2::SOCKET) };

    #[cfg(not(target_os = "windows"))]
    let ret: libc::c_int = unsafe { libc::close(*h_socket) };
    // ------------------------------------------------------------------------------------------

    if ret != 0 {
        #[cfg(target_os = "windows")]
        let err_code = unsafe { winapi::um::winsock2::WSAGetLastError() };

        #[cfg(not(target_os = "windows"))]
        let err_code = last_errno();

        warn!(
            socket = *h_socket as i64,
            ret,
            err_code,
            "Socket close failed: {}. Error: {}",
            *h_socket as i64,
            network_error_string(err_code)
        );
    }

    *h_socket = INVALID_SOCKET;

    ret != SOCKET_ERROR
}
