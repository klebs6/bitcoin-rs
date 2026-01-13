// ---------------- [ File: bitcoinleveldb-dbimpl/src/get_approximate_sizes.rs ]
crate::ix!();

impl DBGetApproximateSizes for DBImpl {
    fn get_approximate_sizes(
        &mut self,
        range: *const Range,
        n: i32,
        sizes: *mut u64,
    ) {
        // TODO(opt): better implementation
        self.mutex.lock();
        let v: *mut Version = unsafe { (*self.versions).current() };
        unsafe {
            (*v).ref_();
        }

        for i in 0..n {
            let r: &Range = unsafe { &*range.add(i as usize) };

            // Convert user_key into a corresponding internal key.
            let k1: InternalKey =
                InternalKey::new(&r.start, MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);
            let k2: InternalKey =
                InternalKey::new(&r.limit, MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);

            let start: u64 = unsafe { (*self.versions).approximate_offset_of(v, &k1) };
            let limit: u64 = unsafe { (*self.versions).approximate_offset_of(v, &k2) };

            unsafe {
                *sizes.add(i as usize) = if limit >= start { limit - start } else { 0 };
            }
        }

        unsafe {
            (*v).unref();
        }
        self.mutex.unlock();
    }
}
