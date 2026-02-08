// ---------------- [ File: bitcoinleveldb-db/src/leveldb_create_snapshot.rs ]
crate::ix!();

pub fn leveldb_create_snapshot(db: *mut LevelDB) -> *const LevelDBSnapshot {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_create_snapshot entry"; "db_is_null" => db.is_null());

    unsafe {
        if db.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_create_snapshot received null db");
            return core::ptr::null();
        }

        let snap: Rc<dyn Snapshot> = (*db).rep.borrow_mut().get_snapshot();
        let wrapper = Box::new(LevelDBSnapshot { rep: snap });
        let p = Box::into_raw(wrapper) as *const LevelDBSnapshot;

        trace!(target: "bitcoinleveldb_db::c_api", "leveldb_create_snapshot exit"; "ptr" => (p as usize));
        p
    }

    /*
        leveldb_snapshot_t* result = new leveldb_snapshot_t;
      result->rep = db->rep->GetSnapshot();
      return result;
    */
}
