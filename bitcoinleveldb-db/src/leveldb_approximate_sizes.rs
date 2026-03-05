// ---------------- [ File: bitcoinleveldb-db/src/leveldb_approximate_sizes.rs ]
crate::ix!();

pub fn leveldb_approximate_sizes(
    db: *mut LevelDB,
    num_ranges: i32,
    range_start_key_: *const *const u8,
    range_start_key_len: *const usize,
    range_limit_key_: *const *const u8,
    range_limit_key_len: *const usize,
    sizes: *mut u64,
) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        db_is_null = db.is_null(),
        num_ranges = num_ranges,
        sizes_is_null = sizes.is_null(),
        "leveldb_approximate_sizes entry"
    );

    unsafe {
        if db.is_null() || sizes.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_approximate_sizes received null db/sizes"
            );
            return;
        }

        if num_ranges <= 0 {
            trace!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_approximate_sizes early-exit (num_ranges<=0)"
            );
            return;
        }

        let n = num_ranges as usize;
        let mut ranges: Vec<bitcoinleveldb_slice::Range> = Vec::with_capacity(n);

        for i in 0..n {
            let start_ptr = *range_start_key_.add(i);
            let start_len = *range_start_key_len.add(i);

            let limit_ptr = *range_limit_key_.add(i);
            let limit_len = *range_limit_key_len.add(i);

            let start = Slice::from_ptr_len(start_ptr, start_len);
            let limit = Slice::from_ptr_len(limit_ptr, limit_len);

            ranges.push(bitcoinleveldb_slice::Range::new(start, limit));
        }

        (*db)
            .rep()
            .borrow_mut()
            .get_approximate_sizes(ranges.as_ptr(), num_ranges, sizes);

        trace!(target: "bitcoinleveldb_db::c_api", "leveldb_approximate_sizes exit");
    }

}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_approximate_sizes_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_approximate_sizes_rs__make_unique_dbname_bytes() -> Vec<u8> {
        let unique_box: Box<u8> = Box::new(0u8);
        let unique_ptr: *mut u8 = Box::into_raw(unique_box);
        let unique_tag: usize = unique_ptr as usize;
        unsafe {
            drop(Box::from_raw(unique_ptr));
        }

        let name: String = format!("bitcoinleveldb_db__approx_sizes__testdb_{}", unique_tag);
        let mut bytes: Vec<u8> = name.into_bytes();
        bytes.push(0u8);
        bytes
    }

    unsafe fn bitcoinleveldb_db__leveldb_approximate_sizes_rs__open_db_or_panic(
    ) -> (*mut LevelDB, *mut LevelDBOptions, Vec<u8>) {
        let options: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
        assert!(!options.is_null());
        crate::leveldb_options::leveldb_options_set_create_if_missing(options, 1u8);

        let dbname_bytes: Vec<u8> = bitcoinleveldb_db__leveldb_approximate_sizes_rs__make_unique_dbname_bytes();
        let mut err: *mut u8 = core::ptr::null_mut();

        let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
            options,
            dbname_bytes.as_ptr(),
            (&mut err) as *mut *mut u8,
        );

        assert!(err.is_null());
        assert!(!db.is_null());

        (db, options, dbname_bytes)
    }

    unsafe fn bitcoinleveldb_db__leveldb_approximate_sizes_rs__destroy_db_best_effort(
        db: *mut LevelDB,
        options: *mut LevelDBOptions,
        dbname_bytes: Vec<u8>,
    ) {
        crate::leveldb_close::leveldb_close(db);

        let mut err: *mut u8 = core::ptr::null_mut();
        crate::leveldb_destroy_db::leveldb_destroy_db(
            options,
            dbname_bytes.as_ptr(),
            (&mut err) as *mut *mut u8,
        );

        if !err.is_null() {
            crate::leveldb_free::leveldb_free(err as *mut core::ffi::c_void);
        }

        crate::leveldb_options::leveldb_options_destroy(options);
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_approximate_sizes_rs__null_db_or_sizes_is_safe() {
        unsafe {
            leveldb_approximate_sizes(
                core::ptr::null_mut(),
                1,
                core::ptr::null(),
                core::ptr::null(),
                core::ptr::null(),
                core::ptr::null(),
                core::ptr::null_mut(),
            );
        }

        assert!(true);
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_approximate_sizes_rs__non_positive_num_ranges_is_noop_and_does_not_touch_sizes() {
        unsafe {
            let (db, options, dbname_bytes) = bitcoinleveldb_db__leveldb_approximate_sizes_rs__open_db_or_panic();

            let mut sizes: [u64; 2] = [123u64, 456u64];
            leveldb_approximate_sizes(
                db,
                0,
                core::ptr::null(),
                core::ptr::null(),
                core::ptr::null(),
                core::ptr::null(),
                sizes.as_mut_ptr(),
            );

            assert_eq!(sizes[0], 123u64);
            assert_eq!(sizes[1], 456u64);

            bitcoinleveldb_db__leveldb_approximate_sizes_rs__destroy_db_best_effort(db, options, dbname_bytes);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_approximate_sizes_rs__writes_output_for_valid_range_set() {
        unsafe {
            let (db, options, dbname_bytes) = bitcoinleveldb_db__leveldb_approximate_sizes_rs__open_db_or_panic();

            let wopt: *mut LevelDBWriteOptions = crate::leveldb_writeoptions::leveldb_writeoptions_create();
            let ropt: *mut LevelDBReadOptions = crate::leveldb_readoptions::leveldb_readoptions_create();
            assert!(!wopt.is_null());
            assert!(!ropt.is_null());

            let mut err: *mut u8 = core::ptr::null_mut();

            let key: [u8; 3] = [b'k', b'0', b'0'];
            let val: [u8; 3] = [b'v', b'0', b'0'];

            crate::leveldb_put::leveldb_put(
                db,
                wopt,
                key.as_ptr(),
                key.len(),
                val.as_ptr(),
                val.len(),
                (&mut err) as *mut *mut u8,
            );

            if !err.is_null() {
                crate::leveldb_free::leveldb_free(err as *mut core::ffi::c_void);
                panic!();
            }

            let start_key: [u8; 1] = [b'k'];
            let limit_key: [u8; 1] = [b'l'];

            let start_ptrs: [*const u8; 1] = [start_key.as_ptr()];
            let start_lens: [usize; 1] = [start_key.len()];
            let limit_ptrs: [*const u8; 1] = [limit_key.as_ptr()];
            let limit_lens: [usize; 1] = [limit_key.len()];

            let mut sizes: [u64; 1] = [u64::MAX];

            leveldb_approximate_sizes(
                db,
                1,
                start_ptrs.as_ptr(),
                start_lens.as_ptr(),
                limit_ptrs.as_ptr(),
                limit_lens.as_ptr(),
                sizes.as_mut_ptr(),
            );

            assert!(sizes[0] != u64::MAX);

            crate::leveldb_readoptions::leveldb_readoptions_destroy(ropt);
            crate::leveldb_writeoptions::leveldb_writeoptions_destroy(wopt);

            bitcoinleveldb_db__leveldb_approximate_sizes_rs__destroy_db_best_effort(db, options, dbname_bytes);
        }
    }
}
