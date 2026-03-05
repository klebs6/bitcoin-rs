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
