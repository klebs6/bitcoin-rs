// ---------------- [ File: bitcoinleveldb-tablecache/src/build_table.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/builder.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/builder.cc]

/**
  | Build a Table file from the contents
  | of *iter.
  |
  | The generated file will be named according
  | to meta->number.
  |
  | On success, the rest of *meta will be
  | filled with metadata about the generated
  | table.
  |
  | If no data is present in *iter, meta->file_size
  | will be set to zero, and no Table file
  | will be produced.
  |
  */
pub fn build_table(
    dbname:      &String,
    env:         Rc<RefCell<dyn Env>>,
    options:     &Options,
    table_cache: *mut TableCache,
    iter:        *mut LevelDBIterator,
    meta:        *mut FileMetaData,
) -> crate::Status {
    unsafe {

        assert!(
            !table_cache.is_null(),
            "build_table: table_cache pointer is null"
        );
        assert!(!iter.is_null(), "build_table: iter pointer is null");
        assert!(!meta.is_null(), "build_table: meta pointer is null");

        let meta_ref: &mut FileMetaData = &mut *meta;
        let iter_ref: &mut LevelDBIterator = &mut *iter;

        // Reset file_size before we start.
        meta_ref.set_file_size(0);

        trace!(
            "build_table: start; dbname='{}', file_number={}",
            dbname,
            meta_ref.number()
        );

        // Seek source iterator to first element.
        iter_ref.seek_to_first();
        trace!("build_table: iterator seek_to_first completed");

        // Compute table file name.
        let fname = table_file_name(dbname, *meta_ref.number());
        trace!("build_table: table filename = '{}'", fname);

        let mut status = crate::Status::ok();

        if iter_ref.valid() {
            // Acquire a WritableFile from the Env.
            let mut file_box_ptr: *mut Box<dyn WritableFile> =
                core::ptr::null_mut();
            {
                let env_rc = env.clone();
                let mut env_ref = env_rc.borrow_mut();
                trace!(
                    "build_table: calling Env::new_writable_file for '{}'",
                    fname
                );
                status = env_ref.new_writable_file(&fname, &mut file_box_ptr);
            }

            if status.is_ok() {
                assert!(
                    !file_box_ptr.is_null(),
                    "build_table: Env returned OK but file pointer is null"
                );

                // Reconstruct outer Box<Box<dyn WritableFile>> and borrow
                // the inner Box<dyn WritableFile> for the builder and I/O.
                let mut file_holder: Box<Box<dyn WritableFile>> =
                    Box::from_raw(file_box_ptr);
                let writable_box: &mut Box<dyn WritableFile> =
                    file_holder.as_mut();
                let file_ptr: *mut dyn WritableFile = &mut **writable_box;

                trace!(
                    "build_table: WritableFile created; constructing TableBuilder (file_number={}, fname='{}')",
                    meta_ref.number(),
                    fname
                );

                let mut builder = TableBuilder::new(options, file_ptr);

                // Initialize smallest key from the first entry.
                let first_key = iter_ref.key();
                meta_ref.smallest_mut().decode_from(&first_key);

                // Add all entries in order.
                while iter_ref.valid() {
                    let key = iter_ref.key();
                    let value = iter_ref.value();

                    meta_ref.largest_mut().decode_from(&key);
                    builder.add(&key, &value);

                    iter_ref.next();
                }

                // Finish and check for builder errors.
                status = builder.finish();
                if status.is_ok() {
                    meta_ref.set_file_size(builder.file_size());
                    debug!(
                        "build_table: builder finished; file_size={} bytes",
                        meta_ref.file_size()
                    );
                    assert!(
                        *meta_ref.file_size() > 0,
                        "build_table: builder finished with zero file_size"
                    );
                } else {
                    error!(
                        "build_table: TableBuilder::finish returned non-OK status (file_number={}, fname='{}')",
                        meta_ref.number(),
                        fname
                    );
                }

                drop(builder);

                // Now sync and close the file if everything is still OK.
                if status.is_ok() {
                    trace!("build_table: syncing file '{}'", fname);
                    status = writable_box.sync();
                }

                if status.is_ok() {
                    trace!("build_table: closing file '{}'", fname);
                    status = writable_box.close();
                }

                // Drop the file holder Box to release heap resources allocated by Env.
                drop(file_holder);

                // Verify that the table is actually usable via TableCache.
                if status.is_ok() {
                    trace!(
                        "build_table: verifying table via TableCache; file_number={}, file_size={}",
                        meta_ref.number(),
                        meta_ref.file_size()
                    );

                    let cache_ref: &mut TableCache = &mut *table_cache;
                    let read_options = ReadOptions::default();

                    let it_ptr = cache_ref.new_iterator(
                        &read_options,
                        *meta_ref.number(),
                        *meta_ref.file_size(),
                        core::ptr::null_mut(),
                    );

                    if !it_ptr.is_null() {
                        let it: &mut LevelDBIterator = &mut *it_ptr;
                        let it_status = it.status();
                        if !it_status.is_ok() {
                            error!(
                                "build_table: verification iterator returned non-OK status (file_number={}, fname='{}')",
                                meta_ref.number(),
                                fname
                            );
                            status = it_status;
                        }
                        // Iterator is heap-allocated; drop it explicitly.
                        drop(Box::from_raw(it_ptr));
                    } else {
                        error!(
                            "build_table: TableCache::new_iterator returned null for verification (file_number={}, fname='{}')",
                            meta_ref.number(),
                            fname
                        );
                        let msg =
                            b"table verification iterator allocation failed";
                        let msg_slice = Slice::from(&msg[..]);
                        status = crate::Status::corruption(&msg_slice, None);
                    }
                }
            } else {
                error!(
                    "build_table: Env::new_writable_file failed for '{}'",
                    fname
                );
            }
        } else {
            trace!(
                "build_table: source iterator invalid at first entry; no table will be created (file_number={})",
                meta_ref.number()
            );
        }

        // Always check for input iterator errors at the end.
        let iter_status = iter_ref.status();
        if !iter_status.is_ok() {
            warn!(
                "build_table: overriding status with underlying iterator status (file_number={})",
                meta_ref.number()
            );
            status = iter_status;
        }

        // Decide whether to keep or delete the file.
        if status.is_ok() && *meta_ref.file_size() > 0 {
            // Keep it
            info!(
                "build_table: successfully built table '{}' (file_number={}, file_size={})",
                fname,
                meta_ref.number(),
                meta_ref.file_size()
            );
        } else {
            warn!(
                ok = status.is_ok(),
                file_size = *meta_ref.file_size(),
                "build_table: build failed or produced empty file; deleting '{}'",
                fname
            );

            let env_rc = env.clone();
            let mut env_ref = env_rc.borrow_mut();
            let delete_status = env_ref.delete_file(&fname);
            if !delete_status.is_ok() {
                error!(
                    "build_table: failed to delete partial table file '{}' after error",
                    fname
                );
            }
        }

        status
    }
}

