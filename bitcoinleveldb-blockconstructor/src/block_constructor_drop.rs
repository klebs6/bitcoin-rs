// ---------------- [ File: bitcoinleveldb-blockconstructor/src/block_constructor_drop.rs ]
crate::ix!();

impl Drop for BlockConstructor {
    fn drop(&mut self) {
        let block_ptr = self.block_ptr();
        let data_len  = self.data_string().len();

        trace!(
            "Dropping BlockConstructor {{ block: {:?}, data_len: {} }}",
            block_ptr,
            data_len
        );

        if !block_ptr.is_null() {
            unsafe {
                debug!(
                    "BlockConstructor::drop: destroying Block at {:?}",
                    block_ptr
                );
                let _owned: Box<Block> = Box::from_raw(block_ptr);
                // `_owned` is dropped here; Block::drop will run and handle its own state.
            }
            *self.block_ptr_mut() = core::ptr::null_mut();
        } else {
            trace!("BlockConstructor::drop: no block to destroy (block pointer is null)");
        }
    }
}
