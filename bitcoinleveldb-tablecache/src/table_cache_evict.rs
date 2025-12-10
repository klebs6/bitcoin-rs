// ---------------- [ File: bitcoinleveldb-tablecache/src/table_cache_evict.rs ]
crate::ix!();

impl TableCache {

    /**
      | Evict any entry for the specified file
      | number
      |
      */
    pub fn evict(&mut self, file_number: u64) {
        unsafe {
            trace!(
                "TableCache::evict: evicting file_number={}",
                file_number
            );

            let mut buf = [0u8; core::mem::size_of::<u64>()];
            let mut v = file_number;
            for i in 0..core::mem::size_of::<u64>() {
                buf[i] = (v & 0xff) as u8;
                v >>= 8;
            }
            let key = Slice::from(&buf[..]);

            let cache_ptr = self.cache_raw();
            if cache_ptr.is_null() {
                debug!(
                    "TableCache::evict: cache pointer is null; nothing to evict (file_number={})",
                    file_number
                );
                return;
            }

            let cache_ref = &mut *cache_ptr;
            cache_ref.erase(&key);
        }
    }
}

#[cfg(test)]
mod table_cache_evict_tests {
    use super::*;
    use crate::table_cache_test_support::*;
    use std::cell::RefCell;
    use std::ffi::c_void;
    use std::rc::Rc;

    #[traced_test]
    fn table_cache_evict_forces_reopen_on_next_get() {
        let (env, state) = make_in_memory_env();
        let mut options = make_options_with_env(env.clone());

        let dbname = String::from("table_cache_evict_db");
        let mut meta = FileMetaData::default();
        meta.set_number(11);

        let mut table_cache = TableCache::new(&dbname, &options, 32);
        let table_cache_ptr: *mut TableCache = &mut table_cache;

        let key = b"k1".to_vec();
        let val = b"v1".to_vec();
        let iter_ptr =
            make_iterator_from_kv_pairs(&[(key.clone(), val.clone())]);
        let meta_ptr: *mut FileMetaData = &mut meta;

        let build_status = build_table(
            &dbname,
            env.clone(),
            &options,
            table_cache_ptr,
            iter_ptr,
            meta_ptr,
        );
        unsafe {
            drop(Box::from_raw(iter_ptr));
        }
        assert!(build_status.is_ok());

        #[derive(Default)]
        struct HitCounter {
            hits: usize,
        }

        fn count_hits(arg: *mut c_void, _k: &Slice, _v: &Slice) -> c_void {
            unsafe {
                let c: &mut HitCounter = &mut *(arg as *mut HitCounter);
                c.hits += 1;
                core::mem::zeroed()
            }
        }

        let read_options = ReadOptions::default();

        let mut counter = HitCounter::default();
        let arg_ptr: *mut c_void = &mut counter as *mut _ as *mut c_void;

        info!("first TableCache::get (expected cache miss + file open)");
        let s1 = table_cache.get(
            &read_options,
            *meta.number(),
            *meta.file_size(),
            &Slice::from("k1"),
            arg_ptr,
            count_hits,
        );
        assert!(s1.is_ok());
        assert_eq!(counter.hits, 1);

        let open_count_after_first = {
            let guard = state.lock();
            let fname = table_file_name(&dbname, *meta.number());
            *guard.random_open_count.get(&fname).unwrap_or(&0)
        };

        info!("second TableCache::get (expected cache hit, no new file open)");
        let s2 = table_cache.get(
            &read_options,
            *meta.number(),
            *meta.file_size(),
            &Slice::from("k1"),
            arg_ptr,
            count_hits,
        );
        assert!(s2.is_ok());
        assert_eq!(counter.hits, 2);

        let open_count_after_second = {
            let guard = state.lock();
            let fname = table_file_name(&dbname, *meta.number());
            *guard.random_open_count.get(&fname).unwrap_or(&0)
        };
        assert_eq!(
            open_count_after_second, open_count_after_first,
            "second get should not reopen file (must be cache hit)"
        );

        trace!("calling TableCache::evict");
        table_cache.evict(*meta.number());

        info!("third TableCache::get (expected cache miss + reopen)");
        let s3 = table_cache.get(
            &read_options,
            *meta.number(),
            *meta.file_size(),
            &Slice::from("k1"),
            arg_ptr,
            count_hits,
        );
        assert!(s3.is_ok());
        assert_eq!(counter.hits, 3);

        let open_count_after_third = {
            let guard = state.lock();
            let fname = table_file_name(&dbname, *meta.number());
            *guard.random_open_count.get(&fname).unwrap_or(&0)
        };

        assert!(
            open_count_after_third > open_count_after_second,
            "after evict, a subsequent get should reopen the table file"
        );
    }
}
