// ---------------- [ File: bitcoinleveldb-dbimpl/src/release_snapshot.rs ]
crate::ix!();

impl DBReleaseSnapshot for DBImpl {

    fn release_snapshot(&mut self, snapshot: Box<dyn Snapshot>) {
        self.mutex.lock();

        let raw: *mut dyn Snapshot = Box::into_raw(snapshot);
        let data: *mut () = raw as *mut ();
        let snap_impl: *mut SnapshotImpl = data as *mut SnapshotImpl;

        self.snapshots_.delete(snap_impl);

        self.mutex.unlock();
    }
}

#[cfg(test)]
#[disable]
mod release_snapshot_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn releasing_snapshot_allows_normal_operations_to_continue() {
        let (dbname, mut db) = open_dbimpl_for_test("releasing_snapshot_allows_normal_operations_to_continue");

        write_kv(&mut *db, "k", "v1");
        let snap: Box<dyn Snapshot> = <DBImpl as DBGetSnapshot>::get_snapshot(&mut *db);

        <DBImpl as DBReleaseSnapshot>::release_snapshot(&mut *db, snap);

        // Continue writes/reads.
        write_kv(&mut *db, "k", "v2");
        assert_read_eq(&mut *db, "k", "v2");

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
