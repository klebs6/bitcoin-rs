// ---------------- [ File: bitcoin-syscall/src/set_syscall_sandbox_policy.rs ]
crate::ix!();

/// Force the current thread (and threads created from the current thread) into
/// a restricted-service operating mode where only a subset of all syscalls are
/// available.
/// 
/// Subsequent calls to this function can reduce the abilities further, but
/// abilities can never be regained.
/// 
/// This function is a no-op unless SetupSyscallSandbox(...) has been called.
/// 
/// SetupSyscallSandbox(...) is called during bitcoind initialization if Bitcoin
/// Core was compiled with seccomp-bpf support (--with-seccomp) *and* the
/// parameter -sandbox=<mode> was passed to bitcoind.
/// 
/// This experimental feature is available under Linux x86_64 only.
///
pub fn set_syscall_sandbox_policy(policy: SyscallSandboxPolicy) {
    // If the sandbox is not enabled yet, this is a no‑op – identical to C++.
    if !g_syscall_sandbox_enabled.load(atomic::Ordering::SeqCst) {
        return;
    }

    /* ----------- build the allow‑list ---------------------------------- */
    let mut b = SeccompPolicyBuilder::default();

    match policy {
        /* -------- 1. initialisation‑phase buckets ---------- */
        // SyscallSandboxPolicy::INITIALIZATION is the first policy loaded.
        //
        // Subsequently loaded policies can reduce the abilities further, but
        // abilities can never be regained.
        //
        // SyscallSandboxPolicy::INITIALIZATION must thus be a superset of all
        // other policies.
        SyscallSandboxPolicy::INITIALIZATION
            | SyscallSandboxPolicy::INITIALIZATION_DNS_SEED
            | SyscallSandboxPolicy::INITIALIZATION_MAP_PORT => {
            b.allow_file_system();
            b.allow_network();
        }
        SyscallSandboxPolicy::INITIALIZATION_LOAD_BLOCKS => {
            b.allow_file_system();
        }

        /* -------- 2. steady‑state buckets ------------------ */
        SyscallSandboxPolicy::MESSAGE_HANDLER
        | SyscallSandboxPolicy::SCHEDULER
        | SyscallSandboxPolicy::TX_INDEX => {
            b.allow_file_system();
        }
        SyscallSandboxPolicy::NET
        | SyscallSandboxPolicy::NET_ADD_CONNECTION
        | SyscallSandboxPolicy::NET_HTTP_SERVER
        | SyscallSandboxPolicy::NET_HTTP_SERVER_WORKER
        | SyscallSandboxPolicy::NET_OPEN_CONNECTION
        | SyscallSandboxPolicy::TOR_CONTROL => {
            b.allow_file_system();
            b.allow_network();
        }
        SyscallSandboxPolicy::VALIDATION_SCRIPT_CHECK => {
            /* nothing extra */
        }

        /* -------- 3. shutdown bucket ---------------------- */
        SyscallSandboxPolicy::SHUTOFF => {
            b.allow_file_system();
        }
    }

    /* ----------- compile the BPF programme ----------------------------- */
    let default_action = if g_syscall_sandbox_log_violation_before_terminating
        .load(atomic::Ordering::SeqCst)
    {
        SyscallSandboxAction::INVOKE_SIGNAL_HANDLER
    } else {
        SyscallSandboxAction::KILL_PROCESS
    };

    let filter = b.build_filter(default_action);

    let mut prog = libc::sock_fprog {
        len: filter.len() as libc::c_ushort,
        filter: filter.as_ptr() as *mut _,
    };

    /* ----------- hand the filter to the kernel ------------------------- */
    // Do not allow abilities to be regained after being dropped.
    //
    // PR_SET_NO_NEW_PRIVS documentation: <https://www.kernel.org/doc/html/latest/userspace-api/no_new_privs.html>
    unsafe {

        // Prevent regaining privileges after seccomp is active
        if libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
            panic!("Syscall sandbox enforcement failed: prctl(PR_SET_NO_NEW_PRIVS)");
        }

        // Install seccomp-bpf syscall filter itself.
        //
        // PR_SET_SECCOMP documentation: <https://www.kernel.org/doc/html/latest/userspace-api/seccomp_filter.html>
        if libc::prctl(
            libc::PR_SET_SECCOMP,
            libc::SECCOMP_MODE_FILTER,
            &mut prog as *mut _,
        ) != 0
        {
            panic!("Syscall sandbox enforcement failed: prctl(PR_SET_SECCOMP)");
        }
    }

    let thread = thread_get_internal_name();

    info!{
        target: "compat::syscall_sandbox",
        "Syscall filter installed for thread \"{}\"",
        if thread.is_empty() { "*unnamed*" } else { &thread }
    };
}
