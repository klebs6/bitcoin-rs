// ---------------- [ File: bitcoinleveldb-dbimpl/src/record_read_sample.rs ]
crate::ix!();

impl DBIterReadSample for DBImpl {
    /// Record a sample of bytes read at the
    /// specified internal key.
    ///
    /// Samples are taken approximately once every
    /// READ_BYTES_PERIOD bytes.
    fn record_read_sample(&mut self, key_: Slice) {
        tracing::trace!("DBImpl::record_read_sample: begin");

        self.mutex.lock();

        let should_schedule: bool =
            unsafe { (*(*self.versions).current()).record_read_sample(key_) };

        if should_schedule {
            tracing::debug!("DBImpl::record_read_sample: compaction scheduling suggested");
            self.maybe_schedule_compaction();
        }

        unsafe {
            self.mutex.unlock();
        }

        tracing::trace!("DBImpl::record_read_sample: end");
    }
}
