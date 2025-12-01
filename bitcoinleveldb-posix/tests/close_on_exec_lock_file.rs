// ---------------- [ File: bitcoinleveldb-posix/tests/close_on_exec_lock_file.rs ]
use bitcoinleveldb_posix::*;
use bitcoin_imports::*;

#[cfg(HAVE_O_CLOEXEC)]
#[traced_test]
fn env_posix_test_close_on_exec_lock_file() {
    use std::collections::HashSet;

    info!("env_posix_test_close_on_exec_lock_file: start");

    let env_rc = make_posix_env_for_tests();

    let mut baseline_open_fds: HashSet<i32> = HashSet::new();
    get_open_file_descriptors(&mut baseline_open_fds as *mut HashSet<i32>);
    debug!(
        "env_posix_test_close_on_exec_lock_file: captured {} baseline FDs",
        baseline_open_fds.len()
    );

    let test_dir = {
        let mut env = env_rc.borrow_mut();
        let mut dir = String::new();
        let status = env.get_test_directory(&mut dir);
        assert!(
            status.is_ok(),
            "env_posix_test_close_on_exec_lock_file: get_test_directory failed: {:?}",
            status
        );
        dir
    };

    let file_path = format!("{}/close_on_exec_lock.txt", test_dir);
    info!(
        "env_posix_test_close_on_exec_lock_file: creating test file at {}",
        file_path
    );

    std::fs::write(&file_path, b"0123456789").expect(
        "env_posix_test_close_on_exec_lock_file: write test data using std::fs::write",
    );

    {
        let mut env = env_rc.borrow_mut();
        let mut lock: Option<Box<dyn FileLock>> = None;
        let status = env.lock_file(&file_path, &mut lock);
        assert!(
            status.is_ok(),
            "env_posix_test_close_on_exec_lock_file: lock_file failed: {:?}",
            status
        );
        let mut lock = lock.expect(
            "env_posix_test_close_on_exec_lock_file: env.lock_file returned Ok but no lock",
        );

        check_close_on_exec_does_not_leak_fds(&baseline_open_fds);

        let status = env.unlock_file(lock.as_mut());
        assert!(
            status.is_ok(),
            "env_posix_test_close_on_exec_lock_file: unlock_file failed: {:?}",
            status
        );
    }

    if let Err(err) = std::fs::remove_file(&file_path) {
        error!(
            "env_posix_test_close_on_exec_lock_file: remove_file failed for {}: {:?}",
            file_path, err
        );
        panic!(
            "env_posix_test_close_on_exec_lock_file: remove_file failed: {:?}",
            err
        );
    }

    info!("env_posix_test_close_on_exec_lock_file: completed successfully");
}
