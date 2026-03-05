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
