// ---------------- [ File: bitcoinleveldb-db/src/leveldb_cache.rs ]
crate::ix!();

pub fn leveldb_cache_create_lru(capacity: usize) -> *mut LevelDBCache {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_cache_create_lru entry"; "capacity" => capacity);

    unsafe {
        let raw: *mut Cache = new_lru_cache(capacity);
        if raw.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "new_lru_cache returned null");
            return core::ptr::null_mut();
        }

        // Take ownership of the allocated Cache and move it into Rc<RefCell<Cache>>.
        let boxed: Box<Cache> = Box::from_raw(raw);
        let cache_val: Cache = Box::into_inner(boxed);

        let wrapper = Box::new(LevelDBCache {
            rep: Rc::new(RefCell::new(cache_val)),
        });

        let p = Box::into_raw(wrapper);

        trace!(target: "bitcoinleveldb_db::c_api", "leveldb_cache_create_lru exit"; "ptr" => (p as usize));
        p
    }

    /*
        leveldb_cache_t* c = new leveldb_cache_t;
      c->rep = NewLRUCache(capacity);
      return c;
    */
}

pub fn leveldb_cache_destroy(cache: *mut LevelDBCache) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_cache_destroy entry"; "cache_is_null" => cache.is_null());

    unsafe {
        if cache.is_null() {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_cache_destroy called with null cache");
            return;
        }

        drop(Box::from_raw(cache));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_cache_destroy exit");

    /*
        delete cache->rep;
      delete cache;
    */
}
