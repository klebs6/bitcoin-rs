// ---------------- [ File: bitcoinleveldb-tablecache/src/table_cache.rs ]
/*!
  | Thread-safe (provides internal synchronization)
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/table_cache.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/table_cache.cc]

pub struct TableCache {
    env:     Rc<RefCell<dyn Env>>,
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
        trace!(
            "TableCache::new: dbname='{}', entries={}",
            dbname,
            entries
        );

        let capacity: usize = if entries <= 0 {
            1
        } else {
            entries as usize
        };

        let cache_ptr: *mut Cache =
            bitcoinleveldb_cache::new_lru_cache(capacity);

        unsafe {
            assert!(
                !cache_ptr.is_null(),
                "TableCache::new: new_lru_cache returned null"
            );
        }

        let env_rc = {
            let opt_env = options.env();
            assert!(
                opt_env.is_some(),
                "TableCache::new: Options.env is None; env must be configured before creating TableCache"
            );
            opt_env.as_ref().unwrap().clone()
        };

        TableCache {
            env:     env_rc,
            dbname:  dbname.clone(),
            options: Rc::new(options.clone()),
            cache:   cache_ptr,
        }
    }

    #[inline]
    pub(crate) fn env_handle(&self) -> Rc<RefCell<dyn Env>> {
        trace!(
            "TableCache::env_handle: cloning Env handle for dbname='{}'",
            self.dbname
        );
        self.env.clone()
    }

    #[inline]
    pub(crate) fn dbname_str(&self) -> &String {
        &self.dbname
    }

    #[inline]
    pub(crate) fn cache_raw(&self) -> *mut Cache {
        self.cache
    }

    #[inline]
    pub(crate) fn options_ref(&self) -> &Options {
        &*self.options
    }
}

#[cfg(test)]
mod table_cache_construction_tests {
    use super::*;
    use crate::table_cache_test_support::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[traced_test]
    fn table_cache_new_initializes_with_valid_cache_pointer() {
        let (env, _state) = make_in_memory_env();
        let mut options = make_options_with_env(env.clone());

        let dbname = String::from("table_cache_new_db");
        options.set_max_open_files(64);

        trace!("constructing TableCache via new");
        let table_cache = TableCache::new(&dbname, &options, 16);

        let _ = table_cache;
    }
}
