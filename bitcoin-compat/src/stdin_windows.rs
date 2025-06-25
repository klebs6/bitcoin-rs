crate::ix!();

#[cfg(target_os = "windows")]
pub(crate) mod windows_impl {
    use super::*;
    use libc::{_fileno, _isatty, stdin as STDIN_FILE};
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::System::Console::{
        GetConsoleMode, GetStdHandle, SetConsoleMode, ENABLE_ECHO_INPUT, STD_INPUT_HANDLE,
    };

    /// Helper to obtain a Win32 `HANDLE` for `STDIN`.
    #[inline]
    fn stdin_handle() -> Option<HANDLE> {
        let handle = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
        if handle == 0 {
            error!(target: "compat::stdin", "GetStdHandle returned NULL");
            None
        } else {
            Some(handle)
        }
    }

    /// Toggle the `ENABLE_ECHO_INPUT` console mode
    /// flag for the process’ `STDIN` handle.
    pub fn set_echo(enable: bool) {
        if let Some(handle) = stdin_handle() {
            unsafe {
                let mut mode: u32 = 0;
                if GetConsoleMode(handle, &mut mode) == 0 {
                    error!(target: "compat::stdin", "GetConsoleMode failed");
                    return;
                }

                if enable {
                    mode |= ENABLE_ECHO_INPUT;
                } else {
                    mode &= !ENABLE_ECHO_INPUT;
                }

                if SetConsoleMode(handle, mode) == 0 {
                    error!(target: "compat::stdin", "SetConsoleMode failed");
                }
            }
        }
    }

    /// Detect whether `STDIN` is attached to a TTY.
    #[inline]
    pub fn is_tty() -> bool {
        unsafe { _isatty(_fileno(STDIN_FILE)) == 1 }
    }
}
