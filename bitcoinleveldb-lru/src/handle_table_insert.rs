// ---------------- [ File: bitcoinleveldb-lru/src/handle_table_insert.rs ]
crate::ix!();

impl HandleTable {

    pub fn insert(&mut self, h: *mut LRUHandle) -> *mut LRUHandle {
        unsafe {
            let hash = (*h).hash_value();
            let key  = (*h).key();

            trace!(
                "HandleTable::insert: handle={:p}, hash={}, elems={}, length={}",
                h,
                hash,
                self.elems(),
                self.length()
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
                self.set_elems(self.elems().wrapping_add(1));
                if self.elems() > self.length() {
                    debug!(
                        "HandleTable::insert: resizing (elems={}, length={})",
                        self.elems(),
                        self.length()
                    );
                    self.resize();
                }
            }

            old
        }
    }
}
