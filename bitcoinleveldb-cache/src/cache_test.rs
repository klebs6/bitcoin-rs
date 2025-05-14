// ---------------- [ File: bitcoinleveldb-cache/src/cache_test.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/cache_test.cc]

/**
  | Conversions between numeric keys/values
  | and the types expected by Cache.
  |
  */
fn encode_key(k: i32) -> String {
    
    todo!();
        /*
            std::string result;
      PutFixed32(&result, k);
      return result;
        */
}

fn decode_key(k: &Slice) -> i32 {
    
    todo!();
        /*
            assert(k.size() == 4);
      return DecodeFixed32(k.data());
        */
}

fn encode_value(v: libc::uintptr_t)  {
    
    todo!();
        /*
            return reinterpret_cast<c_void*>(v);
        */
}

fn decode_value(v: *mut c_void) -> i32 {
    
    todo!();
        /*
            return reinterpret_cast<uintptr_t>(v);
        */
}

///---------------
struct CacheTest {
    deleted_keys:   Vec<i32>,
    deleted_values: Vec<i32>,
    cache:          *mut Cache,
}

mod cache_test {

    use super::*;

    pub const CACHE_SIZE: i32 = 1000;

    lazy_static!{
        /*
        static CacheTest* current_;
        */
    }
}

impl Default for CacheTest {
    
    fn default() -> Self {
        todo!();
        /*
        : cache(NewLRUCache(kCacheSize)),

            current_ = this;
        */
    }
}

impl Drop for CacheTest {
    fn drop(&mut self) {
        todo!();
        /*
            delete cache_;
        */
    }
}

impl CacheTest {

    fn deleter(
        key_: &Slice,
        v:   *mut c_void)  {
        
        todo!();
        /*
            current_->deleted_keys_.push_back(DecodeKey(key));
        current_->deleted_values_.push_back(DecodeValue(v));
        */
    }
    
    fn lookup(&mut self, key_: i32) -> i32 {
        
        todo!();
        /*
            CacheHandle* handle = cache_->Lookup(EncodeKey(key));
        const int r = (handle == nullptr) ? -1 : DecodeValue(cache_->Value(handle));
        if (handle != nullptr) {
          cache_->Release(handle);
        }
        return r;
        */
    }
    
    fn insert(&mut self, 
        key_:    i32,
        value:  i32,
        charge: Option<i32>)  {
        let charge: i32 = charge.unwrap_or(1);

        todo!();
        /*
            cache_->Release(cache_->Insert(EncodeKey(key), EncodeValue(value), charge,
                                       &CacheTest::Deleter));
        */
    }
    
    fn insert_and_return_handle(&mut self, 
        key_:   i32,
        value:  i32,
        charge: Option<i32>) -> *mut CacheHandle {
        let charge: i32 = charge.unwrap_or(1);

        todo!();
        /*
            return cache_->Insert(EncodeKey(key), EncodeValue(value), charge,
                              &CacheTest::Deleter);
        */
    }
    
    fn erase(&mut self, key_: i32)  {
        
        todo!();
        /*
            cache_->Erase(EncodeKey(key));
        */
    }
}

#[test] fn cache_test_hit_and_miss() {
    todo!();
    /*
    
      ASSERT_EQ(-1, Lookup(100));

      Insert(100, 101);
      ASSERT_EQ(101, Lookup(100));
      ASSERT_EQ(-1, Lookup(200));
      ASSERT_EQ(-1, Lookup(300));

      Insert(200, 201);
      ASSERT_EQ(101, Lookup(100));
      ASSERT_EQ(201, Lookup(200));
      ASSERT_EQ(-1, Lookup(300));

      Insert(100, 102);
      ASSERT_EQ(102, Lookup(100));
      ASSERT_EQ(201, Lookup(200));
      ASSERT_EQ(-1, Lookup(300));

      ASSERT_EQ(1, deleted_keys_.size());
      ASSERT_EQ(100, deleted_keys_[0]);
      ASSERT_EQ(101, deleted_values_[0]);

    */
}

#[test] fn cache_test_erase() {
    todo!();
    /*
    
      Erase(200);
      ASSERT_EQ(0, deleted_keys_.size());

      Insert(100, 101);
      Insert(200, 201);
      Erase(100);
      ASSERT_EQ(-1, Lookup(100));
      ASSERT_EQ(201, Lookup(200));
      ASSERT_EQ(1, deleted_keys_.size());
      ASSERT_EQ(100, deleted_keys_[0]);
      ASSERT_EQ(101, deleted_values_[0]);

      Erase(100);
      ASSERT_EQ(-1, Lookup(100));
      ASSERT_EQ(201, Lookup(200));
      ASSERT_EQ(1, deleted_keys_.size());

    */
}

