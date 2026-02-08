// ---------------- [ File: bitcoinleveldb-db/src/leveldb_release_snapshot.rs ]
crate::ix!();

pub fn leveldb_release_snapshot(
        db:       *mut LevelDB,
        snapshot: *const LevelDBSnapshot)  {
    
    todo!();
        /*
            db->rep->ReleaseSnapshot(snapshot->rep);
          delete snapshot;
        */
}

pub fn leveldb_release_snapshot(db: *mut LevelDB, snapshot: *const LevelDBSnapshot) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_release_snapshot entry";
        "db_is_null" => db.is_null(),
        "snapshot_is_null" => snapshot.is_null()
    );

    unsafe {
        if db.is_null() || snapshot.is_null() {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_release_snapshot received null input");
            return;
        }

        // Release through DB, then delete the C wrapper.
        let snap_rc: Rc<dyn Snapshot> = (*snapshot).rep.clone();
        (*db).rep.borrow_mut().release_snapshot(snap_rc);

        drop(Box::from_raw(snapshot as *mut LevelDBSnapshot));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_release_snapshot exit");

    /*
        db->rep->ReleaseSnapshot(snapshot->rep);
      delete snapshot;
    */
}
