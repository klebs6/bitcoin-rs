// ---------------- [ File: bitcoinleveldb-tablecache/src/table_cache_new_iterator.rs ]
crate::ix!();
   
impl TableCache {

    /**
      | Return an iterator for the specified file
      | number (the corresponding file length must be
      | exactly "file_size" bytes).  If "tableptr" is
      | non-null, also sets "*tableptr" to point to
      | the Table object underlying the returned
      | iterator, or to nullptr if no Table object
      | underlies the returned iterator.  The
      | returned "*tableptr" object is owned by the
      | cache and should not be deleted, and is valid
      | for as long as the returned iterator is live.
      */
    pub fn new_iterator(
        &mut self,
        options:     &ReadOptions,
        file_number: u64,
        file_size:   u64,
        tableptr:    *mut *mut crate::table::Table,
    ) -> *mut LevelDBIterator {
        unsafe {
            if !tableptr.is_null() {
                *tableptr = core::ptr::null_mut();
            }

            trace!(
                "TableCache::new_iterator: file_number={}, file_size={}",
                file_number,
                file_size
            );

            let mut handle: *mut CacheHandle = core::ptr::null_mut();
            let status = self.find_table(file_number, file_size, &mut handle);

            if !status.is_ok() {
                warn!(
                    "TableCache::new_iterator: FindTable failed; returning error iterator"
                );
                return bitcoinleveldb_iterator::new_error_iterator(&status);
            }

            assert!(
                !handle.is_null(),
                "TableCache::new_iterator: FindTable returned OK but handle is null"
            );

            let cache_ref = &mut *self.cache;
            let tf_ptr = cache_ref.value(handle) as *mut TableAndFile;

            assert!(
                !tf_ptr.is_null(),
                "TableCache::new_iterator: cache value (TableAndFile) is null"
            );

            let tf = &mut *tf_ptr;

            assert!(
                !tf.table.is_null(),
                "TableCache::new_iterator: Table pointer in TableAndFile is null"
            );

            let table = &mut *tf.table;

            trace!(
                "TableCache::new_iterator: constructing table iterator; cache_handle={:?}, table={:?}",
                handle,
                table as *mut crate::table::Table
            );

            let iter = table.new_iterator(options);

            (*iter).register_cleanup(
                unref_entry,
                self.cache as *mut c_void,
                handle as *mut c_void,
            );

            if !tableptr.is_null() {
                *tableptr = tf.table;
            }

            iter
        }
    }
}
