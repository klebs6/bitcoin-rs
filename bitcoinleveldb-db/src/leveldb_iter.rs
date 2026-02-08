// ---------------- [ File: bitcoinleveldb-db/src/leveldb_iter.rs ]
crate::ix!();

pub fn leveldb_iter_next(iter: *mut LevelDBIterator)  {
    
    todo!();
        /*
            iter->rep->Next();
        */
}

pub fn leveldb_iter_prev(iter: *mut LevelDBIterator)  {
    
    todo!();
        /*
            iter->rep->Prev();
        */
}

pub fn leveldb_iter_key(
        iter: *const LevelDBIterator,
        klen: *mut usize) -> *const u8 {
    
    todo!();
        /*
            Slice s = iter->rep->key();
          *klen = s.size();
          return s.data();
        */
}

pub fn leveldb_iter_value(
        iter: *const LevelDBIterator,
        vlen: *mut usize) -> *const u8 {
    
    todo!();
        /*
            Slice s = iter->rep->value();
          *vlen = s.size();
          return s.data();
        */
}

pub fn leveldb_iter_get_error(
        iter:   *const LevelDBIterator,
        errptr: *mut *mut u8)  {
    
    todo!();
        /*
            SaveError(errptr, iter->rep->status());
        */
}

pub fn leveldb_iter_next(iter: *mut LevelDBIterator) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_iter_next entry"; "iter_is_null" => iter.is_null());

    unsafe {
        if iter.is_null() {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_iter_next called with null iter");
            return;
        }

        (*iter).next();
    }

    /*
        iter->rep->Next();
    */
}

pub fn leveldb_iter_prev(iter: *mut LevelDBIterator) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_iter_prev entry"; "iter_is_null" => iter.is_null());

    unsafe {
        if iter.is_null() {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_iter_prev called with null iter");
            return;
        }

        (*iter).prev();
    }

    /*
        iter->rep->Prev();
    */
}

pub fn leveldb_iter_key(iter: *const LevelDBIterator, klen: *mut usize) -> *const u8 {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_iter_key entry";
        "iter_is_null" => iter.is_null(),
        "klen_is_null" => klen.is_null()
    );

    unsafe {
        if iter.is_null() || klen.is_null() {
            return core::ptr::null();
        }

        let s: Slice = (*iter).key();
        *klen = *s.size();
        *s.data()
    }

    /*
        Slice s = iter->rep->key();
      *klen = s.size();
      return s.data();
    */
}

pub fn leveldb_iter_value(iter: *const LevelDBIterator, vlen: *mut usize) -> *const u8 {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_iter_value entry";
        "iter_is_null" => iter.is_null(),
        "vlen_is_null" => vlen.is_null()
    );

    unsafe {
        if iter.is_null() || vlen.is_null() {
            return core::ptr::null();
        }

        let s: Slice = (*iter).value();
        *vlen = *s.size();
        *s.data()
    }

    /*
        Slice s = iter->rep->value();
      *vlen = s.size();
      return s.data();
    */
}

pub fn leveldb_iter_get_error(iter: *const LevelDBIterator, errptr: *mut *mut u8) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_iter_get_error entry";
        "iter_is_null" => iter.is_null(),
        "errptr_is_null" => errptr.is_null()
    );

    unsafe {
        if iter.is_null() {
            let msg = Slice::from_str("leveldb_iter_get_error: null iterator");
            let s = crate::Status::invalid_argument(&msg, None);
            let _ = save_error(errptr, &s);
            return;
        }

        let st = (*iter).status();
        let _ = save_error(errptr, &st);

        if !st.is_ok() {
            debug!(target: "bitcoinleveldb_db::c_api", "leveldb_iter_get_error non-ok"; "status" => %st.to_string());
        }
    }

    /*
        SaveError(errptr, iter->rep->status());
    */
}
