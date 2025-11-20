// ---------------- [ File: bitcoinleveldb-lru/src/handle_table_resize.rs ]
crate::ix!();

impl HandleTable {

    pub fn resize(&mut self) {
        trace!(
            "HandleTable::resize: current length={}, elems={}",
            self.length(),
            self.elems()
        );

        unsafe {
            let mut new_length: u32 = 4;
            while new_length < *self.elems() {
                new_length = new_length.wrapping_mul(2);
            }

            let bytes =
                (new_length as usize) * core::mem::size_of::<*mut LRUHandle>();

            let new_list: *mut *mut LRUHandle = if bytes == 0 {
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

            let old_length: u32               = *self.length();
            let old_list: *mut *mut LRUHandle = *self.list();
            let mut count: u32                = 0;

            for i in 0..old_length {
                let mut h: *mut LRUHandle = *old_list.add(i as usize);
                while !h.is_null() {
                    let next: *mut LRUHandle = (*h).next_hash_ptr();
                    let hash: u32            = (*h).hash_value();
                    let bucket: usize        =
                        (hash & (new_length - 1)) as usize;
                    let slot: *mut *mut LRUHandle = new_list.add(bucket);
                    let prev_head: *mut LRUHandle = *slot;

                    (*h).set_next_hash_ptr(prev_head);
                    *slot = h;
                    h = next;
                    count = count.wrapping_add(1);
                }
            }

            assert_eq!(
                *self.elems(),
                count,
                "HandleTable::resize: element count mismatch (elems_={}, counted={})",
                self.elems(),
                count
            );

            if !old_list.is_null() {
                libc::free(old_list as *mut libc::c_void);
            }

            self.set_list(new_list);
            self.set_length(new_length);

            debug!(
                "HandleTable::resize: new length={}, elems={}",
                self.length(),
                self.elems()
            );
        }
    }
}
