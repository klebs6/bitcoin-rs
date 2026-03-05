// ---------------- [ File: bitcoinleveldb-db/src/leveldb_iter_destroy.rs ]
crate::ix!();

pub fn leveldb_iter_destroy(iter: *mut LevelDBIterator) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        iter_is_null = iter.is_null(),
        "leveldb_iter_destroy entry"
    );

    unsafe {
        if iter.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_iter_destroy called with null iter"
            );
            return;
        }

        drop(Box::from_raw(iter));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_iter_destroy exit");
}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_iter_destroy_rs__exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_iter_destroy_rs__destroy_null_is_safe() {
        unsafe {
            leveldb_iter_destroy(core::ptr::null_mut());
        }
        assert!(true);
    }
}
