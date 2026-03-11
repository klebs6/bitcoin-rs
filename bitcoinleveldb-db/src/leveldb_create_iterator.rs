// ---------------- [ File: bitcoinleveldb-db/src/leveldb_create_iterator.rs ]
crate::ix!();

pub fn leveldb_create_iterator(db: *mut LevelDB, options: *const LevelDBReadOptions) -> *mut LevelDBIterator {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        db_is_null = db.is_null(),
        options_is_null = options.is_null(),
        "leveldb_create_iterator entry"
    );

    unsafe {
        if db.is_null() || options.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_create_iterator received null input pointer"
            );
            return core::ptr::null_mut();
        }

        let ropt: &ReadOptions = (*options).rep();
        let it = (*db).rep().borrow_mut().new_iterator(ropt);

        trace!(
            target: "bitcoinleveldb_db::c_api",
            iter_is_null = it.is_null(),
            "leveldb_create_iterator exit"
        );
        it
    }

}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_create_iterator_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_create_iterator_rs__make_unique_dbname_bytes() -> Vec<u8> {
        crate::bitcoinleveldb_db__make_temp_dbname_bytes("bitcoinleveldb_db__create_iterator_rs__testdb")
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_create_iterator_rs__null_inputs_return_null() {
        unsafe {
            let it0: *mut LevelDBIterator =
                leveldb_create_iterator(core::ptr::null_mut(), core::ptr::null());
            assert!(it0.is_null());

            let options: *mut LevelDBReadOptions = crate::leveldb_readoptions::leveldb_readoptions_create();
            assert!(!options.is_null());

            let it1: *mut LevelDBIterator = leveldb_create_iterator(core::ptr::null_mut(), options);
            assert!(it1.is_null());

            crate::leveldb_readoptions::leveldb_readoptions_destroy(options);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_create_iterator_rs__valid_inputs_return_non_null_iterator_after_first_write() {
        unsafe {
            let options: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!options.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(options, 1u8);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__leveldb_create_iterator_rs__make_unique_dbname_bytes();

            let mut err: *mut u8 = core::ptr::null_mut();

            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                options,
                dbname_bytes.as_ptr(),
                (&mut err) as *mut *mut u8,
            );

            assert!(err.is_null());
            assert!(!db.is_null());

            let wopt: *mut LevelDBWriteOptions = crate::leveldb_writeoptions::leveldb_writeoptions_create();
            let ropt: *mut LevelDBReadOptions = crate::leveldb_readoptions::leveldb_readoptions_create();
            assert!(!wopt.is_null());
            assert!(!ropt.is_null());

            let key: [u8; 2] = [b'k', b'1'];
            let val: [u8; 2] = [b'v', b'1'];

            crate::leveldb_put::leveldb_put(
                db,
                wopt,
                key.as_ptr(),
                key.len(),
                val.as_ptr(),
                val.len(),
                (&mut err) as *mut *mut u8,
            );
            assert!(err.is_null());

            let it: *mut LevelDBIterator = leveldb_create_iterator(db, ropt);
            assert!(!it.is_null());

            crate::leveldb_iter_destroy::leveldb_iter_destroy(it);
            crate::leveldb_readoptions::leveldb_readoptions_destroy(ropt);
            crate::leveldb_writeoptions::leveldb_writeoptions_destroy(wopt);

            crate::leveldb_close::leveldb_close(db);

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
