// ---------------- [ File: bitcoin-syscall/src/setup_syscall_debug_handler.rs ]
crate::ix!();

/// This function largely follows install_syscall_reporter from Kees Cook's seccomp guide:
/// <https://outflux.net/teach-seccomp/step-3/syscall-reporter.c>
///
/// Rust port of `SetupSyscallSandboxDebugHandler`.
///
/// Registers `SyscallSandboxDebugSignalHandler` as the SIGSYS handler and unblocks SIGSYS so the
/// kernel can deliver the signal.
#[inline]
pub fn setup_syscall_sandbox_debug_handler() -> bool {
    #[cfg(all(USE_SYSCALL_SANDBOX, target_os = "linux", target_arch = "x86_64"))]
    {
        unsafe {
            trace!(
                target: "compat::syscall_sandbox",
                "installing SIGSYS debug handler"
            );

            let mut action: sigaction = std::mem::zeroed();
            let mut mask = std::mem::zeroed();

            sigemptyset(&mut mask);
            sigaddset(&mut mask, SIGSYS);

            action.sa_sigaction = syscall_sandbox_debug_signal_handler as usize;
            action.sa_flags = SA_SIGINFO;

            if libc::sigaction(SIGSYS, &action, ptr::null_mut()) != 0 {
                error!(target: "compat::syscall_sandbox", "sigaction failed");
                return false;
            }
            if sigprocmask(SIG_UNBLOCK, &mask, ptr::null_mut()) != 0 {
                error!(target: "compat::syscall_sandbox", "sigprocmask failed");
                return false;
            }
        }
        return true;
    }

    false
}
