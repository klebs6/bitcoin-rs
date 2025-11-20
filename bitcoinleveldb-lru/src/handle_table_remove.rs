// ---------------- [ File: bitcoinleveldb-lru/src/handle_table_remove.rs ]
crate::ix!();

impl HandleTable {

    pub fn remove(&mut self, key_: &Slice, hash_: u32) -> *mut LRUHandle {
        trace!("HandleTable::remove: hash={}", hash_);
        unsafe {
            let ptr    = self.find_pointer(key_, hash_);
            let result = *ptr;
            if !result.is_null() {
                let next_hash = (*result).next_hash_ptr();
                *ptr = next_hash;
                self.set_elems(self.elems().wrapping_sub(1));
            }
            result
        }
    }
}
