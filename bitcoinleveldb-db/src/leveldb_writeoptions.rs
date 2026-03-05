// ---------------- [ File: bitcoinleveldb-db/src/leveldb_writeoptions.rs ]
crate::ix!();

pub fn leveldb_writeoptions_create() -> *mut LevelDBWriteOptions {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_writeoptions_create entry");

    let result = Box::new(LevelDBWriteOptions::default());

    let p = Box::into_raw(result);

    trace!(
        target: "bitcoinleveldb_db::c_api",
        ptr = (p as usize),
        "leveldb_writeoptions_create exit"
    );
    p

    /*
        return new leveldb_writeoptions_t;
    */
}

pub fn leveldb_writeoptions_destroy(opt: *mut LevelDBWriteOptions) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        "leveldb_writeoptions_destroy entry"
    );

    unsafe {
        if opt.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_writeoptions_destroy called with null opt"
            );
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
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        v = v,
        "leveldb_writeoptions_set_sync entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_writeoptions_set_sync: null opt"
            );
            return;
        }
        (*opt).rep_mut().set_sync(v != 0);
    }

}
