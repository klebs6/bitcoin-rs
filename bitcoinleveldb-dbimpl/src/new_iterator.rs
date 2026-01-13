// ---------------- [ File: bitcoinleveldb-dbimpl/src/new_iterator.rs ]
crate::ix!();

impl DBNewIterator for DBImpl {

    fn new_iterator(&mut self, options: &ReadOptions) -> *mut LevelDBIterator { 
        todo!(); 
        /*
        let mut latest_snapshot: SequenceNumber = 0;
        let mut seed: u32 = 0;

        let iter: *mut LevelDBIterator =
            self.new_internal_iterator(options, &mut latest_snapshot, &mut seed);

        let snapshot: SequenceNumber = if !options.snapshot().is_null() {
            unsafe { (*(options.snapshot() as *const SnapshotImpl)).sequence_number() }
        } else {
            latest_snapshot
        };

        self.new_db_iterator(self, self.user_comparator(), iter, snapshot, seed)
                                                                                    */
    }
}
