// ---------------- [ File: bitcoinleveldb-db/src/leveldb_readoptions.rs ]
crate::ix!();

pub fn leveldb_readoptions_create() -> *mut LevelDBReadOptions {
    
    todo!();
        /*
            return new leveldb_readoptions_t;
        */
}

pub fn leveldb_readoptions_destroy(opt: *mut LevelDBReadOptions)  {
    
    todo!();
        /*
            delete opt;
        */
}

pub fn leveldb_readoptions_set_verify_checksums(
        opt: *mut LevelDBReadOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.verify_checksums = v;
        */
}

pub fn leveldb_readoptions_set_fill_cache(
        opt: *mut LevelDBReadOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.fill_cache = v;
        */
}

pub fn leveldb_readoptions_set_snapshot(
        opt:  *mut LevelDBReadOptions,
        snap: *const LevelDBSnapshot)  {
    
    todo!();
        /*
            opt->rep.snapshot = (snap ? snap->rep : nullptr);
        */
}

pub fn leveldb_readoptions_create() -> *mut LevelDBReadOptions {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_readoptions_create entry");

    let result = Box::new(LevelDBReadOptions {
        rep: ReadOptions::default(),
    });

    let p = Box::into_raw(result);

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_readoptions_create exit"; "ptr" => (p as usize));
    p

    /*
        return new leveldb_readoptions_t;
    */
}

pub fn leveldb_readoptions_destroy(opt: *mut LevelDBReadOptions) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_readoptions_destroy entry"; "opt_is_null" => opt.is_null());

    unsafe {
        if opt.is_null() {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_readoptions_destroy called with null opt");
            return;
        }
        drop(Box::from_raw(opt));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_readoptions_destroy exit");

    /*
        delete opt;
    */
}

pub fn leveldb_readoptions_set_verify_checksums(opt: *mut LevelDBReadOptions, v: u8) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_readoptions_set_verify_checksums entry"; "opt_is_null" => opt.is_null(), "v" => v);

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_readoptions_set_verify_checksums: null opt");
            return;
        }
        (*opt).rep.set_verify_checksums(v != 0);
    }

    /*
        opt->rep.verify_checksums = v;
    */
}

pub fn leveldb_readoptions_set_fill_cache(opt: *mut LevelDBReadOptions, v: u8) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_readoptions_set_fill_cache entry"; "opt_is_null" => opt.is_null(), "v" => v);

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_readoptions_set_fill_cache: null opt");
            return;
        }
        (*opt).rep.set_fill_cache(v != 0);
    }

    /*
        opt->rep.fill_cache = v;
    */
}

pub fn leveldb_readoptions_set_snapshot(opt: *mut LevelDBReadOptions, snap: *const LevelDBSnapshot) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_readoptions_set_snapshot entry";
        "opt_is_null" => opt.is_null(),
        "snap_is_null" => snap.is_null()
    );

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_readoptions_set_snapshot: null opt");
            return;
        }

        if snap.is_null() {
            (*opt).rep.set_snapshot(None);
        } else {
            (*opt).rep.set_snapshot(Some((*snap).rep.clone()));
        }
    }

    /*
        opt->rep.snapshot = (snap ? snap->rep : nullptr);
    */
}
