// ---------------- [ File: bitcoin-compat/src/syscall_sandbox.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/syscall_sandbox.h]

pub enum SyscallSandboxPolicy {

    /* --------------- 1. Initialization  --------------- */
    INITIALIZATION,
    INITIALIZATION_DNS_SEED,
    INITIALIZATION_LOAD_BLOCKS,
    INITIALIZATION_MAP_PORT,

    /* 2. Steady state (non-initialization, non-shutdown) */
    MESSAGE_HANDLER,
    NET,
    NET_ADD_CONNECTION,
    NET_HTTP_SERVER,
    NET_HTTP_SERVER_WORKER,
    NET_OPEN_CONNECTION,
    SCHEDULER,
    TOR_CONTROL,
    TX_INDEX,
    VALIDATION_SCRIPT_CHECK,

    /* ------------------ 3. Shutdown  ------------------ */
    SHUTOFF,
}

//-------------------------------------------[.cpp/bitcoin/src/util/syscall_sandbox.cpp]

#[cfg(USE_SYSCALL_SANDBOX)]
lazy_static!{
    /*
    bool g_syscall_sandbox_enabled{false};
    bool g_syscall_sandbox_log_violation_before_terminating{false};
    */
}

#[cfg(USE_SYSCALL_SANDBOX)]
lazy_static!{
    /*
    #if !defined(__x86_64__)
    #error Syscall sandbox is an experimental feature currently available only under Linux x86-64.
    #endif // defined(__x86_64__)
    */
}

#[cfg(USE_SYSCALL_SANDBOX)]
#[cfg(not(SECCOMP_RET_KILL_PROCESS))]
pub const SECCOMP_RET_KILL_PROCESS: usize = 0x80000000;

/*
  | Define system call numbers for x86_64 that are
  | referenced in the system call profile but not
  | provided by the kernel headers used in the GUIX
  | build.
  |
  | Usually, they can be found via "grep name
  | /usr/include/x86_64-linux-gnu/asm/unistd_64.h"
  */

#[cfg(USE_SYSCALL_SANDBOX)] #[cfg(not(__NR_clone3))]          pub const __NR_clone3:          usize = 435;
#[cfg(USE_SYSCALL_SANDBOX)] #[cfg(not(__NR_statx))]           pub const __NR_statx:           usize = 332;
#[cfg(USE_SYSCALL_SANDBOX)] #[cfg(not(__NR_getrandom))]       pub const __NR_getrandom:       usize = 318;
#[cfg(USE_SYSCALL_SANDBOX)] #[cfg(not(__NR_membarrier))]      pub const __NR_membarrier:      usize = 324;
#[cfg(USE_SYSCALL_SANDBOX)] #[cfg(not(__NR_copy_file_range))] pub const __NR_copy_file_range: usize = 326;

/// Return the symbolic name of `syscall_number` if it
/// is present in `LINUX_SYSCALLS`; otherwise return
/// `"*unknown*"` (mirrors the C++ helper).
#[cfg(USE_SYSCALL_SANDBOX)]
#[inline]
pub fn get_linux_syscall_name(syscall_number: u32) -> String {
    trace!(
        target: "compat::syscall_sandbox",
        syscall_number,
        "lookup syscall name"
    );

    LINUX_SYSCALLS
        .get(&syscall_number)
        .copied()
        .unwrap_or("*unknown*")
        .to_owned()
}

#[cfg(USE_SYSCALL_SANDBOX)]
pub enum SyscallSandboxAction {
    KILL_PROCESS,
    INVOKE_SIGNAL_HANDLER,
}
