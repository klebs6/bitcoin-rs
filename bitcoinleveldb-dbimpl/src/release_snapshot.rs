// ---------------- [ File: bitcoinleveldb-dbimpl/src/release_snapshot.rs ]
crate::ix!();

impl ReleaseSnapshot for DBImpl {

    fn release_snapshot(&mut self, snapshot: Box<dyn Snapshot>)  {
        
        todo!();
        /*
            MutexLock l(&mutex_);
      snapshots_.Delete(static_cast<const SnapshotImpl*>(snapshot));
        */
    }
}
