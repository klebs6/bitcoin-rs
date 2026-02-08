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

#[cfg(test)]
mod record_read_sample_interface_contract_suite {
    use super::*;

    fn assert_dbimpl_implements_record_read_sample() {
        fn _assert<T: DBIterReadSample>() {}
        _assert::<DBImpl>();
    }

    fn compile_only_accepts_record_read_sample_trait_object(_db: &mut dyn DBIterReadSample) {}

    fn compile_only_call_record_read_sample_via_trait_object(db: &mut dyn DBIterReadSample, key: Slice) {
        db.record_read_sample(key);
    }

    #[traced_test]
    fn record_read_sample_trait_is_object_safe_and_dbimpl_implements_it() {
        tracing::info!("Asserting DBIterReadSample is object-safe and DBImpl implements it");

        assert_dbimpl_implements_record_read_sample();

        let _accept =
            compile_only_accepts_record_read_sample_trait_object as fn(&mut dyn DBIterReadSample);
        let _call = compile_only_call_record_read_sample_via_trait_object
            as fn(&mut dyn DBIterReadSample, Slice);

        tracing::debug!("Trait object acceptance + call wrapper compiled");
        let _ = (_accept, _call);
    }

    #[traced_test]
    fn record_read_sample_method_item_is_addressable() {
        tracing::info!("Asserting <DBImpl as DBIterReadSample>::record_read_sample is addressable");
        let _m = <DBImpl as DBIterReadSample>::record_read_sample;
        let _ = _m;
    }
}