#[cfg(test)]
mod build_table_behavior_tests {
    use super::*;
    use crate::table_cache_test_support::*;
    use std::cell::RefCell;
    use std::ffi::c_void;
    use std::rc::Rc;

    #[traced_test]
    fn build_table_with_empty_iterator_produces_no_file() {
        let (env, state) = make_in_memory_env();
        let mut options = make_options_with_env(env.clone());

        let dbname = String::from("build_table_empty_iterator_db");
        let mut meta = FileMetaData::default();
        meta.set_number(1);

        let mut table_cache = TableCache::new(&dbname, &options, 16);
        let table_cache_ptr: *mut TableCache = &mut table_cache;
        let iter_ptr = make_empty_iterator();
        let meta_ptr: *mut FileMetaData = &mut meta;

        trace!("calling build_table (empty iterator case)");
        let status = build_table(
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

        assert!(status.is_ok(), "status must be OK for empty iterator");
        assert_eq!(
            *meta.file_size(),
            0,
            "file_size must remain zero when iterator has no entries"
        );

        let fname = table_file_name(&dbname, *meta.number());
        let files_snapshot = {
            let guard = state.lock();
            guard.files.clone()
        };

        assert!(
            !files_snapshot.contains_key(&fname),
            "no file should be materialized for empty iterator; fname='{}'",
            fname
        );
    }

    #[traced_test]
    fn build_table_with_single_entry_builds_and_verifies_table() {
        let (env, _state) = make_in_memory_env();
        let mut options = make_options_with_env(env.clone());

        let dbname = String::from("build_table_single_entry_db");
        let mut meta = FileMetaData::default();
        meta.set_number(2);

        let mut table_cache = TableCache::new(&dbname, &options, 32);
        let table_cache_ptr: *mut TableCache = &mut table_cache;

        let key = b"alpha".to_vec();
        let value = b"value-alpha".to_vec();
        let iter_ptr =
            make_iterator_from_kv_pairs(&[(key.clone(), value.clone())]);
        let meta_ptr: *mut FileMetaData = &mut meta;

        trace!("calling build_table (single entry)");
        let status = build_table(
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

        assert!(
            status.is_ok(),
            "build_table should succeed: {:?}",
            status.to_string()
        );
        assert!(
            *meta.file_size() > 0,
            "file_size must be > 0 for non-empty table"
        );

        let read_options = ReadOptions::default();

        #[derive(Default)]
        struct CapturedKV {
            key:   Option<String>,
            value: Option<String>,
        }

        fn capture_kv(arg: *mut c_void, k: &Slice, v: &Slice) -> c_void {
            unsafe {
                let c: &mut CapturedKV = &mut *(arg as *mut CapturedKV);
                c.key = Some(k.to_string());
                c.value = Some(v.to_string());
                core::mem::zeroed()
            }
        }

        let mut captured = CapturedKV::default();
        let arg_ptr: *mut c_void = &mut captured as *mut _ as *mut c_void;

        info!("invoking TableCache::get to verify built table contents");
        let get_status = table_cache.get(
            &read_options,
            *meta.number(),
            *meta.file_size(),
            &Slice::from("alpha"),
            arg_ptr,
            capture_kv,
        );

        assert!(
            get_status.is_ok(),
            "TableCache::get must succeed for existing key: {}",
            get_status.to_string()
        );
        assert_eq!(captured.key.as_deref(), Some("alpha"));
        assert_eq!(captured.value.as_deref(), Some("value-alpha"));
    }

    #[traced_test]
    fn build_table_propagates_iterator_status_on_error() {
        use bitcoinleveldb_status::StatusCode;

        let (env, _state) = make_in_memory_env();
        let mut options = make_options_with_env(env.clone());

        let dbname = String::from("build_table_iter_status_db");
        let mut meta = FileMetaData::default();
        meta.set_number(3);

        let mut table_cache = TableCache::new(&dbname, &options, 8);
        let table_cache_ptr: *mut TableCache = &mut table_cache;

        let error_msg = Slice::from("iter-corruption");
        let iter_status = Status::corruption(&error_msg, None);

        let iface =
            VecLevelDBIterator::new(Vec::new(), iter_status.clone());
        let wrapper =
            bitcoinleveldb_iterator::LevelDBIterator::new(Some(Box::new(
                iface,
            )));
        let iter_ptr = Box::into_raw(Box::new(wrapper));

        let meta_ptr: *mut FileMetaData = &mut meta;

        let status = build_table(
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

        assert!(
            !status.is_ok(),
            "status must not be OK when underlying iterator has error"
        );
        assert_eq!(
            status.code(),
            StatusCode::Corruption,
            "build_table must propagate iterator status code"
        );
    }
}
