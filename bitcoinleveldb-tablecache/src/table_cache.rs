// ---------------- [ File: bitcoinleveldb-tablecache/src/table_cache.rs ]
/*!
  | Thread-safe (provides internal synchronization)
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/table_cache.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/table_cache.cc]

pub struct TableCache {
    env:     Box<dyn Env>,
    dbname:  String,
    options: Rc<Options>,
    cache:   *mut Cache,
}

impl Drop for TableCache {
    fn drop(&mut self) {
        unsafe {
            if self.cache.is_null() {
                trace!(
                    "TableCache::drop: cache pointer is null; nothing to free (dbname='{}')",
                    self.dbname
                );
                return;
            }

            trace!(
                "TableCache::drop: deleting Cache @ {:?} for dbname='{}'",
                self.cache,
                self.dbname
            );

            let _cache_box: Box<Cache> = Box::from_raw(self.cache);
            self.cache = core::ptr::null_mut();
        }
    }
}

impl TableCache {

    pub fn new(dbname: &String, options: &Options, entries: i32) -> Self {
        use bitcoinleveldb_cache::NewLRUCache;

        trace!(
            "TableCache::new: dbname='{}', entries={}",
            dbname,
            entries
        );

        let cache_ptr: *mut Cache = unsafe { NewLRUCache(entries) };

        unsafe {
            assert!(
                !cache_ptr.is_null(),
                "TableCache::new: NewLRUCache returned null"
            );
        }

        TableCache {
            env: options.env.clone(),
            dbname: dbname.clone(),
            options: Rc::new(options.clone()),
            cache: cache_ptr,
        }
    }
}
