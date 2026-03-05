// ---------------- [ File: bitcoinleveldb-db/src/leveldb_repair_db.rs ]
crate::ix!();

pub fn leveldb_repair_db(options: *const LevelDBOptions, name: *const u8, errptr: *mut *mut u8) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        options_is_null = options.is_null(),
        name_is_null = name.is_null(),
        "leveldb_repair_db entry"
    );

    unsafe {
        if options.is_null() || name.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_repair_db received null input"
            );
            let msg = Slice::from_str("leveldb_repair_db: null input");
            let s = crate::Status::invalid_argument(&msg, None);
            let _ = save_error(errptr, &s);
            return;
        }

        let cstr = std::ffi::CStr::from_ptr(name as *const core::ffi::c_char);
        let dbname: String = cstr.to_string_lossy().into_owned();

        let status = repairdb(&dbname, (*options).rep());
        let _ = save_error(errptr, &status);

        if !status.is_ok() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                dbname = %dbname,
                status = %status.to_string(),
                "leveldb_repair_db failed"
            );
        }
    }

}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_repair_db_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_repair_db_rs__make_unique_dbname_bytes() -> Vec<u8> {
        let unique_box: Box<u8> = Box::new(0u8);
        let unique_ptr: *mut u8 = Box::into_raw(unique_box);
        let unique_tag: usize = unique_ptr as usize;
        unsafe {
            drop(Box::from_raw(unique_ptr));
        }

        let name: String = format!("bitcoinleveldb_db__repair_db_rs__testdb_{}", unique_tag);
        let mut bytes: Vec<u8> = name.into_bytes();
        bytes.push(0u8);
        bytes
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_repair_db_rs__null_inputs_set_invalid_argument_error() {
        unsafe {
            let mut err: *mut u8 = core::ptr::null_mut();
            leveldb_repair_db(core::ptr::null(), core::ptr::null(), (&mut err) as *mut *mut u8);
            assert!(!err.is_null());
            crate::leveldb_free::leveldb_free(err as *mut core::ffi::c_void);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_repair_db_rs__currently_reports_not_supported_via_errptr() {
        unsafe {
            let opt: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!opt.is_null());

            let dbname_bytes: Vec<u8> = bitcoinleveldb_db__leveldb_repair_db_rs__make_unique_dbname_bytes();

            let mut err: *mut u8 = core::ptr::null_mut();
            leveldb_repair_db(opt, dbname_bytes.as_ptr(), (&mut err) as *mut *mut u8);

            assert!(!err.is_null());
            crate::leveldb_free::leveldb_free(err as *mut core::ffi::c_void);

            crate::leveldb_options::leveldb_options_destroy(opt);
        }
    }
}
