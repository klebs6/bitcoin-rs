// ---------------- [ File: bitcoin-compat/src/compat.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/compat.h]

#[cfg(WIN32)]
#[cfg(not(NOMINMAX))]
pub const NOMINMAX: bool = true;

/**
   prevent redefinition compiler warning
  */
#[cfg(WIN32)]
#[cfg(FD_SETSIZE)]
pub const FD_SETSIZE: bool = false; 

/**
  max number of fds in fd_set
  */
#[cfg(WIN32)]
pub const FD_SETSIZE: usize = 1024;

///--------------------------------
#[cfg(not(target_os = "windows"))] pub type Socket = u32;

#[cfg(not(target_os = "windows"))] 
lazy_static!{
    pub static ref WSA_GET_LAST_ERROR: libc::c_int = errno().0;
}

#[cfg(not(target_os = "windows"))] pub const WSAEINVAL:          libc::c_int = libc::EINVAL;
#[cfg(not(target_os = "windows"))] pub const WSAEALREADY:        libc::c_int = libc::EALREADY;
#[cfg(not(target_os = "windows"))] pub const WSAEWOULDBLOCK:     libc::c_int = libc::EWOULDBLOCK;
#[cfg(not(target_os = "windows"))] pub const WSAEAGAIN:          libc::c_int = libc::EAGAIN;
#[cfg(not(target_os = "windows"))] pub const WSAEMSGSIZE:        libc::c_int = libc::EMSGSIZE;
#[cfg(not(target_os = "windows"))] pub const WSAEINTR:           libc::c_int = libc::EINTR;
#[cfg(not(target_os = "windows"))] pub const WSAEINPROGRESS:     libc::c_int = libc::EINPROGRESS;
#[cfg(not(target_os = "windows"))] pub const WSAEADDRINUSE:      libc::c_int = libc::EADDRINUSE;
#[cfg(not(target_os = "windows"))] pub const WSAENOTSOCK:        libc::c_int = libc::EBADF;
#[cfg(not(target_os = "windows"))] pub const INVALID_SOCKET:     CSocket = !0;
#[cfg(not(target_os = "windows"))] pub const SOCKET_ERROR:       libc::c_int = -1;

#[cfg(target_os = "windows")] #[cfg(not(WSAEAGAIN))] #[cfg(EAGAIN)]      macro_rules! wsaeagain { () => { /* EAGAIN */ } }
#[cfg(target_os = "windows")] #[cfg(not(WSAEAGAIN))] #[cfg(not(EAGAIN))] macro_rules! wsaeagain { () => { /* WSAEWOULDBLOCK */ } }

#[cfg(WIN32)] #[cfg(not(S_IRUSR))] pub const S_IRUSR: usize = 0400;
#[cfg(WIN32)] #[cfg(not(S_IRUSR))] pub const S_IWUSR: usize = 0200;

///--------------------------
#[cfg(not(target_os = "windows"))] pub const MAX_PATH: usize = 1024;

#[cfg(_MSC_VER)] #[cfg(not(ssize_t))] #[cfg(_WIN64)]      pub type ssize_t = i64;
#[cfg(_MSC_VER)] #[cfg(not(ssize_t))] #[cfg(not(_WIN64))] pub type ssize_t = i32;

#[cfg(HAVE_DECL_STRNLEN_EQ_0)]
pub fn strnlen(
        start:   *const u8,
        max_len: usize) -> usize {
    
    todo!();
        /*
        
        */
}

#[cfg(not(WIN32))]
pub type sockopt_arg_type = *mut c_void;

#[cfg(WIN32)]
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

#[inline] pub fn is_selectable_socket(s: &CSocket) -> bool {
    
    todo!();
        /*
            #if defined(USE_POLL) || defined(WIN32)
        return true;
    #else
        return (s < FD_SETSIZE);
    #endif
        */
}

/**
   MSG_NOSIGNAL is not available on some
   platforms, if it doesn't exist define it as 0
  */
#[cfg(not(MSG_NOSIGNAL))]
pub const MSG_NOSIGNAL: usize = 0;

/**
   MSG_DONTWAIT is not available on some
   platforms, if it doesn't exist define it as 0
  */
#[cfg(not(MSG_DONTWAIT))]
pub const MSG_DONTWAIT: usize = 0;
