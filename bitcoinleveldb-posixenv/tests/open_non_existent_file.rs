// ---------------- [ File: bitcoinleveldb-posixenv/tests/open_non_existent_file.rs ]
use bitcoinleveldb_file::*;
use bitcoinleveldb_posixtools::*;
use bitcoinleveldb_posixenv::*;
use bitcoinleveldb_env::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

#[traced_test]
fn env_test_open_non_existent_file() {
    use std::ptr;

    trace!("env_test_open_non_existent_file: start");

    let test_env = EnvTest::default();
    let env_rc = test_env.env().clone();

    let mut test_dir = String::new();
    {
        let mut env = env_rc.borrow_mut();
        let status = env.get_test_directory(&mut test_dir);
        assert!(
            status.is_ok(),
            "GetTestDirectory failed: {}",
            status.to_string()
        );
    }

    let non_existent_file = format!("{}/non_existent_file", test_dir);
    debug!(
        file = %non_existent_file,
        "env_test_open_non_existent_file: using non-existent file"
    );

    {
        let mut env = env_rc.borrow_mut();
        assert!(
            !env.file_exists(&non_existent_file),
            "File unexpectedly exists before test"
        );
    }

    // RandomAccessFile case.
    let mut random_access_ptr: *mut Box<dyn RandomAccessFile> = ptr::null_mut();
    let status = {
        let mut env = env_rc.borrow_mut();
        env.new_random_access_file(&non_existent_file, &mut random_access_ptr)
    };
    assert!(
        status.is_not_found(),
        "Expected NotFound from NewRandomAccessFile, got {}",
        status.to_string()
    );

    // SequentialFile case.
    let mut sequential_ptr: *mut Box<dyn SequentialFile> = ptr::null_mut();
    let status = {
        let mut env = env_rc.borrow_mut();
        env.new_sequential_file(&non_existent_file, &mut sequential_ptr)
    };
    assert!(
        status.is_not_found(),
        "Expected NotFound from NewSequentialFile, got {}",
        status.to_string()
    );

    info!("env_test_open_non_existent_file: completed");
}
