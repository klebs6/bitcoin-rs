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
