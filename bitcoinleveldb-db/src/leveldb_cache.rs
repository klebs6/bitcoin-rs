// ---------------- [ File: bitcoinleveldb-db/src/leveldb_cache.rs ]
crate::ix!();

pub fn leveldb_cache_create_lru(capacity: usize) -> *mut LevelDBCache {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        capacity = capacity,
        "leveldb_cache_create_lru entry"
    );

    unsafe {
        let raw: *mut Cache = new_lru_cache(capacity);
        if raw.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "new_lru_cache returned null"
            );
            return core::ptr::null_mut();
        }

        let boxed: Box<Cache> = Box::from_raw(raw);
        let cache_val: Cache = *boxed;

        let wrapper = Box::new(LevelDBCache::new(cache_val));

        let p = Box::into_raw(wrapper);

        trace!(
            target: "bitcoinleveldb_db::c_api",
            ptr = (p as usize),
            "leveldb_cache_create_lru exit"
        );
        p
    }
}

pub fn leveldb_cache_destroy(cache: *mut LevelDBCache) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        cache_is_null = cache.is_null(),
        "leveldb_cache_destroy entry"
    );

    unsafe {
        if cache.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_cache_destroy called with null cache"
            );
            return;
        }

        drop(Box::from_raw(cache));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_cache_destroy exit");
}
