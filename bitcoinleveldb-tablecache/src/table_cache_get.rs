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

                let cache_ptr = self.cache_raw();
                assert!(
                    !cache_ptr.is_null(),
                    "TableCache::get: cache pointer is null"
                );
                let cache_ref = &mut *cache_ptr;

                let tf_ptr = cache_ref.value(handle) as *mut TableAndFile;

                assert!(
                    !tf_ptr.is_null(),
                    "TableCache::get: cache value (TableAndFile) is null"
                );

                let tf: &mut TableAndFile = &mut *tf_ptr;

                let table_ptr = tf.table_ptr();
                assert!(
                    !table_ptr.is_null(),
                    "TableCache::get: Table pointer in TableAndFile is null"
                );

                let table: &mut bitcoinleveldb_table::Table =
                    &mut *table_ptr;

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

#[cfg(test)]
mod table_cache_get_tests {
    use super::*;
    use crate::table_cache_test_support::*;
    use std::cell::RefCell;
    use std::ffi::c_void;
    use std::rc::Rc;

    #[traced_test]
    fn table_cache_get_successfully_reads_existing_key() {
        let (env, _state) = make_in_memory_env();
        let mut options = make_options_with_env(env.clone());

        let dbname = String::from("table_cache_get_db");
        let mut meta = FileMetaData::default();
        meta.set_number(77);

        let mut table_cache = TableCache::new(&dbname, &options, 64);
        let table_cache_ptr: *mut TableCache = &mut table_cache;

        let key = b"get-key".to_vec();
        let val = b"get-value".to_vec();
        let iter_ptr =
            make_iterator_from_kv_pairs(&[(key.clone(), val.clone())]);
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

        #[derive(Default)]
        struct Capture {
            seen: bool,
            key:  String,
            val:  String,
        }

        fn handle(arg: *mut c_void, k: &Slice, v: &Slice) -> c_void {
            unsafe {
                let c: &mut Capture = &mut *(arg as *mut Capture);
                c.seen = true;
                c.key = k.to_string();
                c.val = v.to_string();
                core::mem::zeroed()
            }
        }

        let read_options = ReadOptions::default();
        let mut capture = Capture::default();
        let arg_ptr: *mut c_void = &mut capture as *mut _ as *mut c_void;

        let status = table_cache.get(
            &read_options,
            *meta.number(),
            *meta.file_size(),
            &Slice::from("get-key"),
            arg_ptr,
            handle,
        );

        assert!(status.is_ok());
        assert!(capture.seen);
        assert_eq!(capture.key, "get-key");
        assert_eq!(capture.val, "get-value");
    }

    #[traced_test]
    fn table_cache_get_propagates_find_table_error_without_invoking_handler() {
        let (env, state) = make_in_memory_env();
        let mut options = make_options_with_env(env.clone());

        {
            let mut guard = state.lock();
            guard.fail_new_random_access = true;
        }

        let dbname = String::from("table_cache_get_find_error_db");
        let mut table_cache = TableCache::new(&dbname, &options, 4);

        #[derive(Default)]
        struct Capture {
            seen: bool,
        }

        fn handle(arg: *mut c_void, _k: &Slice, _v: &Slice) -> c_void {
            unsafe {
                let c: &mut Capture = &mut *(arg as *mut Capture);
                c.seen = true;
                core::mem::zeroed()
            }
        }

        let read_options = ReadOptions::default();
        let mut capture = Capture::default();
        let arg_ptr: *mut c_void = &mut capture as *mut _ as *mut c_void;

        let status = table_cache.get(
            &read_options,
            9999,
            0,
            &Slice::from("nonexistent"),
            arg_ptr,
            handle,
        );

        assert!(
            !status.is_ok(),
            "status must reflect failure from find_table"
        );
        assert!(
            !capture.seen,
            "handle_result must not be invoked when find_table fails"
        );
    }
}
