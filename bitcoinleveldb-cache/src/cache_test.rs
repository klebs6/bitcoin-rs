// ---------------- [ File: bitcoinleveldb-cache/src/cache_test.rs ]
crate::ix!();

pub const CACHE_TEST_CACHE_SIZE: usize = 1000;

/// Internal payload stored as the cache value during tests.
///
/// This lets the deleter know **both** which `CacheTest` fixture
/// owns the entry and what logical integer value was stored,
/// without relying on any global mutable state.
#[derive(Getters, Setters, Builder)]
#[getset(get = "pub(crate)", set = "pub(crate)")]
pub struct CacheTestValue {
    fixture: *mut CacheTest,
    value:   i32,
}

/// Kept for compatibility with the original C++-style tests.
/// It is now a no-op because the fixture pointer is carried
/// inside `CacheTestValue` instead of a global.
pub fn set_current_cache_test(test: *mut CacheTest) {
    trace!(
        "cache_test::set_current_cache_test: registering fixture {:?} (no-op)",
        test
    );
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/cache_test.cc]
#[derive(MutGetters, Getters, Setters, Builder)]
#[getset(get_mut="pub", get="pub", set="pub(crate)")]
pub struct CacheTest {
    deleted_keys:   Vec<i32>,
    deleted_values: Vec<i32>,
    cache:          *mut Cache,
}

impl Default for CacheTest {
    fn default() -> Self {
        CacheTest::new(CACHE_TEST_CACHE_SIZE)
    }
}

impl Drop for CacheTest {
    fn drop(&mut self) {
        unsafe {
            if !self.cache.is_null() {
                debug!("CacheTest::drop: dropping cache {:?}", self.cache);
                let boxed = Box::from_raw(self.cache);
                drop(boxed);
                self.cache = std::ptr::null_mut();
            }
        }
    }
}

impl CacheTest {

    pub fn new(capacity: usize) -> Self {
        info!("CacheTest::new: creating fixture with cache size={}", capacity);
        let cache_ptr = new_lru_cache(capacity);
        CacheTest {
            deleted_keys:   Vec::new(),
            deleted_values: Vec::new(),
            cache:          cache_ptr,
        }
    }

    pub fn reset_cache(&mut self, capacity: usize) {
        unsafe {
            if !self.cache.is_null() {
                debug!(
                    "CacheTest::reset_cache: dropping existing cache {:?}",
                    self.cache
                );
                let boxed = Box::from_raw(self.cache);
                drop(boxed);
            }
        }
        self.cache = new_lru_cache(capacity);
        debug!(
            "CacheTest::reset_cache: new cache {:?} with capacity={}",
            self.cache,
            capacity
        );
    }

    /// Deleter used for all test cache entries.
    ///
    /// The `value` pointer is a `Box<CacheTestValue>` that encodes
    /// both the owning fixture and the logical `i32` value.
    pub fn deleter(key_: &Slice, v: *mut c_void) {
        trace!("CacheTest::deleter: called");
        unsafe {
            if v.is_null() {
                debug!("CacheTest::deleter: value pointer is null, skipping");
                return;
            }

            // Take ownership so the payload is freed exactly once.
            let boxed_value: Box<CacheTestValue> =
                Box::from_raw(v as *mut CacheTestValue);

            let fixture_ptr = *boxed_value.fixture();
            if fixture_ptr.is_null() {
                debug!(
                    "CacheTest::deleter: fixture pointer is null, not recording deletion"
                );
                return;
            }

            let key_int   = decode_key(key_);
            let value_int = *boxed_value.value();

            let fixture: &mut CacheTest = &mut *fixture_ptr;

            trace!(
                "CacheTest::deleter: recording deleted key={} value={}",
                key_int,
                value_int
            );

            fixture.deleted_keys.push(key_int);
            fixture.deleted_values.push(value_int);
        }
    }

    fn make_slice_for_key(encoded: &[u8]) -> Slice {
        if encoded.is_empty() {
            Slice::from_ptr_len(std::ptr::null(), 0)
        } else {
            Slice::from_ptr_len(encoded.as_ptr(), encoded.len())
        }
    }

    pub fn lookup(&mut self, key_: i32) -> i32 {
        let encoded = encode_key(key_);
        let key_slice = Self::make_slice_for_key(&encoded);
        unsafe {
            let cache_ptr = self.cache;
            let cache = &mut *cache_ptr;
            let handle = cache.lookup(&key_slice);
            if handle.is_null() {
                trace!("CacheTest::lookup: miss for key={}", key_);
                return -1;
            }
            let value_ptr = cache.value(handle);
            let r = decode_value(value_ptr);
            cache.release(handle);
            trace!("CacheTest::lookup: hit for key={} value={}", key_, r);
            r
        }
    }

    pub fn insert(&mut self, key_: i32, value: i32, charge: Option<i32>) {
        let charge = charge.unwrap_or(1);
        let encoded = encode_key(key_);
        let key_slice = Self::make_slice_for_key(&encoded);

        let fixture_ptr: *mut CacheTest = self as *mut CacheTest;
        let value_struct = CacheTestValueBuilder::default()
            .fixture(fixture_ptr)
            .value(value)
            .build()
            .expect("CacheTestValueBuilder should be fully initialized");

        let value_ptr: *mut c_void =
            Box::into_raw(Box::new(value_struct)) as *mut c_void;

        unsafe {
            let cache_ptr = self.cache;
            let cache = &mut *cache_ptr;
            let handle = cache.insert(
                &key_slice,
                value_ptr,
                charge as usize,
                CacheTest::deleter,
            );
            cache.release(handle);
        }

        trace!(
            "CacheTest::insert: key={} value={} charge={}",
            key_,
            value,
            charge
        );
    }

    pub fn insert_and_return_handle(
        &mut self,
        key_: i32,
        value: i32,
        charge: Option<i32>,
    ) -> *mut CacheHandle {
        let charge = charge.unwrap_or(1);
        let encoded = encode_key(key_);
        let key_slice = Self::make_slice_for_key(&encoded);

        let fixture_ptr: *mut CacheTest = self as *mut CacheTest;
        let value_struct = CacheTestValueBuilder::default()
            .fixture(fixture_ptr)
            .value(value)
            .build()
            .expect("CacheTestValueBuilder should be fully initialized");

        let value_ptr: *mut c_void =
            Box::into_raw(Box::new(value_struct)) as *mut c_void;

        unsafe {
            let cache_ptr = self.cache;
            let cache = &mut *cache_ptr;
            let handle = cache.insert(
                &key_slice,
                value_ptr,
                charge as usize,
                CacheTest::deleter,
            );
            trace!(
                "CacheTest::insert_and_return_handle: key={} value={} charge={} handle={:?}",
                key_,
                value,
                charge,
                handle
            );
            handle
        }
    }

    pub fn erase(&mut self, key_: i32) {
        let encoded = encode_key(key_);
        let key_slice = Self::make_slice_for_key(&encoded);
        unsafe {
            let cache_ptr = self.cache;
            let cache = &mut *cache_ptr;
            cache.erase(&key_slice);
        }
        trace!("CacheTest::erase: key={}", key_);
    }
}
