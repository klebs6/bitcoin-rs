// ---------------- [ File: bitcoinleveldb-db/src/leveldb_close.rs ]
crate::ix!();

pub fn leveldb_close(db: *mut LevelDB) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        db_is_null = db.is_null(),
        "leveldb_close entry"
    );

    unsafe {
        if db.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_close called with null db"
            );
            return;
        }

        drop(Box::from_raw(db));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_close exit");

    /*
        delete db->rep;
      delete db;
    */
}