#[test] fn cache_test_entries_are_pinned() {
    todo!();
    /*
    
      Insert(100, 101);
      CacheHandle* h1 = cache_->Lookup(EncodeKey(100));
      ASSERT_EQ(101, DecodeValue(cache_->Value(h1)));

      Insert(100, 102);
      CacheHandle* h2 = cache_->Lookup(EncodeKey(100));
      ASSERT_EQ(102, DecodeValue(cache_->Value(h2)));
      ASSERT_EQ(0, deleted_keys_.size());

      cache_->Release(h1);
      ASSERT_EQ(1, deleted_keys_.size());
      ASSERT_EQ(100, deleted_keys_[0]);
      ASSERT_EQ(101, deleted_values_[0]);

      Erase(100);
      ASSERT_EQ(-1, Lookup(100));
      ASSERT_EQ(1, deleted_keys_.size());

      cache_->Release(h2);
      ASSERT_EQ(2, deleted_keys_.size());
      ASSERT_EQ(100, deleted_keys_[1]);
      ASSERT_EQ(102, deleted_values_[1]);

    */
}

#[test] fn cache_test_eviction_policy() {
    todo!();
    /*
    
      Insert(100, 101);
      Insert(200, 201);
      Insert(300, 301);
      CacheHandle* h = cache_->Lookup(EncodeKey(300));

      // Frequently used entry must be kept around,
      // as must things that are still in use.
      for (int i = 0; i < kCacheSize + 100; i++) {
        Insert(1000 + i, 2000 + i);
        ASSERT_EQ(2000 + i, Lookup(1000 + i));
        ASSERT_EQ(101, Lookup(100));
      }
      ASSERT_EQ(101, Lookup(100));
      ASSERT_EQ(-1, Lookup(200));
      ASSERT_EQ(301, Lookup(300));
      cache_->Release(h);

    */
}

#[test] fn cache_test_use_exceeds_size() {
    todo!();
    /*
    
      // Overfill the cache, keeping handles on all inserted entries.
      std::vector<CacheHandle*> h;
      for (int i = 0; i < kCacheSize + 100; i++) {
        h.push_back(InsertAndReturnHandle(1000 + i, 2000 + i));
      }

      // Check that all the entries can be found in the cache.
      for (int i = 0; i < h.size(); i++) {
        ASSERT_EQ(2000 + i, Lookup(1000 + i));
      }

      for (int i = 0; i < h.size(); i++) {
        cache_->Release(h[i]);
      }

    */
}

#[test] fn cache_test_heavy_entries() {
    todo!();
    /*
    
      // Add a bunch of light and heavy entries and then count the combined
      // size of items still in the cache, which must be approximately the
      // same as the total capacity.
      const int kLight = 1;
      const int kHeavy = 10;
      int added = 0;
      int index = 0;
      while (added < 2 * kCacheSize) {
        const int weight = (index & 1) ? kLight : kHeavy;
        Insert(index, 1000 + index, weight);
        added += weight;
        index++;
      }

      int cached_weight = 0;
      for (int i = 0; i < index; i++) {
        const int weight = (i & 1 ? kLight : kHeavy);
        int r = Lookup(i);
        if (r >= 0) {
          cached_weight += weight;
          ASSERT_EQ(1000 + i, r);
        }
      }
      ASSERT_LE(cached_weight, kCacheSize + kCacheSize / 10);

    */
}

#[test] fn cache_test_new_id() {
    todo!();
    /*
    
      uint64_t a = cache_->NewId();
      uint64_t b = cache_->NewId();
      ASSERT_NE(a, b);

    */
}

#[test] fn cache_test_prune() {
    todo!();
    /*
    
      Insert(1, 100);
      Insert(2, 200);

      CacheHandle* handle = cache_->Lookup(EncodeKey(1));
      ASSERT_TRUE(handle);
      cache_->Prune();
      cache_->Release(handle);

      ASSERT_EQ(100, Lookup(1));
      ASSERT_EQ(-1, Lookup(2));

    */
}

#[test] fn cache_test_zero_size() {
    todo!();
    /*
    
      delete cache_;
      cache_ = NewLRUCache(0);

      Insert(1, 100);
      ASSERT_EQ(-1, Lookup(1));

    */
}

fn testcache_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
