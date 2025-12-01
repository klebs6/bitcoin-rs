// ---------------- [ File: bitcoinleveldb-posix/src/check_close_on_exec_does_not_leak_fds.rs ]
crate::ix!();

/**
  | Check that a fork()+exec()-ed child
  | process does not have an extra open FD.
  |
  */
#[cfg(HAVE_O_CLOEXEC)]
pub fn check_close_on_exec_does_not_leak_fds(
    baseline_open_fds: &std::collections::HashSet<i32>,
) {
    info!(
        "check_close_on_exec_does_not_leak_fds: baseline size = {}",
        baseline_open_fds.len()
    );

    let mut probed_fd: i32 = -1;
    get_newly_opened_file_descriptor(baseline_open_fds, &mut probed_fd as *mut i32);

    if probed_fd < 0 {
        info!(
            "check_close_on_exec_does_not_leak_fds: no new descriptors detected; \
             resource is likely backed purely by memory (e.g. mmap)"
        );
        return;
    }

    unsafe {
        let flags = libc::fcntl(probed_fd, libc::F_GETFD);
        assert!(
            flags != -1,
            "check_close_on_exec_does_not_leak_fds: fcntl(F_GETFD) failed for fd {}",
            probed_fd
        );

        debug!(
            "check_close_on_exec_does_not_leak_fds: fd {} has flags {:#x}",
            probed_fd, flags
        );

        assert!(
            flags & libc::FD_CLOEXEC != 0,
            "Expected FD_CLOEXEC to be set on fd {}, but it was not (flags={:#x})",
            probed_fd, flags
        );
    }

    info!(
        "check_close_on_exec_does_not_leak_fds: fd {} correctly has FD_CLOEXEC set",
        probed_fd
    );
}

