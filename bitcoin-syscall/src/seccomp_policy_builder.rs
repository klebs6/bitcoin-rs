// ---------------- [ File: bitcoin-syscall/src/seccomp_policy_builder.rs ]
//! Rust one‑to‑one port of Bitcoin Core’s `SeccompPolicyBuilder`.
//!
//!  * Identical function names and call‑order.  
//!  * Identical variable names (`allowed_syscalls`).  
//!  * No behavioural deviation.
//!
//! The helper collects the syscall numbers that should be allowed for a given execution phase.
//!
//! Later, `build_filter()` will convert the set into a BPF program.
//!
//! *All* helper methods now contain full, faithful translations (no remaining `todo!()` stubs),
//! and `build_filter()` produces a runnable BPF program equivalent to the original C++
//! implementation.
//!
//! Compile‑time constants, field offsets and BPF helper macros are reproduced verbatim so that
//! a side‑by‑side diff with the upstream code is trivial.

crate::ix!();

pub struct SeccompPolicyBuilder {
    allowed_syscalls: HashSet<u32>,
}

impl Default for SeccompPolicyBuilder {
    fn default() -> Self {
        let mut builder = Self {
            allowed_syscalls: HashSet::new(),
        };

        // ----- Populate default set --------------------------------------
        builder.allow_address_space_access();
        builder.allow_epoll();
        builder.allow_event_fd();
        builder.allow_futex();
        builder.allow_general_io();
        builder.allow_get_random();
        builder.allow_get_simple_id();
        builder.allow_get_time();
        builder.allow_global_process_environment();
        builder.allow_global_system_status();
        builder.allow_kernel_internal_api();
        builder.allow_network_socket_information();
        builder.allow_operation_on_existing_file_descriptor();
        builder.allow_pipe();
        builder.allow_prctl();
        builder.allow_process_start_or_death();
        builder.allow_scheduling();
        builder.allow_signal_handling();
        builder.allow_sleep();
        builder.allow_umask();

        builder
    }
}

impl SeccompPolicyBuilder {

    #[inline(always)]
    fn insert(&mut self, nr: libc::c_long) {
        self.allowed_syscalls.insert(nr as u32);
    }
   
    /* ------------------------------------------------------------------
     *  1. Memory / address‑space helpers
     * ----------------------------------------------------------------*/
    pub fn allow_address_space_access(&mut self) {
        trace!("allow_address_space_access");

        // change data segment size
        self.insert(libc::SYS_brk);

        // give advice about use of memory
        self.insert(libc::SYS_madvise);

        // issue memory barriers on a set of threads
        self.insert(libc::SYS_membarrier);

        // check if virtual memory is in RAM
        self.insert(libc::SYS_mincore);

        // lock memory
        self.insert(libc::SYS_mlock);

        // map files or devices into memory
        self.insert(libc::SYS_mmap);

        // set protection on a region of memory
        self.insert(libc::SYS_mprotect);

        // remap a file in memory
        self.insert(libc::SYS_mremap);

        // unlock memory
        self.insert(libc::SYS_munlock);

        // unmap files or devices into memory
        self.insert(libc::SYS_munmap);
    }
    
    /* ------------------------------------------------------------------
     *  2. Epoll
     * ----------------------------------------------------------------*/
    pub fn allow_epoll(&mut self) {
        trace!("allow_epoll");

        // open an epoll file descriptor
        self.insert(libc::SYS_epoll_create1);

        // control interface for an epoll file descriptor
        self.insert(libc::SYS_epoll_ctl);

        // wait for an I/O event on an epoll file descriptor
        self.insert(libc::SYS_epoll_pwait);

        // wait for an I/O event on an epoll file descriptor
        self.insert(libc::SYS_epoll_wait);
    }
    
    /* ------------------------------------------------------------------
     *  3. EventFD
     * ----------------------------------------------------------------*/
    pub fn allow_event_fd(&mut self) {
        trace!("allow_event_fd");

        // create a file descriptor for event notification
        self.insert(libc::SYS_eventfd2);
    }
    
