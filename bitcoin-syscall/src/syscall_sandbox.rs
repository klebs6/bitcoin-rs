// ---------------- [ File: bitcoin-syscall/src/syscall_sandbox.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/syscall_sandbox.h]

pub enum SyscallSandboxPolicy {

    /* --------------- 1. Initialization  --------------- */
    Initialization,
    InitializationDnsSeed,
    InitializationLoadBlocks,
    InitializationMapPort,

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

lazy_static!{
    static G_SYSCALL_SANDBOX_ENABLED:                          AtomicBool = AtomicBool::new(false);
    static G_SYSCALL_SANDBOX_LOG_VIOLATION_BEFORE_TERMINATING: AtomicBool = AtomicBool::new(false);
}

/// Exactly the same numeric values the C++ uses.
pub const SECCOMP_RET_KILL_PROCESS: usize = 0x8000_0000;
pub const SECCOMP_RET_TRAP:         usize = 0x0003_0000;
pub const SECCOMP_RET_ALLOW:        usize = 0x7fff_0000;

/*
  | Define system call numbers for x86_64 that are
  | referenced in the system call profile but not
  | provided by the kernel headers used in the GUIX
  | build.
  |
  | Usually, they can be found via "grep name
  | /usr/include/x86_64-linux-gnu/asm/unistd_64.h"
  */

pub const __NR_clone3:          usize = 435;
pub const __NR_statx:           usize = 332;
pub const __NR_getrandom:       usize = 318;
pub const __NR_membarrier:      usize = 324;
pub const __NR_copy_file_range: usize = 326;

pub enum SyscallSandboxAction {
    KILL_PROCESS,
    INVOKE_SIGNAL_HANDLER,
}
