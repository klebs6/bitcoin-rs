// ---------------- [ File: bitcoinleveldb-db/src/leveldb_create_iterator.rs ]
crate::ix!();

pub fn leveldb_create_iterator(db: *mut LevelDB, options: *const LevelDBReadOptions) -> *mut LevelDBIterator {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_create_iterator entry";
        "db_is_null" => db.is_null(),
        "options_is_null" => options.is_null()
    );

    unsafe {
        if db.is_null() || options.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_create_iterator received null input pointer");
            return core::ptr::null_mut();
        }

        let ropt: &ReadOptions = &(*options).rep;
        let it = (*db).rep.borrow_mut().new_iterator(ropt);

        trace!(target: "bitcoinleveldb_db::c_api", "leveldb_create_iterator exit"; "iter_is_null" => it.is_null());
        it
    }

    /*
        leveldb_iterator_t* result = new leveldb_iterator_t;
      result->rep = db->rep->NewIterator(options->rep);
      return result;
    */
}
