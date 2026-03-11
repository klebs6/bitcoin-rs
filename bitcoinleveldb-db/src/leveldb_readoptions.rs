// ---------------- [ File: bitcoinleveldb-db/src/leveldb_readoptions.rs ]
crate::ix!();

pub fn leveldb_readoptions_create() -> *mut LevelDBReadOptions {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_readoptions_create entry");

    let result = Box::new(LevelDBReadOptions::default());

    let p = Box::into_raw(result);

    trace!(
        target: "bitcoinleveldb_db::c_api",
        ptr = (p as usize),
        "leveldb_readoptions_create exit"
    );
    p
}

pub fn leveldb_readoptions_destroy(opt: *mut LevelDBReadOptions) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        "leveldb_readoptions_destroy entry"
    );

    unsafe {
        if opt.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_readoptions_destroy called with null opt"
            );
            return;
        }
        drop(Box::from_raw(opt));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_readoptions_destroy exit");
}

pub fn leveldb_readoptions_set_verify_checksums(opt: *mut LevelDBReadOptions, v: u8) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        v = v,
        "leveldb_readoptions_set_verify_checksums entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_readoptions_set_verify_checksums: null opt"
            );
            return;
        }
        (*opt).rep_mut().set_verify_checksums(v != 0);
    }
}

pub fn leveldb_readoptions_set_fill_cache(opt: *mut LevelDBReadOptions, v: u8) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        v = v,
        "leveldb_readoptions_set_fill_cache entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_readoptions_set_fill_cache: null opt"
            );
            return;
        }
        (*opt).rep_mut().set_fill_cache(v != 0);
    }
}

pub fn leveldb_readoptions_set_snapshot(opt: *mut LevelDBReadOptions, snap: *const LevelDBSnapshot) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        snap_is_null = snap.is_null(),
        "leveldb_readoptions_set_snapshot entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_readoptions_set_snapshot: null opt"
            );
            return;
        }

        if snap.is_null() {
            (*opt).rep_mut().set_snapshot(None);
            return;
        }

        let arc: Arc<LevelDBSnapshot> = Arc::from_raw(snap as *const LevelDBSnapshot);
        let cloned: Arc<dyn Snapshot> = arc.clone();
        (*opt).rep_mut().set_snapshot(Some(cloned));
        let _ = Arc::into_raw(arc);
    }
}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_readoptions_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_readoptions_rs__make_unique_dbname_bytes() -> Vec<u8> {
        crate::bitcoinleveldb_db__make_temp_dbname_bytes("bitcoinleveldb_db__readoptions_rs__testdb")
    }

    unsafe fn bitcoinleveldb_db__leveldb_readoptions_rs__free_err_if_non_null(err: *mut u8) {
        if !err.is_null() {
            crate::leveldb_free::leveldb_free(err as *mut core::ffi::c_void);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_readoptions_rs__create_destroy_roundtrip_is_safe() {
        unsafe {
            let ropt: *mut LevelDBReadOptions = leveldb_readoptions_create();
            assert!(!ropt.is_null());
            leveldb_readoptions_destroy(ropt);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_readoptions_rs__setters_handle_null_opt_safely() {
        unsafe {
            leveldb_readoptions_set_verify_checksums(core::ptr::null_mut(), 1u8);
            leveldb_readoptions_set_fill_cache(core::ptr::null_mut(), 1u8);
            leveldb_readoptions_set_snapshot(core::ptr::null_mut(), core::ptr::null());
        }
        assert!(true);
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_readoptions_rs__snapshot_refcount_allows_releasing_snapshot_handle_after_setting_on_readoptions() {
        unsafe {
            let opt: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!opt.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(opt, 1u8);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__leveldb_readoptions_rs__make_unique_dbname_bytes();

            let mut oerr: *mut u8 = core::ptr::null_mut();

            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                opt,
                dbname_bytes.as_ptr(),
                (&mut oerr) as *mut *mut u8,
            );

            assert!(oerr.is_null());
            assert!(!db.is_null());

            let wopt: *mut LevelDBWriteOptions = crate::leveldb_writeoptions::leveldb_writeoptions_create();
            assert!(!wopt.is_null());

            let ropt_snap: *mut LevelDBReadOptions = leveldb_readoptions_create();
            assert!(!ropt_snap.is_null());

            let key: [u8; 1] = [b'k'];
            let v1: [u8; 2] = [b'v', b'1'];
            let v2: [u8; 2] = [b'v', b'2'];

            let mut perr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_put::leveldb_put(
                db,
                wopt,
                key.as_ptr(),
                key.len(),
                v1.as_ptr(),
                v1.len(),
                (&mut perr) as *mut *mut u8,
            );
            assert!(perr.is_null());

            let snap: *const LevelDBSnapshot = crate::leveldb_create_snapshot::leveldb_create_snapshot(db);
            assert!(!snap.is_null());

            leveldb_readoptions_set_snapshot(ropt_snap, snap);

            crate::leveldb_release_snapshot::leveldb_release_snapshot(db, snap);

            crate::leveldb_put::leveldb_put(
                db,
                wopt,
                key.as_ptr(),
                key.len(),
                v2.as_ptr(),
                v2.len(),
                (&mut perr) as *mut *mut u8,
            );
            assert!(perr.is_null());

            let mut vallen: usize = 0usize;
            let mut gerr: *mut u8 = core::ptr::null_mut();
            let out: *mut u8 = crate::leveldb_get::leveldb_get(
                db,
                ropt_snap,
                key.as_ptr(),
                key.len(),
                (&mut vallen) as *mut usize,
                (&mut gerr) as *mut *mut u8,
            );

            assert!(gerr.is_null());
            assert!(!out.is_null());
            assert_eq!(vallen, v1.len());

            let got: Vec<u8> = core::slice::from_raw_parts(out as *const u8, vallen).to_vec();
            assert_eq!(got.as_slice(), v1.as_slice());

            crate::leveldb_free::leveldb_free(out as *mut core::ffi::c_void);

            leveldb_readoptions_destroy(ropt_snap);
            crate::leveldb_writeoptions::leveldb_writeoptions_destroy(wopt);

            crate::leveldb_close::leveldb_close(db);

            let mut derr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(
                opt,
                dbname_bytes.as_ptr(),
                (&mut derr) as *mut *mut u8,
            );

            bitcoinleveldb_db__leveldb_readoptions_rs__free_err_if_non_null(derr);
            crate::leveldb_options::leveldb_options_destroy(opt);
        }
    }
}
