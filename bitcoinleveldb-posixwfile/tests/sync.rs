// ---------------- [ File: bitcoinleveldb-posixwfile/tests/sync.rs ]
#![cfg(unix)]

use bitcoinleveldb_posixwfile::*;
use bitcoinleveldb_file::*;
use bitcoin_imports::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_slice::*;
use bitcoin_support::*;
use std::fs::OpenOptions;
use std::os::unix::io::IntoRawFd;

#[traced_test]
fn sync_flushes_and_syncs_regular_file() {
    trace!("sync_flushes_and_syncs_regular_file: start");

    let dir = std::env::temp_dir();
    let path = dir.join("posix_sync_regular.txt");
    let path_str = path.to_string_lossy().into_owned();

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .read(true)
        .open(&path)
        .expect("failed to open test file");

    let fd = file.into_raw_fd();
    let mut w = PosixWritableFile::new(path_str.clone(), fd);

    let msg = "sync-regular-file-data".to_string();
    let slice = Slice::from(&msg);

    let status = w.append(&slice);
    assert!(status.is_ok());

    let status = w.sync();
    assert!(
        status.is_ok(),
        "sync on regular file should succeed: {}",
        status.to_string()
    );

    let status = w.close();
    assert!(status.is_ok());

    let data = std::fs::read_to_string(&path).expect("failed to read back test file");
    assert_eq!(data, msg);

    let _ = std::fs::remove_file(&path);

    info!("sync_flushes_and_syncs_regular_file: completed");
}

#[traced_test]
fn sync_flushes_and_syncs_manifest_file_and_dir() {
    trace!("sync_flushes_and_syncs_manifest_file_and_dir: start");

    let dir = std::env::temp_dir();
    let manifest_path = dir.join("MANIFEST-000777");
    let manifest_str = manifest_path.to_string_lossy().into_owned();

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .read(true)
        .open(&manifest_path)
        .expect("failed to open manifest file");

    let fd = file.into_raw_fd();
    let mut w = PosixWritableFile::new(manifest_str.clone(), fd);

    let msg = "sync-manifest-file-data".to_string();
    let slice = Slice::from(&msg);

    let status = w.append(&slice);
    assert!(status.is_ok());

    let status = w.sync();
    assert!(
        status.is_ok(),
        "sync on manifest file with existing dir should succeed: {}",
        status.to_string()
    );

    let status = w.close();
    assert!(status.is_ok());

    let data = std::fs::read_to_string(&manifest_path).expect("failed to read back manifest file");
    assert_eq!(data, msg);

    let _ = std::fs::remove_file(&manifest_path);

    info!("sync_flushes_and_syncs_manifest_file_and_dir: completed");
}

#[traced_test]
fn sync_propagates_error_when_manifest_directory_missing() {
    trace!("sync_propagates_error_when_manifest_directory_missing: start");

    let dir = std::env::temp_dir();
    let real_file = dir.join("real_file_for_sync_fd.txt");
    let real_file_str = real_file.to_string_lossy().into_owned();

    // Real file providing a valid fd.
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .read(true)
        .open(&real_file)
        .expect("failed to open real file for fd");
    let fd = file.into_raw_fd();

    // Fake manifest path with nonexistent directory.
    let fake_manifest_path = "nonexistent_dir_for_sync/MANIFEST-000002".to_string();

    let mut w = PosixWritableFile::new(fake_manifest_path.clone(), fd);

    let msg = "data-before-failing-sync".to_string();
    let slice = Slice::from(&msg);

    let status = w.append(&slice);
    assert!(status.is_ok());

    let status = w.sync();
    assert!(
        status.is_not_found(),
        "sync should report NotFound when manifest directory does not exist: {}",
        status.to_string()
    );

    // We still close to release the real fd.
    let _ = w.close();
    let _ = std::fs::remove_file(&real_file);

    info!("sync_propagates_error_when_manifest_directory_missing: completed");
}
