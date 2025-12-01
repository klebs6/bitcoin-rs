// ---------------- [ File: bitcoinleveldb-posix/tests/open_on_read.rs ]
use bitcoinleveldb_posix::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_log::*;
use bitcoin_imports::*;

#[traced_test]
fn env_posix_test_open_on_read() {

    info!("env_posix_test_open_on_read: start");

    let env_rc = make_posix_env_for_tests();

    let test_dir = {
        let mut env = env_rc.borrow_mut();
        let mut dir = String::new();
        let status = env.get_test_directory(&mut dir);
        assert!(
            status.is_ok(),
            "env_posix_test_open_on_read: get_test_directory failed: {:?}",
            status
        );
        dir
    };

    let test_file = format!("{}/open_on_read.txt", test_dir);
    info!(
        "env_posix_test_open_on_read: creating test file at {}",
        test_file
    );

    {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(&test_file)
            .expect("env_posix_test_open_on_read: create test file");
        file.write_all(b"abcdefghijklmnopqrstuvwxyz")
            .expect("env_posix_test_open_on_read: write test data");
        file.sync_all()
            .expect("env_posix_test_open_on_read: sync test data");
    }

    let num_files_raw = TEST_READ_ONLY_FILE_LIMIT + TEST_MMAP_LIMIT + 5;
    assert!(
        num_files_raw > 0,
        "env_posix_test_open_on_read: computed non-positive file count"
    );
    let num_files: usize = num_files_raw as usize;

    info!(
        "env_posix_test_open_on_read: opening {} RandomAccessFile handles",
        num_files
    );

    let mut random_access_files: Vec<Box<dyn RandomAccessFile>> =
        Vec::with_capacity(num_files);

    {
        let mut env = env_rc.borrow_mut();

        for i in 0..num_files {
            debug!(
                "env_posix_test_open_on_read: creating RandomAccessFile handle {}",
                i
            );

            // -------------------------------------------------------------
            // CORRECT ABI: prepare a raw *mut *mut Box<dyn RandomAccessFile>
            // -------------------------------------------------------------
            let mut out_ptr: *mut Box<dyn RandomAccessFile> = std::ptr::null_mut();

            let status = env.new_random_access_file(
                &test_file,
                &mut out_ptr as *mut *mut Box<dyn RandomAccessFile>,
            );

            assert!(
                status.is_ok(),
                "env_posix_test_open_on_read: new_random_access_file failed at index {}: {:?}",
                i,
                status
            );

            assert!(
                !out_ptr.is_null(),
                "env_posix_test_open_on_read: new_random_access_file returned Ok but null pointer"
            );

            // -------------------------------------------------------------
            // Convert raw pointer to owned value
            // -------------------------------------------------------------
            let handle: Box<dyn RandomAccessFile> = unsafe { std::ptr::read(out_ptr) };
            random_access_files.push(handle);
        }
    }

    debug!(
        "env_posix_test_open_on_read: created {} RandomAccessFile handles; \
         dropping them to release resources",
        random_access_files.len()
    );

    random_access_files.clear();

    if let Err(err) = std::fs::remove_file(&test_file) {
        error!(
            "env_posix_test_open_on_read: failed to remove test file {}: {:?}",
            test_file, err
        );
        panic!(
            "env_posix_test_open_on_read: remove_file failed for {}: {:?}",
            test_file, err
        );
    }

    info!("env_posix_test_open_on_read: completed successfully");
}
