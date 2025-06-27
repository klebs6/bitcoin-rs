// ---------------- [ File: bitcoin-syscall/src/setup_syscall_sandbox.rs ]
crate::ix!();

/**
  | Setup and enable the experimental syscall
  | sandbox for the running process.
  |
  | SetSyscallSandboxPolicy(SyscallSandboxPolicy::INITIALIZATION)
  | is called as part of SetupSyscallSandbox(...).
  */
pub fn setup_syscall_sandbox(log_before_terminate: bool) -> bool {
    // Only succeed on first invocation (same assert as C++).
    if G_SYSCALL_SANDBOX_ENABLED.swap(true, atomic::Ordering::SeqCst) {
        panic!("SetupSyscallSandbox(...) should only be called once.");
    }
    G_SYSCALL_SANDBOX_LOG_VIOLATION_BEFORE_TERMINATING
        .store(log_before_terminate, atomic::Ordering::SeqCst);

    if log_before_terminate && !setup_syscall_sandbox_debug_handler() {
        return false;
    }

    // Load the most permissive (initialisation) profile first.
    set_syscall_sandbox_policy(SyscallSandboxPolicy::INITIALIZATION);
    true
}

/**
  | Invoke a disallowed syscall. Use for
  | testing purposes.
  |
  */
pub fn test_disallowed_sandbox_call() {

    // Intentionally use a syscall (getgroups) that is *not* on any allow‑list.
    //
    // The getgroups syscall is assumed NOT to be allowed by the syscall sandbox policy.
    unsafe {
        let mut buf: [libc::gid_t; 1] = [0];
        libc::getgroups(buf.len() as libc::c_int, buf.as_mut_ptr());
    }
}
