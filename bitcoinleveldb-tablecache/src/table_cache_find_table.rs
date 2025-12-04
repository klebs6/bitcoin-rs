// ---------------- [ File: bitcoinleveldb-tablecache/src/table_cache_find_table.rs ]
crate::ix!();

impl TableCache {

    pub fn find_table(
        &mut self,
        file_number: u64,
        file_size:   u64,
        handle:      *mut *mut CacheHandle,
    ) -> crate::Status {
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

            // Build cache key from file_number
            let mut buf = [0u8; core::mem::size_of::<u64>()];
            bitcoinleveldb_coding::encode_fixed64(&mut buf, file_number);
            let key = Slice::from(&buf[..]);

            let cache_ref = &mut *self.cache;
            let mut cache_handle: *mut CacheHandle = cache_ref.lookup(&key);
            *handle = cache_handle;

            if cache_handle.is_null() {
                trace!(
                    "TableCache::find_table: cache miss for file_number={}",
                    file_number
                );

                let mut file_ptr: *mut dyn RandomAccessFile = core::ptr::null_mut();
                let mut table_ptr: *mut crate::table::Table =
                    core::ptr::null_mut();

                // First try canonical table file name.
                let mut fname = table_file_name(&self.dbname, file_number);

                {
                    use bitcoinleveldb_env::NewRandomAccessFile;

                    let env_ref = &mut *self.env;
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
                            "TableCache::find_table: Env returned OK but file pointer is null"
                        );
                        let file_box: Box<dyn RandomAccessFile> =
                            Box::from_raw(file_box_ptr);
                        file_ptr = Box::into_raw(file_box);
                    }
                }

                // If canonical name fails, try legacy .sst filename.
                if !status.is_ok() {
                    use bitcoinleveldb_env::NewRandomAccessFile;

                    let old_fname =
                        sst_table_file_name(&self.dbname, file_number);
                    let env_ref = &mut *self.env;
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
                            "TableCache::find_table: Env returned OK but legacy file pointer is null"
                        );
                        let file_box: Box<dyn RandomAccessFile> =
                            Box::from_raw(file_box_ptr);
                        file_ptr = Box::into_raw(file_box);
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

                    // Open the Table. This mirrors the static Table::Open in C++.
                    let mut table_out: *mut crate::table::Table =
                        core::ptr::null_mut();

                    status = crate::table::Table::open(
                        &*self.options,
                        file_ptr,
                        file_size,
                        &mut table_out,
                    );

                    if status.is_ok() {
                        assert!(
                            !table_out.is_null(),
                            "TableCache::find_table: Table::open returned OK but table pointer is null"
                        );
                        table_ptr = table_out;
                    } else {
                        error!(
                            "TableCache::find_table: Table::open failed for file='{}'",
                            fname
                        );
                    }
                }

                if !status.is_ok() {
                    debug_assert!(
                        table_ptr.is_null(),
                        "TableCache::find_table: non-OK status but table pointer is non-null"
                    );

                    if !file_ptr.is_null() {
                        trace!(
                            "TableCache::find_table: deleting RandomAccessFile @ {:?} after open failure",
                            file_ptr
                        );
                        let _file_box: Box<dyn RandomAccessFile> =
                            Box::from_raw(file_ptr);
                    }

                    // We do not cache error results so that if the error is transient,
                    // or somebody repairs the file, we recover automatically.
                } else {
                    trace!(
                        "TableCache::find_table: inserting TableAndFile into cache for file_number={}",
                        file_number
                    );

                    let tf = Box::new(TableAndFile {
                        file:  file_ptr,
                        table: table_ptr,
                    });

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
