// ---------------- [ File: bitcoinleveldb-db/src/leveldb_get.rs ]
crate::ix!();

pub fn leveldb_get(
    db: *mut LevelDB,
    options: *const LevelDBReadOptions,
    key_: *const u8,
    keylen: usize,
    vallen: *mut usize,
    errptr: *mut *mut u8,
) -> *mut u8 {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        db_is_null = db.is_null(),
        options_is_null = options.is_null(),
        vallen_is_null = vallen.is_null(),
        keylen = keylen,
        "leveldb_get entry"
    );

    unsafe {
        if db.is_null() || options.is_null() || vallen.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_get received null input pointer"
            );
            let msg = Slice::from_str("leveldb_get: null input");
            let s = crate::Status::invalid_argument(&msg, None);
            let _ = save_error(errptr, &s);
            if !vallen.is_null() {
                *vallen = 0;
            }
            return core::ptr::null_mut();
        }

        let ropt: &ReadOptions = (*options).rep();
        let k = Slice::from_ptr_len(key_, keylen);

        let mut tmp: String = String::new();
        let status = (*db)
            .rep()
            .borrow_mut()
            .get(ropt, &k, (&mut tmp) as *mut String);

        if status.is_ok() {
            *vallen = tmp.as_bytes().len();
            let out = copy_string(&tmp);
            trace!(
                target: "bitcoinleveldb_db::c_api",
                vallen = *vallen,
                out_is_null = out.is_null(),
                "leveldb_get ok"
            );
            out
        } else {
            *vallen = 0;
            if !status.is_not_found() {
                let _ = save_error(errptr, &status);
                warn!(
                    target: "bitcoinleveldb_db::c_api",
                    status = %status.to_string(),
                    "leveldb_get failed (non-NotFound)"
                );
            } else {
                trace!(target: "bitcoinleveldb_db::c_api", "leveldb_get not found");
            }
            core::ptr::null_mut()
        }
    }

}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_get_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_get_rs__make_unique_dbname_bytes() -> Vec<u8> {
        let unique_box: Box<u8> = Box::new(0u8);
        let unique_ptr: *mut u8 = Box::into_raw(unique_box);
        let unique_tag: usize = unique_ptr as usize;
        unsafe {
            drop(Box::from_raw(unique_ptr));
        }

        let name: String = format!("bitcoinleveldb_db__get_rs__testdb_{}", unique_tag);
        let mut bytes: Vec<u8> = name.into_bytes();
        bytes.push(0u8);
        bytes
    }

    unsafe fn bitcoinleveldb_db__leveldb_get_rs__free_err_if_non_null(err: *mut u8) {
        if !err.is_null() {
            crate::leveldb_free::leveldb_free(err as *mut core::ffi::c_void);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_get_rs__null_inputs_set_error_and_return_null() {
        unsafe {
            let mut err: *mut u8 = core::ptr::null_mut();
            let mut vallen: usize = 999usize;

            let out: *mut u8 = leveldb_get(
                core::ptr::null_mut(),
                core::ptr::null(),
                core::ptr::null(),
                0usize,
                (&mut vallen) as *mut usize,
                (&mut err) as *mut *mut u8,
            );

            assert!(out.is_null());
            assert_eq!(vallen, 0usize);
            assert!(!err.is_null());
            bitcoinleveldb_db__leveldb_get_rs__free_err_if_non_null(err);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_get_rs__get_roundtrip_returns_bytes_and_len_and_errptr_stays_null() {
        unsafe {
            let options: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!options.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(options, 1u8);

            let dbname_bytes: Vec<u8> = bitcoinleveldb_db__leveldb_get_rs__make_unique_dbname_bytes();
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

            let key: [u8; 3] = [b'k', b'0', b'1'];
            let val: [u8; 3] = [b'v', b'0', b'1'];

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

            let mut vallen: usize = 0usize;
            let mut gerr: *mut u8 = core::ptr::null_mut();
            let out: *mut u8 = leveldb_get(
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

            let out_bytes: Vec<u8> = unsafe { core::slice::from_raw_parts(out as *const u8, vallen).to_vec() };
            assert_eq!(out_bytes.as_slice(), val.as_slice());

            crate::leveldb_free::leveldb_free(out as *mut core::ffi::c_void);

            let missing_key: [u8; 3] = [b'k', b'9', b'9'];
            let mut miss_len: usize = 123usize;
            let mut merr: *mut u8 = core::ptr::null_mut();
            let miss_out: *mut u8 = leveldb_get(
                db,
                ropt,
                missing_key.as_ptr(),
                missing_key.len(),
                (&mut miss_len) as *mut usize,
                (&mut merr) as *mut *mut u8,
            );

            assert!(miss_out.is_null());
            assert_eq!(miss_len, 0usize);
            assert!(merr.is_null());

            crate::leveldb_readoptions::leveldb_readoptions_destroy(ropt);
            crate::leveldb_writeoptions::leveldb_writeoptions_destroy(wopt);

            crate::leveldb_close::leveldb_close(db);

            let mut derr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(
                options,
                dbname_bytes.as_ptr(),
                (&mut derr) as *mut *mut u8,
            );

            bitcoinleveldb_db__leveldb_get_rs__free_err_if_non_null(derr);
            crate::leveldb_options::leveldb_options_destroy(options);
        }
    }
}
