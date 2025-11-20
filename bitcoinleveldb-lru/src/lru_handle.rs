// ---------------- [ File: bitcoinleveldb-lru/src/lru_handle.rs ]
crate::ix!();

/// An entry is a variable length heap-allocated structure. 
///
/// Entries are kept in a circular doubly linked list ordered by access time.
/// 
#[repr(C)]
pub struct LRUHandle {
    value:      *mut c_void,
    deleter:    fn(_0: &Slice, value: *mut c_void) -> c_void,
    next_hash:  *mut LRUHandle,
    next:       *mut LRUHandle,
    prev:       *mut LRUHandle,

    /// TODO(opt): Only allow uint32_t?
    charge:     usize,
    key_length: usize,

    /// Whether entry is in the cache.
    in_cache:   bool,

    /// References, including cache reference, if present.
    refs:       u32,

    /// Hash of key(); used for fast sharding and comparisons
    hash:       u32,

    /// Beginning of key
    key_data:   [u8; 1],
}

impl LRUHandle {

    pub fn make_sentinel() -> LRUHandle {
        trace!("LRUHandle::make_sentinel: constructing LRUHandle sentinel");

        LRUHandle {
            value:      core::ptr::null_mut(),
            deleter:    lru_noop_deleter,
            next_hash:  core::ptr::null_mut(),
            next:       core::ptr::null_mut(), // will be set to self in LRUCacheInner::new
            prev:       core::ptr::null_mut(), // will be set to self in LRUCacheInner::new
            charge:     0,
            key_length: 0,
            in_cache:   false,
            refs:       0,
            hash:       0,
            key_data:   [0u8; 1],
        }
    }

    pub fn key(&self) -> Slice {
        trace!("LRUHandle::key: key_length={}", self.key_length);

        unsafe {
            // next is only equal to this if the LRU handle is the list head of an
            // empty list. List heads never have meaningful keys.
            let self_ptr = self as *const LRUHandle as *mut LRUHandle;
            assert!(
                self.next != self_ptr,
                "LRUHandle::key called on list head / sentinel"
            );

            Slice::from_ptr_len(self.key_data_ptr(), self.key_length)
        }
    }

    /// Pointer to the beginning of the inlined key bytes.
    pub fn key_data_ptr(&self) -> *const u8 {
        self.key_data.as_ptr()
    }

    // ----- value / deleter -----

    pub fn value_ptr(&self) -> *mut c_void {
        self.value
    }

    pub fn set_value_ptr(&mut self, value: *mut c_void) {
        self.value = value;
    }

    pub fn deleter_fn(&self) -> fn(&Slice, *mut c_void) -> c_void {
        self.deleter
    }

    pub fn set_deleter_fn(&mut self, deleter: fn(&Slice, *mut c_void) -> c_void) {
        self.deleter = deleter;
    }

    // ----- hash-chain pointers -----

    pub fn next_hash_ptr(&self) -> *mut LRUHandle {
        self.next_hash
    }

    pub fn set_next_hash_ptr(&mut self, ptr: *mut LRUHandle) {
        self.next_hash = ptr;
    }

    /// Mutable reference to the `next_hash` link, used by HandleTable::find_pointer.
    pub fn next_hash_link(&mut self) -> &mut *mut LRUHandle {
        &mut self.next_hash
    }

    // ----- LRU list pointers -----

    pub fn next_ptr(&self) -> *mut LRUHandle {
        self.next
    }

    pub fn set_next_ptr(&mut self, ptr: *mut LRUHandle) {
        self.next = ptr;
    }

    pub fn prev_ptr(&self) -> *mut LRUHandle {
        self.prev
    }

    pub fn set_prev_ptr(&mut self, ptr: *mut LRUHandle) {
        self.prev = ptr;
    }

    // ----- accounting -----

    pub fn charge_value(&self) -> usize {
        self.charge
    }

    pub fn set_charge_value(&mut self, charge: usize) {
        self.charge = charge;
    }

    pub fn key_len(&self) -> usize {
        self.key_length
    }

    pub fn set_key_length(&mut self, len: usize) {
        self.key_length = len;
    }

    pub fn is_in_cache(&self) -> bool {
        self.in_cache
    }

    pub fn set_in_cache(&mut self, in_cache: bool) {
        self.in_cache = in_cache;
    }

    pub fn refs(&self) -> u32 {
        self.refs
    }

