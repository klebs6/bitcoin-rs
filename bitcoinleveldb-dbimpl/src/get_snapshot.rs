// ---------------- [ File: bitcoinleveldb-dbimpl/src/get_snapshot.rs ]
crate::ix!();

impl DBGetSnapshot for DBImpl {
    fn get_snapshot(&mut self) -> Box<dyn Snapshot> {
        self.mutex.lock();

        let last_seq: SequenceNumber = unsafe { (*self.versions).last_sequence() };
        let snap_ptr: *mut SnapshotImpl = self.snapshots.new(last_seq);

        unsafe { self.mutex.unlock() };

        assert!(
            !snap_ptr.is_null(),
            "SnapshotList::new returned a null SnapshotImpl pointer"
        );

        tracing::debug!(
            sequence_number = last_seq,
            snapshot_ptr = snap_ptr as usize,
            "Created snapshot"
        );

        // SAFETY: SnapshotList::new() is expected to allocate the SnapshotImpl via Box and
        // return it as a raw pointer. The caller owns the Box (released via release_snapshot()).
        unsafe { Box::from_raw(snap_ptr) }
    }
}

#[cfg(test)]
mod db_get_snapshot_behavior_and_interface_suite {
    use super::*;
    use bitcoinleveldb_dbinterface::{DBGetSnapshot, DBReleaseSnapshot};

    fn build_temp_db_path_for_snapshot_suite() -> String {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                tracing::error!(error = %format!("{:?}", e), "SystemTime before UNIX_EPOCH");
                panic!();
            })
            .as_nanos();

        std::env::temp_dir()
            .join(format!("bitcoinleveldb_dbimpl_snapshot_suite_{}", nanos))
            .to_string_lossy()
            .to_string()
    }

    #[traced_test]
    fn get_snapshot_and_release_snapshot_are_callable_via_traits() {
        let dbname = build_temp_db_path_for_snapshot_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);

        let mut db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        let get_sig: fn(&mut DBImpl) -> Box<dyn Snapshot> = <DBImpl as DBGetSnapshot>::get_snapshot;
        let rel_sig: fn(&mut DBImpl, Box<dyn Snapshot>) = <DBImpl as DBReleaseSnapshot>::release_snapshot;
        let _ = (get_sig, rel_sig);

        tracing::info!("Acquiring multiple snapshots via DBGetSnapshot and releasing via DBReleaseSnapshot");

        let s1 = DBGetSnapshot::get_snapshot(&mut *db);
        let s2 = DBGetSnapshot::get_snapshot(&mut *db);
        let s3 = DBGetSnapshot::get_snapshot(&mut *db);

        DBReleaseSnapshot::release_snapshot(&mut *db, s2);
        DBReleaseSnapshot::release_snapshot(&mut *db, s1);
        DBReleaseSnapshot::release_snapshot(&mut *db, s3);

        let reacquired = db.mutex.try_lock();
        tracing::debug!(reacquired, "Mutex try_lock after snapshot acquire/release sequence");
        assert!(reacquired, "Snapshot operations must not leak the DB mutex lock");
        unsafe { db.mutex.unlock() };

        let _ = std::fs::remove_dir_all(&dbname);
    }
}
