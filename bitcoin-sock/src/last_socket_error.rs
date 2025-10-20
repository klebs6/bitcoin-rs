crate::ix!();

#[inline(always)]
pub const fn msg_nosignal_const() -> i32 {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        libc::MSG_NOSIGNAL
    }
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    {
        0
    }
}

#[inline(always)]
pub fn last_socket_error() -> i32 {
    #[cfg(target_os = "windows")]
    {
        unsafe { winapi::um::winsock2::WSAGetLastError() }
    }
    #[cfg(not(target_os = "windows"))]
    {
        last_errno()
    }
}

#[cfg(unix)]
pub fn make_socket_pair() -> (libc::c_int, libc::c_int) {
    let mut sv = [-1; 2];
    let ret =
        unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, sv.as_mut_ptr()) };
    assert_eq!(ret, 0);
    (sv[0], sv[1])
}

#[inline(always)]
pub const fn msg_peek_const() -> i32 {
    #[cfg(target_os = "windows")]
    {
        winapi::um::winsock2::MSG_PEEK
    }
    #[cfg(not(target_os = "windows"))]
    {
        libc::MSG_PEEK
    }
}
