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
        let unique_box: Box<u8> = Box::new(0u8);
        let unique_ptr: *mut u8 = Box::into_raw(unique_box);
        let unique_tag: usize = unique_ptr as usize;
        unsafe {
            drop(Box::from_raw(unique_ptr));
        }

        let name: String = format!("bitcoinleveldb_db__iter_valid_rs__testdb_{}", unique_tag);
        let mut bytes: Vec<u8> = name.into_bytes();
        bytes.push(0u8);
        bytes
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_iter_valid_rs__null_iter_is_invalid() {
        let v: u8 = leveldb_iter_valid(core::ptr::null());
        assert_eq!(v, 0u8);
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_iter_valid_rs__empty_db_iterator_is_invalid_after_seek_to_first() {
        unsafe {
            let options: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!options.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(options, 1u8);

            let dbname_bytes: Vec<u8> = bitcoinleveldb_db__leveldb_iter_valid_rs__make_unique_dbname_bytes();
            let mut oerr: *mut u8 = core::ptr::null_mut();

            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                options,
                dbname_bytes.as_ptr(),
                (&mut oerr) as *mut *mut u8,
            );

            assert!(oerr.is_null());
            assert!(!db.is_null());

            let ropt: *mut LevelDBReadOptions = crate::leveldb_readoptions::leveldb_readoptions_create();
            assert!(!ropt.is_null());

            let it: *mut LevelDBIterator = crate::leveldb_create_iterator::leveldb_create_iterator(db, ropt);
            assert!(!it.is_null());

            crate::leveldb_iter_seek::leveldb_iter_seek_to_first(it);

            let v: u8 = leveldb_iter_valid(it as *const LevelDBIterator);
            assert_eq!(v, 0u8);

            crate::leveldb_iter_destroy::leveldb_iter_destroy(it);
            crate::leveldb_readoptions::leveldb_readoptions_destroy(ropt);

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