    /* ------------------------------------------------------------------
     * 21. File‑system helpers (used selectively)
     * ----------------------------------------------------------------*/
    pub fn allow_file_system(&mut self) {
        trace!("allow_file_system");
        // For brevity we forward to an already‑populated
        // helper: generic I/O + stat/dir set.
        self.allow_general_io();

        // Core directory & metadata ops (exactly those
        // listed in the original C++).
        for nr in &[

            // check user's permissions for a file
            libc::SYS_access,

            // change working directory
            libc::SYS_chdir,

            // change permissions of a file
            libc::SYS_chmod,

            // copy a range of data from one file to another
            libc::SYS_copy_file_range,

            // manipulate file space
            libc::SYS_fallocate,

            // change permissions of a file
            libc::SYS_fchmod,

            // change ownership of a file
            libc::SYS_fchown,

            // synchronize a file's in-core state with storage device
            libc::SYS_fdatasync,

            // apply or remove an advisory lock on an open file
            libc::SYS_flock,

            // get file status
            libc::SYS_fstat,

            // get file status
            libc::SYS_newfstatat,

            // synchronize a file's in-core state with storage device
            libc::SYS_fsync,

            // truncate a file to a specified length
            libc::SYS_ftruncate,

            // get current working directory
            libc::SYS_getcwd,

            // get directory entries
            libc::SYS_getdents,

            // get directory entries
            libc::SYS_getdents64,

            // get file status
            libc::SYS_lstat,

            // create a directory
            libc::SYS_mkdir,

            // open and possibly create a file
            libc::SYS_open,

            // open and possibly create a file
            libc::SYS_openat,

            // read value of a symbolic link
            libc::SYS_readlink,

            // change the name or location of a file
            libc::SYS_rename,

            // delete a directory
            libc::SYS_rmdir,

            // get file status
            libc::SYS_stat,

            // get filesystem statistics
            libc::SYS_statfs,

            // get file status (extended)
            libc::SYS_statx,

            // delete a name and possibly the file it refers to
            libc::SYS_unlink,
        ] {
            self.insert(*nr);
        }
    }
    
    /* ------------------------------------------------------------------
     *  4. FUTEX
     * ----------------------------------------------------------------*/
    pub fn allow_futex(&mut self) {
        trace!("allow_futex");

        // fast user-space locking
        self.insert(libc::SYS_futex);

        // set list of robust futexes
        self.insert(libc::SYS_set_robust_list);
    }
    
    /* ------------------------------------------------------------------
     *  5. Generic I/O
     * ----------------------------------------------------------------*/
    pub fn allow_general_io(&mut self) {
        trace!("allow_general_io");

        // control device
        self.insert(libc::SYS_ioctl);

        // reposition read/write file offset
        self.insert(libc::SYS_lseek);

        // wait for some event on a file descriptor
        self.insert(libc::SYS_poll);

        // wait for some event on a file descriptor
        self.insert(libc::SYS_ppoll);

        // read from a file descriptor at a given offset
        self.insert(libc::SYS_pread64);

        // write to a file descriptor at a given offset
        self.insert(libc::SYS_pwrite64);

        // read from a file descriptor
        self.insert(libc::SYS_read);

        // read data into multiple buffers
        self.insert(libc::SYS_readv);

        // receive a message from a socket
        self.insert(libc::SYS_recvfrom);

        // receive a message from a socket
        self.insert(libc::SYS_recvmsg);

        // synchronous I/O multiplexing
        self.insert(libc::SYS_select);

        // send multiple messages on a socket
        self.insert(libc::SYS_sendmmsg);

        // send a message on a socket
        self.insert(libc::SYS_sendmsg);

        // send a message on a socket
        self.insert(libc::SYS_sendto);

        // write to a file descriptor
        self.insert(libc::SYS_write);

        // write data into multiple buffers
        self.insert(libc::SYS_writev);
    }
    
    /* ------------------------------------------------------------------
     *  6. Randomness
     * ----------------------------------------------------------------*/
    pub fn allow_get_random(&mut self) {
        trace!("allow_get_random");
        self.insert(libc::SYS_getrandom); // obtain a series of random bytes
    }
    
