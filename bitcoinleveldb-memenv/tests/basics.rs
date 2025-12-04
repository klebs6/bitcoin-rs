// ---------------- [ File: bitcoinleveldb-memenv/tests/basics.rs ]
use bitcoinleveldb_memenv::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_env::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/helpers/memenv/memenv_test.cc]

#[traced_test]
fn mem_env_test_basics() {
    let mut env = make_mem_env();

    let dir = "/dir".to_string();
    let missing = "/dir/non_existent".to_string();
    let fname_f = "/dir/f".to_string();

    // Create directory.
    let status = env.create_dir(&dir);
    assert!(status.is_ok());

    // Directory is empty.
    assert!(!env.file_exists(&missing));

    let mut file_size: u64 = 0;
    let status = env.get_file_size(&missing, &mut file_size as *mut u64);
    assert!(status.is_io_error());

    let mut children: Vec<String> = Vec::new();
    let status = env.get_children(&dir, &mut children as *mut Vec<String>);
    assert!(status.is_ok());
    assert_eq!(children.len(), 0);

    // Create a file.
    let mut writable_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
    let status = env.new_writable_file(
        &fname_f,
        &mut writable_ptr as *mut *mut Box<dyn WritableFile>,
    );
    assert!(status.is_ok());

    let mut size_zero: u64 = 0;
    let status = env.get_file_size(&fname_f, &mut size_zero as *mut u64);
    assert!(status.is_ok());
    assert_eq!(size_zero, 0);

    // Delete writable handle.
    unsafe {
        if !writable_ptr.is_null() {
            let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(writable_ptr);
        }
    }

    // File should now exist with size 0.
    assert!(env.file_exists(&fname_f));

    let mut size_check: u64 = 0;
    let status = env.get_file_size(&fname_f, &mut size_check as *mut u64);
    assert!(status.is_ok());
    assert_eq!(size_check, 0);

    let mut children_after_file: Vec<String> = Vec::new();
    let status =
        env.get_children(&dir, &mut children_after_file as *mut Vec<String>);
    assert!(status.is_ok());
    assert_eq!(children_after_file.len(), 1);
    assert_eq!(children_after_file[0], "f".to_string());

    // Write "abc" to the file via NewWritableFile.
    let mut wf_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
    let status = env.new_writable_file(
        &fname_f,
        &mut wf_ptr as *mut *mut Box<dyn WritableFile>,
    );
    assert!(status.is_ok());

    unsafe {
        assert!(!wf_ptr.is_null());
        let writable: &mut Box<dyn WritableFile> = &mut *wf_ptr;
        let slice = Slice::from("abc".as_bytes());
        let append_status =
            WritableFileAppend::append(writable.as_mut(), &slice);
        assert!(append_status.is_ok());

        let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(wf_ptr);
    }

    // Open appendable file and append "hello".
    let mut append_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
    let status = env.new_appendable_file(
        &fname_f,
        &mut append_ptr as *mut *mut Box<dyn WritableFile>,
    );
    assert!(status.is_ok());

    let mut size_after_append_open: u64 = 0;
    let status =
        env.get_file_size(&fname_f, &mut size_after_append_open as *mut u64);
    assert!(status.is_ok());
    assert_eq!(size_after_append_open, 3);

    unsafe {
        assert!(!append_ptr.is_null());
        let writable: &mut Box<dyn WritableFile> = &mut *append_ptr;
        let slice = Slice::from("hello".as_bytes());
        let append_status =
            WritableFileAppend::append(writable.as_mut(), &slice);
        assert!(append_status.is_ok());

        let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(append_ptr);
    }

    // Final size should be 8.
    let mut final_size: u64 = 0;
    let status =
        env.get_file_size(&fname_f, &mut final_size as *mut u64);
    assert!(status.is_ok());
    assert_eq!(final_size, 8);

    // Renaming: missing source should be an error.
    let fname_g = "/dir/g".to_string();
    let missing_src = "/dir/non_existent".to_string();

    let status = env.rename_file(&missing_src, &fname_g);
    assert!(status.is_io_error());

    // Rename "/dir/f" -> "/dir/g".
    let status = env.rename_file(&fname_f, &fname_g);
    assert!(status.is_ok());
    assert!(!env.file_exists(&fname_f));
    assert!(env.file_exists(&fname_g));

    let mut size_renamed: u64 = 0;
    let status =
        env.get_file_size(&fname_g, &mut size_renamed as *mut u64);
    assert!(status.is_ok());
    assert_eq!(size_renamed, 8);

    // Non-existent sequential/random access file must fail.
    let missing_seq = "/dir/non_existent".to_string();

    let mut seq_ptr: *mut Box<dyn SequentialFile> = core::ptr::null_mut();
    let status = env.new_sequential_file(
        &missing_seq,
        &mut seq_ptr as *mut *mut Box<dyn SequentialFile>,
    );
    assert!(status.is_io_error());
    unsafe {
        assert!(seq_ptr.is_null());
    }

    let mut rand_ptr: *mut Box<dyn RandomAccessFile> = core::ptr::null_mut();
    let status = env.new_random_access_file(
        &missing_seq,
        &mut rand_ptr as *mut *mut Box<dyn RandomAccessFile>,
    );
    assert!(status.is_io_error());
    unsafe {
        assert!(rand_ptr.is_null());
    }

    // Delete non-existent file should be error; deleting existing is OK.
    let status_missing_delete = env.delete_file(&missing_seq);
    assert!(status_missing_delete.is_io_error());

    let status_delete_g = env.delete_file(&fname_g);
    assert!(status_delete_g.is_ok());
    assert!(!env.file_exists(&fname_g));

    let mut children_after_delete: Vec<String> = Vec::new();
    let status = env.get_children(
        &dir,
        &mut children_after_delete as *mut Vec<String>,
    );
    assert!(status.is_ok());
    assert_eq!(children_after_delete.len(), 0);

    let status = env.delete_dir(&dir);
    assert!(status.is_ok());
}

#[traced_test]
fn mem_env_test_misc() {
    let mut env = make_mem_env();

    // GetTestDirectory should return a non-empty path.
    let mut test_dir = String::new();
    let status = env.get_test_directory(&mut test_dir as *mut String);
    assert!(status.is_ok());
    assert!(!test_dir.is_empty());

    // NewWritableFile + Sync/Flush/Close should all succeed.
    let fname = "/a/b".to_string();
    let mut wf_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();

    let status = env.new_writable_file(
        &fname,
        &mut wf_ptr as *mut *mut Box<dyn WritableFile>,
    );
    assert!(status.is_ok());

    unsafe {
        if !wf_ptr.is_null() {
            let writable: &mut Box<dyn WritableFile> = &mut *wf_ptr;

            let sync_status = WritableFileSync::sync(writable.as_mut());
            assert!(sync_status.is_ok());

            let flush_status = WritableFileFlush::flush(writable.as_mut());
            assert!(flush_status.is_ok());

            let close_status = WritableFileClose::close(writable.as_mut());
            assert!(close_status.is_ok());

            let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(wf_ptr);
        }
    }
}
