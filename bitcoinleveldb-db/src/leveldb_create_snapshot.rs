// ---------------- [ File: bitcoinleveldb-db/src/leveldb_create_snapshot.rs ]
crate::ix!();

pub fn leveldb_create_snapshot(db: *mut LevelDB) -> *const LevelDBSnapshot {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        db_is_null = db.is_null(),
        "leveldb_create_snapshot entry"
    );

    unsafe {
        if db.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_create_snapshot received null db"
            );
            return core::ptr::null();
        }

        let snap: Box<dyn Snapshot> = (*db).rep().borrow_mut().get_snapshot();
        let wrapper = Arc::new(
            LevelDBSnapshot::new((*db).rep().clone(), Some(snap))
        );

        let p = Arc::into_raw(wrapper) as *const LevelDBSnapshot;

        trace!(
            target: "bitcoinleveldb_db::c_api",
            ptr = (p as usize),
            "leveldb_create_snapshot exit"
        );
        p
    }

}
