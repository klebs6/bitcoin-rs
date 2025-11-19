// ---------------- [ File: bitcoinleveldb-lru/src/handle_table.rs ]
crate::ix!();

/**
  | We provide our own simple hash table since it
  | removes a whole bunch of porting hacks and is
  | also faster than some of the built-in hash
  | table implementations in some of the
  | compiler/runtime combinations we have tested.
  | E.g., readrandom speeds up by ~5% over the g++
  | 4.4.3's builtin hashtable.
  */
pub struct HandleTable {

    /**
      | The table consists of an array of buckets
      | where each bucket is a linked list of
      | cache entries that hash into the bucket.
      |
      */
    length: u32,
    elems:  u32,
    list:   *mut *mut LRUHandle,
}

impl Default for HandleTable {

    fn default() -> Self {
        trace!("HandleTable::default: initializing");

        let mut table = HandleTable {
            length: 0,
            elems:  0,
            list:   core::ptr::null_mut(),
        };

        table.resize();
        table
    }
}

impl Drop for HandleTable {

    fn drop(&mut self) {
        trace!(
            "HandleTable::drop: length={}, elems={}",
            self.length,
            self.elems
        );

        unsafe {
            if !self.list.is_null() {
                libc::free(self.list as *mut libc::c_void);
                self.list = core::ptr::null_mut();
            }
        }
    }
}

impl HandleTable {

    pub fn lookup(&mut self, key_: &Slice, hash_: u32) -> *mut LRUHandle {
        trace!("HandleTable::lookup: hash={}", hash_);
        unsafe { *self.find_pointer(key_, hash_) }
    }

    pub fn insert(&mut self, h: *mut LRUHandle) -> *mut LRUHandle {
        unsafe {
            let hash = (*h).hash_value();
            let key  = (*h).key();

            trace!(
                "HandleTable::insert: handle={:p}, hash={}, elems={}, length={}",
                h,
                hash,
                self.elems,
                self.length
            );

            let ptr = self.find_pointer(&key, hash);
            let old = *ptr;

            if old.is_null() {
                (*h).set_next_hash_ptr(core::ptr::null_mut());
            } else {
                let next_hash = (*old).next_hash_ptr();
                (*h).set_next_hash_ptr(next_hash);
            }

            *ptr = h;

            if old.is_null() {
                self.elems = self.elems.wrapping_add(1);
                if self.elems > self.length {
                    debug!(
                        "HandleTable::insert: resizing (elems={}, length={})",
                        self.elems,
                        self.length
                    );
                    self.resize();
                }
            }

            old
        }
    }

    pub fn remove(&mut self, key_: &Slice, hash_: u32) -> *mut LRUHandle {
        trace!("HandleTable::remove: hash={}", hash_);
        unsafe {
            let ptr    = self.find_pointer(key_, hash_);
            let result = *ptr;
            if !result.is_null() {
                let next_hash = (*result).next_hash_ptr();
                *ptr = next_hash;
                self.elems = self.elems.wrapping_sub(1);
            }
            result
        }
    }
 
    /**
      | Return a pointer to slot that points to
      | a cache entry that matches key/hash.  If
      | there is no such cache entry, return
      | a pointer to the trailing slot in the
      | corresponding linked list.
      */
    pub fn find_pointer(&mut self, key_: &Slice, hash_: u32) -> *mut *mut LRUHandle {
        trace!("HandleTable::find_pointer: hash={}", hash_);

        unsafe {
            debug_assert!(
                self.length != 0,
                "HandleTable::find_pointer: length is zero"
            );

            let mask = self.length.wrapping_sub(1);
            let mut ptr: *mut *mut LRUHandle =
                self.list.add((hash_ & mask) as usize);

            loop {
                let entry = *ptr;
                if entry.is_null() {
                    break;
                }

                if (*entry).hash_value() == hash_ && *key_ == (*entry).key() {
                    break;
                }

                let next_hash_ref: &mut *mut LRUHandle = (*entry).next_hash_link();
                ptr = next_hash_ref as *mut *mut LRUHandle;
            }

            ptr
        }
    }
   
    pub fn resize(&mut self) {
        trace!(
            "HandleTable::resize: current length={}, elems={}",
            self.length,
            self.elems
        );

        unsafe {
            let mut new_length: u32 = 4;
            while new_length < self.elems {
                new_length = new_length.wrapping_mul(2);
            }

            let bytes = (new_length as usize) * core::mem::size_of::<*mut LRUHandle>();
            let new_list = if bytes == 0 {
                core::ptr::null_mut()
            } else {
                let ptr = libc::malloc(bytes) as *mut *mut LRUHandle;
                if !ptr.is_null() {
                    core::ptr::write_bytes(ptr as *mut u8, 0u8, bytes);
                }
                ptr
            };

            if new_list.is_null() {
                error!(
                    "HandleTable::resize: allocation failed for {} bytes; leaving table unchanged",
                    bytes
                );
                return;
            }

            let old_length = self.length;
            let old_list   = self.list;
            let mut count: u32 = 0;

            for i in 0..old_length {
                let mut h = *old_list.add(i as usize);
                while !h.is_null() {
                    let next = (*h).next_hash_ptr();
                    let hash = (*h).hash_value();
                    let bucket = (hash & (new_length - 1)) as usize;
                    let ptr = new_list.add(bucket);
                    let prev_head = *ptr;
                    (*h).set_next_hash_ptr(prev_head);
                    *ptr = h;
                    h = next;
                    count = count.wrapping_add(1);
                }
            }

            assert_eq!(
                self.elems, count,
                "HandleTable::resize: element count mismatch (elems_={}, counted={})",
                self.elems, count
            );

            if !old_list.is_null() {
                libc::free(old_list as *mut libc::c_void);
            }

            self.list   = new_list;
            self.length = new_length;

            debug!(
                "HandleTable::resize: new length={}, elems={}",
                self.length,
                self.elems
            );
        }
    }
}

