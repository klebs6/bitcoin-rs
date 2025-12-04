// ---------------- [ File: bitcoinleveldb-tablecache/src/table_cache_get.rs ]
crate::ix!();

impl TableCache {

    /**
      | If a seek to internal key "k" in specified
      | file finds an entry, call
      | (*handle_result)(arg, found_key,
      | found_value).
      */
    pub fn get(
        &mut self,
        options:       &ReadOptions,
        file_number:   u64,
        file_size:     u64,
        k:             &Slice,
        arg:           *mut c_void,
        handle_result: fn(*mut c_void, &Slice, &Slice) -> c_void,
    ) -> crate::Status {
        unsafe {
            trace!(
                "TableCache::get: file_number={}, file_size={}",
                file_number,
                file_size
            );

            let mut handle: *mut CacheHandle = core::ptr::null_mut();
            let mut status =
                self.find_table(file_number, file_size, &mut handle);

            if status.is_ok() {
                assert!(
                    !handle.is_null(),
                    "TableCache::get: FindTable returned OK but handle is null"
                );

                let cache_ref = &mut *self.cache;
                let tf_ptr = cache_ref.value(handle) as *mut TableAndFile;

                assert!(
                    !tf_ptr.is_null(),
                    "TableCache::get: cache value (TableAndFile) is null"
                );

                let tf = &mut *tf_ptr;

                assert!(
                    !tf.table.is_null(),
                    "TableCache::get: Table pointer in TableAndFile is null"
                );

                let table = &mut *tf.table;

                trace!(
                    "TableCache::get: delegating to Table::internal_get for key"
                );

                status = table.internal_get(
                    options,
                    k,
                    arg,
                    handle_result,
                );

                cache_ref.release(handle);
            } else {
                warn!(
                    "TableCache::get: FindTable failed; status not OK"
                );
            }

            status
        }
    }
}
