crate::ix!();

pub struct LevelDBWriteBatch {
    rep: WriteBatch,
}

pub fn leveldb_writebatch_create() -> *mut LevelDBWriteBatch {
    
    todo!();
        /*
            return new leveldb_writebatch_t;
        */
}

pub fn leveldb_writebatch_destroy(b: *mut LevelDBWriteBatch)  {
    
    todo!();
        /*
            delete b;
        */
}

pub fn leveldb_writebatch_clear(b: *mut LevelDBWriteBatch)  {
    
    todo!();
        /*
            b->rep.Clear();
        */
}

pub fn leveldb_writebatch_put(
        b:    *mut LevelDBWriteBatch,
        key_:  *const u8,
        klen: usize,
        val:  *const u8,
        vlen: usize)  {
    
    todo!();
        /*
            b->rep.Put(Slice(key, klen), Slice(val, vlen));
        */
}

pub fn leveldb_writebatch_delete(
        b:    *mut LevelDBWriteBatch,
        key_:  *const u8,
        klen: usize)  {
    
    todo!();
        /*
            b->rep.Delete(Slice(key, klen));
        */
}

pub fn leveldb_writebatch_iterate(
        b:       *const LevelDBWriteBatch,
        state:   *mut c_void,
        put:     fn(
                _0:   *mut c_void,
                k:    *const u8,
                klen: usize,
                v:    *const u8,
                vlen: usize
        ) -> c_void,
        deleted: fn(
                _0:   *mut c_void,
                k:    *const u8,
                klen: usize
        ) -> c_void)  {
    
    todo!();
        /*
            class H : public WriteBatch::Handler {
           
            c_void* state_;
            c_void (*put_)(c_void*, const char* k, size_t klen, const char* v, size_t vlen);
            c_void (*deleted_)(c_void*, const char* k, size_t klen);
            c_void Put(const Slice& key, const Slice& value) override {
              (*put_)(state_, key.data(), key.size(), value.data(), value.size());
            }
            c_void Delete(const Slice& key) override {
              (*deleted_)(state_, key.data(), key.size());
            }
          };
          H handler;
          handler.state_ = state;
          handler.put_ = put;
          handler.deleted_ = deleted;
          b->rep.Iterate(&handler);
        */
}

pub fn leveldb_writebatch_append(
        destination: *mut LevelDBWriteBatch,
        source:      *const LevelDBWriteBatch)  {
    
    todo!();
        /*
            destination->rep.Append(source->rep);
        */
}
