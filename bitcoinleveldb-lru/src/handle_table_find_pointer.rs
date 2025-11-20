// ---------------- [ File: bitcoinleveldb-lru/src/handle_table_find_pointer.rs ]
crate::ix!();

impl HandleTable {

    pub fn lookup(&mut self, key_: &Slice, hash_: u32) -> *mut LRUHandle {
        trace!("HandleTable::lookup: hash={}", hash_);
        unsafe { *self.find_pointer(key_, hash_) }
    }

    /// Return a pointer to slot that points to a cache entry that matches key/hash.  
    ///
    /// If there is no such cache entry, return a pointer to the trailing slot in the corresponding
    /// linked list.
    ///
    pub fn find_pointer(&mut self, key_: &Slice, hash_: u32) -> *mut *mut LRUHandle {
        trace!("HandleTable::find_pointer: hash={}", hash_);

        unsafe {
            debug_assert!(
                *self.length() != 0,
                "HandleTable::find_pointer: length is zero"
            );

            let mask = self.length().wrapping_sub(1);
            let mut ptr: *mut *mut LRUHandle =
                self.list().add((hash_ & mask) as usize);

            loop {
                let entry = *ptr;
                if entry.is_null() {
                    trace!(
                        "HandleTable::find_pointer: reached end of chain without match (hash={})",
                        hash_
                    );
                    break;
                }

                let entry_hash = (*entry).hash_value();
                if entry_hash == hash_ {
                    if handle_table_keys_equal(entry as *const LRUHandle, key_) {
                        trace!(
                            "HandleTable::find_pointer: found matching entry {:p} for hash={}",
                            entry,
                            hash_
                        );
                        break;
                    } else {
                        trace!(
                            "HandleTable::find_pointer: hash match but key mismatch for entry {:p}, hash={}",
                            entry,
                            hash_
                        );
                    }
                } else {
                    trace!(
                        "HandleTable::find_pointer: skipping entry {:p} with non-matching hash={}",
                        entry,
                        entry_hash
                    );
                }

                let next_hash_ref: &mut *mut LRUHandle = (*entry).next_hash_link();
                ptr = next_hash_ref as *mut *mut LRUHandle;
            }

            ptr
        }
    }
}

#[cfg(test)]
mod handle_table_find_pointer_test_suite {
    use super::*;
    use core::ffi::c_void;

    fn handle_table_find_pointer_test_deleter(_: &Slice, _: *mut c_void) -> c_void {
        unsafe { core::mem::zeroed() }
    }

    unsafe fn handle_table_find_pointer_make_handle(
        key_bytes: &[u8],
        hash:      u32,
    ) -> *mut LRUHandle {
        let key_len   = key_bytes.len();
        let alloc_len = core::mem::size_of::<LRUHandle>() + key_len.saturating_sub(1);

        let handle = libc::malloc(alloc_len) as *mut LRUHandle;
        assert!(
            !handle.is_null(),
            "handle_table_find_pointer_make_handle: allocation failed"
        );

        (*handle).set_value_ptr(core::ptr::null_mut());
        (*handle).set_deleter_fn(handle_table_find_pointer_test_deleter);
        (*handle).set_charge_value(0);
        (*handle).set_key_length(key_len);
        (*handle).set_hash_value(hash);
        (*handle).set_in_cache(false);
        (*handle).set_refs(1);
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

    fn handle_table_find_pointer_slice_from_bytes(bytes: &[u8]) -> Slice {
        Slice::from_ptr_len(bytes.as_ptr(), bytes.len())
    }

    #[traced_test]
    fn handle_table_find_pointer_returns_trailing_slot_for_missing_key() {
        bitcoin_cfg::setup();

        let mut table = HandleTable::default();

        let key_bytes = b"ht-fp-missing";
        let hash: u32 = 0xDEAD_BEEFu32;

        let key_slice = handle_table_find_pointer_slice_from_bytes(key_bytes);

        let slot = table.find_pointer(&key_slice, hash);
        unsafe {
            let entry = *slot;
            assert!(
                entry.is_null(),
                "find_pointer should return a slot pointing to null for a missing key"
            );
        }
    }

    #[traced_test]
    fn handle_table_find_pointer_finds_correct_entry_in_bucket_chain() {
        bitcoin_cfg::setup();

        let mut table = HandleTable::default();

        let key1_bytes = b"ht-fp-key-1";
        let key2_bytes = b"ht-fp-key-2";

        // Choose hashes that collide into the same bucket for the default
        // length (4), exercising chained lookup.
        let hash1: u32 = 0;
        let hash2: u32 = 4;

        let key1_slice = handle_table_find_pointer_slice_from_bytes(key1_bytes);
        let key2_slice = handle_table_find_pointer_slice_from_bytes(key2_bytes);

        unsafe {
            let h1 = handle_table_find_pointer_make_handle(key1_bytes, hash1);
            let h2 = handle_table_find_pointer_make_handle(key2_bytes, hash2);

            let old1 = table.insert(h1);
            assert!(
                old1.is_null(),
                "first insert should not replace an existing entry"
            );

            let old2 = table.insert(h2);
            assert!(
                old2.is_null(),
                "second insert with distinct key/hash should not replace an existing entry"
            );

            let slot1 = table.find_pointer(&key1_slice, hash1);
            let found1 = *slot1;
            assert_eq!(
                found1, h1,
                "find_pointer must return the slot pointing at the first entry in the bucket chain"
            );

            let slot2 = table.find_pointer(&key2_slice, hash2);
            let found2 = *slot2;
            assert_eq!(
                found2, h2,
                "find_pointer must return the slot for the matching entry deeper in the bucket chain"
            );

            let removed1 = table.remove(&key1_slice, hash1);
            let removed2 = table.remove(&key2_slice, hash2);

            assert_eq!(removed1, h1);
            assert_eq!(removed2, h2);

            libc::free(h1 as *mut libc::c_void);
            libc::free(h2 as *mut libc::c_void);
        }
    }
}
