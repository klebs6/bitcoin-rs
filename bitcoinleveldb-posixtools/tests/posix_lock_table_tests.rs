// ---------------- [ File: bitcoinleveldb-posixtools/tests/posix_lock_table_tests.rs ]
use bitcoinleveldb_posixtools::*;
use bitcoin_imports::*;
use bitcoin_support::*;

#[traced_test]
fn posix_lock_table_insert_and_remove_lifecycle() {
    trace!("posix_lock_table_insert_and_remove_lifecycle: start");

    let mut table = PosixLockTable::default();

    let first_insert = table.insert("alpha.lock");
    assert!(
        first_insert,
        "first insert for a filename should succeed"
    );

    let second_insert = table.insert("alpha.lock");
    assert!(
        !second_insert,
        "second insert for the same filename should report false"
    );

    table.remove("alpha.lock");

    let insert_after_remove = table.insert("alpha.lock");
    assert!(
        insert_after_remove,
        "after remove, insert should succeed again for the same filename"
    );

    info!(
        "posix_lock_table_insert_and_remove_lifecycle: completed"
    );
}

#[traced_test]
fn posix_lock_table_handles_multiple_distinct_filenames() {
    trace!("posix_lock_table_handles_multiple_distinct_filenames: start");

    let mut table = PosixLockTable::default();

    assert!(
        table.insert("file-a.lock"),
        "first insert for file-a.lock should succeed"
    );
    assert!(
        table.insert("file-b.lock"),
        "first insert for file-b.lock should succeed"
    );

    assert!(
        !table.insert("file-a.lock"),
        "second insert for file-a.lock should report false"
    );

    table.remove("file-a.lock");
    assert!(
        table.insert("file-a.lock"),
        "after removing file-a.lock, inserting again should succeed"
    );

    info!(
        "posix_lock_table_handles_multiple_distinct_filenames: completed"
    );
}
