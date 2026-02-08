// ---------------- [ File: bitcoinleveldb-db/src/leveldb_writeoptions.rs ]
crate::ix!();

pub fn leveldb_writeoptions_create() -> *mut LevelDBWriteOptions {
    
    todo!();
        /*
            return new leveldb_writeoptions_t;
        */
}

pub fn leveldb_writeoptions_destroy(opt: *mut LevelDBWriteOptions)  {
    
    todo!();
        /*
            delete opt;
        */
}

pub fn leveldb_writeoptions_set_sync(
        opt: *mut LevelDBWriteOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.sync = v;
        */
}

pub fn leveldb_writeoptions_create() -> *mut LevelDBWriteOptions {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_writeoptions_create entry");

    let result = Box::new(LevelDBWriteOptions {
        rep: WriteOptions::default(),
    });

    let p = Box::into_raw(result);

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_writeoptions_create exit"; "ptr" => (p as usize));
    p

    /*
        return new leveldb_writeoptions_t;
    */
}

pub fn leveldb_writeoptions_destroy(opt: *mut LevelDBWriteOptions) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_writeoptions_destroy entry"; "opt_is_null" => opt.is_null());

    unsafe {
        if opt.is_null() {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_writeoptions_destroy called with null opt");
            return;
        }
        drop(Box::from_raw(opt));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_writeoptions_destroy exit");

    /*
        delete opt;
    */
}

pub fn leveldb_writeoptions_set_sync(opt: *mut LevelDBWriteOptions, v: u8) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_writeoptions_set_sync entry"; "opt_is_null" => opt.is_null(), "v" => v);

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_writeoptions_set_sync: null opt");
            return;
        }
        (*opt).rep.set_sync(v != 0);
    }

    /*
        opt->rep.sync = v;
    */
}
