// ---------------- [ File: bitcoin-syscall/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

/// Everything that genuinely works only on Linux × x86‑64.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod linux_impl {
    use super::*;
    // The real thing – exactly the files you already have.
    x!{linux_syscalls}
    x!{seccomp_policy_builder}
    x!{set_syscall_sandbox_policy}
    x!{setup_syscall_debug_handler}
    x!{setup_syscall_sandbox}
    x!{syscall_sandbox_debug_signal_handler}
    x!{syscall_sandbox}
    x!{thread_get_internal_name}
    x!{get_linux_syscall_name}
}

/// Tiny fall‑back shims that satisfy the same public API on *every*
/// other target so the crate still compiles.
#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
mod stubs {
    use super::*;

    // ---------------- public data ----------------
    lazy_static!{
        pub static ref LINUX_SYSCALLS: HashMap<u32, &'static str> = HashMap::new();
    }

    // ---------------- small enums ----------------
    #[derive(Copy,Clone)]
    pub enum SyscallSandboxAction { KILL_PROCESS, INVOKE_SIGNAL_HANDLER }

    #[derive(Copy,Clone)]
    pub enum SyscallSandboxPolicy { }

    // ---------------- helper fns -----------------
    pub fn get_linux_syscall_name(_n: u32) -> String { "*unknown*".into() }

    // ---------------- dummy builders -------------
    pub struct SeccompPolicyBuilder;
    impl Default for SeccompPolicyBuilder { fn default() -> Self { Self } }
    impl SeccompPolicyBuilder {
        pub fn build_filter(&mut self, _a: SyscallSandboxAction)
                    -> Vec<()> { Vec::new() }
    }

    // ---------------- other API surface ----------
    pub fn setup_syscall_sandbox_debug_handler() -> bool { false }
    pub fn setup_syscall_sandbox(_log: bool) -> bool { false }
    pub fn set_syscall_sandbox_policy(_p: SyscallSandboxPolicy) { }
    pub fn test_disallowed_sandbox_call() { }
    pub fn thread_get_internal_name() -> String { String::new() }

    // Constants that the Linux code exposes
    pub const SECCOMP_RET_KILL_PROCESS: usize = 0;
    pub const SECCOMP_RET_TRAP:         usize = 0;
    pub const SECCOMP_RET_ALLOW:        usize = 0;
}

// Re‑export everything so the crate’s public interface is identical
// on every platform.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use linux_impl::*;
#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
pub use stubs::*;
