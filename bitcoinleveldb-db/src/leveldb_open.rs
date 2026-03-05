// ---------------- [ File: bitcoinleveldb-db/src/leveldb_open.rs ]
crate::ix!();

pub fn leveldb_open(
    options: *const LevelDBOptions,
    name: *const u8,
    errptr: *mut *mut u8,
) -> *mut LevelDB {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        options_is_null = options.is_null(),
        name_is_null = name.is_null(),
        "leveldb_open entry"
    );

    unsafe {
        if options.is_null() || name.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_open received null input pointer"
            );
            let msg = Slice::from_str("leveldb_open: null input");
            let s = crate::Status::invalid_argument(&msg, None);
            let _ = save_error(errptr, &s);
            return core::ptr::null_mut();
        }

        let opts: &Options = (*options).rep();

        let cstr = std::ffi::CStr::from_ptr(name as *const core::ffi::c_char);
        let dbname: String = cstr.to_string_lossy().into_owned();

        debug!(
            target: "bitcoinleveldb_db::c_api",
            dbname = %dbname,
            "leveldb_open dbname parsed"
        );

        let rep: Rc<RefCell<DBImpl>> = Rc::new(RefCell::new(DBImpl::new(opts, &dbname)));

        let result = Box::new(LevelDB::new(rep));
        let p = Box::into_raw(result);

        info!(
            target: "bitcoinleveldb_db::c_api",
            dbname = %dbname,
            handle = (p as usize),
            "leveldb_open ok"
        );

        p
    }

}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_open_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_open_rs__make_unique_dbname_bytes() -> Vec<u8> {
        let unique_box: Box<u8> = Box::new(0u8);
        let unique_ptr: *mut u8 = Box::into_raw(unique_box);
        let unique_tag: usize = unique_ptr as usize;
        unsafe {
            drop(Box::from_raw(unique_ptr));
        }

        let name: String = format!("bitcoinleveldb_db__open_rs__testdb_{}", unique_tag);
        let mut bytes: Vec<u8> = name.into_bytes();
        bytes.push(0u8);
        bytes
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_open_rs__null_inputs_set_error_and_return_null() {
        unsafe {
            let mut err: *mut u8 = core::ptr::null_mut();
            let db: *mut LevelDB = leveldb_open(core::ptr::null(), core::ptr::null(), (&mut err) as *mut *mut u8);
            assert!(db.is_null());
            assert!(!err.is_null());
            crate::leveldb_free::leveldb_free(err as *mut core::ffi::c_void);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_open_rs__valid_open_returns_handle_and_errptr_is_null() {
        unsafe {
            let options: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!options.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(options, 1u8);

            let dbname_bytes: Vec<u8> = bitcoinleveldb_db__leveldb_open_rs__make_unique_dbname_bytes();

            let mut err: *mut u8 = core::ptr::null_mut();
            let db: *mut LevelDB =
                leveldb_open(options, dbname_bytes.as_ptr(), (&mut err) as *mut *mut u8);

            assert!(err.is_null());
            assert!(!db.is_null());

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
