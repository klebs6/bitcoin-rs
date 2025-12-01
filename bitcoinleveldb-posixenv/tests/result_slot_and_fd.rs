// ---------------- [ File: bitcoinleveldb-posixenv/tests/result_slot_and_fd.rs ]
use bitcoinleveldb_file::*;
use bitcoinleveldb_posixtools::*;
use bitcoinleveldb_posixenv::*;
use bitcoinleveldb_env::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

#[traced_test]
fn env_helper_initialize_and_store_result_slot() {
    trace!("env_helper_initialize_and_store_result_slot: start");

    let mut slot: *mut Box<usize> = std::ptr::null_mut();

    initialize_posix_env_result_slot::<usize>(
        "env_helper_initialize_and_store_result_slot",
        &mut slot,
    );

    assert!(
        slot.is_null(),
        "initialize_posix_env_result_slot should set output slot to null"
    );

    let boxed_value: Box<usize> = Box::new(123usize);

    let status = store_posix_env_boxed_result::<usize>(
        "env_helper_initialize_and_store_result_slot",
        &mut slot,
        boxed_value,
    );

    assert!(
        status.is_ok(),
        "store_posix_env_boxed_result should return OK, got {}",
        status.to_string()
    );

    assert!(
        !slot.is_null(),
        "store_posix_env_boxed_result should write a non-null pointer"
    );

    // Recover Box and assert content; also prevents leaking the allocation.
    let outer: Box<Box<usize>> = unsafe { Box::from_raw(slot) };

    assert_eq!(
        **outer, 123usize,
        "Recovered value from stored Box does not match expected"
    );

    info!("env_helper_initialize_and_store_result_slot: completed");
}

#[traced_test]
fn env_helper_open_posix_file_descriptor_success_and_not_found() {
    trace!(
        "env_helper_open_posix_file_descriptor_success_and_not_found: start"
    );

    let test_env = EnvTest::default();
    let env_rc   = test_env.env().clone();

    let mut base_dir = String::new();
    {
        let mut env = env_rc.borrow_mut();
        let status  = env.get_test_directory(&mut base_dir);
        assert!(
            status.is_ok(),
            "GetTestDirectory failed: {}",
            status.to_string()
        );
    }

    let existing_path = format!("{}/helper_open_fd_existing.txt", base_dir);

    // Best-effort delete any pre-existing file.
    {
        let mut env = env_rc.borrow_mut();
        let status  = env.delete_file(&existing_path);
        if !(status.is_ok() || status.is_not_found()) {
            warn!(
                file   = %existing_path,
                status = %status.to_string(),
                "env_helper_open_posix_file_descriptor_success_and_not_found: \
                 initial DeleteFile returned unexpected status (ignored)"
            );
        }
    }

    let flags_create = libc::O_TRUNC | libc::O_WRONLY | libc::O_CREAT | OPEN_BASE_FLAGS;
    let mode: libc::mode_t = 0o644;

    let fd_res = open_posix_file_descriptor(
        "env_helper_open_posix_file_descriptor_success_and_not_found",
        &existing_path,
        flags_create,
        mode,
    );

    let fd = match fd_res {
        Ok(fd) => fd,
        Err(status) => {
            panic!(
                "open_posix_file_descriptor (create) failed unexpectedly: {}",
                status.to_string()
            );
        }
    };

    unsafe {
        if libc::close(fd) != 0 {
            let errno = std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or(0);
            warn!(
                file  = %existing_path,
                fd,
                errno,
                "env_helper_open_posix_file_descriptor_success_and_not_found: \
                 close(fd) failed (ignored)"
            );
        }
    }

    // Now attempt to open a definitely non-existent file without O_CREAT.
    let missing_path = format!("{}/helper_open_fd_missing/child.txt", base_dir);
    let flags_read   = libc::O_RDONLY | OPEN_BASE_FLAGS;

    let missing_res = open_posix_file_descriptor(
        "env_helper_open_posix_file_descriptor_success_and_not_found",
        &missing_path,
        flags_read,
        0,
    );

    match missing_res {
        Ok(fd2) => {
            unsafe {
                libc::close(fd2);
            }
            panic!(
                "open_posix_file_descriptor unexpectedly succeeded on missing path {}",
                missing_path
            );
        }
        Err(status) => {
            assert!(
                status.is_not_found(),
                "Expected NotFound status for missing path, got {}",
                status.to_string()
            );
        }
    }

    info!(
        "env_helper_open_posix_file_descriptor_success_and_not_found: completed"
    );
}

#[traced_test]
fn env_helper_open_posix_log_stream_success() {
    trace!("env_helper_open_posix_log_stream_success: start");

    let test_env = EnvTest::default();
    let env_rc   = test_env.env().clone();

    let mut base_dir = String::new();
    {
        let mut env = env_rc.borrow_mut();
        let status  = env.get_test_directory(&mut base_dir);
        assert!(
            status.is_ok(),
            "GetTestDirectory failed: {}",
            status.to_string()
        );
    }

    let log_path = format!("{}/helper_open_log_stream.log", base_dir);

    // Use open_posix_file_descriptor to create the underlying fd.
    let flags = libc::O_APPEND | libc::O_WRONLY | libc::O_CREAT | OPEN_BASE_FLAGS;
    let mode: libc::mode_t = 0o644;

    let fd = open_posix_file_descriptor(
        "env_helper_open_posix_log_stream_success",
        &log_path,
        flags,
        mode,
    )
    .expect("open_posix_file_descriptor should succeed for log file");

    let fp_res = open_posix_log_stream(
        "env_helper_open_posix_log_stream_success",
        &log_path,
        fd,
        "w",
    );

    let fp = match fp_res {
        Ok(fp) => fp,
        Err(status) => {
            unsafe {
                libc::close(fd);
            }
            panic!(
                "open_posix_log_stream failed unexpectedly: {}",
                status.to_string()
            );
        }
    };

    assert!(
        !fp.is_null(),
        "open_posix_log_stream returned a null FILE*"
    );

    unsafe {
        if libc::fclose(fp) != 0 {
            let errno = std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or(0);
            warn!(
                file  = %log_path,
                errno,
                "env_helper_open_posix_log_stream_success: fclose() failed (ignored)"
            );
        }
    }

    info!("env_helper_open_posix_log_stream_success: completed");
}
