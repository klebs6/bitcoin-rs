// ---------------- [ File: bitcoinleveldb-db/src/leveldb_compact_range.rs ]
crate::ix!();

pub fn leveldb_compact_range(
    db: *mut LevelDB,
    start_key_: *const u8,
    start_key_len: usize,
    limit_key_: *const u8,
    limit_key_len: usize,
) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        db_is_null = db.is_null(),
        has_start = (!start_key_.is_null()),
        has_limit = (!limit_key_.is_null()),
        "leveldb_compact_range entry"
    );

    unsafe {
        if db.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_compact_range received null db"
            );
            return;
        }

        let mut a: Slice = Slice::default();
        let mut b: Slice = Slice::default();

        let begin: *const Slice = if start_key_.is_null() {
            core::ptr::null()
        } else {
            a = Slice::from_ptr_len(start_key_, start_key_len);
            (&a) as *const Slice
        };

        let end: *const Slice = if limit_key_.is_null() {
            core::ptr::null()
        } else {
            b = Slice::from_ptr_len(limit_key_, limit_key_len);
            (&b) as *const Slice
        };

        // Pass null Slice if corresponding "const char*" is null
        (*db).rep().borrow_mut().compact_range(begin, end);

        trace!(target: "bitcoinleveldb_db::c_api", "leveldb_compact_range exit");
    }
}