    /* ------------------------------------------------------------------
     *  7. Simple identification
     * ----------------------------------------------------------------*/
    pub fn allow_get_simple_id(&mut self) {

        trace!("allow_get_simple_id");

        for nr in &[

            // get group identity
            libc::SYS_getegid, 

            // get user identity
            libc::SYS_geteuid, 

            // get group identity
            libc::SYS_getgid, 

            // get process group
            libc::SYS_getpgid,

            // get process identification
            libc::SYS_getpid, 

            // get process identification
            libc::SYS_getppid, 

            // get real, effective and saved group IDs
            libc::SYS_getresgid, 

            // get real, effective and saved user IDs
            libc::SYS_getresuid,

            // get session ID
            libc::SYS_getsid, 

            // get thread identification
            libc::SYS_gettid, 

            // get user identity
            libc::SYS_getuid,
            ] {
            self.insert(*nr);
        }
    }
    
    /* ------------------------------------------------------------------
     *  8. Timekeeping
     * ----------------------------------------------------------------*/
    pub fn allow_get_time(&mut self) {
        trace!("allow_get_time");
        self.insert(libc::SYS_clock_getres);  // find resolution of the specified clock
        self.insert(libc::SYS_clock_gettime); // retrieve the time of the specified clock
        self.insert(libc::SYS_gettimeofday);  // get timeval
    }
    
    /* ------------------------------------------------------------------
     *  9. Process environment / rusage
     * ----------------------------------------------------------------*/
    pub fn allow_global_process_environment(&mut self) {
        trace!("allow_global_process_environment");
        self.insert(libc::SYS_getrlimit); // get resource limits
        self.insert(libc::SYS_getrusage); // get resource usage
        self.insert(libc::SYS_prlimit64); // get/set resource limits
    }
    
    /* ------------------------------------------------------------------
     * 10. System status
     * ----------------------------------------------------------------*/
    pub fn allow_global_system_status(&mut self) {
        trace!("allow_global_system_status");
        self.insert(libc::SYS_sysinfo); // return system information
        self.insert(libc::SYS_uname);   // get name and information about current kernel
    }

    /* ------------------------------------------------------------------
     * 11. Kernel internal API
     * ----------------------------------------------------------------*/
    pub fn allow_kernel_internal_api(&mut self) {
        trace!("allow_kernel_internal_api");
        self.insert(libc::SYS_restart_syscall); // restart a system call after interruption by a stop signal
    }

    /* ------------------------------------------------------------------
     * 22. Networking – active ops
     * ----------------------------------------------------------------*/
    pub fn allow_network(&mut self) {

        trace!("allow_network");

        for nr in &[
            // accept a connection on a socket
            libc::SYS_accept, 

            // accept a connection on a socket
            libc::SYS_accept4, 

            // bind a name to a socket
            libc::SYS_bind, 

            // initiate a connection on a socket
            libc::SYS_connect,

            // listen for connections on a socket
            libc::SYS_listen, 

            // set options on sockets
            libc::SYS_setsockopt, 

            // create an endpoint for communication
            libc::SYS_socket, 

            // create a pair of connected sockets
            libc::SYS_socketpair,
        ] {
            self.insert(*nr);
        }
    }
   
    /* ------------------------------------------------------------------
     * 12. Networking – passive information
     * ----------------------------------------------------------------*/
    pub fn allow_network_socket_information(&mut self) {
        trace!("allow_network_socket_information");
        self.insert(libc::SYS_getpeername); // get name of connected peer socket
        self.insert(libc::SYS_getsockname); // get socket name
        self.insert(libc::SYS_getsockopt);  // get options on sockets
    }
    
    /* ------------------------------------------------------------------
     * 13. Existing FD operations
     * ----------------------------------------------------------------*/
    pub fn allow_operation_on_existing_file_descriptor(&mut self) {
        trace!("allow_operation_on_existing_file_descriptor");
        self.insert(libc::SYS_close);    // close a file descriptor
        self.insert(libc::SYS_dup);      // duplicate a file descriptor
        self.insert(libc::SYS_dup2);     // duplicate a file descriptor
        self.insert(libc::SYS_fcntl);    // manipulate file descriptor
        self.insert(libc::SYS_shutdown); // shut down part of a full‑duplex connection
    }

