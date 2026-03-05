// ---------------- [ File: bitcoinleveldb-db/src/leveldb_delete.rs ]
crate::ix!();

pub fn leveldb_delete(
    db: *mut LevelDB,
    options: *const LevelDBWriteOptions,
    key_: *const u8,
    keylen: usize,
    errptr: *mut *mut u8,
) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        db_is_null = db.is_null(),
        options_is_null = options.is_null(),
        keylen = keylen,
        "leveldb_delete entry"
    );

    unsafe {
        if db.is_null() || options.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_delete received null db/options"
            );
            let msg = Slice::from_str("leveldb_delete: null input");
            let s = crate::Status::invalid_argument(&msg, None);
            let _ = save_error(errptr, &s);
            return;
        }

        let wopt: &WriteOptions = (*options).rep();
        let k = Slice::from_ptr_len(key_, keylen);

        let status = (*db).rep().borrow_mut().delete(wopt, &k);
        let _ = save_error(errptr, &status);

        if !status.is_ok() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                status = %status.to_string(),
                "leveldb_delete failed"
            );
        }
    }

}


