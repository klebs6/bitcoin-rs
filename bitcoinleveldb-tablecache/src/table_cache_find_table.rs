// ---------------- [ File: bitcoinleveldb-tablecache/src/table_cache_find_table.rs ]
crate::ix!();

impl TableCache {
    pub fn find_table(
        &mut self,
        file_number: u64,
        file_size:   u64,
        handle:      *mut *mut CacheHandle,
    ) -> Status {
        unsafe {
            assert!(
                !handle.is_null(),
                "TableCache::find_table: handle out-parameter pointer is null"
            );

            trace!(
                "TableCache::find_table: file_number={}, file_size={}",
                file_number,
                file_size
            );

            let mut status = Status::ok();

            let mut buf = [0u8; core::mem::size_of::<u64>()];
            let mut v = file_number;
            for i in 0..core::mem::size_of::<u64>() {
                buf[i] = (v & 0xff) as u8;
                v >>= 8;
            }
            let key = Slice::from(&buf[..]);

            let cache_ptr = self.cache_raw();
            assert!(
                !cache_ptr.is_null(),
                "TableCache::find_table: cache pointer is null"
            );
            let cache_ref = &mut *cache_ptr;

            let mut cache_handle: *mut CacheHandle = cache_ref.lookup(&key);
            *handle = cache_handle;

            if cache_handle.is_null() {
                trace!(
                    "TableCache::find_table: cache miss for file_number={}",
                    file_number
                );

                let mut file_holder_ptr_opt: Option<*mut Box<dyn RandomAccessFile>> = None;
                let mut table_ptr: *mut bitcoinleveldb_table::Table =
                    core::ptr::null_mut();

                let mut fname =
                    table_file_name(self.dbname_str(), file_number);

                {
                    let env_rc = self.env_handle();
                    let mut env_ref = env_rc.borrow_mut();
                    let mut file_box_ptr: *mut Box<dyn RandomAccessFile> =
                        core::ptr::null_mut();

                    trace!(
                        "TableCache::find_table: calling Env::new_random_access_file('{}')",
                        fname
                    );

                    status = env_ref.new_random_access_file(
                        &fname,
                        &mut file_box_ptr,
                    );

                    if status.is_ok() {
                        assert!(
                            !file_box_ptr.is_null(),
                            "TableCache::find_table: Env returned OK but file holder pointer is null"
                        );
                        file_holder_ptr_opt = Some(file_box_ptr);
                    }
                }

                if !status.is_ok() {
                    let old_fname =
                        sst_table_file_name(self.dbname_str(), file_number);

                    let env_rc = self.env_handle();
                    let mut env_ref = env_rc.borrow_mut();
                    let mut file_box_ptr: *mut Box<dyn RandomAccessFile> =
                        core::ptr::null_mut();

                    trace!(
                        "TableCache::find_table: primary name '{}' failed; trying legacy name '{}'",
                        fname,
                        old_fname
                    );

                    let s2 = env_ref.new_random_access_file(
                        &old_fname,
                        &mut file_box_ptr,
                    );

                    if s2.is_ok() {
                        status = Status::ok();
                        assert!(
                            !file_box_ptr.is_null(),
                            "TableCache::find_table: Env returned OK but legacy file holder pointer is null"
                        );
                        file_holder_ptr_opt = Some(file_box_ptr);
                        fname = old_fname;
                    } else {
                        trace!(
                            "TableCache::find_table: legacy name '{}' also failed",
                            old_fname
                        );
                    }
                }

                if status.is_ok() {
                    trace!(
                        "TableCache::find_table: opening Table for file='{}'",
                        fname
                    );

                    if let Some(file_holder_ptr) = file_holder_ptr_opt {
                        let adapter =
                            BorrowedRandomAccessFileAdapter::new(file_holder_ptr, &fname);
                        let file_rc: Rc<RefCell<dyn RandomAccessFile>> =
                            Rc::new(RefCell::new(adapter));

                        let open_result = bitcoinleveldb_table::Table::open(
                            self.options_ref(),
                            file_rc,
                            file_size,
                        );

                        match open_result {
                            Ok(table_box) => {
                                let table_out: *mut bitcoinleveldb_table::Table =
                                    Box::into_raw(table_box);
                                assert!(
                                    !table_out.is_null(),
                                    "TableCache::find_table: Table::open returned Ok but table pointer is null"
                                );
                                table_ptr = table_out;
                                status = Status::ok();
                            }
                            Err(s) => {
                                error!(
                                    "TableCache::find_table: Table::open failed for file='{}'",
                                    fname
                                );
                                status = s;
                            }
                        }
                    } else {
                        let msg = b"TableCache::find_table: file holder pointer missing despite OK status";
                        let msg_slice = Slice::from(&msg[..]);
                        error!(
                            "TableCache::find_table: missing file holder for file='{}'",
                            fname
                        );
                        status = Status::corruption(&msg_slice, None);
                    }
                }

                if !status.is_ok() {
                    debug_assert!(
                        table_ptr.is_null(),
                        "TableCache::find_table: non-OK status but table pointer is non-null"
                    );

                    if let Some(file_holder_ptr) = file_holder_ptr_opt {
                        trace!(
                            "TableCache::find_table: deleting RandomAccessFile holder @ {:?} after open failure",
                            file_holder_ptr
                        );
                        let _file_holder: Box<Box<dyn RandomAccessFile>> =
                            Box::from_raw(file_holder_ptr);
                    }
                } else {
                    trace!(
                        "TableCache::find_table: inserting TableAndFile into cache for file_number={}",
                        file_number
                    );

                    let file_holder_ptr = file_holder_ptr_opt.expect(
                        "TableCache::find_table: file holder pointer missing when inserting into cache",
                    );

                    let tf = Box::new(TableAndFile::new(
                        file_holder_ptr,
                        table_ptr,
                    ));

                    let tf_ptr: *mut TableAndFile = Box::into_raw(tf);

                    cache_handle = cache_ref.insert(
                        &key,
                        tf_ptr as *mut c_void,
                        1,
                        delete_entry,
                    );
                    *handle = cache_handle;

                    debug_assert!(
                        !cache_handle.is_null(),
                        "TableCache::find_table: cache insert returned null handle"
                    );
                }
            } else {
                trace!(
                    "TableCache::find_table: cache hit for file_number={}",
                    file_number
                );
            }

            status
        }
    }
}


