// ---------------- [ File: bitcoinleveldb-posix/src/get_max_file_descriptor.rs ]
crate::ix!();

/**
  | File descriptors are small non-negative
  | integers.
  |
  | Returns c_void so the implementation can use
  | ASSERT_EQ.
  */
#[cfg(have_o_cloexec)]
pub fn get_max_file_descriptor(result_fd: *mut i32) {
    trace!("get_max_file_descriptor: start");

    assert!(
        !result_fd.is_null(),
        "get_max_file_descriptor: result_fd pointer must not be null"
    );

    unsafe {
        let mut fd_rlimit: libc::rlimit = std::mem::zeroed();
        let rc = libc::getrlimit(libc::RLIMIT_NOFILE, &mut fd_rlimit as *mut libc::rlimit);
        assert_eq!(
            rc, 0,
            "get_max_file_descriptor: getrlimit(RLIMIT_NOFILE) failed with rc={}",
            rc
        );

        *result_fd = fd_rlimit.rlim_cur as i32;

        debug!(
            "get_max_file_descriptor: rlim_cur (soft limit) = {}",
            fd_rlimit.rlim_cur
        );
    }
}
