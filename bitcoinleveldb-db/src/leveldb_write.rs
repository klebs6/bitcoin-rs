// ---------------- [ File: bitcoinleveldb-db/src/leveldb_write.rs ]
crate::ix!();

pub fn leveldb_write(
        db:      *mut LevelDB,
        options: *const LevelDBWriteOptions,
        batch:   *mut LevelDBWriteBatch,
        errptr:  *mut *mut u8)  {
    
    todo!();
        /*
            SaveError(errptr, db->rep->Write(options->rep, &batch->rep));
        */
}

pub fn leveldb_write(
    db: *mut LevelDB,
    options: *const LevelDBWriteOptions,
    batch: *mut LevelDBWriteBatch,
    errptr: *mut *mut u8,
) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_write entry";
        "db_is_null" => db.is_null(),
        "options_is_null" => options.is_null(),
        "batch_is_null" => batch.is_null()
    );

    unsafe {
        if db.is_null() || options.is_null() || batch.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_write received null input pointer");
            let msg = Slice::from_str("leveldb_write: null input");
            let s = crate::Status::invalid_argument(&msg, None);
            let _ = save_error(errptr, &s);
            return;
        }

        let wopt: &WriteOptions = &(*options).rep;
        let updates_ptr = (&mut (*batch).rep) as *mut WriteBatch;

        let status = (*db).rep.borrow_mut().write(wopt, updates_ptr);
        let _ = save_error(errptr, &status);

        if !status.is_ok() {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_write failed"; "status" => %status.to_string());
        }
    }

    /*
        SaveError(errptr, db->rep->Write(options->rep, &batch->rep));
    */
}
