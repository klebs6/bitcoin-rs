//! Cross‑platform helpers for controlling echo on
//! `STDIN`, plus a small RAII guard (`NoechoInst`)
//! that disables echo for the lifetime of a scope.
//!
//! The original C++ implementation used either
//! `termios` (POSIX) or the Win32 console APIs.
//! We replicate the same behaviour while adding
//! *robust* `tracing` so operational issues are
//! surfaced in production.

// ---------------- [ File: bitcoin-compat/src/stdin.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/compat/stdin.h]

/// RAII guard that disables terminal echo on
/// construction and re‑enables it on drop.
pub struct NoechoInst;

impl NoechoInst {
    /// Disable echo on `STDIN` for the lifetime of
    /// the returned guard.
    #[inline]
    pub fn new() -> Self {
        trace!(target: "compat::stdin", "disabling echo");
        set_stdin_echo(false);
        NoechoInst
    }
}

impl Drop for NoechoInst {
    #[inline]
    fn drop(&mut self) {
        trace!(target: "compat::stdin", "re‑enabling echo");
        set_stdin_echo(true);
    }
}

/// Create an unnamed `NoechoInst` guard for the
/// current scope.
///
/// ```ignore
/// {
///     no_stdin_echo!();       // echo disabled
///     // read secret input…
/// }                           // echo automatically re‑enabled
/// ```
#[macro_export]
macro_rules! no_stdin_echo {
    () => {
        let _no_echo = $crate::stdin::NoechoInst::new();
    };
}

//-------------------------------------------[.cpp/bitcoin/src/compat/stdin.cpp]

/// Enable (`true`) or disable (`false`) terminal echo
/// on the calling process’ standard input.
///
/// https://stackoverflow.com/questions/1413445/reading-a-password-from-stdcin
#[inline]
pub fn set_stdin_echo(enable: bool) {
    #[cfg(target_os = "windows")]
    windows_impl::set_echo(enable);

    #[cfg(unix)]
    unix_impl::set_echo(enable);
}

/// Return `true` if `STDIN` is connected to a TTY.
#[inline]
pub fn stdin_terminal() -> bool {
    #[cfg(target_os = "windows")]
    {
        windows_impl::is_tty()
    }

    #[cfg(unix)]
    {
        unix_impl::is_tty()
    }
}

/// Return `true` if there is buffered data available
/// for immediate, non‑blocking read on `STDIN`.
///
/// On Windows the console subsystem does not expose
/// a polling API that is reliable across redirected
/// input channels, so the function always returns
/// `false` when compiled for that platform.
#[inline]
pub fn stdin_ready() -> bool {
    if !stdin_terminal() {
        debug!(target: "compat::stdin", "STDIN is not a TTY – assuming ready");
        return true;
    }

    #[cfg(target_os = "windows")]
    {
        false
    }

    #[cfg(unix)]
    {
        unix_impl::is_ready()
    }
}
