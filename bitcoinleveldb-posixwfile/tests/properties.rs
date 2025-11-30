// ---------------- [ File: bitcoinleveldb-posixwfile/tests/properties.rs ]
#![cfg(unix)]

use bitcoinleveldb_posixwfile::*;
use bitcoin_imports::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_status::*;
use bitcoin_support::*;

#[traced_test]
fn dirname_returns_dot_when_no_slash() {
    trace!("dirname_returns_dot_when_no_slash: start");

    let filename = "foo.txt".to_string();
    let dir = PosixWritableFile::dirname_static(&filename);
    assert_eq!(
        dir, ".",
        "dirname should return '.' when the path has no '/'"
    );

    info!("dirname_returns_dot_when_no_slash: completed");
}

#[traced_test]
fn dirname_extracts_parent_directory() {
    trace!("dirname_extracts_parent_directory: start");

    let filename = "/a/b/c.txt".to_string();
    let dir = PosixWritableFile::dirname_static(&filename);
    assert_eq!(dir, "/a/b", "dirname should strip last path component");

    let filename2 = "dir/subdir/file".to_string();
    let dir2 = PosixWritableFile::dirname_static(&filename2);
    assert_eq!(dir2, "dir/subdir");

    info!("dirname_extracts_parent_directory: completed");
}

#[traced_test]
fn basename_returns_full_name_when_no_slash() {
    trace!("basename_returns_full_name_when_no_slash: start");

    let filename = "foo_bar".to_string();
    let base = PosixWritableFile::basename(&filename);
    assert_eq!(
        base.to_string(),
        "foo_bar".to_string(),
        "basename should be whole string when no '/'"
    );

    info!("basename_returns_full_name_when_no_slash: completed");
}

#[traced_test]
fn basename_extracts_file_component() {
    trace!("basename_extracts_file_component: start");

    let filename = "/a/b/c.txt".to_string();
    let base = PosixWritableFile::basename(&filename);
    assert_eq!(base.to_string(), "c.txt".to_string());

    let filename2 = "dir/subdir/file".to_string();
    let base2 = PosixWritableFile::basename(&filename2);
    assert_eq!(base2.to_string(), "file".to_string());

    info!("basename_extracts_file_component: completed");
}

#[traced_test]
fn is_manifest_true_for_manifest_prefix() {
    trace!("is_manifest_true_for_manifest_prefix: start");

    let paths = [
        "MANIFEST",
        "MANIFEST-000001",
        "/tmp/db/MANIFEST-000123",
        "./MANIFEST-something",
    ];

    for p in &paths {
        let s = p.to_string();
        assert!(
            PosixWritableFile::is_manifest_static(&s),
            "expected {:?} to be recognized as manifest",
            p
        );
    }

    info!("is_manifest_true_for_manifest_prefix: completed");
}

#[traced_test]
fn is_manifest_false_for_non_manifest() {
    trace!("is_manifest_false_for_non_manifest: start");

    let paths = [
        "CURRENT",
        "LOG",
        "/tmp/db/NON_MANIFEST",
        "/tmp/db/manifest-lowercase",
        "not-a-manifest",
    ];

    for p in &paths {
        let s = p.to_string();
        assert!(
            !PosixWritableFile::is_manifest_static(&s),
            "expected {:?} NOT to be recognized as manifest",
            p
        );
    }

    info!("is_manifest_false_for_non_manifest: completed");
}
