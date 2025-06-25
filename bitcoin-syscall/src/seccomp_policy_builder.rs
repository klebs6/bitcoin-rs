// ---------------- [ File: bitcoin-syscall/src/seccomp_policy_builder.rs ]
crate::ix!();

#[cfg(USE_SYSCALL_SANDBOX)]
pub struct SeccompPolicyBuilder {
    allowed_syscalls: HashSet<u32>,
}

#[cfg(USE_SYSCALL_SANDBOX)]
impl Default for SeccompPolicyBuilder {
    
    fn default() -> Self {
        todo!();
        /*


            // Allowed by default.
            AllowAddressSpaceAccess();
            AllowEpoll();
            AllowEventFd();
            AllowFutex();
            AllowGeneralIo();
            AllowGetRandom();
            AllowGetSimpleId();
            AllowGetTime();
            AllowGlobalProcessEnvironment();
            AllowGlobalSystemStatus();
            AllowKernelInternalApi();
            AllowNetworkSocketInformation();
            AllowOperationOnExistingFileDescriptor();
            AllowPipe();
            AllowPrctl();
            AllowProcessStartOrDeath();
            AllowScheduling();
            AllowSignalHandling();
            AllowSleep();
            AllowUmask();
        */
    }
}

#[cfg(USE_SYSCALL_SANDBOX)]
impl SeccompPolicyBuilder {
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_address_space_access(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_brk);        // change data segment size
            allowed_syscalls.insert(__NR_madvise);    // give advice about use of memory
            allowed_syscalls.insert(__NR_membarrier); // issue memory barriers on a set of threads
            allowed_syscalls.insert(__NR_mincore);    // check if virtual memory is in RAM
            allowed_syscalls.insert(__NR_mlock);      // lock memory
            allowed_syscalls.insert(__NR_mmap);       // map files or devices into memory
            allowed_syscalls.insert(__NR_mprotect);   // set protection on a region of memory
            allowed_syscalls.insert(__NR_mremap);     // remap a file in memory
            allowed_syscalls.insert(__NR_munlock);    // unlock memory
            allowed_syscalls.insert(__NR_munmap);     // unmap files or devices into memory
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_epoll(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_epoll_create1); // open an epoll file descriptor
            allowed_syscalls.insert(__NR_epoll_ctl);     // control interface for an epoll file descriptor
            allowed_syscalls.insert(__NR_epoll_pwait);   // wait for an I/O event on an epoll file descriptor
            allowed_syscalls.insert(__NR_epoll_wait);    // wait for an I/O event on an epoll file descriptor
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_event_fd(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_eventfd2); // create a file descriptor for event notification
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_file_system(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_access);          // check user's permissions for a file
            allowed_syscalls.insert(__NR_chdir);           // change working directory
            allowed_syscalls.insert(__NR_chmod);           // change permissions of a file
            allowed_syscalls.insert(__NR_copy_file_range); // copy a range of data from one file to another
            allowed_syscalls.insert(__NR_fallocate);       // manipulate file space
            allowed_syscalls.insert(__NR_fchmod);          // change permissions of a file
            allowed_syscalls.insert(__NR_fchown);          // change ownership of a file
            allowed_syscalls.insert(__NR_fdatasync);       // synchronize a file's in-core state with storage device
            allowed_syscalls.insert(__NR_flock);           // apply or remove an advisory lock on an open file
            allowed_syscalls.insert(__NR_fstat);           // get file status
            allowed_syscalls.insert(__NR_newfstatat);      // get file status
            allowed_syscalls.insert(__NR_fsync);           // synchronize a file's in-core state with storage device
            allowed_syscalls.insert(__NR_ftruncate);       // truncate a file to a specified length
            allowed_syscalls.insert(__NR_getcwd);          // get current working directory
            allowed_syscalls.insert(__NR_getdents);        // get directory entries
            allowed_syscalls.insert(__NR_getdents64);      // get directory entries
            allowed_syscalls.insert(__NR_lstat);           // get file status
            allowed_syscalls.insert(__NR_mkdir);           // create a directory
            allowed_syscalls.insert(__NR_open);            // open and possibly create a file
            allowed_syscalls.insert(__NR_openat);          // open and possibly create a file
            allowed_syscalls.insert(__NR_readlink);        // read value of a symbolic link
            allowed_syscalls.insert(__NR_rename);          // change the name or location of a file
            allowed_syscalls.insert(__NR_rmdir);           // delete a directory
            allowed_syscalls.insert(__NR_stat);            // get file status
            allowed_syscalls.insert(__NR_statfs);          // get filesystem statistics
            allowed_syscalls.insert(__NR_statx);           // get file status (extended)
            allowed_syscalls.insert(__NR_unlink);          // delete a name and possibly the file it refers to
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_futex(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_futex);           // fast user-space locking
            allowed_syscalls.insert(__NR_set_robust_list); // set list of robust futexes
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_general_io(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_ioctl);    // control device
            allowed_syscalls.insert(__NR_lseek);    // reposition read/write file offset
            allowed_syscalls.insert(__NR_poll);     // wait for some event on a file descriptor
            allowed_syscalls.insert(__NR_ppoll);    // wait for some event on a file descriptor
            allowed_syscalls.insert(__NR_pread64);  // read from a file descriptor at a given offset
            allowed_syscalls.insert(__NR_pwrite64); // write to a file descriptor at a given offset
            allowed_syscalls.insert(__NR_read);     // read from a file descriptor
            allowed_syscalls.insert(__NR_readv);    // read data into multiple buffers
            allowed_syscalls.insert(__NR_recvfrom); // receive a message from a socket
            allowed_syscalls.insert(__NR_recvmsg);  // receive a message from a socket
            allowed_syscalls.insert(__NR_select);   // synchronous I/O multiplexing
            allowed_syscalls.insert(__NR_sendmmsg); // send multiple messages on a socket
            allowed_syscalls.insert(__NR_sendmsg);  // send a message on a socket
            allowed_syscalls.insert(__NR_sendto);   // send a message on a socket
            allowed_syscalls.insert(__NR_write);    // write to a file descriptor
            allowed_syscalls.insert(__NR_writev);   // write data into multiple buffers
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_get_random(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_getrandom); // obtain a series of random bytes
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_get_simple_id(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_getegid);   // get group identity
            allowed_syscalls.insert(__NR_geteuid);   // get user identity
            allowed_syscalls.insert(__NR_getgid);    // get group identity
            allowed_syscalls.insert(__NR_getpgid);   // get process group
            allowed_syscalls.insert(__NR_getpid);    // get process identification
            allowed_syscalls.insert(__NR_getppid);   // get process identification
            allowed_syscalls.insert(__NR_getresgid); // get real, effective and saved group IDs
            allowed_syscalls.insert(__NR_getresuid); // get real, effective and saved user IDs
            allowed_syscalls.insert(__NR_getsid);    // get session ID
            allowed_syscalls.insert(__NR_gettid);    // get thread identification
            allowed_syscalls.insert(__NR_getuid);    // get user identity
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_get_time(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_clock_getres);  // find the resolution (precision) of the specified clock
            allowed_syscalls.insert(__NR_clock_gettime); // retrieve the time of the specified clock
            allowed_syscalls.insert(__NR_gettimeofday);  // get timeval
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_global_process_environment(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_getrlimit); // get resource limits
            allowed_syscalls.insert(__NR_getrusage); // get resource usage
            allowed_syscalls.insert(__NR_prlimit64); // get/set resource limits
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_global_system_status(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_sysinfo); // return system information
            allowed_syscalls.insert(__NR_uname);   // get name and information about current kernel
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_kernel_internal_api(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_restart_syscall); // restart a system call after interruption by a stop signal
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_network(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_accept);     // accept a connection on a socket
            allowed_syscalls.insert(__NR_accept4);    // accept a connection on a socket
            allowed_syscalls.insert(__NR_bind);       // bind a name to a socket
            allowed_syscalls.insert(__NR_connect);    // initiate a connection on a socket
            allowed_syscalls.insert(__NR_listen);     // listen for connections on a socket
            allowed_syscalls.insert(__NR_setsockopt); // set options on sockets
            allowed_syscalls.insert(__NR_socket);     // create an endpoint for communication
            allowed_syscalls.insert(__NR_socketpair); // create a pair of connected sockets
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_network_socket_information(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_getpeername); // get name of connected peer socket
            allowed_syscalls.insert(__NR_getsockname); // get socket name
            allowed_syscalls.insert(__NR_getsockopt);  // get options on sockets
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_operation_on_existing_file_descriptor(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_close);    // close a file descriptor
            allowed_syscalls.insert(__NR_dup);      // duplicate a file descriptor
            allowed_syscalls.insert(__NR_dup2);     // duplicate a file descriptor
            allowed_syscalls.insert(__NR_fcntl);    // manipulate file descriptor
            allowed_syscalls.insert(__NR_shutdown); // shut down part of a full-duplex connection
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_pipe(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_pipe);  // create pipe
            allowed_syscalls.insert(__NR_pipe2); // create pipe
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_prctl(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_arch_prctl); // set architecture-specific thread state
            allowed_syscalls.insert(__NR_prctl);      // operations on a process
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_process_start_or_death(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_clone);      // create a child process
            allowed_syscalls.insert(__NR_clone3);     // create a child process
            allowed_syscalls.insert(__NR_exit);       // terminate the calling process
            allowed_syscalls.insert(__NR_exit_group); // exit all threads in a process
            allowed_syscalls.insert(__NR_fork);       // create a child process
            allowed_syscalls.insert(__NR_tgkill);     // send a signal to a thread
            allowed_syscalls.insert(__NR_wait4);      // wait for process to change state, BSD style
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_scheduling(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_sched_getaffinity);  // set a thread's CPU affinity mask
            allowed_syscalls.insert(__NR_sched_getparam);     // get scheduling parameters
            allowed_syscalls.insert(__NR_sched_getscheduler); // get scheduling policy/parameters
            allowed_syscalls.insert(__NR_sched_setscheduler); // set scheduling policy/parameters
            allowed_syscalls.insert(__NR_sched_yield);        // yield the processor
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_signal_handling(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_rt_sigaction);   // examine and change a signal action
            allowed_syscalls.insert(__NR_rt_sigprocmask); // examine and change blocked signals
            allowed_syscalls.insert(__NR_rt_sigreturn);   // return from signal handler and cleanup stack frame
            allowed_syscalls.insert(__NR_sigaltstack);    // set and/or get signal stack context
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_sleep(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_clock_nanosleep); // high-resolution sleep with specifiable clock
            allowed_syscalls.insert(__NR_nanosleep);       // high-resolution sleep
        */
    }
    
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn allow_umask(&mut self)  {
        
        todo!();
        /*
            allowed_syscalls.insert(__NR_umask); // set file mode creation mask
        */
    }

    /**
      | See Linux kernel developer Kees Cook's
      | seccomp guide at
      | <https://outflux.net/teach-seccomp/> for an
      | accessible introduction to using seccomp.
      |
      | This function largely follows
      | <https://outflux.net/teach-seccomp/step-3/seccomp-bpf.h>.
      */
    #[cfg(USE_SYSCALL_SANDBOX)]
    pub fn build_filter(&mut self, default_action: SyscallSandboxAction) -> Vec<SockFilter> {
        
        todo!();
        /*
            std::vector<sock_filter> bpf_policy;
            // See VALIDATE_ARCHITECTURE in seccomp-bpf.h referenced above.
            bpf_policy.push_back(BPF_STMT(BPF_LD + BPF_W + BPF_ABS, offsetof(struct seccomp_data, arch)));
            // Portability note: AUDIT_ARCH_X86_64 is Linux x86_64 specific.
            bpf_policy.push_back(BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, AUDIT_ARCH_X86_64, 1, 0));
            bpf_policy.push_back(BPF_STMT(BPF_RET + BPF_K, SECCOMP_RET_KILL_PROCESS));
            // See EXAMINE_SYSCALL in seccomp-bpf.h referenced above.
            bpf_policy.push_back(BPF_STMT(BPF_LD + BPF_W + BPF_ABS, offsetof(struct seccomp_data, nr)));
            for (const uint32_t allowed_syscall : allowed_syscalls) {
                // See ALLOW_SYSCALL in seccomp-bpf.h referenced above.
                bpf_policy.push_back(BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, allowed_syscall, 0, 1));
                bpf_policy.push_back(BPF_STMT(BPF_RET + BPF_K, SECCOMP_RET_ALLOW));
            }
            switch (default_action) {
            case SyscallSandboxAction::KILL_PROCESS:
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
                bpf_policy.push_back(BPF_STMT(BPF_RET + BPF_K, SECCOMP_RET_KILL_PROCESS));
                break;
            case SyscallSandboxAction::INVOKE_SIGNAL_HANDLER:
                // Disallow syscall and force a SIGSYS to trigger syscall debug reporter.
                //
                // SECCOMP_RET_TRAP: Results in the kernel sending a SIGSYS signal to the triggering
                // task without executing the system call.
                //
                // SECCOMP_RET_TRAP documentation:
                // <https://www.kernel.org/doc/html/latest/userspace-api/seccomp_filter.html>
                bpf_policy.push_back(BPF_STMT(BPF_RET + BPF_K, SECCOMP_RET_TRAP));
                break;
            }
            return bpf_policy;
        */
    }
}
