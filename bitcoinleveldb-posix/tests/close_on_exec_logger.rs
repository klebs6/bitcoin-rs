// ---------------- [ File: bitcoinleveldb-posix/tests/close_on_exec_logger.rs ]
use bitcoinleveldb_posix::*;
use bitcoin_imports::*;

#[cfg(HAVE_O_CLOEXEC)]
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
        let mut logger: Option<Box<dyn Logger>> = None;
        let status = env.new_logger(&file_path, &mut logger);
        assert!(
            status.is_ok(),
            "env_posix_test_close_on_exec_logger: new_logger failed: {:?}",
            status
        );
        assert!(
            logger.is_some(),
            "env_posix_test_close_on_exec_logger: env.new_logger returned Ok but no logger"
        );

        check_close_on_exec_does_not_leak_fds(&baseline_open_fds);

        drop(logger);
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
