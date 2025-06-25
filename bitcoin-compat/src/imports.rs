// ---------------- [ File: bitcoin-compat/src/imports.rs ]
pub(crate) use bitcoin_imports::*;

#[cfg(all(USE_SYSCALL_SANDBOX, target_os = "linux", target_arch = "x86_64"))]
pub(crate) use {
    libc::{
        c_void, sigaction, sigaddset, sigemptyset, siginfo_t, sigprocmask, ucontext_t, SA_SIGINFO,
        SIGSYS, SIG_UNBLOCK,
    },
    std::ptr,
};
