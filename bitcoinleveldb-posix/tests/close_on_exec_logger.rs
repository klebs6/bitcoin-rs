// ---------------- [ File: bitcoinleveldb-posix/tests/close_on_exec_logger.rs ]
use bitcoinleveldb_posix::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_log::*;
use bitcoin_imports::*;

#[cfg(have_o_cloexec)]
#[traced_test]
fn env_posix_test_close_on_exec_logger() {
    use std::collections::HashSet;

    info!("env_posix_test_close_on_exec_logger: start");

    let env_rc = make_posix_env_for_tests();

    let mut baseline_open_fds: HashSet<i32> = HashSet::new();
    get_open_file_descriptors(&mut baseline_open_fds as *mut HashSet<i32>);
    debug!(
        "env_posix_test_close_on_exec_logger: captured {} baseline FDs",
        baseline_open_fds.len()
    );

    let test_dir = {
        let mut env = env_rc.borrow_mut();
        let mut dir = String::new();
        let status = env.get_test_directory(&mut dir);
        assert!(
            status.is_ok(),
            "env_posix_test_close_on_exec_logger: get_test_directory failed: {:?}",
            status
        );
        dir
    };

    let file_path = format!("{}/close_on_exec_logger.txt", test_dir);
    info!(
        "env_posix_test_close_on_exec_logger: creating log file at {}",
        file_path
    );

    std::fs::write(&file_path, b"0123456789").expect(
        "env_posix_test_close_on_exec_logger: write initial data using std::fs::write",
    );

    {
        let mut env = env_rc.borrow_mut();

        let mut out_ptr: *mut Box<dyn Logger> = std::ptr::null_mut();
        let status = env.new_logger(
            &file_path,
            &mut out_ptr as *mut *mut Box<dyn Logger>,
        );
        assert!(
            status.is_ok(),
            "env_posix_test_close_on_exec_logger: new_logger failed: {:?}",
            status
        );
        assert!(
            !out_ptr.is_null(),
            "env_posix_test_close_on_exec_logger: \
             env.new_logger returned Ok but null pointer"
        );

        let logger_box: Box<dyn Logger> = unsafe { std::ptr::read(out_ptr) };

        check_close_on_exec_does_not_leak_fds(&baseline_open_fds);

        drop(logger_box);
    }

    if let Err(err) = std::fs::remove_file(&file_path) {
        error!(
            "env_posix_test_close_on_exec_logger: remove_file failed for {}: {:?}",
            file_path, err
        );
        panic!(
            "env_posix_test_close_on_exec_logger: remove_file failed: {:?}",
            err
        );
    }

    info!("env_posix_test_close_on_exec_logger: completed successfully");
}
