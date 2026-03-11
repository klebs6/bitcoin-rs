// ---------------- [ File: bitcoinleveldb-db/src/leveldb_iter_valid.rs ]
crate::ix!();

pub fn leveldb_iter_valid(iter: *const LevelDBIterator) -> u8 {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        iter_is_null = iter.is_null(),
        "leveldb_iter_valid entry"
    );

    unsafe {
        if iter.is_null() {
            return 0;
        }

        let v = (*iter).valid();
        v as u8
    }
}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_iter_valid_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_iter_valid_rs__make_unique_dbname_bytes() -> Vec<u8> {
        crate::bitcoinleveldb_db__make_temp_dbname_bytes("bitcoinleveldb_db__iter_valid_rs__testdb")
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_iter_valid_rs__null_iter_is_invalid() {
        let v: u8 = leveldb_iter_valid(core::ptr::null());
        assert_eq!(v, 0u8);
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_iter_valid_rs__iterator_is_valid_after_seek_to_first_when_db_contains_a_key() {
        unsafe {
            let options: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!options.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(options, 1u8);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__leveldb_iter_valid_rs__make_unique_dbname_bytes();

            let mut oerr: *mut u8 = core::ptr::null_mut();

            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                options,
                dbname_bytes.as_ptr(),
                (&mut oerr) as *mut *mut u8,
            );

            assert!(oerr.is_null());
            assert!(!db.is_null());

            let wopt: *mut LevelDBWriteOptions = crate::leveldb_writeoptions::leveldb_writeoptions_create();
            let ropt: *mut LevelDBReadOptions = crate::leveldb_readoptions::leveldb_readoptions_create();
            assert!(!wopt.is_null());
            assert!(!ropt.is_null());

            let key: [u8; 2] = [b'k', b'1'];
            let val: [u8; 2] = [b'v', b'1'];
            let mut perr: *mut u8 = core::ptr::null_mut();

            crate::leveldb_put::leveldb_put(
                db,
                wopt,
                key.as_ptr(),
                key.len(),
                val.as_ptr(),
                val.len(),
                (&mut perr) as *mut *mut u8,
            );
            assert!(perr.is_null());

            let it: *mut LevelDBIterator =
                crate::leveldb_create_iterator::leveldb_create_iterator(db, ropt);
            assert!(!it.is_null());

            crate::leveldb_iter_seek::leveldb_iter_seek_to_first(it);

            let v: u8 = leveldb_iter_valid(it as *const LevelDBIterator);
            assert_eq!(v, 1u8);

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
