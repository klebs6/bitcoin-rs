// ---------------- [ File: bitcoin-sock/src/compat.rs ]
crate::ix!();

#[cfg(not(target_os = "windows"))] 
pub const WSAENOTSOCK: libc::c_int = libc::EBADF;

#[cfg(not(target_os = "windows"))] 
pub const INVALID_SOCKET: CSocket = !0;

#[cfg(not(target_os = "windows"))] 
pub const SOCKET_ERROR: libc::c_int = -1;

#[cfg(not(WIN32))]
#[allow(non_camel_case_types)]
pub type sockopt_arg_type = *mut c_void;

#[cfg(WIN32)]
#[allow(non_camel_case_types)]
pub type sockopt_arg_type = *mut u8;

/**
  | Note these both should work with the current
  | usage of poll, but best to be safe WIN32 poll
  | is broken
  | https://daniel.haxx.se/blog/2012/10/10/wsapoll-is-broken/
  | __APPLE__ poll is broke
  | https://github.com/bitcoin/bitcoin/pull/14336#issuecomment-437384408
  */
#[cfg(__linux__)]
pub const USE_POLL: bool = true;

/// Native Windows `SOCKET` handle (always `usize`‑sized).
#[cfg(target_os = "windows")]
pub type CSocket = usize;

/// POSIX file‑descriptor representing a socket.
#[cfg(not(target_os = "windows"))]
pub type CSocket = libc::c_int;

/// Determine whether the socket `s` can safely be
/// used with `select(2)` on the current platform.
///
/// * On Windows **or** when the build is configured
///   with `USE_POLL`, every socket is considered
///   selectable (mirrors the original C++ logic).
/// * On Unix builds that rely on classic `select(2)`
///   we ensure the descriptor is `< FD_SETSIZE`.
#[inline]
pub fn is_selectable_socket(s: &CSocket) -> bool {
    trace!(
        target: "compat::socket",
        fd = *s as u64,
        "is_selectable_socket"
    );

    #[cfg(any(target_os = "windows", feature = "use_poll"))]
    {
        true
    }

    #[cfg(not(any(target_os = "windows", feature = "use_poll")))]
    {
        (*s as usize) < libc::FD_SETSIZE as usize
    }
}
