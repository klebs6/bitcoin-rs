use bitcoin_syscall::*;
use bitcoin_imports::*;

#[cfg(all(test, USE_SYSCALL_SANDBOX))]
mod tests {
    use super::*;

    #[traced_test]
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
