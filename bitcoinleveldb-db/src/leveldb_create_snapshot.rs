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

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_create_snapshot_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_create_snapshot_rs__make_unique_dbname_bytes() -> Vec<u8> {
        crate::bitcoinleveldb_db__make_temp_dbname_bytes("bitcoinleveldb_db__create_snapshot_rs__testdb")
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_create_snapshot_rs__null_db_returns_null_snapshot() {
        unsafe {
            let snap: *const LevelDBSnapshot = leveldb_create_snapshot(core::ptr::null_mut());
            assert!(snap.is_null());
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_create_snapshot_rs__create_and_release_snapshot_is_safe() {
        unsafe {
            let options: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!options.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(options, 1u8);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__leveldb_create_snapshot_rs__make_unique_dbname_bytes();

            let mut err: *mut u8 = core::ptr::null_mut();

            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                options,
                dbname_bytes.as_ptr(),
                (&mut err) as *mut *mut u8,
            );

            assert!(err.is_null());
            assert!(!db.is_null());

            let snap: *const LevelDBSnapshot = leveldb_create_snapshot(db);
            assert!(!snap.is_null());

            crate::leveldb_release_snapshot::leveldb_release_snapshot(db, snap);

            crate::leveldb_close::leveldb_close(db);

            let mut derr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(
                options,
                dbname_bytes.as_ptr(),
                (&mut derr) as *mut *mut u8,
            );

            if !derr.is_null() {
                crate::leveldb_free::leveldb_free(derr as *mut core::ffi::c_void);
            }

            crate::leveldb_options::leveldb_options_destroy(options);
        }
    }
}
