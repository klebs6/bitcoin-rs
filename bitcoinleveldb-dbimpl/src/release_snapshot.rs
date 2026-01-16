// ---------------- [ File: bitcoinleveldb-dbimpl/src/release_snapshot.rs ]
crate::ix!();

impl DBReleaseSnapshot for DBImpl {

    fn release_snapshot(&mut self, snapshot: Box<dyn Snapshot>) {
        self.mutex.lock();

        let raw: *mut dyn Snapshot = Box::into_raw(snapshot);
        let data: *mut () = raw as *mut ();
        let snap_impl: *mut SnapshotImpl = data as *mut SnapshotImpl;

        self.snapshots.delete(snap_impl);

        unsafe {
            self.mutex.unlock();
        }
    }
}

#[cfg(test)]
mod db_release_snapshot_interface_contract_suite {
    use super::*;
    use bitcoinleveldb_dbinterface::DBReleaseSnapshot;

    fn assert_dbimpl_implements_db_release_snapshot() {
        fn _assert<T: DBReleaseSnapshot>() {}
        _assert::<DBImpl>();
    }

    #[traced_test]
    fn release_snapshot_contract_dbimpl_implements_trait() {
        tracing::info!("Asserting DBImpl implements DBReleaseSnapshot");
        assert_dbimpl_implements_db_release_snapshot();
    }

    #[traced_test]
    fn release_snapshot_method_item_is_addressable() {
        tracing::info!("Asserting <DBImpl as DBReleaseSnapshot>::release_snapshot is addressable");
        let _m = <DBImpl as DBReleaseSnapshot>::release_snapshot;
        let _ = _m;
    }
}
