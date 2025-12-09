// ---------------- [ File: bitcoinleveldb-duplex/src/skip_empty_data_blocks.rs ]
crate::ix!();

impl TwoLevelIterator {

    pub fn skip_empty_data_blocks_forward(&mut self) {
        trace!(
            "TwoLevelIterator::skip_empty_data_blocks_forward: \
             start; index_valid={}, data_valid={}, data_has_iter={}",
            self.index_iter().valid(),
            self.data_iter().valid(),
            self.data_iter().has_iterator(),
        );

        while self.data_iter().iter().is_none() || !self.data_iter().valid() {
            // Move to next block
            if !self.index_iter().valid() {
                trace!(
                    "TwoLevelIterator::skip_empty_data_blocks_forward: \
                     index iterator exhausted; clearing data iterator"
                );
                self.set_data_iterator(None);
                return;
            }

            self.index_iter_mut().next();
            self.init_data_block();

            if self.data_iter().iter().is_some() {
                self.data_iter_mut().seek_to_first();
            }

            trace!(
                "TwoLevelIterator::skip_empty_data_blocks_forward: \
                 advanced block; index_valid={}, data_valid={}, data_has_iter={}",
                self.index_iter().valid(),
                self.data_iter().valid(),
                self.data_iter().has_iterator(),
            );
        }

        trace!(
            "TwoLevelIterator::skip_empty_data_blocks_forward: done; data_valid={}",
            self.data_iter().valid()
        );
    }

    pub fn skip_empty_data_blocks_backward(&mut self) {
        trace!(
            "TwoLevelIterator::skip_empty_data_blocks_backward: \
             start; index_valid={}, data_valid={}, data_has_iter={}",
            self.index_iter().valid(),
            self.data_iter().valid(),
            self.data_iter().has_iterator(),
        );

        while self.data_iter().iter().is_none() || !self.data_iter().valid() {
            // Move to previous block
            if !self.index_iter().valid() {
                trace!(
                    "TwoLevelIterator::skip_empty_data_blocks_backward: \
                     index iterator exhausted; clearing data iterator"
                );
                self.set_data_iterator(None);
                return;
            }

            self.index_iter_mut().prev();
            self.init_data_block();

            if self.data_iter().iter().is_some() {
                self.data_iter_mut().seek_to_last();
            }

            trace!(
                "TwoLevelIterator::skip_empty_data_blocks_backward: \
                 moved block; index_valid={}, data_valid={}, data_has_iter={}",
                self.index_iter().valid(),
                self.data_iter().valid(),
                self.data_iter().has_iterator(),
            );
        }

        trace!(
            "TwoLevelIterator::skip_empty_data_blocks_backward: done; data_valid={}",
            self.data_iter().valid()
        );
    }
}

#[cfg(test)]
mod two_level_iterator_skip_empty_data_blocks_tests {
    use super::*;
    use core::ffi::c_void;

    fn test_block_function(
        arg: *mut c_void,
        _options: &ReadOptions,
        handle: &Slice,
    ) -> Option<Box<dyn LevelDBIteratorInterface>> {
        let handle_str = handle.to_string();
        let block_index: usize = handle_str
            .parse()
            .expect("handle should be a valid ASCII usize");

        let blocks_ptr = arg as *mut Vec<Vec<(Vec<u8>, Vec<u8>)>>;
        assert!(
            !blocks_ptr.is_null(),
            "blocks pointer passed to test_block_function must not be null"
        );

        let blocks = unsafe { &mut *blocks_ptr };

        if block_index >= blocks.len() {
            return None;
        }

        let entries_ref = &blocks[block_index];
        if entries_ref.is_empty() {
            let iter = MockStubIterator::new_empty();
            return Some(Box::new(iter));
        }

        let mut pairs: Vec<(&[u8], &[u8])> = Vec::with_capacity(entries_ref.len());
        for (k, v) in entries_ref.iter() {
            pairs.push((k.as_slice(), v.as_slice()));
        }

        let iter = MockStubIterator::new_with_entries(&pairs);
        Some(Box::new(iter))
    }

    fn make_index_iterator(num_blocks: usize) -> Box<dyn LevelDBIteratorInterface> {
        let mut pairs: Vec<(Vec<u8>, Vec<u8>)> = Vec::with_capacity(num_blocks);
        for i in 0..num_blocks {
            let key_str = format!("k{}", i);
            let val_str = format!("{}", i);
            pairs.push((key_str.into_bytes(), val_str.into_bytes()));
        }

        let pairs_slice: Vec<(&[u8], &[u8])> = pairs
            .iter()
            .map(|(k, v)| (k.as_slice(), v.as_slice()))
            .collect();

        let mut iter = MockStubIterator::new_with_entries(&pairs_slice);
        iter.seek_to_first();
        Box::new(iter)
    }