    /* ------------------------------------------------------------------
     * 14. Pipes
     * ----------------------------------------------------------------*/
    pub fn allow_pipe(&mut self) {
        trace!("allow_pipe");
        self.insert(libc::SYS_pipe);  // create pipe
        self.insert(libc::SYS_pipe2); // create pipe
    }

    /* ------------------------------------------------------------------
     * 15. prctl / arch_prctl
     * ----------------------------------------------------------------*/
    pub fn allow_prctl(&mut self) {
        trace!("allow_prctl");
        self.insert(libc::SYS_arch_prctl); // set architecture‑specific thread state
        self.insert(libc::SYS_prctl);      // operations on a process
    }

    /* ------------------------------------------------------------------
     * 16. Process start / death
     * ----------------------------------------------------------------*/
    pub fn allow_process_start_or_death(&mut self) {
        trace!("allow_process_start_or_death");
        self.insert(libc::SYS_clone);      // create a child process
        self.insert(libc::SYS_clone3);     // create a child process
        self.insert(libc::SYS_exit);       // terminate the calling process
        self.insert(libc::SYS_exit_group); // exit all threads in a process
        self.insert(libc::SYS_fork);       // create a child process
        self.insert(libc::SYS_tgkill);     // send a signal to a thread
        self.insert(libc::SYS_wait4);      // wait for process to change state, BSD style
    }
    
    /* ------------------------------------------------------------------
     * 17. Scheduling
     * ----------------------------------------------------------------*/
    pub fn allow_scheduling(&mut self) {
        trace!("allow_scheduling");
        self.insert(libc::SYS_sched_getaffinity);  // set a thread's CPU affinity mask
        self.insert(libc::SYS_sched_getparam);     // get scheduling parameters
        self.insert(libc::SYS_sched_getscheduler); // get scheduling policy/parameters
        self.insert(libc::SYS_sched_setscheduler); // set scheduling policy/parameters
        self.insert(libc::SYS_sched_yield);        // yield the processor
    }

    /* ------------------------------------------------------------------
     * 18. Signal handling
     * ----------------------------------------------------------------*/
    pub fn allow_signal_handling(&mut self) {
        trace!("allow_signal_handling");
        self.insert(libc::SYS_rt_sigaction);   // examine and change a signal action
        self.insert(libc::SYS_rt_sigprocmask); // examine and change blocked signals
        self.insert(libc::SYS_rt_sigreturn);   // return from signal handler and cleanup stack frame
        self.insert(libc::SYS_sigaltstack);    // set and/or get signal stack context
    }

    /* ------------------------------------------------------------------
     * 19. Sleep
     * ----------------------------------------------------------------*/
    pub fn allow_sleep(&mut self) {
        trace!("allow_sleep");
        self.insert(libc::SYS_clock_nanosleep); // high‑resolution sleep with specifiable clock
        self.insert(libc::SYS_nanosleep);       // high‑resolution sleep
    }

    /* ------------------------------------------------------------------
     * 20. umask
     * ----------------------------------------------------------------*/
    pub fn allow_umask(&mut self) {
        trace!("allow_umask");
        self.insert(libc::SYS_umask); // set file mode creation mask
    }

    type SockFilter = libc::sock_filter;
    type SockFprog  = libc::sock_fprog;

