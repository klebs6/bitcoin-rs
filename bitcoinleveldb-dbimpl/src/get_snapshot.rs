// ---------------- [ File: bitcoinleveldb-dbimpl/src/get_snapshot.rs ]
crate::ix!();

impl DBGetSnapshot for DBImpl {

    fn get_snapshot(&mut self) -> Box<dyn Snapshot> {
        self.mutex.lock();
        let snap = self
            .snapshots_
            .new(unsafe { (*self.versions).last_sequence() });
        self.mutex.unlock();
        snap
    }
}

#[cfg(test)]
#[disable]
mod get_snapshot_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn snapshots_can_be_created_used_and_released_without_leaking_visibility() {
        let (dbname, mut db) =
            open_dbimpl_for_test("snapshots_can_be_created_used_and_released_without_leaking_visibility");

        write_kv(&mut *db, "k", "v1");
        let snap: Box<dyn Snapshot> = <DBImpl as DBGetSnapshot>::get_snapshot(&mut *db);

        write_kv(&mut *db, "k", "v2");

        let mut ro: ReadOptions = Default::default();
        ro.snapshot = (&*snap) as *const dyn Snapshot;

        let (s, v) = read_value(&mut *db, &ro, "k");
        tracing::info!(status = %s.to_string(), value = %v, "snapshot read");
        assert!(s.is_ok(), "snapshot read must succeed");
        assert_eq!(v, "v1", "snapshot must isolate view");

        <DBImpl as DBReleaseSnapshot>::release_snapshot(&mut *db, snap);

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
