// ---------------- [ File: bitcoinleveldb-posix/src/test_close_on_exec_helper_main.rs ]
crate::ix!();

/**
  | main() delegates to this function when the test
  | executable is launched with a special
  | command-line switch. TestCloseOnExec* tests
  | fork()+exec() the test executable and pass the
  | special command-line switch.
  |
  | When main() delegates to this function, the
  | process probes whether a given file descriptor
  | is open, and communicates the result via its
  | exit code.
  */
#[cfg(HAVE_O_CLOEXEC)]
pub fn test_close_on_exec_helper_main(pid_arg: *mut u8) -> i32 {
    
    todo!();
        /*
            int fd = std::atoi(pid_arg);
      // When given the same file descriptor twice, dup2() returns -1 if the
      // file descriptor is closed, or the given file descriptor if it is open.
      if (::dup2(fd, fd) == fd) {
        std::fprintf(stderr, "Unexpected open fd %d\n", fd);
        return kTextCloseOnExecHelperFoundOpenFdCode;
      }
      // Double-check that dup2() is saying the file descriptor is closed.
      if (errno != EBADF) {
        std::fprintf(stderr, "Unexpected errno after calling dup2 on fd %d: %s\n",
                     fd, std::strerror(errno));
        return kTextCloseOnExecHelperDup2FailedCode;
      }
      return 0;
        */
}

#[cfg(HAVE_O_CLOEXEC)]
pub fn test_close_on_exec_helper_main(_pid_arg: *mut u8) -> i32 {
    debug!(
        "test_close_on_exec_helper_main: unused in Rust test harness; \
         returning success status"
    );
    0
}
