// ---------------- [ File: bitcoinleveldb-block/src/block_create_iterator.rs ]
crate::ix!();

impl Block {

    pub fn new_iterator(
        &mut self,
        comparator: *const dyn SliceComparator,
    ) -> *mut LevelDBIterator {
        trace!(
            "Block::new_iterator: size={}, restart_offset={}, owned={}, data={:p}, comparator_ptr={:p}",
            self.size(),
            self.restart_offset(),
            self.is_owned(),
            self.data_ptr(),
            comparator
        );

        let trailer_len = core::mem::size_of::<u32>();

        if self.size() < trailer_len {
            let msg_bytes = b"bad block contents";
            let msg_slice = Slice::from(&msg_bytes[..]);
            let status    = Status::corruption(&msg_slice, None);

            error!(
                "Block::new_iterator: block too small for trailer (size={}); returning error iterator",
                self.size()
            );

            return new_error_iterator(&status);
        }

        let num_restarts = self.num_restarts();
        trace!(
            "Block::new_iterator: num_restarts={}",
            num_restarts
        );

        if num_restarts == 0 {
            trace!(
                "Block::new_iterator: no restart points; returning empty iterator"
            );
            return new_empty_iterator();
        }

        trace!(
            "Block::new_iterator: creating BlockIter (data={:p}, restart_offset={}, num_restarts={}, comparator_ptr={:p})",
            self.data_ptr(),
            self.restart_offset(),
            num_restarts,
            comparator
        );

        let iter = BlockIter::new(
            comparator,
            self.data_ptr(),
            self.restart_offset(),
            num_restarts,
        );

        let boxed    = Box::new(iter);
        let raw_iter: *mut BlockIter = Box::into_raw(boxed);

        unsafe {
            let base_ptr: *mut LevelDBIterator = (*raw_iter).base_mut_ptr();
            trace!(
                "Block::new_iterator: created BlockIter at {:?}, base_ptr={:?}",
                raw_iter,
                base_ptr
            );
            base_ptr
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
