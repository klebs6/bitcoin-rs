// ---------------- [ File: bitcoinleveldb-db/src/leveldb_iter.rs ]
crate::ix!();

pub fn leveldb_iter_next(iter: *mut LevelDBIterator) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        iter_is_null = iter.is_null(),
        "leveldb_iter_next entry"
    );

    unsafe {
        if iter.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_iter_next called with null iter"
            );
            return;
        }

        (*iter).next();
    }
}

pub fn leveldb_iter_prev(iter: *mut LevelDBIterator) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        iter_is_null = iter.is_null(),
        "leveldb_iter_prev entry"
    );

    unsafe {
        if iter.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_iter_prev called with null iter"
            );
            return;
        }

        (*iter).prev();
    }
}

pub fn leveldb_iter_key(iter: *const LevelDBIterator, klen: *mut usize) -> *const u8 {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        iter_is_null = iter.is_null(),
        klen_is_null = klen.is_null(),
        "leveldb_iter_key entry"
    );

    unsafe {
        if iter.is_null() || klen.is_null() {
            return core::ptr::null();
        }

        let s: Slice = (*iter).key();
        *klen = *s.size();
        *s.data()
    }
}

pub fn leveldb_iter_value(iter: *const LevelDBIterator, vlen: *mut usize) -> *const u8 {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        iter_is_null = iter.is_null(),
        vlen_is_null = vlen.is_null(),
        "leveldb_iter_value entry"
    );

    unsafe {
        if iter.is_null() || vlen.is_null() {
            return core::ptr::null();
        }

        let s: Slice = (*iter).value();
        *vlen = *s.size();
        *s.data()
    }
}

pub fn leveldb_iter_get_error(iter: *const LevelDBIterator, errptr: *mut *mut u8) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        iter_is_null = iter.is_null(),
        errptr_is_null = errptr.is_null(),
        "leveldb_iter_get_error entry"
    );

    unsafe {
        if iter.is_null() {
            let msg = Slice::from_str("leveldb_iter_get_error: null iterator");
            let s = crate::Status::invalid_argument(&msg, None);
            let _ = save_error(errptr, &s);
            return;
        }

        let st = (*iter).status();
        let _ = save_error(errptr, &st);

        if !st.is_ok() {
            debug!(
                target: "bitcoinleveldb_db::c_api",
                status = %st.to_string(),
                "leveldb_iter_get_error non-ok"
            );
        }
    }
}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_iter_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_iter_rs__make_unique_dbname_bytes() -> Vec<u8> {
        let unique_box: Box<u8> = Box::new(0u8);
        let unique_ptr: *mut u8 = Box::into_raw(unique_box);
        let unique_tag: usize = unique_ptr as usize;
        unsafe {
            drop(Box::from_raw(unique_ptr));
        }

        let name: String = format!("bitcoinleveldb_db__iter_rs__testdb_{}", unique_tag);
        let mut bytes: Vec<u8> = name.into_bytes();
        bytes.push(0u8);
        bytes
    }

    unsafe fn bitcoinleveldb_db__leveldb_iter_rs__free_err_if_non_null(err: *mut u8) {
        if !err.is_null() {
            crate::leveldb_free::leveldb_free(err as *mut core::ffi::c_void);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_iter_rs__null_iter_operations_are_safe() {
        unsafe {
            leveldb_iter_next(core::ptr::null_mut());
            leveldb_iter_prev(core::ptr::null_mut());
            let mut klen: usize = 0usize;
            let k: *const u8 = leveldb_iter_key(core::ptr::null(), (&mut klen) as *mut usize);
            assert!(k.is_null());
            let mut vlen: usize = 0usize;
            let v: *const u8 = leveldb_iter_value(core::ptr::null(), (&mut vlen) as *mut usize);
            assert!(v.is_null());

            let mut err: *mut u8 = core::ptr::null_mut();
            leveldb_iter_get_error(core::ptr::null(), (&mut err) as *mut *mut u8);
            assert!(!err.is_null());
            bitcoinleveldb_db__leveldb_iter_rs__free_err_if_non_null(err);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_iter_rs__iteration_yields_keys_and_values_in_order_and_error_is_ok() {
        unsafe {
            let options: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!options.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(options, 1u8);

            let dbname_bytes: Vec<u8> = bitcoinleveldb_db__leveldb_iter_rs__make_unique_dbname_bytes();
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
            let v1: [u8; 2] = [b'v', b'1'];
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

            let k2: [u8; 2] = [b'a', b'2'];
            let v2: [u8; 2] = [b'v', b'2'];
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

            let it: *mut LevelDBIterator = crate::leveldb_create_iterator::leveldb_create_iterator(db, ropt);
            assert!(!it.is_null());

            crate::leveldb_iter_seek::leveldb_iter_seek_to_first(it);

            let valid0: u8 = crate::leveldb_iter_valid::leveldb_iter_valid(it as *const LevelDBIterator);
            assert_eq!(valid0, 1u8);

            let mut klen0: usize = 0usize;
            let kptr0: *const u8 = leveldb_iter_key(it as *const LevelDBIterator, (&mut klen0) as *mut usize);
            assert!(!kptr0.is_null());
            assert_eq!(klen0, 2usize);

            let kbytes0: Vec<u8> = core::slice::from_raw_parts(kptr0, klen0).to_vec();
            assert_eq!(kbytes0.as_slice(), k1.as_slice());

            let mut vlen0: usize = 0usize;
            let vptr0: *const u8 = leveldb_iter_value(it as *const LevelDBIterator, (&mut vlen0) as *mut usize);
            assert!(!vptr0.is_null());
            assert_eq!(vlen0, 2usize);

            let vbytes0: Vec<u8> = core::slice::from_raw_parts(vptr0, vlen0).to_vec();
            assert_eq!(vbytes0.as_slice(), v1.as_slice());

            leveldb_iter_next(it);

            let valid1: u8 = crate::leveldb_iter_valid::leveldb_iter_valid(it as *const LevelDBIterator);
            assert_eq!(valid1, 1u8);

            let mut klen1: usize = 0usize;
            let kptr1: *const u8 = leveldb_iter_key(it as *const LevelDBIterator, (&mut klen1) as *mut usize);
            assert!(!kptr1.is_null());

            let kbytes1: Vec<u8> = core::slice::from_raw_parts(kptr1, klen1).to_vec();
            assert_eq!(kbytes1.as_slice(), k2.as_slice());

            let mut err: *mut u8 = core::ptr::null_mut();
            leveldb_iter_get_error(it as *const LevelDBIterator, (&mut err) as *mut *mut u8);
            assert!(err.is_null());

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

            bitcoinleveldb_db__leveldb_iter_rs__free_err_if_non_null(derr);
            crate::leveldb_options::leveldb_options_destroy(options);
        }
    }
}
