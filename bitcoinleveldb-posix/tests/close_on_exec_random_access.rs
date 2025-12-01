// ---------------- [ File: bitcoinleveldb-posix/tests/close_on_exec_random_access.rs ]
use bitcoinleveldb_posix::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_log::*;
use bitcoin_imports::*;

#[cfg(have_o_cloexec)]
#[traced_test]
fn env_posix_test_close_on_exec_random_access_file() {
    use std::collections::HashSet;

    info!("env_posix_test_close_on_exec_random_access_file: start");

    let env_rc = make_posix_env_for_tests();

    let mut baseline_open_fds: HashSet<i32> = HashSet::new();
    get_open_file_descriptors(&mut baseline_open_fds as *mut HashSet<i32>);
    debug!(
        "env_posix_test_close_on_exec_random_access_file: captured {} baseline FDs",
        baseline_open_fds.len()
    );

    let test_dir = {
        let mut env = env_rc.borrow_mut();
        let mut dir = String::new();
        let status = env.get_test_directory(&mut dir);
        assert!(
            status.is_ok(),
            "env_posix_test_close_on_exec_random_access_file: get_test_directory failed: {:?}",
            status
        );
        dir
    };

    let file_path = format!("{}/close_on_exec_random_access.txt", test_dir);
    info!(
        "env_posix_test_close_on_exec_random_access_file: creating test file at {}",
        file_path
    );

    std::fs::write(&file_path, b"0123456789").expect(
        "env_posix_test_close_on_exec_random_access_file: write test data using std::fs::write",
    );

    {
        let mut env = env_rc.borrow_mut();

        let mut warmup_files: Vec<Box<dyn RandomAccessFile>> =
            Vec::with_capacity(TEST_READ_ONLY_FILE_LIMIT as usize);

        for i in 0..TEST_READ_ONLY_FILE_LIMIT {
            let mut out_ptr: *mut Box<dyn RandomAccessFile> = std::ptr::null_mut();
            let status = env.new_random_access_file(
                &file_path,
                &mut out_ptr as *mut *mut Box<dyn RandomAccessFile>,
            );
            assert!(
                status.is_ok(),
                "env_posix_test_close_on_exec_random_access_file: \
                 warmup new_random_access_file failed at index {}: {:?}",
                i,
                status
            );

            if out_ptr.is_null() {
                warn!(
                    "env_posix_test_close_on_exec_random_access_file: \
                     warmup new_random_access_file returned Ok but null pointer at index {}",
                    i
                );
            } else {
                let handle: Box<dyn RandomAccessFile> = unsafe { std::ptr::read(out_ptr) };
                warmup_files.push(handle);
            }
        }

        let mut probed_ptr: *mut Box<dyn RandomAccessFile> = std::ptr::null_mut();
        let status = env.new_random_access_file(
            &file_path,
            &mut probed_ptr as *mut *mut Box<dyn RandomAccessFile>,
        );
        assert!(
            status.is_ok(),
            "env_posix_test_close_on_exec_random_access_file: \
             probed new_random_access_file failed: {:?}",
            status
        );
        assert!(
            !probed_ptr.is_null(),
            "env_posix_test_close_on_exec_random_access_file: \
             probed new_random_access_file returned Ok but null pointer"
        );

        let probed_file: Box<dyn RandomAccessFile> = unsafe { std::ptr::read(probed_ptr) };

        check_close_on_exec_does_not_leak_fds(&baseline_open_fds);

        drop(probed_file);
        drop(warmup_files);
    }

    if let Err(err) = std::fs::remove_file(&file_path) {
        error!(
            "env_posix_test_close_on_exec_random_access_file: remove_file failed for {}: {:?}",
            file_path, err
        );
        panic!(
            "env_posix_test_close_on_exec_random_access_file: remove_file failed: {:?}",
            err
        );
    }

    info!("env_posix_test_close_on_exec_random_access_file: completed successfully");
}
