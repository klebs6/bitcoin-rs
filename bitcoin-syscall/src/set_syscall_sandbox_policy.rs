// ---------------- [ File: bitcoin-syscall/src/set_syscall_sandbox_policy.rs ]
crate::ix!();

/**
  | Force the current thread (and threads created
  | from the current thread) into
  | a restricted-service operating mode where only
  | a subset of all syscalls are available.
  |
  | Subsequent calls to this function can reduce
  | the abilities further, but abilities can never
  | be regained.
  |
  | This function is a no-op unless
  | SetupSyscallSandbox(...) has been called.
  |
  | SetupSyscallSandbox(...) is called during
  | bitcoind initialization if Bitcoin Core was
  | compiled with seccomp-bpf support
  | (--with-seccomp) *and* the parameter
  | -sandbox=<mode> was passed to bitcoind.
  |
  | This experimental feature is available under
  | Linux x86_64 only.
  */
pub fn set_syscall_sandbox_policy(syscall_policy: SyscallSandboxPolicy)  {
    
    todo!();
        /*
            #if defined(USE_SYSCALL_SANDBOX)
        if (!g_syscall_sandbox_enabled) {
            return;
        }
        SeccompPolicyBuilder seccomp_policy_builder;
        switch (syscall_policy) {
        case SyscallSandboxPolicy::INITIALIZATION: // Thread: main thread (state: init)
            // SyscallSandboxPolicy::INITIALIZATION is the first policy loaded.
            //
            // Subsequently loaded policies can reduce the abilities further, but
            // abilities can never be regained.
            //
            // SyscallSandboxPolicy::INITIALIZATION must thus be a superset of all
            // other policies.
            seccomp_policy_builder.AllowFileSystem();
            seccomp_policy_builder.AllowNetwork();
            break;
        case SyscallSandboxPolicy::INITIALIZATION_DNS_SEED: // Thread: dnsseed
            seccomp_policy_builder.AllowFileSystem();
            seccomp_policy_builder.AllowNetwork();
            break;
        case SyscallSandboxPolicy::INITIALIZATION_LOAD_BLOCKS: // Thread: loadblk
            seccomp_policy_builder.AllowFileSystem();
            break;
        case SyscallSandboxPolicy::INITIALIZATION_MAP_PORT: // Thread: mapport
            seccomp_policy_builder.AllowFileSystem();
            seccomp_policy_builder.AllowNetwork();
            break;
        case SyscallSandboxPolicy::MESSAGE_HANDLER: // Thread: msghand
            seccomp_policy_builder.AllowFileSystem();
            break;
        case SyscallSandboxPolicy::NET: // Thread: net
            seccomp_policy_builder.AllowFileSystem();
            seccomp_policy_builder.AllowNetwork();
            break;
        case SyscallSandboxPolicy::NET_ADD_CONNECTION: // Thread: addcon
            seccomp_policy_builder.AllowFileSystem();
            seccomp_policy_builder.AllowNetwork();
            break;
        case SyscallSandboxPolicy::NET_HTTP_SERVER: // Thread: http
            seccomp_policy_builder.AllowFileSystem();
            seccomp_policy_builder.AllowNetwork();
            break;
        case SyscallSandboxPolicy::NET_HTTP_SERVER_WORKER: // Thread: httpworker.<N>
            seccomp_policy_builder.AllowFileSystem();
            seccomp_policy_builder.AllowNetwork();
            break;
        case SyscallSandboxPolicy::NET_OPEN_CONNECTION: // Thread: opencon
            seccomp_policy_builder.AllowFileSystem();
            seccomp_policy_builder.AllowNetwork();
            break;
        case SyscallSandboxPolicy::SCHEDULER: // Thread: scheduler
            seccomp_policy_builder.AllowFileSystem();
            break;
        case SyscallSandboxPolicy::TOR_CONTROL: // Thread: torcontrol
            seccomp_policy_builder.AllowFileSystem();
            seccomp_policy_builder.AllowNetwork();
            break;
        case SyscallSandboxPolicy::TX_INDEX: // Thread: txindex
            seccomp_policy_builder.AllowFileSystem();
            break;
        case SyscallSandboxPolicy::VALIDATION_SCRIPT_CHECK: // Thread: scriptch.<N>
            break;
        case SyscallSandboxPolicy::SHUTOFF: // Thread: main thread (state: shutoff)
            seccomp_policy_builder.AllowFileSystem();
            break;
        }

        const SyscallSandboxAction default_action = g_syscall_sandbox_log_violation_before_terminating ? SyscallSandboxAction::INVOKE_SIGNAL_HANDLER : SyscallSandboxAction::KILL_PROCESS;
        std::vector<sock_filter> filter = seccomp_policy_builder.BuildFilter(default_action);
        const sock_fprog prog = {
            .len = static_cast<uint16_t>(filter.size()),
            .filter = filter.data(),
        };
        // Do not allow abilities to be regained after being dropped.
        //
        // PR_SET_NO_NEW_PRIVS documentation: <https://www.kernel.org/doc/html/latest/userspace-api/no_new_privs.html>
        if (prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0) {
            throw std::runtime_error("Syscall sandbox enforcement failed: prctl(PR_SET_NO_NEW_PRIVS)");
        }
        // Install seccomp-bpf syscall filter.
        //
        // PR_SET_SECCOMP documentation: <https://www.kernel.org/doc/html/latest/userspace-api/seccomp_filter.html>
        if (prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &prog) != 0) {
            throw std::runtime_error("Syscall sandbox enforcement failed: prctl(PR_SET_SECCOMP)");
        }

        const std::string thread_name = !util::ThreadGetInternalName().empty() ? util::ThreadGetInternalName() : "*unnamed*";
        LogPrint(BCLog::UTIL, "Syscall filter installed for thread \"%s\"\n", thread_name);
    #endif // defined(USE_SYSCALL_SANDBOX)
        */
}