#[cfg(test)]
mod handle_table_test_suite {
    use super::*;
    use core::ffi::c_void;

    fn handle_table_test_deleter(_: &Slice, _: *mut c_void) -> c_void {
        unsafe { core::mem::zeroed() }
    }

    unsafe fn handle_table_make_lru_handle_for_key(
        key_bytes: &[u8],
        hash:      u32,
    ) -> *mut LRUHandle {
        let key_len   = key_bytes.len();
        let alloc_len = core::mem::size_of::<LRUHandle>() + key_len.saturating_sub(1);

        let handle = libc::malloc(alloc_len) as *mut LRUHandle;
        assert!(
            !handle.is_null(),
            "handle_table_make_lru_handle_for_key: allocation failed"
        );

        (*handle).set_value_ptr(core::ptr::null_mut());
        (*handle).set_deleter_fn(handle_table_test_deleter);
        (*handle).set_charge_value(0);
        (*handle).set_key_length(key_len);
        (*handle).set_hash_value(hash);
        (*handle).set_in_cache(false);
        (*handle).set_refs(0);
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
    fn handle_table_insert_and_lookup_single_entry() {
        bitcoin_cfg::setup();
        let mut table = HandleTable::default();

        let key_bytes = b"ht-key-1";
        let hash      = 0x1234_5678u32;
        let key_slice = Slice::from_ptr_len(key_bytes.as_ptr(), key_bytes.len());

        unsafe {
            let handle = handle_table_make_lru_handle_for_key(key_bytes, hash);

            let old = table.insert(handle);
            assert!(
                old.is_null(),
                "expected first insert to have no previous entry"
            );

            let found = table.lookup(&key_slice, hash);
            assert_eq!(
                found, handle,
                "lookup should return the handle that was inserted"
            );

            let removed = table.remove(&key_slice, hash);
            assert_eq!(
                removed, handle,
                "remove should yield the same handle that was inserted"
            );

            libc::free(handle as *mut libc::c_void);
        }
    }

    #[traced_test]
    fn handle_table_insert_replaces_existing_entry_for_same_key_and_hash() {
        bitcoin_cfg::setup();
        let mut table = HandleTable::default();

        let key_bytes = b"ht-key-2";
        let hash      = 0xCAFEBABEu32;
        let key_slice = Slice::from_ptr_len(key_bytes.as_ptr(), key_bytes.len());

        unsafe {
            let first  = handle_table_make_lru_handle_for_key(key_bytes, hash);
            let second = handle_table_make_lru_handle_for_key(key_bytes, hash);

            let old_first = table.insert(first);
            assert!(
                old_first.is_null(),
                "first insert should not replace an existing entry"
            );

            let old_second = table.insert(second);
            assert_eq!(
                old_second, first,
                "second insert with same key/hash should return the old handle"
            );

            let found = table.lookup(&key_slice, hash);
            assert_eq!(
                found, second,
                "lookup after replacement should return the new handle"
            );

            let removed = table.remove(&key_slice, hash);
            assert_eq!(
                removed, second,
                "remove should yield the most recently inserted handle"
            );

            libc::free(first as *mut libc::c_void);
            libc::free(second as *mut libc::c_void);
        }
    }

    #[traced_test]
    fn handle_table_remove_nonexistent_key_returns_null() {
        bitcoin_cfg::setup();
        let mut table = HandleTable::default();

        let key_bytes = b"missing-key";
        let hash      = 0x0BAD_F00Du32;
        let key_slice = Slice::from_ptr_len(key_bytes.as_ptr(), key_bytes.len());

        let removed = table.remove(&key_slice, hash);
        assert!(
            removed.is_null(),
            "removing a key that was never inserted should return null"
        );
    }

    #[traced_test]
    fn handle_table_resize_retains_all_entries() {
        bitcoin_cfg::setup();
        let mut table = HandleTable::default();

        const RESIZE_KEYS: [&[u8; 2]; 8] = [
            b"k0", b"k1", b"k2", b"k3", b"k4", b"k5", b"k6", b"k7",
        ];

        let mut handles: [*mut LRUHandle; 8] = [core::ptr::null_mut(); 8];
        let mut hashes:  [u32; 8]            = [0; 8];

        unsafe {
            for (i, key_bytes) in RESIZE_KEYS.iter().enumerate() {
                let hash = 0xA000_0000u32.wrapping_add(i as u32);
                let h    = handle_table_make_lru_handle_for_key(key_bytes.as_slice(), hash);

                let old = table.insert(h);
                assert!(
                    old.is_null(),
                    "no previous entry expected while growing table (i={})",
                    i
                );

                handles[i] = h;
                hashes[i]  = hash;
            }

            for (i, key_bytes) in RESIZE_KEYS.iter().enumerate() {
                let key_slice =
                    Slice::from_ptr_len(key_bytes.as_ptr(), key_bytes.len());
                let found = table.lookup(&key_slice, hashes[i]);
                assert_eq!(
                    found, handles[i],
                    "after resize, lookup should still find correct handle (i={})",
                    i
                );
            }

            for (i, key_bytes) in RESIZE_KEYS.iter().enumerate() {
                let key_slice =
                    Slice::from_ptr_len(key_bytes.as_ptr(), key_bytes.len());
                let removed = table.remove(&key_slice, hashes[i]);
                assert_eq!(
                    removed, handles[i],
                    "remove should return handle for key index {}",
                    i
                );
                libc::free(handles[i] as *mut libc::c_void);
            }
        }
    }
}
