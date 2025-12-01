// ---------------- [ File: bitcoinleveldb-posix/src/get_open_file_descriptors.rs ]
crate::ix!();

/**
  | Iterates through all possible FDs and returns
  | the currently open ones.
  |
  | Returns c_void so the implementation can use
  | ASSERT_EQ.
  */
#[cfg(have_o_cloexec)]
pub fn get_open_file_descriptors(open_fds: *mut std::collections::HashSet<i32>) {
    use std::collections::HashSet;

    trace!("get_open_file_descriptors: start");

    assert!(
        !open_fds.is_null(),
        "get_open_file_descriptors: open_fds pointer must not be null"
    );

    unsafe {
        let open_fds_ref: &mut HashSet<i32> = &mut *open_fds;
        open_fds_ref.clear();

        let mut max_fd: i32 = 0;
        get_max_file_descriptor(&mut max_fd as *mut i32);

        debug!(
            "get_open_file_descriptors: probing descriptors in range [0, {})",
            max_fd
        );

        for fd in 0..max_fd {
            let dup_result = libc::dup2(fd, fd);
            // When given the same file descriptor twice, dup2() returns -1 if the
            // file descriptor is closed, or the given file descriptor if it is open.
            //
            // Double-check that dup2() is saying the fd is closed.
            if dup_result != fd {
                // Closed descriptor: dup2() returns -1. We do not assert on errno
                // here; the original C++ test did this only for debugging.
                continue;
            }

            open_fds_ref.insert(fd);
        }

        debug!(
            "get_open_file_descriptors: collected {} open descriptors",
            open_fds_ref.len()
        );
    }
}
