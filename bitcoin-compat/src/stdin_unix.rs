// ---------------- [ File: bitcoin-compat/src/stdin_unix.rs ]
crate::ix!();

#[cfg(unix)]
pub(crate) mod unix_impl {
    use super::*;
    use libc::{
        c_int, isatty, poll, tcgetattr, tcsetattr, termios, POLLIN, STDIN_FILENO, TCSANOW,
    };

    /// Toggle the `ECHO` flag in the current `termios`
    /// settings for `STDIN`.
    pub fn set_echo(enable: bool) {
        unsafe {
            let mut tty: termios = core::mem::zeroed();
            if tcgetattr(STDIN_FILENO, &mut tty as *mut _) != 0 {
                error!(target: "compat::stdin", "tcgetattr failed");
                return;
            }

            if enable {
                tty.c_lflag |= libc::ECHO;
            } else {
                tty.c_lflag &= !libc::ECHO;
            }

            if tcsetattr(STDIN_FILENO, TCSANOW, &tty as *const _) != 0 {
                error!(target: "compat::stdin", "tcsetattr failed");
            }
        }
    }

    /// Detect whether `STDIN` is attached to a TTY.
    #[inline]
    pub fn is_tty() -> bool {
        unsafe { isatty(STDIN_FILENO) == 1 }
    }

    /// Poll `STDIN` with a zero‑timeout for pending
    /// data.
    pub fn is_ready() -> bool {
        unsafe {
            let mut fds = libc::pollfd {
                fd: STDIN_FILENO,
                events: POLLIN,
                revents: 0,
            };
            poll(&mut fds as *mut _, 1, 0) == 1
        }
    }
}
