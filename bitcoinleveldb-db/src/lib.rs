// ---------------- [ File: bitcoinleveldb-db/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{comparator}
x!{copy_string}
x!{db}
//x!{destroy_db}
x!{filter_policy}
x!{leveldb_approximate_sizes}
x!{leveldb_cache}
x!{leveldb_close}
x!{leveldb_compact_range}
x!{leveldb_comparator}
x!{leveldb_create_iterator}
x!{leveldb_create_snapshot}
x!{leveldb_delete}
x!{leveldb_destroy_db}
x!{leveldb_env}
x!{leveldb_filterpolicy}
x!{leveldb_filterpolicy_create_bloom}
x!{leveldb_free}
x!{leveldb_get}
x!{leveldb_iter}
x!{leveldb_iter_destroy}
x!{leveldb_iter_seek}
x!{leveldb_iter_valid}
x!{leveldb_open}
x!{leveldb_options}
x!{leveldb_property_value}
x!{leveldb_put}
x!{leveldb_readoptions}
x!{leveldb_release_snapshot}
x!{leveldb_repair_db}
x!{leveldb_version}
x!{leveldb_write}
x!{leveldb_writeoptions}
x!{repairdb}
x!{save_error}

pub(crate) fn bitcoinleveldb_db__make_temp_dbname_bytes(prefix: &str) -> Vec<u8> {
    let unique_box: Box<u8> = Box::new(0u8);
    let unique_ptr: *mut u8 = Box::into_raw(unique_box);
    let unique_tag: usize = unique_ptr as usize;

    unsafe {
        drop(Box::from_raw(unique_ptr));
    }

    let mut root: std::path::PathBuf = std::env::temp_dir();
    root.push("bitcoinleveldb-db");

    let _ = std::fs::create_dir_all(&root);

    root.push(format!("{}_{}", prefix, unique_tag));

    let mut bytes: Vec<u8> = root.to_string_lossy().into_owned().into_bytes();
    bytes.push(0u8);
    bytes
}

#[macro_export]
macro_rules! bitcoinleveldb_db__define_unique_dbname_bytes_fn {
    ($name:ident, $prefix:expr) => {
        fn $name() -> Vec<u8> {
            $crate::bitcoinleveldb_db__make_temp_dbname_bytes($prefix)
        }
    }
}

#[cfg(test)]
mod bitcoinleveldb_db__lib_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__lib_rs__make_unique_dbname_bytes() -> Vec<u8> {
        crate::bitcoinleveldb_db__make_temp_dbname_bytes("bitcoinleveldb_db__lib_rs__integration_testdb")
    }

    unsafe fn bitcoinleveldb_db__lib_rs__free_err_if_non_null(err: *mut u8) {
        if !err.is_null() {
            crate::leveldb_free::leveldb_free(err as *mut core::ffi::c_void);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__lib_rs__end_to_end_c_api_smoke_open_put_get_iter_snapshot_close_destroy() {
        unsafe {
            let opt: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!opt.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(opt, 1u8);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__lib_rs__make_unique_dbname_bytes();

            let mut oerr: *mut u8 = core::ptr::null_mut();

            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                opt,
                dbname_bytes.as_ptr(),
                (&mut oerr) as *mut *mut u8,
            );

            assert!(oerr.is_null());
            assert!(!db.is_null());

            let wopt: *mut LevelDBWriteOptions = crate::leveldb_writeoptions::leveldb_writeoptions_create();
            let ropt: *mut LevelDBReadOptions = crate::leveldb_readoptions::leveldb_readoptions_create();
            assert!(!wopt.is_null());
            assert!(!ropt.is_null());

            let mut err: *mut u8 = core::ptr::null_mut();

            let key: [u8; 2] = [b'k', b'1'];
            let val: [u8; 2] = [b'v', b'1'];

            crate::leveldb_put::leveldb_put(
                db,
                wopt,
                key.as_ptr(),
                key.len(),
                val.as_ptr(),
                val.len(),
                (&mut err) as *mut *mut u8,
            );
            assert!(err.is_null());

            let snap: *const LevelDBSnapshot = crate::leveldb_create_snapshot::leveldb_create_snapshot(db);
            assert!(!snap.is_null());

            crate::leveldb_readoptions::leveldb_readoptions_set_snapshot(ropt, snap);

            crate::leveldb_release_snapshot::leveldb_release_snapshot(db, snap);

            let mut vallen: usize = 0usize;
            let out: *mut u8 = crate::leveldb_get::leveldb_get(
                db,
                ropt,
                key.as_ptr(),
                key.len(),
                (&mut vallen) as *mut usize,
                (&mut err) as *mut *mut u8,
            );

            assert!(err.is_null());
            assert!(!out.is_null());
            assert_eq!(vallen, val.len());

            let got: Vec<u8> = core::slice::from_raw_parts(out as *const u8, vallen).to_vec();
            assert_eq!(got.as_slice(), val.as_slice());

            crate::leveldb_free::leveldb_free(out as *mut core::ffi::c_void);

            let it: *mut LevelDBIterator = crate::leveldb_create_iterator::leveldb_create_iterator(db, ropt);
            assert!(!it.is_null());
            crate::leveldb_iter_seek::leveldb_iter_seek_to_first(it);
            let ok: u8 = crate::leveldb_iter_valid::leveldb_iter_valid(it as *const LevelDBIterator);
            assert_eq!(ok, 1u8);
            crate::leveldb_iter_destroy::leveldb_iter_destroy(it);

            crate::leveldb_readoptions::leveldb_readoptions_destroy(ropt);
            crate::leveldb_writeoptions::leveldb_writeoptions_destroy(wopt);

            crate::leveldb_close::leveldb_close(db);

            let mut derr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(
                opt,
                dbname_bytes.as_ptr(),
                (&mut derr) as *mut *mut u8,
            );

            bitcoinleveldb_db__lib_rs__free_err_if_non_null(derr);
            crate::leveldb_options::leveldb_options_destroy(opt);
        }
    }
}
