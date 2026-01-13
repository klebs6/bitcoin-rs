// ---------------- [ File: bitcoinleveldb-dbimpl/src/release_snapshot.rs ]
crate::ix!();

impl DBReleaseSnapshot for DBImpl {

    fn release_snapshot(&mut self, snapshot: Box<dyn Snapshot>) { 
        todo!(); 
        /*
        self.mutex.lock();

        let raw: *mut dyn Snapshot = Box::into_raw(snapshot);
        let data: *mut () = raw as *mut ();
        let snap_impl: *mut SnapshotImpl = data as *mut SnapshotImpl;

        self.snapshots.delete(snap_impl);

        self.mutex.unlock();
                                                                      */
    }
}
