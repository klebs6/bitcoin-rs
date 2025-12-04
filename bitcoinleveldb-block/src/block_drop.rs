// ---------------- [ File: bitcoinleveldb-block/src/block_drop.rs ]
crate::ix!();

impl Drop for Block {

    fn drop(&mut self) {
        let size           = self.size();
        let data_ptr       = self.data_ptr();
        let restart_offset = self.restart_offset();
        let owned          = self.is_owned();

        trace!(
            "Dropping Block {{ data: {:?}, size: {}, restart_offset: {}, owned: {} }}",
            data_ptr,
            size,
            restart_offset,
            owned
        );

        if owned && !data_ptr.is_null() && size > 0 {
            unsafe {
                let ptr      = data_ptr as *mut u8;
                let len      = size;
                let capacity = len;

                debug!(
                    "Block::drop reclaiming owned buffer at {:?} (len={}, cap={})",
                    ptr,
                    len,
                    capacity
                );

                let _reclaimed: Vec<u8> = Vec::from_raw_parts(ptr, len, capacity);
            }
        } else {
            trace!(
                "Block::drop: no owned buffer to reclaim (owned={}, data_null={}, size={})",
                owned,
                data_ptr.is_null(),
                size
            );
        }
    }
}

#[cfg(test)]
mod block_drop_memory_ownership_tests {
    use super::*;

    #[traced_test]
    fn block_drop_does_not_panic_when_not_owned() {
        let mut backing = vec![0u8; 8];
        backing[4..].copy_from_slice(&1u32.to_le_bytes());

        let block = Block {
            data:           backing.as_ptr(),
            size:           backing.len(),
            restart_offset: 0,
            owned:          false,
        };

        trace!("dropping non-owned Block in test");
        drop(block);
    }

    #[traced_test]
    fn block_drop_reclaims_heap_allocated_buffer_without_panic() {
        let boxed: Box<[u8]> = vec![0u8; 8].into_boxed_slice();
        let len = boxed.len();
        let ptr = Box::into_raw(boxed) as *mut u8;

        unsafe {
            // Encode a single restart for completeness.
            let trailer = 1u32.to_le_bytes();
            core::ptr::copy_nonoverlapping(trailer.as_ptr(), ptr.add(4), 4);
        }

        let slice    = Slice::from_ptr_len(ptr as *const u8, len);
        let contents = BlockContents::new(slice, false, true);
        let block    = Block::new(&contents);

        debug!(
            "constructed owned Block for drop test: data={:?}, size={}",
            block.data_ptr(),
            block.size()
        );

        drop(block);
    }
}
