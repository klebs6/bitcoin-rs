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
