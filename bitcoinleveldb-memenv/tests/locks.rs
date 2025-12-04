// ---------------- [ File: bitcoinleveldb-memenv/tests/locks.rs ]
use bitcoinleveldb_memenv::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_file::*;
use bitcoin_imports::*;

#[traced_test]
fn mem_env_test_locks() {
    let mut env = make_mem_env();

    let fname = "some file".to_string();
    let mut lock_ptr: *mut Box<dyn FileLock> = core::ptr::null_mut();

    // LockFile should succeed.
    let status =
        env.lock_file(&fname, &mut lock_ptr as *mut *mut Box<dyn FileLock>);
    assert!(status.is_ok());
    assert!(!lock_ptr.is_null());

    // UnlockFile should also succeed and free the lock.
    let status = env.unlock_file(lock_ptr);
    assert!(status.is_ok());
}
