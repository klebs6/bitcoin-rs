use bitcoinleveldb_posixenv::*;
use bitcoinleveldb_posixtools::*;
use bitcoin_imports::*;

#[traced_test]
fn posix_open_file_descriptor_non_existent_path_returns_not_found() {
    trace!(
        "posix_open_file_descriptor_non_existent_path_returns_not_found: start"
    );

    let tmp_dir = std::env::temp_dir();
    let file_path = tmp_dir.join("posixenv_open_helper_non_existent.tmp");
    let file_str = file_path
        .to_str()
        .expect("temp path should be valid UTF-8")
        .to_owned();

    // Ensure the file does not exist.
    let _ = std::fs::remove_file(&file_path);

    let result = open_posix_file_descriptor(
        "posix_open_file_descriptor_non_existent_path_returns_not_found",
        &file_str,
        libc::O_RDONLY | OPEN_BASE_FLAGS,
        0,
    );

    match result {
        Ok(fd) => {
            unsafe {
                libc::close(fd);
            }
            panic!(
                "Expected NotFound status, but open_posix_file_descriptor returned fd={fd}"
            );
        }
        Err(status) => {
            assert!(
                status.is_not_found(),
                "Expected NotFound, got {}",
                status.to_string()
            );
        }
    }

    info!(
        file = %file_str,
        "posix_open_file_descriptor_non_existent_path_returns_not_found: completed"
    );
}

#[traced_test]
fn posix_open_file_descriptor_can_open_existing_file_readonly() {
    trace!(
        "posix_open_file_descriptor_can_open_existing_file_readonly: start"
    );

    let tmp_dir = std::env::temp_dir();
    let file_path = tmp_dir.join("posixenv_open_helper_existing.tmp");
    let file_str = file_path
        .to_str()
        .expect("temp path should be valid UTF-8")
        .to_owned();

    std::fs::write(&file_path, b"hello world").expect("failed to create test file");

    let result = open_posix_file_descriptor(
        "posix_open_file_descriptor_can_open_existing_file_readonly",
        &file_str,
        libc::O_RDONLY | OPEN_BASE_FLAGS,
        0,
    );

    let fd = match result {
        Ok(fd) => fd,
        Err(status) => {
            panic!(
                "Expected Ok(fd) opening existing file, got {}",
                status.to_string()
            );
        }
    };

    debug!(
        file = %file_str,
        fd,
        "posix_open_file_descriptor_can_open_existing_file_readonly: open succeeded"
    );

    unsafe {
        libc::close(fd);
    }

    std::fs::remove_file(&file_path).expect("failed to remove test file");

    info!(
        file = %file_str,
        "posix_open_file_descriptor_can_open_existing_file_readonly: completed"
    );
}

#[traced_test]
fn posix_open_log_stream_can_create_and_write_log_file() {
    trace!("posix_open_log_stream_can_create_and_write_log_file: start");

    let tmp_dir = std::env::temp_dir();
    let file_path = tmp_dir.join("posixenv_log_helper.log");
    let file_str = file_path
        .to_str()
        .expect("temp path should be valid UTF-8")
        .to_owned();

    // Best-effort cleanup before the test.
    let _ = std::fs::remove_file(&file_path);

    let flags = libc::O_APPEND | libc::O_WRONLY | libc::O_CREAT | OPEN_BASE_FLAGS;
    let mode: libc::mode_t = 0o644;

    let fd = match open_posix_file_descriptor(
        "posix_open_log_stream_can_create_and_write_log_file",
        &file_str,
        flags,
        mode,
    ) {
        Ok(fd) => fd,
        Err(status) => {
            panic!(
                "Expected Ok(fd) when opening log file, got {}",
                status.to_string()
            );
        }
    };

    let fp = match open_posix_log_stream(
        "posix_open_log_stream_can_create_and_write_log_file",
        &file_str,
        fd,
        "a",
    ) {
        Ok(fp) => fp,
        Err(status) => {
            unsafe {
                libc::close(fd);
            }
            panic!(
                "Expected Ok(FILE*) from open_posix_log_stream, got {}",
                status.to_string()
            );
        }
    };

    unsafe {
        let msg = b"posix_open_log_stream_can_create_and_write_log_file\n";
        let write_rc = libc::fwrite(
            msg.as_ptr() as *const libc::c_void,
            1,
            msg.len(),
            fp,
        );
        assert!(
            write_rc == msg.len(),
            "Expected to write {} bytes via FILE*, wrote {}",
            msg.len(),
            write_rc
        );

        let flush_rc = libc::fflush(fp);
        assert_eq!(
            flush_rc, 0,
            "fflush should succeed on FILE* from open_posix_log_stream"
        );

        libc::fclose(fp);
    }

    assert!(
        std::path::Path::new(&file_str).exists(),
        "Log file should exist after writing via open_posix_log_stream"
    );

    std::fs::remove_file(&file_path).expect("failed to remove log file");

    info!(
        file = %file_str,
        "posix_open_log_stream_can_create_and_write_log_file: completed"
    );
}
