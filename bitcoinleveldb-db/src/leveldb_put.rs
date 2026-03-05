// ---------------- [ File: bitcoinleveldb-db/src/leveldb_put.rs ]
crate::ix!();

pub fn leveldb_put(
    db: *mut LevelDB,
    options: *const LevelDBWriteOptions,
    key_: *const u8,
    keylen: usize,
    val: *const u8,
    vallen: usize,
    errptr: *mut *mut u8,
) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        db_is_null = db.is_null(),
        options_is_null = options.is_null(),
        keylen = keylen,
        vallen = vallen,
        "leveldb_put entry"
    );

    unsafe {
        if db.is_null() || options.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_put received null db/options"
            );
            let msg = Slice::from_str("leveldb_put: null input");
            let s = crate::Status::invalid_argument(&msg, None);
            let _ = save_error(errptr, &s);
            return;
        }

        let wopt: &WriteOptions = (*options).rep();

        let k = Slice::from_ptr_len(key_, keylen);
        let v = Slice::from_ptr_len(val, vallen);

        let status = (*db).rep().borrow_mut().put(wopt, &k, &v);
        let _ = save_error(errptr, &status);

        if !status.is_ok() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                status = %status.to_string(),
                "leveldb_put failed"
            );
        }
    }

}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_put_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_put_rs__make_unique_dbname_bytes() -> Vec<u8> {
        let unique_box: Box<u8> = Box::new(0u8);
        let unique_ptr: *mut u8 = Box::into_raw(unique_box);
        let unique_tag: usize = unique_ptr as usize;
        unsafe {
            drop(Box::from_raw(unique_ptr));
        }

        let name: String = format!("bitcoinleveldb_db__put_rs__testdb_{}", unique_tag);
        let mut bytes: Vec<u8> = name.into_bytes();
        bytes.push(0u8);
        bytes
    }

    unsafe fn bitcoinleveldb_db__leveldb_put_rs__free_err_if_non_null(err: *mut u8) {
        if !err.is_null() {
            crate::leveldb_free::leveldb_free(err as *mut core::ffi::c_void);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_put_rs__null_db_or_options_sets_error() {
        unsafe {
            let key: [u8; 1] = [b'k'];
            let val: [u8; 1] = [b'v'];
            let mut err: *mut u8 = core::ptr::null_mut();

            leveldb_put(
                core::ptr::null_mut(),
                core::ptr::null(),
                key.as_ptr(),
                key.len(),
                val.as_ptr(),
                val.len(),
                (&mut err) as *mut *mut u8,
            );

            assert!(!err.is_null());
            bitcoinleveldb_db__leveldb_put_rs__free_err_if_non_null(err);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_put_rs__put_then_get_roundtrip_is_ok() {
        unsafe {
            let opt: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!opt.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(opt, 1u8);

            let dbname_bytes: Vec<u8> = bitcoinleveldb_db__leveldb_put_rs__make_unique_dbname_bytes();
            let mut oerr: *mut u8 = core::ptr::null_mut();

            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                opt,
                dbname_bytes.as_ptr(),
                (&mut oerr) as *mut *mut u8,
            );

            assert!(oerr.is_null());
            assert!(!db.is_null());

            let wopt: *mut LevelDBWriteOptions = crate::leveldb_writeoptions::leveldb_writeoptions_create();
            let ropt: *mut LevelDBReadOptions = crate::leveldb_readoptions::leveldb_readoptions_create();
            assert!(!wopt.is_null());
            assert!(!ropt.is_null());

            let key: [u8; 3] = [b'k', b'0', b'1'];
            let val: [u8; 3] = [b'v', b'0', b'1'];

            let mut perr: *mut u8 = core::ptr::null_mut();
            leveldb_put(
                db,
                wopt,
                key.as_ptr(),
                key.len(),
                val.as_ptr(),
                val.len(),
                (&mut perr) as *mut *mut u8,
            );

            assert!(perr.is_null());

            let mut vallen: usize = 0usize;
            let mut gerr: *mut u8 = core::ptr::null_mut();
            let out: *mut u8 = crate::leveldb_get::leveldb_get(
                db,
                ropt,
                key.as_ptr(),
                key.len(),
                (&mut vallen) as *mut usize,
                (&mut gerr) as *mut *mut u8,
            );

            assert!(gerr.is_null());
            assert!(!out.is_null());
            assert_eq!(vallen, val.len());

            let out_bytes: Vec<u8> = core::slice::from_raw_parts(out as *const u8, vallen).to_vec();
            assert_eq!(out_bytes.as_slice(), val.as_slice());

            crate::leveldb_free::leveldb_free(out as *mut core::ffi::c_void);

            crate::leveldb_readoptions::leveldb_readoptions_destroy(ropt);
            crate::leveldb_writeoptions::leveldb_writeoptions_destroy(wopt);

            crate::leveldb_close::leveldb_close(db);

            let mut derr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(opt, dbname_bytes.as_ptr(), (&mut derr) as *mut *mut u8);

            bitcoinleveldb_db__leveldb_put_rs__free_err_if_non_null(derr);
            crate::leveldb_options::leveldb_options_destroy(opt);
        }
    }
}
