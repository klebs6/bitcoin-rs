// ---------------- [ File: bitcoinleveldb-posix/src/get_newly_opened_file_descriptor.rs ]
crate::ix!();

/**
  | Finds an FD open since a previous call to
  | GetOpenFileDescriptors().
  |
  | |baseline_open_fds| is the result of a previous
  | GetOpenFileDescriptors() call. Assumes that
  | exactly one FD was opened since that call.
  |
  | Returns c_void so the implementation can use
  | ASSERT_EQ.
  */
#[cfg(HAVE_O_CLOEXEC)]
pub fn get_newly_opened_file_descriptor(
    baseline_open_fds: &std::collections::HashSet<i32>,
    result_fd: *mut i32,
) {
    use std::collections::HashSet;

    trace!(
        "get_newly_opened_file_descriptor: baseline size = {}",
        baseline_open_fds.len()
    );

    assert!(
        !result_fd.is_null(),
        "get_newly_opened_file_descriptor: result_fd pointer must not be null"
    );

    unsafe {
        let mut current_open_fds: HashSet<i32> = HashSet::new();
        get_open_file_descriptors(&mut current_open_fds as *mut HashSet<i32>);

        for &fd in baseline_open_fds.iter() {
            if !current_open_fds.contains(&fd) {
                warn!(
                    "get_newly_opened_file_descriptor: baseline fd {} \
                     is no longer open; it was closed during test setup",
                    fd
                );
            }
            current_open_fds.remove(&fd);
        }

        match current_open_fds.len() {
            0 => {
                info!(
                    "get_newly_opened_file_descriptor: no newly opened descriptors detected"
                );
                *result_fd = -1;
            }
            1 => {
                let fd = *current_open_fds.iter().next().expect("non-empty set");
                debug!(
                    "get_newly_opened_file_descriptor: detected new descriptor fd={}",
                    fd
                );
                *result_fd = fd;
            }
            n => {
                error!(
                    "get_newly_opened_file_descriptor: expected at most one newly opened \
                     descriptor, but found {} ({:?})",
                    n, current_open_fds
                );
                let fd = *current_open_fds.iter().next().expect("non-empty set");
                *result_fd = fd;
            }
        }
    }
}
