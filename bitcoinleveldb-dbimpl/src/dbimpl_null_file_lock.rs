// ---------------- [ File: bitcoinleveldb-dbimpl/src/dbimpl_null_file_lock.rs ]
crate::ix!();

pub struct DbImplNullFileLock;

impl FileLock for DbImplNullFileLock {}

#[cfg(test)]
mod null_file_lock_interface_suite {
    use super::*;

    fn assert_dbimpl_null_file_lock_implements_filelock() {
        fn _assert<T: FileLock>() {}
        _assert::<DbImplNullFileLock>();
    }

    fn compile_only_accepts_filelock_trait_object(_l: &dyn FileLock) {}

    #[traced_test]
    fn dbimpl_null_file_lock_is_zst_and_object_safe() {
        tracing::info!("Asserting DbImplNullFileLock is a ZST and implements FileLock");

        assert_dbimpl_null_file_lock_implements_filelock();

        let lock = DbImplNullFileLock;
        let obj: &dyn FileLock = &lock;
        compile_only_accepts_filelock_trait_object(obj);

        let size = core::mem::size_of::<DbImplNullFileLock>();
        tracing::debug!(size, "DbImplNullFileLock size");
        assert_eq!(size, 0, "DbImplNullFileLock should be a zero-sized marker type");

        let boxed: Box<dyn FileLock> = Box::new(DbImplNullFileLock);
        let _ = boxed;
    }
}
