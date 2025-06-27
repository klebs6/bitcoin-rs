crate::ix!();

/// Return the symbolic name of `syscall_number` if it
/// is present in `LINUX_SYSCALLS`; otherwise return
/// `"*unknown*"` (mirrors the C++ helper).
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

#[cfg(test)]
mod get_linux_syscall_name_tests {

    #[traced_test]
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    fn known_syscall_is_resolved() {
        // `write` must be present in the table.
        let nr = libc::SYS_write as u32;
        assert_eq!(get_linux_syscall_name(nr), "write");
    }

    #[traced_test]
    fn unknown_syscall_falls_back() {
        // Choose an obviously invalid number.
        let nr = 0xffff_ffff;
        assert_eq!(get_linux_syscall_name(nr), "*unknown*");
    }
}
