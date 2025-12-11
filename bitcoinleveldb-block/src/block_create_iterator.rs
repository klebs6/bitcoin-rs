// ---------------- [ File: bitcoinleveldb-block/src/block_create_iterator.rs ]
crate::ix!();

impl Block {
    /// Create a `LevelDBIterator` wrapper over a `BlockIter` for this block.
    ///
    /// Mirrors LevelDB's `Block::NewIterator` semantics:
    /// - small/invalid blocks yield an error iterator
    /// - blocks with zero restarts yield an empty iterator
    /// - otherwise a `BlockIter` over the block's contents is returned
    pub fn new_iterator(
        &self,
        cmp: *const dyn SliceComparator,
    ) -> *mut LevelDBIterator {
        unsafe {
            assert!(
                !cmp.is_null(),
                "Block::new_iterator: comparator pointer is null"
            );

            let size           = self.size();
            let restart_offset = self.restart_offset();
            let data_ptr       = self.data_ptr();
            let num_restarts   = self.num_restarts();

            trace!(
                "Block::new_iterator: size={}, restart_offset={}, num_restarts={}, data={:?}, comparator_ptr={:p}",
                size,
                restart_offset,
                num_restarts,
                data_ptr,
                cmp
            );

            // Corruption case: not enough room for at least one restart offset
            // plus the num_restarts trailer.
            if size < 2 * core::mem::size_of::<u32>() {
                trace!(
                    "Block::new_iterator: block too small for restart metadata (size={}); returning error iterator",
                    size
                );

                let msg_bytes = b"bad block contents"[..].to_vec();
                let msg_slice = Slice::from(msg_bytes.as_slice());
                let status    = Status::corruption(&msg_slice, None);

                let error_iter = EmptyIterator::new(status);
                let error_iter_box: Box<dyn LevelDBIteratorInterface> =
                    Box::new(error_iter);

                let wrapper     = LevelDBIterator::new(Some(error_iter_box));
                let wrapper_box = Box::new(wrapper);
                let wrapper_ptr = Box::into_raw(wrapper_box);

                trace!(
                    "Block::new_iterator: returning error iterator wrapper_ptr={:?}",
                    wrapper_ptr
                );

                return wrapper_ptr;
            }

            // Empty block case: structurally valid but reports zero restart
            // points.
            if num_restarts == 0 {
                trace!(
                    "Block::new_iterator: num_restarts is zero; returning empty iterator (size={}, restart_offset={}, data={:?})",
                    size,
                    restart_offset,
                    data_ptr
                );

                let status     = Status::default();
                let empty_iter = EmptyIterator::new(status);
                let empty_box: Box<dyn LevelDBIteratorInterface> =
                    Box::new(empty_iter);

                let wrapper     = LevelDBIterator::new(Some(empty_box));
                let wrapper_box = Box::new(wrapper);
                let wrapper_ptr = Box::into_raw(wrapper_box);

                trace!(
                    "Block::new_iterator: returning empty iterator wrapper_ptr={:?}",
                    wrapper_ptr
                );

                return wrapper_ptr;
            }

            // Normal case: construct the underlying BlockIter over this block's
            // contents.
            let block_iter = bitcoinleveldb_blockiter::BlockIter::new(
                cmp,
                data_ptr,
                restart_offset,
                num_restarts,
            );

            let block_iter_box = Box::new(block_iter);
            let block_iter_ptr: *const bitcoinleveldb_blockiter::BlockIter =
                &*block_iter_box;

            // Treat the BlockIter as a generic LevelDB iterator interface.
            let block_iter_iface: Box<dyn LevelDBIteratorInterface> =
                block_iter_box;

            // Wrap the underlying iterator in LevelDBIterator so callers see the
            // uniform facade expected by the rest of the table code.
            let wrapper     = LevelDBIterator::new(Some(block_iter_iface));
            let wrapper_box = Box::new(wrapper);
            let wrapper_ptr: *mut LevelDBIterator = Box::into_raw(wrapper_box);

            trace!(
                "Block::new_iterator: created BlockIter at {:?}, wrapper_ptr={:?}",
                block_iter_ptr,
                wrapper_ptr
            );

            wrapper_ptr
        }
    }
}

#[cfg(test)]
mod block_new_iterator_tests {
    use super::*;

    fn build_minimal_block_with_zero_restarts() -> Block {
        let mut bytes = vec![0u8; 4];
        bytes[..].copy_from_slice(&0u32.to_le_bytes());
        let slice    = Slice::from(bytes.as_slice());
        let contents = BlockContents::new(slice, false, false);
        Block::new(&contents)
    }

    #[traced_test]
    fn new_iterator_returns_error_iterator_for_too_small_block() {
        let mut block = Block {
            data:           core::ptr::null(),
            size:           core::mem::size_of::<u32>() - 1,
            restart_offset: 0,
            owned:          false,
        };

        let cmp = bitcoinleveldb_comparator::BytewiseComparatorImpl::default();
        let cmp_ref: &dyn SliceComparator = &cmp;
        let cmp_ptr: *const dyn SliceComparator = cmp_ref as *const dyn SliceComparator;

        trace!("calling Block::new_iterator on undersized block");
        let iter_ptr = block.new_iterator(cmp_ptr);
        debug!("new_iterator returned pointer {:?}", iter_ptr);
        assert!(!iter_ptr.is_null());
    }

    #[traced_test]
    fn new_iterator_returns_empty_iterator_when_no_restarts() {
        let mut block = build_minimal_block_with_zero_restarts();

        let cmp = bitcoinleveldb_comparator::BytewiseComparatorImpl::default();
        let cmp_ref: &dyn SliceComparator = &cmp;
        let cmp_ptr: *const dyn SliceComparator = cmp_ref as *const dyn SliceComparator;

        trace!("calling Block::new_iterator on block with num_restarts=0");
        let iter_ptr = block.new_iterator(cmp_ptr);
        debug!(
            "new_iterator for block with zero restarts returned pointer {:?}",
            iter_ptr
        );
        assert!(!iter_ptr.is_null());
    }
}