    pub fn set_refs(&mut self, refs: u32) {
        self.refs = refs;
    }

    pub fn increment_refs(&mut self) {
        self.refs = self.refs.wrapping_add(1);
    }

    pub fn decrement_refs(&mut self) {
        self.refs = self.refs.wrapping_sub(1);
    }

    pub fn hash_value(&self) -> u32 {
        self.hash
    }

    pub fn set_hash_value(&mut self, hash: u32) {
        self.hash = hash;
    }

    /// Pointer to the beginning of the inlined key bytes (mutable).
    pub fn key_data_mut(&mut self) -> *mut u8 {
        self.key_data.as_mut_ptr()
    }
}

#[cfg(test)]
mod lru_handle_test_suite {
    use super::*;
    use core::ffi::c_void;

    fn lru_handle_test_deleter(_: &Slice, _: *mut c_void) -> c_void {
        unsafe { core::mem::zeroed() }
    }

    unsafe fn lru_handle_make_for_bytes(key_bytes: &[u8]) -> *mut LRUHandle {
        let key_len   = key_bytes.len();
        let alloc_len = core::mem::size_of::<LRUHandle>() + key_len.saturating_sub(1);

        let handle = libc::malloc(alloc_len) as *mut LRUHandle;
        assert!(
            !handle.is_null(),
            "lru_handle_make_for_bytes: allocation failed"
        );

        (*handle).set_value_ptr(core::ptr::null_mut());
        (*handle).set_deleter_fn(lru_handle_test_deleter);
        (*handle).set_charge_value(17);
        (*handle).set_key_length(key_len);
        (*handle).set_hash_value(0xDEAD_BEEFu32);
        (*handle).set_in_cache(false);
        (*handle).set_refs(3);
        (*handle).set_next_hash_ptr(core::ptr::null_mut());
        (*handle).set_next_ptr(core::ptr::null_mut());
        (*handle).set_prev_ptr(core::ptr::null_mut());

        core::ptr::copy_nonoverlapping(
            key_bytes.as_ptr(),
            (*handle).key_data_mut(),
            key_len,
        );

        handle
    }

    #[traced_test]
    fn lru_handle_key_returns_correct_slice() {
        bitcoin_cfg::setup();

        let key_bytes = b"lh-key-123";
        unsafe {
            let handle = lru_handle_make_for_bytes(key_bytes);
            let key    = (*handle).key();
            let s      = key.to_string();

            assert_eq!(
                s,
                String::from_utf8_lossy(key_bytes),
                "LRUHandle::key should expose the inlined key bytes"
            );

            libc::free(handle as *mut libc::c_void);
        }
    }

    #[traced_test]
    fn lru_handle_accessors_round_trip_values() {
        bitcoin_cfg::setup();

        unsafe {
            let handle = lru_handle_make_for_bytes(b"lh-acc");

            let value_box = Box::new(42i32);
            let value_ptr = Box::into_raw(value_box) as *mut c_void;

            (*handle).set_value_ptr(value_ptr);
            (*handle).set_charge_value(128);
            (*handle).set_in_cache(true);
            (*handle).set_refs(5);
            (*handle).set_hash_value(0xABCDu32);

            assert_eq!(
                (*handle).value_ptr(),
                value_ptr,
                "value_ptr should reflect latest assignment"
            );
            assert_eq!(
                (*handle).charge_value(),
                128,
                "charge_value should match setter"
            );
            assert!(
                (*handle).is_in_cache(),
                "is_in_cache should reflect assignment"
            );
            assert_eq!(
                (*handle).refs(),
                5,
                "refs should match explicit setter"
            );
            assert_eq!(
                (*handle).hash_value(),
                0xABCDu32,
                "hash_value should match setter"
            );

            (*handle).increment_refs();
            assert_eq!(
                (*handle).refs(),
                6,
                "increment_refs should bump refcount"
            );

            (*handle).decrement_refs();
            assert_eq!(
                (*handle).refs(),
                5,
                "decrement_refs should lower refcount"
            );

            // Clean up heap-allocated test value.
            let reclaimed = Box::from_raw((*handle).value_ptr() as *mut i32);
            assert_eq!(
                *reclaimed, 42,
                "round-tripped value through LRUHandle should be preserved"
            );

            libc::free(handle as *mut libc::c_void);
        }
    }
}
