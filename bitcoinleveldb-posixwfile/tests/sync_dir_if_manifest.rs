// ---------------- [ File: bitcoinleveldb-posixwfile/tests/sync_dir_if_manifest.rs ]
#![cfg(unix)]

use bitcoinleveldb_posixwfile::*;
use bitcoinleveldb_file::*;
use bitcoin_imports::*;
use bitcoinleveldb_status::*;
use bitcoin_support::*;
use std::fs::OpenOptions;
use std::os::unix::io::IntoRawFd;

#[traced_test]
fn sync_dir_if_manifest_is_noop_for_non_manifest_files() {
    trace!("sync_dir_if_manifest_is_noop_for_non_manifest_files: start");

    let dir = std::env::temp_dir();
    let path = dir.join("not_manifest_file.txt");
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

    let status = w.sync_dir_if_manifest();
    assert!(
        status.is_ok(),
        "sync_dir_if_manifest should be a no-op and succeed for non-manifest files: {}",
        status.to_string()
    );

    let _ = w.close();
    let _ = std::fs::remove_file(&path);

    info!("sync_dir_if_manifest_is_noop_for_non_manifest_files: completed");
}

#[traced_test]
fn sync_dir_if_manifest_syncs_existing_directory_for_manifest_file() {
    trace!("sync_dir_if_manifest_syncs_existing_directory_for_manifest_file: start");

    let dir = std::env::temp_dir();
    let manifest_path = dir.join("MANIFEST-000001");
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

    let status = w.sync_dir_if_manifest();
    assert!(
        status.is_ok(),
        "sync_dir_if_manifest on existing manifest dir should succeed: {}",
        status.to_string()
    );

    let _ = w.close();
    let _ = std::fs::remove_file(&manifest_path);

    info!("sync_dir_if_manifest_syncs_existing_directory_for_manifest_file: completed");
}

#[traced_test]
fn sync_dir_if_manifest_propagates_error_when_directory_missing() {
    trace!("sync_dir_if_manifest_propagates_error_when_directory_missing: start");

    let dir = std::env::temp_dir();
    let real_file = dir.join("real_file_for_fd.txt");
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

    // Fake manifest path whose directory does not exist.
    let fake_manifest_path = "nonexistent_dir_for_manifest/MANIFEST-000001".to_string();

    let mut w = PosixWritableFile::new(fake_manifest_path.clone(), fd);

    let status = w.sync_dir_if_manifest();

    // It must fail:
    assert!(
        !status.is_ok(),
        "sync_dir_if_manifest should fail when manifest directory does not exist: {}",
        status.to_string()
    );

    // PosixError maps ENOENT (missing dir/file) to NotFound in LevelDB style.
    // Some environments / future tweaks could return IOError instead, so accept both.
    assert!(
        status.is_not_found() || status.is_io_error(),
        "sync_dir_if_manifest error for missing manifest directory should be NotFound or IO error, got: {}",
        status.to_string()
    );

    let _ = w.close();
    let _ = std::fs::remove_file(&real_file);

    info!("sync_dir_if_manifest_propagates_error_when_directory_missing: completed");
}
