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

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_compact_range_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_compact_range_rs__make_unique_dbname_bytes() -> Vec<u8> {
        crate::bitcoinleveldb_db__make_temp_dbname_bytes("bitcoinleveldb_db__compact_range_rs__testdb")
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_compact_range_rs__null_db_is_safe() {
        unsafe {
            leveldb_compact_range(
                core::ptr::null_mut(),
                core::ptr::null(),
                0usize,
                core::ptr::null(),
                0usize,
            );
        }
        assert!(true);
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_compact_range_rs__compact_range_with_null_bounds_is_safe() {
        unsafe {
            let options: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!options.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(options, 1u8);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__leveldb_compact_range_rs__make_unique_dbname_bytes();

            let mut err: *mut u8 = core::ptr::null_mut();

            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                options,
                dbname_bytes.as_ptr(),
                (&mut err) as *mut *mut u8,
            );

            assert!(err.is_null());
            assert!(!db.is_null());

            leveldb_compact_range(
                db,
                core::ptr::null(),
                0usize,
                core::ptr::null(),
                0usize,
            );

            crate::leveldb_close::leveldb_close(db);

            let mut derr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(
                options,
                dbname_bytes.as_ptr(),
                (&mut derr) as *mut *mut u8,
            );

            if !derr.is_null() {
                crate::leveldb_free::leveldb_free(derr as *mut core::ffi::c_void);
            }

            crate::leveldb_options::leveldb_options_destroy(options);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_compact_range_rs__compact_range_with_explicit_bounds_is_safe() {
        unsafe {
            let options: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!options.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(options, 1u8);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__leveldb_compact_range_rs__make_unique_dbname_bytes();
            let mut err: *mut u8 = core::ptr::null_mut();

            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                options,
                dbname_bytes.as_ptr(),
                (&mut err) as *mut *mut u8,
            );

            assert!(err.is_null());
            assert!(!db.is_null());

            let start: [u8; 1] = [b'a'];
            let limit: [u8; 1] = [b'z'];

            leveldb_compact_range(db, start.as_ptr(), start.len(), limit.as_ptr(), limit.len());

            crate::leveldb_close::leveldb_close(db);

            let mut derr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(
                options,
                dbname_bytes.as_ptr(),
                (&mut derr) as *mut *mut u8,
            );

            if !derr.is_null() {
                crate::leveldb_free::leveldb_free(derr as *mut core::ffi::c_void);
            }

            crate::leveldb_options::leveldb_options_destroy(options);
        }
    }
}
