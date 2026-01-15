// ---------------- [ File: bitcoinleveldb-dbimpl/src/get_approximate_sizes.rs ]
crate::ix!();

impl DBGetApproximateSizes for DBImpl {
    fn get_approximate_sizes(&mut self, range: *const Range, n: i32, sizes: *mut u64) {
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
                InternalKey::new(r.start(), MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);
            let k2: InternalKey =
                InternalKey::new(r.limit(), MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);

            let start: u64 = unsafe { (*self.versions).approximate_offset_of(v, &k1) };
            let limit: u64 = unsafe { (*self.versions).approximate_offset_of(v, &k2) };

            unsafe {
                *sizes.add(i as usize) = if limit >= start { limit - start } else { 0 };
            }
        }

        unsafe {
            (*v).unref();
        }
        unsafe { self.mutex.unlock() };
    }
}

#[cfg(test)]
mod db_get_approximate_sizes_contract_suite {
    use super::*;
    use bitcoinleveldb_dbinterface::DBGetApproximateSizes;

    fn assert_dbimpl_implements_db_get_approximate_sizes() {
        fn _assert<T: DBGetApproximateSizes>() {}
        _assert::<DBImpl>();
    }

    fn compile_only_accepts_db_get_approximate_sizes_trait_object(_db: &mut dyn DBGetApproximateSizes) {}

    fn compile_only_get_approximate_sizes_call_via_trait_object(
        db: &mut dyn DBGetApproximateSizes,
        ranges: &[Range],
        sizes_out: &mut [u64],
    ) {
        // Keep the wrapper slice-based; the underlying interface is expected to be
        // pointer-and-count based (C++-style). This function is compile-only.
        db.get_approximate_sizes(ranges.as_ptr(), ranges.len() as i32, sizes_out.as_mut_ptr());
    }

    #[traced_test]
    fn db_get_approximate_sizes_contract_dbimpl_implements_trait() {
        tracing::trace!("begin DBGetApproximateSizes contract: DBImpl implements DBGetApproximateSizes");
        assert_dbimpl_implements_db_get_approximate_sizes();
        tracing::info!("DBGetApproximateSizes contract satisfied: DBImpl implements DBGetApproximateSizes");
    }

    #[traced_test]
    fn db_get_approximate_sizes_contract_trait_is_object_safe_and_callable() {
        tracing::trace!("begin DBGetApproximateSizes contract: trait object safety + callable shape");
        let _accept =
            compile_only_accepts_db_get_approximate_sizes_trait_object as fn(&mut dyn DBGetApproximateSizes);
        let _call = compile_only_get_approximate_sizes_call_via_trait_object
            as fn(&mut dyn DBGetApproximateSizes, &[Range], &mut [u64]);
        tracing::info!("DBGetApproximateSizes contract satisfied: usable as a trait object and callable via dyn dispatch");
    }

    #[traced_test]
    fn db_get_approximate_sizes_contract_method_item_is_addressable() {
        tracing::trace!("begin DBGetApproximateSizes contract: method item addressability");
        let _method_item = <DBImpl as DBGetApproximateSizes>::get_approximate_sizes;
        let _ = _method_item;
        tracing::info!("DBGetApproximateSizes contract satisfied: <DBImpl as DBGetApproximateSizes>::get_approximate_sizes method item can be referenced");
    }
}
