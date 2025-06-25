// ---------------- [ File: bitcoin-syscall/src/syscall_sandbox_debug_signal_handler.rs ]
crate::ix!();

/// See Linux kernel developer Kees Cook's seccomp guide at <https://outflux.net/teach-seccomp/>
/// for an accessible introduction to using seccomp.
/// 
/// This function largely follows <https://outflux.net/teach-seccomp/step-3/syscall-reporter.c> and
/// <https://outflux.net/teach-seccomp/step-3/seccomp-bpf.h>.
/// 
/// Seccomp BPF resources:
/// 
/// - Seccomp BPF documentation:
/// <https://www.kernel.org/doc/html/latest/userspace-api/seccomp_filter.html>
/// 
/// - seccomp(2) manual page:
/// <https://www.kernel.org/doc/man-pages/online/pages/man2/seccomp.2.html>
/// 
/// - Seccomp BPF demo code samples:
/// <https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/samples/seccomp>
///
/// Rust port of `SyscallSandboxDebugSignalHandler`.
///
/// The signature, variable names and control‑flow mirror the original C++ so that a side‑by‑side
/// comparison is trivial.
#[cfg(all(USE_SYSCALL_SANDBOX, target_os = "linux", target_arch = "x86_64"))]
#[no_mangle]
pub extern "C" fn syscall_sandbox_debug_signal_handler(
    _0: i32,                              // signal number (unused)
    signal_info: *mut siginfo_t,          // → `signal_info`
    void_signal_context: *mut c_void,     // → `void_signal_context`
) {
    trace!(target: "compat::syscall_sandbox", "SyscallSandboxDebugSignalHandler entered");

    unsafe {
        if signal_info.is_null() {
            error!(target: "compat::syscall_sandbox", "null siginfo_t pointer");
            std::process::abort();
        }
        assert_eq!((*signal_info).si_code, SYS_SECCOMP_SI_CODE);

        if void_signal_context.is_null() {
            error!(target: "compat::syscall_sandbox", "null ucontext_t pointer");
            std::process::abort();
        }
        let signal_context: &ucontext_t = &*(void_signal_context as *const ucontext_t);

        #[allow(clippy::cast_sign_loss)]
        let syscall_number: u32 = signal_context.uc_mcontext.gregs[libc::REG_RAX as usize] as u32;

        let syscall_name = get_linux_syscall_name(syscall_number);
        let thread_name = thread_get_internal_name();
        let thread_name = if thread_name.is_empty() {
            "*unnamed*".to_owned()
        } else {
            thread_name
        };

        let error_message = format!(
            "ERROR: The syscall \"{}\" (syscall number {}) is not allowed by the syscall sandbox \
             in thread \"{}\". Please report.",
            syscall_name, syscall_number, thread_name
        );

        eprintln!("{error_message}");
        error!(target: "compat::syscall_sandbox", "{error_message}");

        std::process::abort();
    }
}

/// On non‑Linux / non‑x86‑64 targets the handler is a
/// no‑op stub so the crate still compiles when the
/// feature is toggled accidentally.
#[cfg(any(
    all(USE_SYSCALL_SANDBOX, not(target_os = "linux")),
    all(USE_SYSCALL_SANDBOX, target_os = "linux", not(target_arch = "x86_64"))
))]
#[no_mangle]
pub extern "C" fn syscall_sandbox_debug_signal_handler(
    _0: i32,
    _signal_info: *mut libc::siginfo_t,
    _void_signal_context: *mut c_void,
) {
    // Unsupported platform – just terminate.
    error!(
        target: "compat::syscall_sandbox",
        "Syscall sandbox debug handler invoked on unsupported platform"
    );
    std::process::abort();
}
