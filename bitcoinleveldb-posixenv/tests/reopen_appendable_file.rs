// ---------------- [ File: bitcoinleveldb-posixenv/tests/reopen_appendable_file.rs ]
use bitcoinleveldb_file::*;
use bitcoinleveldb_posixtools::*;
use bitcoinleveldb_posixenv::*;
use bitcoinleveldb_env::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

#[traced_test]
fn env_test_reopen_appendable_file() {
    use std::cmp;
    use std::ptr;

    trace!("env_test_reopen_appendable_file: start");

    let test_env = EnvTest::default();
    let env_rc = test_env.env().clone();

    // Determine test directory.
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

    let test_file_name = format!("{}/reopen_appendable_file.txt", test_dir);
    debug!(
        file = %test_file_name,
        "env_test_reopen_appendable_file: using test file"
    );

    // Best-effort delete any pre-existing file.
    {
        let mut env = env_rc.borrow_mut();
        let status = env.delete_file(&test_file_name);
        if !(status.is_ok() || status.is_not_found()) {
            warn!(
                file   = %test_file_name,
                status = %status.to_string(),
                "env_test_reopen_appendable_file: initial DeleteFile returned unexpected status (ignored)"
            );
        }
    }

    // ---- First creation: appendable, write "hello world!", close. ----
    let mut appendable_file_ptr: *mut Box<dyn WritableFile> = ptr::null_mut();
    {
        let mut env = env_rc.borrow_mut();
        let status = bitcoinleveldb_env::Env::new_appendable_file(
            &mut *env,
            &test_file_name,
            &mut appendable_file_ptr,
        );
        assert!(
            status.is_ok(),
            "NewAppendableFile (first open) failed: {}",
            status.to_string()
        );
    }

    let mut appendable_holder: Box<Box<dyn WritableFile>> = unsafe {
        assert!(
            !appendable_file_ptr.is_null(),
            "Env::NewAppendableFile (first open) returned null"
        );
        Box::from_raw(appendable_file_ptr)
    };
    let appendable_file: &mut Box<dyn WritableFile> = appendable_holder.as_mut();

    let mut data = String::from("hello world!");
    let slice = Slice::from(&data);
    let status = appendable_file.append(&slice);
    assert!(
        status.is_ok(),
        "WritableFile::Append(\"hello world!\") failed: {}",
        status.to_string()
    );

    let status = appendable_file.close();
    assert!(
        status.is_ok(),
        "WritableFile::Close (first appendable open) failed: {}",
        status.to_string()
    );
    drop(appendable_holder);

    // ---- Second creation: appendable, write "42" (appends), close. ----
    let mut appendable_file_ptr2: *mut Box<dyn WritableFile> = ptr::null_mut();
    {
        let mut env = env_rc.borrow_mut();
        let status = bitcoinleveldb_env::Env::new_appendable_file(
            &mut *env,
            &test_file_name,
            &mut appendable_file_ptr2,
        );
        assert!(
            status.is_ok(),
            "NewAppendableFile (second open) failed: {}",
            status.to_string()
        );
    }

    let mut appendable_holder2: Box<Box<dyn WritableFile>> = unsafe {
        assert!(
            !appendable_file_ptr2.is_null(),
            "Env::NewAppendableFile (second open) returned null"
        );
        Box::from_raw(appendable_file_ptr2)
    };
    let appendable_file2: &mut Box<dyn WritableFile> = appendable_holder2.as_mut();

    data = "42".to_owned();
    let slice = Slice::from(&data);
    let status = appendable_file2.append(&slice);
    assert!(
        status.is_ok(),
        "WritableFile::Append(\"42\") failed: {}",
        status.to_string()
    );

    let status = appendable_file2.close();
    assert!(
        status.is_ok(),
        "WritableFile::Close (second appendable open) failed: {}",
        status.to_string()
    );
    drop(appendable_holder2);

    // ---- Read file back and verify contents are "hello world!42". ----
    let mut file_size: u64 = 0;
    {
        let mut env = env_rc.borrow_mut();
        let status = env.get_file_size(&test_file_name, &mut file_size);
        assert!(
            status.is_ok(),
            "GetFileSize failed: {}",
            status.to_string()
        );
    }
    debug!(
        file      = %test_file_name,
        file_size,
        "env_test_reopen_appendable_file: reading back contents"
    );

    let mut sequential_file_ptr: *mut Box<dyn SequentialFile> = ptr::null_mut();
    {
        let mut env = env_rc.borrow_mut();
        let status = env.new_sequential_file(&test_file_name, &mut sequential_file_ptr);
        assert!(
            status.is_ok(),
            "NewSequentialFile (readback) failed: {}",
            status.to_string()
        );
    }

    let mut sequential_holder: Box<Box<dyn SequentialFile>> = unsafe {
        assert!(
            !sequential_file_ptr.is_null(),
            "Env::NewSequentialFile (readback) returned null"
        );
        Box::from_raw(sequential_file_ptr)
    };
    let sequential_file: &mut Box<dyn SequentialFile> = sequential_holder.as_mut();

    let mut result    = String::new();
    let mut remaining = file_size as usize;
    let mut scratch   = Vec::<u8>::new();

    while remaining > 0 {
        let chunk_len = cmp::max(remaining, 1);
        scratch.resize(chunk_len, 0u8);

        let mut read_slice = Slice::default();
        let status = sequential_file.read(chunk_len, &mut read_slice, scratch.as_mut_ptr());
        assert!(
            status.is_ok(),
            "SequentialFile::Read (reopen_appendable) failed: {}",
            status.to_string()
        );

        let read_size = *read_slice.size();
        if read_size == 0 {
            break;
        }
        assert!(
            read_size <= chunk_len,
            "Read size {} exceeds requested chunk_len {}",
            read_size,
            chunk_len
        );

        unsafe {
            let ptr   = *read_slice.data() as *const u8;
            let bytes = std::slice::from_raw_parts(ptr, read_size);
            result.push_str(&String::from_utf8_lossy(bytes));
        }

        remaining = remaining.saturating_sub(read_size);
    }

    assert_eq!(
        result,
        "hello world!42",
        "File contents after reopen-append should be \"hello world!42\""
    );

    // Cleanup: delete test file (ignore NotFound).
    {
        let mut env = env_rc.borrow_mut();
        let status = env.delete_file(&test_file_name);
        if !(status.is_ok() || status.is_not_found()) {
            warn!(
                file   = %test_file_name,
                status = %status.to_string(),
                "env_test_reopen_appendable_file: final DeleteFile returned unexpected status (ignored)"
            );
        }
    }

    info!(
        file = %test_file_name,
        "env_test_reopen_appendable_file: completed"
    );
}
