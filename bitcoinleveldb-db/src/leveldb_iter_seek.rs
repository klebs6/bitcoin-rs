// ---------------- [ File: bitcoinleveldb-db/src/leveldb_iter_seek.rs ]
crate::ix!();

pub fn leveldb_iter_seek_to_first(iter: *mut LevelDBIterator)  {
    
    todo!();
        /*
            iter->rep->SeekToFirst();
        */
}

pub fn leveldb_iter_seek_to_last(iter: *mut LevelDBIterator)  {
    
    todo!();
        /*
            iter->rep->SeekToLast();
        */
}

pub fn leveldb_iter_seek(
        iter: *mut LevelDBIterator,
        k:    *const u8,
        klen: usize)  {
    
    todo!();
        /*
            iter->rep->Seek(Slice(k, klen));
        */
}

pub fn leveldb_iter_seek_to_first(iter: *mut LevelDBIterator) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_iter_seek_to_first entry"; "iter_is_null" => iter.is_null());

    unsafe {
        if iter.is_null() {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_iter_seek_to_first called with null iter");
            return;
        }

        (*iter).seek_to_first();
    }

    /*
        iter->rep->SeekToFirst();
    */
}

pub fn leveldb_iter_seek_to_last(iter: *mut LevelDBIterator) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_iter_seek_to_last entry"; "iter_is_null" => iter.is_null());

    unsafe {
        if iter.is_null() {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_iter_seek_to_last called with null iter");
            return;
        }

        (*iter).seek_to_last();
    }

    /*
        iter->rep->SeekToLast();
    */
}

pub fn leveldb_iter_seek(iter: *mut LevelDBIterator, k: *const u8, klen: usize) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_iter_seek entry";
        "iter_is_null" => iter.is_null(),
        "k_is_null" => k.is_null(),
        "klen" => klen
    );

    unsafe {
        if iter.is_null() {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_iter_seek called with null iter");
            return;
        }

        let target = Slice::from_ptr_len(k, klen);
        (*iter).seek(&target);
    }

    /*
        iter->rep->Seek(Slice(k, klen));
    */
}