    #[traced_test]
    fn skip_empty_data_blocks_forward_with_invalid_index_clears_data_iterator() {
        fn counting_none_block_function(
            arg: *mut c_void,
            _options: &ReadOptions,
            _handle: &Slice,
        ) -> Option<Box<dyn LevelDBIteratorInterface>> {
            if !arg.is_null() {
                let counter = unsafe { &mut *(arg as *mut usize) };
                *counter += 1;
            }
            None
        }

        let mut block_function_calls: usize = 0;
        let arg: *mut c_void = &mut block_function_calls as *mut usize as *mut c_void;

        let index_iter: Box<dyn LevelDBIteratorInterface> =
            Box::new(MockStubIterator::new_empty());
        let options = ReadOptions::default();
        let mut two = TwoLevelIterator::new(
            index_iter,
            counting_none_block_function,
            arg,
            options,
        );

        two.set_data_iterator(Some(Box::new(MockStubIterator::new_empty())));
        assert!(
            two.data_iter().iter().is_some(),
            "precondition: data iterator should be present before skipping"
        );
        assert!(
            !two.index_iter().valid(),
            "precondition: index iterator must be invalid for this test"
        );

        two.skip_empty_data_blocks_forward();

        assert_eq!(
            block_function_calls, 0,
            "block_function must not be invoked when the index iterator is already invalid"
        );
        assert!(
            two.data_iter().iter().is_none(),
            "skip_empty_data_blocks_forward should clear the data iterator when the index is invalid"
        );
        assert!(
            !two.valid(),
            "TwoLevelIterator should report invalid after clearing the data iterator"
        );
    }

    #[traced_test]
    fn skip_empty_data_blocks_forward_skips_over_empty_blocks_and_finds_next_entry() {
        let mut blocks: Vec<Vec<(Vec<u8>, Vec<u8>)>> = Vec::new();
        blocks.push(Vec::new()); // block 0: empty
        blocks.push(vec![(b"k1".to_vec(), b"v1".to_vec())]); // block 1: single entry
        blocks.push(Vec::new()); // block 2: empty
        blocks.push(vec![(b"k3".to_vec(), b"v3".to_vec())]); // block 3: single entry

        let mut blocks_holder = blocks;
        let arg: *mut c_void =
            &mut blocks_holder as *mut Vec<Vec<(Vec<u8>, Vec<u8>)>> as *mut c_void;

        let index_iter = make_index_iterator(blocks_holder.len());
        let options = ReadOptions::default();
        let mut two = TwoLevelIterator::new(index_iter, test_block_function, arg, options);

        two.index_iter_mut().seek_to_first();
        assert!(
            two.index_iter().valid(),
            "index iterator should be valid after seek_to_first on the wrapper"
        );
        assert!(
            two.data_iter().iter().is_none(),
            "data iterator should be empty before skipping"
        );

        two.skip_empty_data_blocks_forward();

        assert!(
            two.valid(),
            "iterator should be valid after skipping the initial empty block"
        );
        assert_eq!(
            two.key().to_string(),
            "k1",
            "skip_empty_data_blocks_forward should land on the first key in the first non-empty block"
        );

        two.data_iter_mut().next();
        assert!(
            !two.data_iter().valid(),
            "data iterator should be invalid after advancing past the only entry in block 1"
        );

        two.skip_empty_data_blocks_forward();

        assert!(
            two.valid(),
            "iterator should be valid after skipping another empty block and moving to the next non-empty block"
        );
        assert_eq!(
            two.key().to_string(),
            "k3",
            "skip_empty_data_blocks_forward should land on the entry in block 3 after skipping block 2"
        );

        two.data_iter_mut().next();
        assert!(
            !two.data_iter().valid(),
            "data iterator should be invalid after consuming the last entry"
        );

        two.skip_empty_data_blocks_forward();

        assert!(
            !two.valid(),
            "iterator should be invalid after skipping past the final block"
        );
        assert!(
            two.data_iter().iter().is_none(),
            "data iterator should be cleared once the index iterator is exhausted"
        );
    }

    #[traced_test]
    fn skip_empty_data_blocks_backward_skips_empty_blocks_and_finds_previous_entry() {
        let mut blocks: Vec<Vec<(Vec<u8>, Vec<u8>)>> = Vec::new();
        blocks.push(vec![(b"k0".to_vec(), b"v0".to_vec())]); // block 0: single entry
        blocks.push(Vec::new()); // block 1: empty
        blocks.push(vec![(b"k2".to_vec(), b"v2".to_vec())]); // block 2: single entry

        let mut blocks_holder = blocks;
        let arg: *mut c_void =
            &mut blocks_holder as *mut Vec<Vec<(Vec<u8>, Vec<u8>)>> as *mut c_void;

        let index_iter = make_index_iterator(blocks_holder.len());
        let options = ReadOptions::default();
        let mut two = TwoLevelIterator::new(index_iter, test_block_function, arg, options);

        two.index_iter_mut().seek_to_last();
        two.init_data_block();
        if two.data_iter().iter().is_some() {
            two.data_iter_mut().seek_to_last();
        }

        assert!(
            two.valid(),
            "iterator should be valid after initializing the last non-empty block"
        );
        assert_eq!(
            two.key().to_string(),
            "k2",
            "precondition: iterator should be positioned at the last key in the last block"
        );

        two.data_iter_mut().prev();
        assert!(
            two.data_iter().iter().is_some(),
            "data iterator should still be present after moving within the last block"
        );
        assert!(
            !two.data_iter().valid(),
            "data iterator should be invalid after moving before the first element in the last block"
        );

        two.skip_empty_data_blocks_backward();

        assert!(
            two.valid(),
            "iterator should become valid again after skipping the intervening empty block"
        );
        assert_eq!(
            two.key().to_string(),
            "k0",
            "skip_empty_data_blocks_backward should land on the entry from the previous non-empty block"
        );
    }
}
