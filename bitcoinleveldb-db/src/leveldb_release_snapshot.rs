// ---------------- [ File: bitcoinleveldb-db/src/leveldb_release_snapshot.rs ]
crate::ix!();

pub fn leveldb_release_snapshot(db: *mut LevelDB, snapshot: *const LevelDBSnapshot) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        db_is_null = db.is_null(),
        snapshot_is_null = snapshot.is_null(),
        "leveldb_release_snapshot entry"
    );

    unsafe {
        if db.is_null() || snapshot.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_release_snapshot received null input"
            );
            return;
        }

        drop(Arc::from_raw(snapshot as *const LevelDBSnapshot));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_release_snapshot exit");

}
