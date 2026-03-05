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
