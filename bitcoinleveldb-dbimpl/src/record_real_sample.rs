// ---------------- [ File: bitcoinleveldb-dbimpl/src/record_real_sample.rs ]
crate::ix!();

impl DBImpl {
    /// Record a sample of bytes read at the
    /// specified internal key.
    /// 
    /// Samples are taken approximately once every
    /// READ_BYTES_PERIOD bytes.
    pub fn record_read_sample(&mut self, key_: Slice) {
        self.mutex.lock();

        if unsafe { (*(*self.versions).current()).record_read_sample(key_) } {
            self.maybe_schedule_compaction();
        }

        self.mutex.unlock();
    }
}