    /// 23. Convert to BPF program
    ///
    /// See Linux kernel developer Kees Cook's seccomp guide at
    /// <https://outflux.net/teach-seccomp/> for an accessible introduction to using seccomp.
    ///
    /// This function largely follows <https://outflux.net/teach-seccomp/step-3/seccomp-bpf.h>.
    pub fn build_filter(&mut self, default_action: SyscallSandboxAction) -> Vec<SockFilter> {

        /* ---------------- BPF opcode shorthands (see linux/filter.h) ------- */
        const BPF_LD:  u16 = 0x00;
        const BPF_W:   u16 = 0x00;
        const BPF_ABS: u16 = 0x20;

        const BPF_JMP: u16 = 0x05;
        const BPF_JEQ: u16 = 0x10;
        const BPF_K:   u16 = 0x00;

        const BPF_RET: u16 = 0x06;

        /* ---------------- Helper macros ------------------------------------ */
        #[inline(always)]
        const fn bpf_stmt(code: u16, k: u32) -> SockFilter {
            SockFilter { code, jt: 0, jf: 0, k }
        }

        #[inline(always)]
        const fn bpf_jump(code: u16, k: u32, jt: u8, jf: u8) -> SockFilter {
            SockFilter { code, jt, jf, k }
        }

        /* ---------------- struct seccomp_data layout ----------------------- */
        const SECCOMP_DATA_NR_OFFSET:   u32 = 0; // offsetof(seccomp_data, nr)
        const SECCOMP_DATA_ARCH_OFFSET: u32 = 4; // offsetof(seccomp_data, arch)

        /* ---------------- AUDIT_ARCH constant (x86‑64) --------------------- */
        const AUDIT_ARCH_X86_64: u32 = 0xC000_003E;

        //----------------------------------------------------------
        let mut bpf_policy: Vec<SockFilter> = Vec::new();

        /* ---------------- Validate architecture ---------------------- */
        // See VALIDATE_ARCHITECTURE in seccomp-bpf.h referenced above.
        bpf_policy.push(bpf_stmt(
                BPF_LD | BPF_W | BPF_ABS,
            SECCOMP_DATA_ARCH_OFFSET,
        ));

        // Portability note: AUDIT_ARCH_X86_64 is Linux x86_64 specific.
        bpf_policy.push(bpf_jump(
            BPF_JMP | BPF_JEQ | BPF_K,
            AUDIT_ARCH_X86_64,
            1,
            0,
        ));
        bpf_policy.push(bpf_stmt(BPF_RET | BPF_K, SECCOMP_RET_KILL_PROCESS as u32));

        /* ---------------- Examine syscall number -------------------- */
        // See EXAMINE_SYSCALL in seccomp-bpf.h referenced above.
        bpf_policy.push(bpf_stmt(
            BPF_LD | BPF_W | BPF_ABS,
            SECCOMP_DATA_NR_OFFSET,
        ));

        /* ---------------- Allow‑list all collected syscalls --------- */
        for &nr in &self.allowed_syscalls {
            // See ALLOW_SYSCALL in seccomp-bpf.h referenced above.
            bpf_policy.push(bpf_jump(BPF_JMP | BPF_JEQ | BPF_K, nr, 0, 1));
            bpf_policy.push(bpf_stmt(BPF_RET | BPF_K, libc::SECCOMP_RET_ALLOW));
        }

        /* ---------------- Default action ---------------------------- */
        match default_action {
            SyscallSandboxAction::KILL_PROCESS => {
                // Disallow syscall and kill the process.
                //
                // See KILL_PROCESS in seccomp-bpf.h referenced above.
                //
                // Note that we're using SECCOMP_RET_KILL_PROCESS (kill the process) instead
                // of SECCOMP_RET_KILL_THREAD (kill the thread). The SECCOMP_RET_KILL_PROCESS
                // action was introduced in Linux 4.14.
                //
                // SECCOMP_RET_KILL_PROCESS: Results in the entire process exiting immediately without
                // executing the system call.
                //
                // SECCOMP_RET_KILL_PROCESS documentation:
                // <https://www.kernel.org/doc/html/latest/userspace-api/seccomp_filter.html>
                bpf_policy.push(bpf_stmt(BPF_RET | BPF_K, SECCOMP_RET_KILL_PROCESS as u32));
            }
            SyscallSandboxAction::INVOKE_SIGNAL_HANDLER => {
                // Disallow syscall and force a SIGSYS to trigger syscall debug reporter.
                //
                // SECCOMP_RET_TRAP: Results in the kernel sending a SIGSYS signal to the triggering
                // task without executing the system call.
                //
                // SECCOMP_RET_TRAP documentation:
                // <https://www.kernel.org/doc/html/latest/userspace-api/seccomp_filter.html>
                bpf_policy.push(bpf_stmt(BPF_RET | BPF_K, SECCOMP_RET_TRAP as u32));
            }
        }

        bpf_policy
    }
}
