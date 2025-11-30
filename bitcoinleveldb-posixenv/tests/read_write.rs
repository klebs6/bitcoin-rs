// ---------------- [ File: bitcoinleveldb-posixenv/tests/read_write.rs ]
use bitcoinleveldb_file::*;
use bitcoinleveldb_posixtools::*;
use bitcoinleveldb_posixenv::*;
use bitcoinleveldb_env::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

#[traced_test]
fn env_test_read_write() {
    use std::cmp;
    use std::ptr;

    trace!("env_test_read_write: start");

    let mut rnd = TestRandom::new(test_random_seed());

    // Get file to use for testing.
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

    let test_file_name = format!("{}/open_on_read.txt", test_dir);
    debug!(file = %test_file_name, "env_test_read_write: using test file");

    // Create writable file.
    let mut writable_file_ptr: *mut Box<dyn WritableFile> = ptr::null_mut();
    {
        let mut env = env_rc.borrow_mut();
        let status = env.new_writable_file(&test_file_name, &mut writable_file_ptr);
        assert!(
            status.is_ok(),
            "NewWritableFile failed: {}",
            status.to_string()
        );
    }

    // Take ownership of WritableFile (mirrors delete in C++).
    let mut writable_holder: Box<Box<dyn WritableFile>> = unsafe {
        assert!(
            !writable_file_ptr.is_null(),
            "Env::NewWritableFile returned null"
        );
        Box::from_raw(writable_file_ptr)
    };
    let writable_file: &mut Box<dyn WritableFile> = writable_holder.as_mut();

    // Fill a file with data generated via a sequence of randomly sized writes.
    const K_DATA_SIZE: usize = 10 * 1048576;
    let mut data = String::new();

    while data.len() < K_DATA_SIZE {
        let len = rnd.skewed(18); // Up to 2^18 - 1, typically smaller.
        let r = test_random_string(&mut rnd, len);
        let slice = Slice::from(&r);

        let status = writable_file.append(&slice);
        assert!(
            status.is_ok(),
            "WritableFile::Append failed: {}",
            status.to_string()
        );

        data.push_str(&r);

        if rnd.one_in(10) {
            let status = writable_file.flush();
            assert!(
                status.is_ok(),
                "WritableFile::Flush failed: {}",
                status.to_string()
            );
        }
    }

    let status = writable_file.sync();
    assert!(
        status.is_ok(),
        "WritableFile::Sync failed: {}",
        status.to_string()
    );
    let status = writable_file.close();
    assert!(
        status.is_ok(),
        "WritableFile::Close failed: {}",
        status.to_string()
    );
    // writable_holder drops here (delete writable_file).

    // Read all data using a sequence of randomly sized reads.
    let mut sequential_file_ptr: *mut Box<dyn SequentialFile> = ptr::null_mut();
    {
        let mut env = env_rc.borrow_mut();
        let status = env.new_sequential_file(&test_file_name, &mut sequential_file_ptr);
        assert!(
            status.is_ok(),
            "NewSequentialFile failed: {}",
            status.to_string()
        );
    }

    let mut sequential_holder: Box<Box<dyn SequentialFile>> = unsafe {
        assert!(
            !sequential_file_ptr.is_null(),
            "Env::NewSequentialFile returned null"
        );
        Box::from_raw(sequential_file_ptr)
    };
    let sequential_file: &mut Box<dyn SequentialFile> = sequential_holder.as_mut();

    let mut read_result = String::new();
    let mut scratch = Vec::<u8>::new();

    while read_result.len() < data.len() {
        let remaining = data.len() - read_result.len();
        let len = cmp::min(rnd.skewed(18), remaining);
        let scratch_len = cmp::max(len, 1);
        scratch.resize(scratch_len, 0u8);

        let mut read = Slice::default();
        let status = sequential_file.read(len, &mut read, scratch.as_mut_ptr());
        assert!(
            status.is_ok(),
            "SequentialFile::Read failed: {}",
            status.to_string()
        );

        let read_size = *read.size();
        if len > 0 {
            assert!(read_size > 0, "Expected read.size() > 0 when len > 0");
        }
        assert!(
            read_size <= len,
            "Read size {} exceeds requested len {}",
            read_size,
            len
        );

        if read_size > 0 {
            unsafe {
                let ptr = *read.data() as *const u8;
                let bytes = std::slice::from_raw_parts(ptr, read_size);
                let chunk = String::from_utf8_lossy(bytes);
                read_result.push_str(&chunk);
            }
        }
    }

    assert_eq!(
        read_result, data,
        "Data read back from file does not match data written"
    );
    // sequential_holder drops here (delete sequential_file).

    info!(
        file = %test_file_name,
        total_bytes = data.len(),
        "env_test_read_write: completed"
    );
}
