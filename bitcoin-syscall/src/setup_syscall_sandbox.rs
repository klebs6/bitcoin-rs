// ---------------- [ File: bitcoin-syscall/src/setup_syscall_sandbox.rs ]
crate::ix!();

/**
  | Setup and enable the experimental syscall
  | sandbox for the running process.
  |
  | SetSyscallSandboxPolicy(SyscallSandboxPolicy::INITIALIZATION)
  | is called as part of SetupSyscallSandbox(...).
  */
#[cfg(USE_SYSCALL_SANDBOX)]
pub fn setup_syscall_sandbox(log_syscall_violation_before_terminating: bool) -> bool {
    
    todo!();
        /*
            assert(!g_syscall_sandbox_enabled && "SetupSyscallSandbox(...) should only be called once.");
        g_syscall_sandbox_enabled = true;
        g_syscall_sandbox_log_violation_before_terminating = log_syscall_violation_before_terminating;
        if (log_syscall_violation_before_terminating) {
            if (!SetupSyscallSandboxDebugHandler()) {
                return false;
            }
        }
        SetSyscallSandboxPolicy(SyscallSandboxPolicy::INITIALIZATION);
        return true;
        */
}

/**
  | Invoke a disallowed syscall. Use for
  | testing purposes.
  |
  */
#[cfg(USE_SYSCALL_SANDBOX)]
pub fn test_disallowed_sandbox_call()  {
    
    todo!();
        /*
            // The getgroups syscall is assumed NOT to be allowed by the syscall sandbox policy.
        std::array<gid_t, 1> groups;
        [[maybe_unused]] int32_t ignored = getgroups(groups.size(), groups.data());
        */
}
