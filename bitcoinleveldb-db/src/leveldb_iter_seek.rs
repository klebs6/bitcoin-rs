// ---------------- [ File: bitcoinleveldb-db/src/leveldb_iter_seek.rs ]
crate::ix!();

pub fn leveldb_iter_seek_to_first(iter: *mut LevelDBIterator) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        iter_is_null = iter.is_null(),
        "leveldb_iter_seek_to_first entry"
    );

    unsafe {
        if iter.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_iter_seek_to_first called with null iter"
            );
            return;
        }

        (*iter).seek_to_first();
    }
}

pub fn leveldb_iter_seek_to_last(iter: *mut LevelDBIterator) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        iter_is_null = iter.is_null(),
        "leveldb_iter_seek_to_last entry"
    );

    unsafe {
        if iter.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_iter_seek_to_last called with null iter"
            );
            return;
        }

        (*iter).seek_to_last();
    }
}

pub fn leveldb_iter_seek(iter: *mut LevelDBIterator, k: *const u8, klen: usize) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        iter_is_null = iter.is_null(),
        k_is_null = k.is_null(),
        klen = klen,
        "leveldb_iter_seek entry"
    );

    unsafe {
        if iter.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_iter_seek called with null iter"
            );
            return;
        }

        let target = Slice::from_ptr_len(k, klen);
        (*iter).seek(&target);
    }
}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_iter_seek_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_iter_seek_rs__make_unique_dbname_bytes() -> Vec<u8> {
        crate::bitcoinleveldb_db__make_temp_dbname_bytes("bitcoinleveldb_db__iter_seek_rs__testdb")
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_iter_seek_rs__seek_functions_handle_null_iter_safely() {
        unsafe {
            leveldb_iter_seek_to_first(core::ptr::null_mut());
            leveldb_iter_seek_to_last(core::ptr::null_mut());

            let empty: [u8; 0] = [];
            leveldb_iter_seek(core::ptr::null_mut(), empty.as_ptr(), empty.len());
        }

        assert!(true);
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_iter_seek_rs__seek_positions_iterator_and_valid_reflects_state() {
        unsafe {
            let options: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!options.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(options, 1u8);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__leveldb_iter_seek_rs__make_unique_dbname_bytes();

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

            let mut perr: *mut u8 = core::ptr::null_mut();
            let k1: [u8; 2] = [b'a', b'1'];
            let v1: [u8; 1] = [b'1'];
            crate::leveldb_put::leveldb_put(
                db,
                wopt,
                k1.as_ptr(),
                k1.len(),
                v1.as_ptr(),
                v1.len(),
                (&mut perr) as *mut *mut u8,
            );
            assert!(perr.is_null());

            let k2: [u8; 2] = [b'b', b'1'];
            let v2: [u8; 1] = [b'2'];
            crate::leveldb_put::leveldb_put(
                db,
                wopt,
                k2.as_ptr(),
                k2.len(),
                v2.as_ptr(),
                v2.len(),
                (&mut perr) as *mut *mut u8,
            );
            assert!(perr.is_null());

            let it: *mut LevelDBIterator =
                crate::leveldb_create_iterator::leveldb_create_iterator(db, ropt);
            assert!(!it.is_null());

            leveldb_iter_seek_to_first(it);
            let v0: u8 = crate::leveldb_iter_valid::leveldb_iter_valid(it as *const LevelDBIterator);
            assert_eq!(v0, 1u8);

            let target: [u8; 2] = [b'b', b'0'];
            leveldb_iter_seek(it, target.as_ptr(), target.len());

            let v1ok: u8 = crate::leveldb_iter_valid::leveldb_iter_valid(it as *const LevelDBIterator);
            assert_eq!(v1ok, 1u8);

            let mut klen: usize = 0usize;
            let kptr: *const u8 = crate::leveldb_iter::leveldb_iter_key(
                it as *const LevelDBIterator,
                (&mut klen) as *mut usize,
            );
            assert!(!kptr.is_null());
            let kbytes: Vec<u8> = core::slice::from_raw_parts(kptr, klen).to_vec();
            assert_eq!(kbytes.as_slice(), k2.as_slice());

            leveldb_iter_seek_to_last(it);
            let v2ok: u8 = crate::leveldb_iter_valid::leveldb_iter_valid(it as *const LevelDBIterator);
            assert_eq!(v2ok, 1u8);

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
