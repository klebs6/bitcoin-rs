// ---------------- [ File: bitcoinleveldb-db/src/leveldb_close.rs ]
crate::ix!();

pub fn leveldb_close(db: *mut LevelDB) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        db_is_null = db.is_null(),
        "leveldb_close entry"
    );

    unsafe {
        if db.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_close called with null db"
            );
            return;
        }

        drop(Box::from_raw(db));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_close exit");
}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_close_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_close_rs__make_unique_dbname_bytes() -> Vec<u8> {
        crate::bitcoinleveldb_db__make_temp_dbname_bytes("bitcoinleveldb_db__close_rs__testdb")
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_close_rs__close_null_is_safe() {
        unsafe {
            leveldb_close(core::ptr::null_mut());
        }
        assert!(true);
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_close_rs__close_valid_db_is_safe() {
        unsafe {
            let options: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!options.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(options, 1u8);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__leveldb_close_rs__make_unique_dbname_bytes();

            let mut err: *mut u8 = core::ptr::null_mut();

            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                options,
                dbname_bytes.as_ptr(),
                (&mut err) as *mut *mut u8,
            );

            assert!(err.is_null());
            assert!(!db.is_null());

            leveldb_close(db);

            let mut derr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(
                options,
                dbname_bytes.as_ptr(),
                (&mut derr) as *mut *mut u8,
            );

            if !derr.is_null() {
                crate::leveldb_free::leveldb_free(derr as *mut core::ffi::c_void);
            }

            crate::leveldb_options::leveldb_options_destroy(options);
        }
    }
}
