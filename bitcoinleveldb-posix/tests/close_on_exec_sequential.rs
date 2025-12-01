// ---------------- [ File: bitcoinleveldb-posix/tests/close_on_exec_sequential.rs ]
use bitcoinleveldb_posix::*;
use bitcoin_imports::*;

#[cfg(HAVE_O_CLOEXEC)]
#[traced_test]
fn env_posix_test_close_on_exec_sequential_file() {
    use std::collections::HashSet;

    info!("env_posix_test_close_on_exec_sequential_file: start");

    let env_rc = make_posix_env_for_tests();

    let mut baseline_open_fds: HashSet<i32> = HashSet::new();
    get_open_file_descriptors(&mut baseline_open_fds as *mut HashSet<i32>);
    debug!(
        "env_posix_test_close_on_exec_sequential_file: captured {} baseline FDs",
        baseline_open_fds.len()
    );

    let test_dir = {
        let mut env = env_rc.borrow_mut();
        let mut dir = String::new();
        let status = env.get_test_directory(&mut dir);
        assert!(
            status.is_ok(),
            "env_posix_test_close_on_exec_sequential_file: get_test_directory failed: {:?}",
            status
        );
        dir
    };

    let file_path = format!("{}/close_on_exec_sequential.txt", test_dir);
    info!(
        "env_posix_test_close_on_exec_sequential_file: creating test file at {}",
        file_path
    );

    std::fs::write(&file_path, b"0123456789").expect(
        "env_posix_test_close_on_exec_sequential_file: write test data using std::fs::write",
    );

    {
        let mut env = env_rc.borrow_mut();
        let mut file: Option<Box<dyn SequentialFile>> = None;
        let status = env.new_sequential_file(&file_path, &mut file);
        assert!(
            status.is_ok(),
            "env_posix_test_close_on_exec_sequential_file: new_sequential_file failed: {:?}",
            status
        );
        assert!(
            file.is_some(),
            "env_posix_test_close_on_exec_sequential_file: \
             env.new_sequential_file returned Ok but no file"
        );

        check_close_on_exec_does_not_leak_fds(&baseline_open_fds);

        drop(file);
    }

    if let Err(err) = std::fs::remove_file(&file_path) {
        error!(
            "env_posix_test_close_on_exec_sequential_file: remove_file failed for {}: {:?}",
            file_path, err
        );
        panic!(
            "env_posix_test_close_on_exec_sequential_file: remove_file failed: {:?}",
            err
        );
    }

    info!("env_posix_test_close_on_exec_sequential_file: completed successfully");
}