#[cfg(test)]
mod table_cache_find_table_tests {
    use super::*;
    use crate::table_cache_test_support::*;
    use std::cell::RefCell;
    use std::ptr;
    use std::rc::Rc;

    #[traced_test]
    fn find_table_returns_error_when_random_access_creation_fails() {
        let (env, state) = make_in_memory_env();
        let mut options = make_options_with_env(env.clone());

        {
            let mut guard = state.lock();
            guard.fail_new_random_access = true;
        }

        let dbname = String::from("find_table_fail_env_db");
        let mut table_cache = TableCache::new(&dbname, &options, 4);

        let mut handle: *mut CacheHandle = ptr::null_mut();
        let status = table_cache.find_table(100, 1024, &mut handle);

        assert!(
            !status.is_ok(),
            "find_table must fail if env::NewRandomAccessFile fails"
        );
        assert!(handle.is_null(), "handle must remain null on failure");
    }

    #[traced_test]
    fn find_table_caches_and_reuses_table_handles() {
        let (env, state) = make_in_memory_env();
        let mut options = make_options_with_env(env.clone());

        let dbname = String::from("find_table_cache_db");
        let mut meta = FileMetaData::default();
        meta.set_number(51);

        let mut table_cache = TableCache::new(&dbname, &options, 32);
        let table_cache_ptr: *mut TableCache = &mut table_cache;

        let key = b"k-cache".to_vec();
        let val = b"v-cache".to_vec();
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

        let fname = table_file_name(&dbname, *meta.number());

        let mut handle1: *mut CacheHandle = std::ptr::null_mut();
        let s1 =
            table_cache.find_table(*meta.number(), *meta.file_size(), &mut handle1);
        assert!(s1.is_ok());
        assert!(!handle1.is_null());

        let open_count_after_first = {
            let guard = state.lock();
            *guard.random_open_count.get(&fname).unwrap_or(&0)
        };

        let mut handle2: *mut CacheHandle = std::ptr::null_mut();
        let s2 =
            table_cache.find_table(*meta.number(), *meta.file_size(), &mut handle2);
        assert!(s2.is_ok());
        assert!(!handle2.is_null());

        let open_count_after_second = {
            let guard = state.lock();
            *guard.random_open_count.get(&fname).unwrap_or(&0)
        };

        assert_eq!(
            open_count_after_second, open_count_after_first,
            "subsequent find_table calls should hit cache and not reopen file"
        );

        info!(
            "handles: handle1={:?}, handle2={:?}",
            handle1, handle2
        );
    }
}
