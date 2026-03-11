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

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_release_snapshot_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_release_snapshot_rs__make_unique_dbname_bytes() -> Vec<u8> {
        crate::bitcoinleveldb_db__make_temp_dbname_bytes("bitcoinleveldb_db__release_snapshot_rs__testdb")
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_release_snapshot_rs__null_inputs_are_safe() {
        unsafe {
            leveldb_release_snapshot(core::ptr::null_mut(), core::ptr::null());
        }
        assert!(true);
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_release_snapshot_rs__does_not_drop_snapshot_when_db_is_null_and_is_safe_to_release_later() {
        unsafe {
            let opt: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!opt.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(opt, 1u8);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__leveldb_release_snapshot_rs__make_unique_dbname_bytes();

            let mut oerr: *mut u8 = core::ptr::null_mut();
            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                opt,
                dbname_bytes.as_ptr(),
                (&mut oerr) as *mut *mut u8,
            );

            assert!(oerr.is_null());
            assert!(!db.is_null());

            let snap: *const LevelDBSnapshot = crate::leveldb_create_snapshot::leveldb_create_snapshot(db);
            assert!(!snap.is_null());

            leveldb_release_snapshot(core::ptr::null_mut(), snap);

            leveldb_release_snapshot(db, snap);

            crate::leveldb_close::leveldb_close(db);

            let mut derr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(
                opt,
                dbname_bytes.as_ptr(),
                (&mut derr) as *mut *mut u8,
            );

            if !derr.is_null() {
                crate::leveldb_free::leveldb_free(derr as *mut core::ffi::c_void);
            }

            crate::leveldb_options::leveldb_options_destroy(opt);
        }
    }
}
