// ---------------- [ File: bitcoinleveldb-db/src/leveldb_iter_destroy.rs ]
crate::ix!();

pub fn leveldb_iter_destroy(iter: *mut LevelDBIterator)  {
    
    todo!();
        /*
            delete iter->rep;
          delete iter;
        */
}

pub fn leveldb_iter_destroy(iter: *mut LevelDBIterator) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_iter_destroy entry"; "iter_is_null" => iter.is_null());

    unsafe {
        if iter.is_null() {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_iter_destroy called with null iter");
            return;
        }

        drop(Box::from_raw(iter));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_iter_destroy exit");

    /*
        delete iter->rep;
      delete iter;
    */
}
