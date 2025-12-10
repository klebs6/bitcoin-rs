// ---------------- [ File: bitcoinleveldb-tablecache/src/table_cache_new_iterator.rs ]
crate::ix!();

impl TableCache {

    /// Return an iterator for the specified file number (the corresponding file
    /// length must be exactly "file_size" bytes).  If "tableptr" is non-null,
    /// also sets "*tableptr" to point to the Table object underlying the
    /// returned iterator, or to nullptr if no Table object underlies the
    /// returned iterator.  
    ///
    /// The returned "*tableptr" object is owned by the cache and should not be
    /// deleted, and is valid for as long as the returned iterator is live.
    ///
    pub fn new_iterator(
        &mut self,
        options:     &ReadOptions,
        file_number: u64,
        file_size:   u64,
        tableptr:    *mut *mut bitcoinleveldb_table::Table,
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
                return new_error_iterator(&status);
            }

            assert!(
                !handle.is_null(),
                "TableCache::new_iterator: FindTable returned OK but handle is null"
            );

            let cache_ptr = self.cache_raw();
            assert!(
                !cache_ptr.is_null(),
                "TableCache::new_iterator: cache pointer is null"
            );
            let cache_ref = &mut *cache_ptr;

            let tf_ptr = cache_ref.value(handle) as *mut TableAndFile;

            assert!(
                !tf_ptr.is_null(),
                "TableCache::new_iterator: cache value (TableAndFile) is null"
            );

            let tf: &mut TableAndFile = &mut *tf_ptr;

            let table_ptr_inner = tf.table_ptr();
            assert!(
                !table_ptr_inner.is_null(),
                "TableCache::new_iterator: Table pointer in TableAndFile is null"
            );

            let table: &mut bitcoinleveldb_table::Table =
                &mut *table_ptr_inner;

            trace!(
                "TableCache::new_iterator: constructing table iterator; cache_handle={:?}, table={:?}",
                handle,
                table as *mut bitcoinleveldb_table::Table
            );

            let iter = table.new_iterator(options);

            (*iter).register_cleanup(
                unref_entry,
                cache_ptr as *mut c_void,
                handle as *mut c_void,
            );

            if !tableptr.is_null() {
                *tableptr = table_ptr_inner;
            }

            iter
        }
    }
}

#[cfg(test)]
mod table_cache_new_iterator_tests {
    use super::*;
    use crate::table_cache_test_support::*;
    use std::cell::RefCell;
    use std::ptr;
    use std::rc::Rc;

    #[traced_test]
    fn new_iterator_returns_error_iterator_when_find_table_fails() {
        let (env, state) = make_in_memory_env();
        let mut options = make_options_with_env(env.clone());

        {
            let mut guard = state.lock();
            guard.fail_new_random_access = true;
        }

        let dbname = String::from("table_cache_new_iterator_error_db");
        let mut table_cache = TableCache::new(&dbname, &options, 4);

        let read_options = ReadOptions::default();
        let iter_ptr = table_cache.new_iterator(
            &read_options,
            4242,
            1024,
            ptr::null_mut(),
        );

        unsafe {
            assert!(!iter_ptr.is_null(), "iterator pointer must not be null");
            assert!(
                !(*iter_ptr).valid(),
                "error iterator must not be valid at start"
            );
            let st = (*iter_ptr).status();
            assert!(
                !st.is_ok(),
                "iterator status must carry failure from find_table"
            );
            drop(Box::from_raw(iter_ptr));
        }
    }

    #[traced_test]
    fn new_iterator_walks_over_all_table_entries_in_order() {
        let (env, _state) = make_in_memory_env();
        let mut options = make_options_with_env(env.clone());

        let dbname = String::from("table_cache_new_iterator_ok_db");
        let mut meta = FileMetaData::default();
        meta.set_number(9090);

        let mut table_cache = TableCache::new(&dbname, &options, 64);
        let table_cache_ptr: *mut TableCache = &mut table_cache;

        let pairs = vec![
            (b"a".to_vec(), b"1".to_vec()),
            (b"b".to_vec(), b"2".to_vec()),
            (b"c".to_vec(), b"3".to_vec()),
        ];
        let iter_ptr = make_iterator_from_kv_pairs(&pairs);
        let meta_ptr: *mut FileMetaData = &mut meta;

        let build_status = build_table(
            &dbname,
            env.clone(),
            &options,
            table_cache_ptr,
            iter_ptr,
            meta_ptr,
        );
        unsafe {
            drop(Box::from_raw(iter_ptr));
        }
        assert!(build_status.is_ok());

        let read_options = ReadOptions::default();
        let mut table_out: *mut Table = ptr::null_mut();
        let table_ptr_out: *mut *mut Table = &mut table_out;

        let it_ptr = table_cache.new_iterator(
            &read_options,
            *meta.number(),
            *meta.file_size(),
            table_ptr_out,
        );

        unsafe {
            assert!(!it_ptr.is_null(), "iterator pointer must not be null");
            (*it_ptr).seek_to_first();

            let mut seen: Vec<(String, String)> = Vec::new();
            while (*it_ptr).valid() {
                let k = (*it_ptr).key().to_string();
                let v = (*it_ptr).value().to_string();
                trace!("iterator saw key='{}' value='{}'", k, v);
                seen.push((k, v));
                (*it_ptr).next();
            }

            let st = (*it_ptr).status();
            assert!(st.is_ok(), "iterator status must be OK");
            drop(Box::from_raw(it_ptr));

            assert_eq!(
                seen,
                vec![
                    ("a".to_string(), "1".to_string()),
                    ("b".to_string(), "2".to_string()),
                    ("c".to_string(), "3".to_string())
                ]
            );
        }
    }
}
