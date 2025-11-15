// ---------------- [ File: bitcoinleveldb-cache/src/cache_test.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/cache_test.cc]

#[cfg(test)]
pub struct CacheTest {
    deleted_keys:   Vec<i32>,
    deleted_values: Vec<i32>,
    cache:          *mut Cache,
}

#[cfg(test)]
mod cache_test {
    use super::*;

    pub const CACHE_SIZE: usize = 1000;

    lazy_static! {
        /// Pointer to the currently active CacheTest fixture.
        pub static ref CURRENT_TEST: Mutex<*mut CacheTest> =
            Mutex::new(std::ptr::null_mut());
    }

    pub fn set_current(test: *mut CacheTest) {
        let mut guard = CURRENT_TEST.lock().unwrap();
        *guard = test;
        trace!("cache_test::set_current: set current fixture {:?}", test);
    }
}

#[cfg(test)]
impl Default for CacheTest {
    fn default() -> Self {
        CacheTest::new(cache_test::CACHE_SIZE)
    }
}

#[cfg(test)]
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
        let mut guard = cache_test::CURRENT_TEST.lock().unwrap();
        let current_ptr = *guard;
        let self_ptr = self as *mut CacheTest;
        if current_ptr == self_ptr {
            *guard = std::ptr::null_mut();
            trace!("CacheTest::drop: cleared CURRENT_TEST");
        }
    }
}

#[cfg(test)]
impl CacheTest {
    fn new(capacity: usize) -> Self {
        info!("CacheTest::new: creating fixture with cache size={}", capacity);
        let cache_ptr = new_lru_cache(capacity);
        CacheTest {
            deleted_keys:   Vec::new(),
            deleted_values: Vec::new(),
            cache:          cache_ptr,
        }
    }

    fn reset_cache(&mut self, capacity: usize) {
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
            self.cache, capacity
        );
    }

    fn deleter(key_: &Slice, v: *mut c_void) {
        trace!("CacheTest::deleter: called");
        let mut guard = cache_test::CURRENT_TEST.lock().unwrap();
        let current_ptr = *guard;
        if current_ptr.is_null() {
            debug!("CacheTest::deleter: no current CacheTest fixture set");
            return;
        }
        unsafe {
            let current = &mut *current_ptr;
            let key_int = decode_key(key_);
            let value_int = decode_value(v);
            trace!(
                "CacheTest::deleter: recording deleted key={} value={}",
                key_int,
                value_int
            );
            current.deleted_keys.push(key_int);
            current.deleted_values.push(value_int);
        }
    }

    fn lookup(&mut self, key_: i32) -> i32 {
        let encoded = encode_key(key_);
        let key_slice: Slice = (&encoded[..]).into();
        unsafe {
            let cache = &mut *self.cache;
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

    fn insert(&mut self, key_: i32, value: i32, charge: Option<i32>) {
        let charge = charge.unwrap_or(1);
        let encoded = encode_key(key_);
        let key_slice: Slice = (&encoded[..]).into();
        let value_ptr = encode_value(value as uintptr_t);
        unsafe {
            let cache = &mut *self.cache;
            let handle =
                cache.insert(&key_slice, value_ptr, charge as usize, CacheTest::deleter);
            cache.release(handle);
        }
        trace!(
            "CacheTest::insert: key={} value={} charge={}",
            key_,
            value,
            charge
        );
    }

    fn insert_and_return_handle(
        &mut self,
        key_: i32,
        value: i32,
        charge: Option<i32>,
    ) -> *mut CacheHandle {
        let charge = charge.unwrap_or(1);
        let encoded = encode_key(key_);
        let key_slice: Slice = (&encoded[..]).into();
        let value_ptr = encode_value(value as uintptr_t);
        unsafe {
            let cache = &mut *self.cache;
            let handle =
                cache.insert(&key_slice, value_ptr, charge as usize, CacheTest::deleter);
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

    fn erase(&mut self, key_: i32) {
        let encoded = encode_key(key_);
        let key_slice: Slice = (&encoded[..]).into();
        unsafe {
            let cache = &mut *self.cache;
            cache.erase(&key_slice);
        }
        trace!("CacheTest::erase: key={}", key_);
    }
}
