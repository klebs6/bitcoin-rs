// ---------------- [ File: bitcoinleveldb-posixtools/tests/lock_or_unlock.rs ]
use bitcoinleveldb_posixtools::*;
use bitcoin_imports::*;
use std::sync::atomic::{AtomicI32, Ordering};

#[cfg(unix)]
#[traced_test]
fn lock_or_unlock_applies_and_releases_write_lock_on_regular_file() {
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::os::unix::io::AsRawFd;

    trace!(
        "lock_or_unlock_applies_and_releases_write_lock_on_regular_file: start"
    );

    let tmp_dir = std::env::temp_dir();
    let path = tmp_dir.join("lock_or_unlock_test_file.lock");

    {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .expect("failed to create lock test file");

        file.write_all(b"lock-test-data")
            .expect("failed to write initial data");

        let fd = file.as_raw_fd();

        let lock_result = lock_or_unlock(fd, true);
        assert_eq!(
            lock_result,
            0,
            "lock_or_unlock should succeed when acquiring a write lock"
        );

        let unlock_result = lock_or_unlock(fd, false);
        assert_eq!(
            unlock_result,
            0,
            "lock_or_unlock should succeed when releasing a write lock"
        );
    }

    let _ = std::fs::remove_file(&path);

    info!(
        "lock_or_unlock_applies_and_releases_write_lock_on_regular_file: completed"
    );
}

#[traced_test]
fn max_mmaps_reports_value_from_mmap_limit_atomic() {
    trace!("max_mmaps_reports_value_from_mmap_limit_atomic: start");

    // We shadow the global MMAP_LIMIT with a local alias to emphasize atomic semantics.
    let mmap_limit_ref: &AtomicI32 = &MMAP_LIMIT;

    let configured = 1_234_i32;
    mmap_limit_ref.store(configured, Ordering::SeqCst);

    let reported = max_mmaps();
    assert_eq!(
        reported, configured,
        "max_mmaps should return the value configured in MMAP_LIMIT"
    );

    info!(
        configured,
        reported,
        "max_mmaps_reports_value_from_mmap_limit_atomic: completed"
    );
}
