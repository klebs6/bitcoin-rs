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
