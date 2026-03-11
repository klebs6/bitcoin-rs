// ---------------- [ File: bitcoinleveldb-db/src/leveldb_destroy_db.rs ]
crate::ix!();

pub fn leveldb_destroy_db(options: *const LevelDBOptions, name: *const u8, errptr: *mut *mut u8) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        options_is_null = options.is_null(),
        name_is_null = name.is_null(),
        "leveldb_destroy_db entry"
    );

    unsafe {
        if options.is_null() || name.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_destroy_db received null input"
            );
            let msg = Slice::from_str("leveldb_destroy_db: null input");
            let s = crate::Status::invalid_argument(&msg, None);
            let _ = save_error(errptr, &s);
            return;
        }

        let cstr = std::ffi::CStr::from_ptr(name as *const core::ffi::c_char);
        let dbname: String = cstr.to_string_lossy().into_owned();

        let status = destroydb(&dbname, (*options).rep());
        let _ = save_error(errptr, &status);

        if !status.is_ok() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                dbname = %dbname,
                status = %status.to_string(),
                "leveldb_destroy_db failed"
            );
        }
    }
}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_destroy_db_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_destroy_db_rs__make_unique_dbname_bytes() -> Vec<u8> {
        crate::bitcoinleveldb_db__make_temp_dbname_bytes("bitcoinleveldb_db__destroy_db_rs__testdb")
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_destroy_db_rs__null_inputs_set_error() {
        unsafe {
            let mut err: *mut u8 = core::ptr::null_mut();
            leveldb_destroy_db(core::ptr::null(), core::ptr::null(), (&mut err) as *mut *mut u8);
            assert!(!err.is_null());
            crate::leveldb_free::leveldb_free(err as *mut core::ffi::c_void);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_destroy_db_rs__destroy_db_after_open_close_is_ok_or_reports_error_via_errptr() {
        unsafe {
            let options: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!options.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(options, 1u8);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__leveldb_destroy_db_rs__make_unique_dbname_bytes();

            let mut oerr: *mut u8 = core::ptr::null_mut();
            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                options,
                dbname_bytes.as_ptr(),
                (&mut oerr) as *mut *mut u8,
            );

            assert!(oerr.is_null());
            assert!(!db.is_null());

            crate::leveldb_close::leveldb_close(db);

            let mut derr: *mut u8 = core::ptr::null_mut();
            leveldb_destroy_db(options, dbname_bytes.as_ptr(), (&mut derr) as *mut *mut u8);

            if !derr.is_null() {
                crate::leveldb_free::leveldb_free(derr as *mut core::ffi::c_void);
            }

            crate::leveldb_options::leveldb_options_destroy(options);
        }
    }
}
