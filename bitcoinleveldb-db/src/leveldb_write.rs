// ---------------- [ File: bitcoinleveldb-db/src/leveldb_write.rs ]
crate::ix!();

pub fn leveldb_write(
    db: *mut LevelDB,
    options: *const LevelDBWriteOptions,
    batch: *mut LevelDBWriteBatch,
    errptr: *mut *mut u8,
) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        db_is_null = db.is_null(),
        options_is_null = options.is_null(),
        batch_is_null = batch.is_null(),
        "leveldb_write entry"
    );

    unsafe {
        if db.is_null() || options.is_null() || batch.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_write received null input pointer"
            );
            let msg = Slice::from_str("leveldb_write: null input");
            let s = crate::Status::invalid_argument(&msg, None);
            let _ = save_error(errptr, &s);
            return;
        }

        let wopt: &WriteOptions = (*options).rep();
        let updates_ptr: *mut WriteBatch = (*batch).rep_mut() as *mut WriteBatch;

        let status = (*db).rep().borrow_mut().write(wopt, updates_ptr);
        let _ = save_error(errptr, &status);

        if !status.is_ok() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                status = %status.to_string(),
                "leveldb_write failed"
            );
        }
    }

}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_write_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_write_rs__make_unique_dbname_bytes() -> Vec<u8> {
        crate::bitcoinleveldb_db__make_temp_dbname_bytes("bitcoinleveldb_db__write_rs__testdb")
    }

    unsafe fn bitcoinleveldb_db__leveldb_write_rs__free_err_if_non_null(err: *mut u8) {
        if !err.is_null() {
            crate::leveldb_free::leveldb_free(err as *mut core::ffi::c_void);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_write_rs__null_inputs_set_error() {
        unsafe {
            let mut err: *mut u8 = core::ptr::null_mut();
            leveldb_write(
                core::ptr::null_mut(),
                core::ptr::null(),
                core::ptr::null_mut(),
                (&mut err) as *mut *mut u8,
            );

            assert!(!err.is_null());
            bitcoinleveldb_db__leveldb_write_rs__free_err_if_non_null(err);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_write_rs__empty_batch_is_ok_and_does_not_set_error() {
        unsafe {
            let opt: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!opt.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(opt, 1u8);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__leveldb_write_rs__make_unique_dbname_bytes();

            let mut oerr: *mut u8 = core::ptr::null_mut();
            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                opt,
                dbname_bytes.as_ptr(),
                (&mut oerr) as *mut *mut u8,
            );

            assert!(oerr.is_null());
            assert!(!db.is_null());

            let wopt: *mut LevelDBWriteOptions = crate::leveldb_writeoptions::leveldb_writeoptions_create();
            assert!(!wopt.is_null());

            let mut batch_box: Box<crate::db::LevelDBWriteBatch> =
                Box::new(crate::db::LevelDBWriteBatch::default());
            let batch_ptr: *mut crate::db::LevelDBWriteBatch =
                batch_box.as_mut() as *mut crate::db::LevelDBWriteBatch;

            let mut err: *mut u8 = core::ptr::null_mut();
            leveldb_write(db, wopt, batch_ptr, (&mut err) as *mut *mut u8);
            assert!(err.is_null());

            drop(batch_box);

            crate::leveldb_writeoptions::leveldb_writeoptions_destroy(wopt);

            crate::leveldb_close::leveldb_close(db);

            let mut derr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(
                opt,
                dbname_bytes.as_ptr(),
                (&mut derr) as *mut *mut u8,
            );

            bitcoinleveldb_db__leveldb_write_rs__free_err_if_non_null(derr);
            crate::leveldb_options::leveldb_options_destroy(opt);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_write_rs__batch_put_then_read_roundtrip() {
        unsafe {
            let opt: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!opt.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(opt, 1u8);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__leveldb_write_rs__make_unique_dbname_bytes();

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

            let mut batch_box: Box<crate::db::LevelDBWriteBatch> =
                Box::new(crate::db::LevelDBWriteBatch::default());

            let key_buf: [u8; 2] = [b'k', b'1'];
            let val_buf: [u8; 2] = [b'v', b'1'];

            let k: Slice = Slice::from_ptr_len(key_buf.as_ptr(), key_buf.len());
            let v: Slice = Slice::from_ptr_len(val_buf.as_ptr(), val_buf.len());

            batch_box.rep_mut().put(&k, &v);

            let batch_ptr: *mut crate::db::LevelDBWriteBatch =
                batch_box.as_mut() as *mut crate::db::LevelDBWriteBatch;

            let mut werr: *mut u8 = core::ptr::null_mut();
            leveldb_write(db, wopt, batch_ptr, (&mut werr) as *mut *mut u8);
            assert!(werr.is_null());

            drop(batch_box);

            let mut vallen: usize = 0usize;
            let mut gerr: *mut u8 = core::ptr::null_mut();
            let out: *mut u8 = crate::leveldb_get::leveldb_get(
                db,
                ropt,
                key_buf.as_ptr(),
                key_buf.len(),
                (&mut vallen) as *mut usize,
                (&mut gerr) as *mut *mut u8,
            );

            assert!(gerr.is_null());
            assert!(!out.is_null());
            assert_eq!(vallen, val_buf.len());

            let got: Vec<u8> = core::slice::from_raw_parts(out as *const u8, vallen).to_vec();
            assert_eq!(got.as_slice(), val_buf.as_slice());

            crate::leveldb_free::leveldb_free(out as *mut core::ffi::c_void);

            crate::leveldb_readoptions::leveldb_readoptions_destroy(ropt);
            crate::leveldb_writeoptions::leveldb_writeoptions_destroy(wopt);

            crate::leveldb_close::leveldb_close(db);

            let mut derr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(
                opt,
                dbname_bytes.as_ptr(),
                (&mut derr) as *mut *mut u8,
            );

            bitcoinleveldb_db__leveldb_write_rs__free_err_if_non_null(derr);
            crate::leveldb_options::leveldb_options_destroy(opt);
        }
    }
}
