// ---------------- [ File: bitcoinleveldb-dbimpl/src/new_iterator.rs ]
crate::ix!();

impl NewIterator for DBImpl {

    fn new_iterator(&mut self, options: &ReadOptions) -> *mut LevelDBIterator {
        
        todo!();
        /*
            SequenceNumber latest_snapshot;
      uint32_t seed;
      Iterator* iter = NewInternalIterator(options, &latest_snapshot, &seed);
      return NewDBIterator(this, user_comparator(), iter,
                           (options.snapshot != nullptr
                                ? static_cast<const SnapshotImpl*>(options.snapshot)
                                      ->sequence_number()
                                : latest_snapshot),
                           seed);
        */
    }
}
