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

        let meta_ref = &mut *meta;
        let iter_ref = &mut *iter;

        // Reset file_size before we start.
        meta_ref.file_size = 0;

        trace!(
            "build_table: start; dbname='{}', file_number={}",
            dbname,
            meta_ref.number
        );

        // Seek source iterator to first element.
        iter_ref.seek_to_first();
        trace!("build_table: iterator seek_to_first completed");

        // Compute table file name.
        let fname = table_file_name(dbname, meta_ref.number);
        trace!("build_table: table filename = '{}'", fname);

        let mut status = crate::Status::ok();

        if iter_ref.valid() {
            use bitcoinleveldb_env::NewWritableFile;

            // Acquire a WritableFile from the Env.
            let mut file_box_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
            {
                let mut env_ref = env.borrow_mut();
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

                // Take ownership of the allocated WritableFile.
                let mut file_box: Box<dyn WritableFile> = Box::from_raw(file_box_ptr);
                let file_ptr: *mut dyn WritableFile = &mut *file_box;

                trace!(
                    "build_table: WritableFile created; constructing TableBuilder (file_number={}, fname='{}')",
                    meta_ref.number,
                    fname
                );

                let mut builder = TableBuilder::new(options, file_ptr);

                // Initialize smallest key from the first entry.
                let first_key = iter_ref.key();
                meta_ref.smallest.decode_from(&first_key);

                // Add all entries in order.
                while iter_ref.valid() {
                    let key = iter_ref.key();
                    let value = iter_ref.value();

                    meta_ref.largest.decode_from(&key);
                    builder.add(&key, &value);

                    iter_ref.next();
                }

                // Finish and check for builder errors.
                status = builder.finish();
                if status.is_ok() {
                    meta_ref.file_size = builder.file_size();
                    debug!(
                        "build_table: builder finished; file_size={} bytes",
                        meta_ref.file_size
                    );
                    assert!(
                        meta_ref.file_size > 0,
                        "build_table: builder finished with zero file_size"
                    );
                } else {
                    error!(
                        "build_table: TableBuilder::finish returned non-OK status (file_number={}, fname='{}')",
                        meta_ref.number,
                        fname
                    );
                }

                drop(builder);

                // Now sync and close the file if everything is still OK.
                if status.is_ok() {
                    trace!("build_table: syncing file '{}'", fname);
                    status = file_box.sync();
                }

                if status.is_ok() {
                    trace!("build_table: closing file '{}'", fname);
                    status = file_box.close();
                }

                // Drop the file Box to release heap resources allocated by Env.
                drop(file_box);

                // Verify that the table is actually usable via TableCache.
                if status.is_ok() {
                    trace!(
                        "build_table: verifying table via TableCache; file_number={}, file_size={}",
                        meta_ref.number,
                        meta_ref.file_size
                    );

                    let cache_ref = &mut *table_cache;
                    let read_options = ReadOptions::default();

                    let it_ptr = cache_ref.new_iterator(
                        &read_options,
                        meta_ref.number,
                        meta_ref.file_size,
                        core::ptr::null_mut(),
                    );

                    if !it_ptr.is_null() {
                        let it = &mut *it_ptr;
                        let it_status = it.status();
                        if !it_status.is_ok() {
                            error!(
                                "build_table: verification iterator returned non-OK status (file_number={}, fname='{}')",
                                meta_ref.number,
                                fname
                            );
                            status = it_status;
                        }
                        // Iterator is heap-allocated; drop it explicitly.
                        drop(Box::from_raw(it_ptr));
                    } else {
                        error!(
                            "build_table: TableCache::new_iterator returned null for verification (file_number={}, fname='{}')",
                            meta_ref.number,
                            fname
                        );
                        let msg = b"table verification iterator allocation failed";
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
                meta_ref.number
            );
        }

        // Always check for input iterator errors at the end.
        let iter_status = iter_ref.status();
        if !iter_status.is_ok() {
            warn!(
                "build_table: overriding status with underlying iterator status (file_number={})",
                meta_ref.number
            );
            status = iter_status;
        }

        // Decide whether to keep or delete the file.
        if status.is_ok() && meta_ref.file_size > 0 {
            // Keep it
            info!(
                "build_table: successfully built table '{}' (file_number={}, file_size={})",
                fname,
                meta_ref.number,
                meta_ref.file_size
            );
        } else {
            use bitcoinleveldb_env::DeleteFile;

            warn!(
                ok = status.is_ok(),
                file_size = meta_ref.file_size,
                "build_table: build failed or produced empty file; deleting '{}'",
                fname
            );

            let mut env_ref = env.borrow_mut();
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
