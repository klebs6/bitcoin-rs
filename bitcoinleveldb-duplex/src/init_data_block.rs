// ---------------- [ File: bitcoinleveldb-duplex/src/init_data_block.rs ]
crate::ix!();

impl TwoLevelIterator {
   
    pub fn init_data_block(&mut self) {
        trace!(
            "TwoLevelIterator::init_data_block: index_valid={}, \
             current_data_has_iter={}, current_data_valid={}",
            self.index_iter().valid(),
            self.data_iter().has_iterator(),
            self.data_iter().valid(),
        );

        if !self.index_iter().valid() {
            trace!(
                "TwoLevelIterator::init_data_block: index iterator invalid; \
                 clearing data iterator"
            );
            self.set_data_iterator(None);
            self.data_block_handle_mut().clear();
            return;
        }

        let handle = self.index_iter().value();
        let handle_len = *handle.size();

        let reuse_existing = if self.data_iter().iter().is_some() && !self.data_block_handle().is_empty() {
            let saved = Slice::from(self.data_block_handle().as_slice());
            let cmp = handle.compare(&saved);
            trace!(
                "TwoLevelIterator::init_data_block: comparing handle vs cached; \
                 handle_len={}, cached_len={}, cmp={}",
                handle_len,
                self.data_block_handle().len(),
                cmp,
            );
            cmp == 0
        } else {
            false
        };

        if reuse_existing {
            trace!(
                "TwoLevelIterator::init_data_block: \
                 reusing existing data iterator for same block handle"
            );
            return;
        }

        trace!(
            "TwoLevelIterator::init_data_block: \
             constructing new data iterator via block_function; handle_len={}",
            handle_len
        );

        let iter_opt = (self.block_function())(*self.arg_mut(), self.options(), &handle);

        match iter_opt {
            None => {
                trace!(
                    "TwoLevelIterator::init_data_block: block_function returned None; \
                     clearing data iterator"
                );
                self.data_block_handle_mut().clear();
                self.set_data_iterator(None);
            }
            Some(iter_box) => {
                // Copy handle bytes into `data_block_handle_`.
                self.data_block_handle_mut().clear();

                if handle_len > 0 {
                    unsafe {
                        let data_ptr = *handle.data();
                        let slice = std::slice::from_raw_parts(data_ptr, handle_len);
                        self.data_block_handle_mut().extend_from_slice(slice);
                    }
                }

                let raw_iter: *const dyn LevelDBIteratorInterface = &*iter_box;
                trace!(
                    "TwoLevelIterator::init_data_block: new data iterator={:p}, \
                     cached_handle_len={}",
                    raw_iter,
                    self.data_block_handle().len(),
                );

                self.set_data_iterator(Some(iter_box));
            }
        }
    }
}

#[cfg(test)]
mod two_level_iterator_init_data_block_tests {
    use super::*;
    use core::ffi::c_void;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn counting_none_block_function(
        arg: *mut c_void,
        _options: &ReadOptions,
        _handle: &Slice,
    ) -> Option<Box<dyn LevelDBIteratorInterface>> {
        assert!(
            !arg.is_null(),
            "counting_none_block_function requires non-null arg"
        );
        let counter = unsafe { &mut *(arg as *mut usize) };
        *counter += 1;
        None
    }

    fn test_block_function_for_blocks(
        arg: *mut c_void,
        _options: &ReadOptions,
        handle: &Slice,
    ) -> Option<Box<dyn LevelDBIteratorInterface>> {
        let handle_str = handle.to_string();
        let block_index: usize = handle_str
            .parse()
            .expect("handle should be valid ASCII usize");

        let blocks_ptr = arg as *mut Vec<Vec<(Vec<u8>, Vec<u8>)>>;
        assert!(
            !blocks_ptr.is_null(),
            "blocks pointer passed to test_block_function_for_blocks must not be null"
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

    fn make_simple_index_iterator(num_blocks: usize) -> Box<dyn LevelDBIteratorInterface> {
        let mut pairs: Vec<(Vec<u8>, Vec<u8>)> = Vec::with_capacity(num_blocks);
        for i in 0..num_blocks {
            let key = format!("k{}", i).into_bytes();
            let value = format!("{}", i).into_bytes();
            pairs.push((key, value));
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
    fn init_data_block_clears_state_when_index_iterator_is_invalid() {
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
        two.data_block_handle_mut().extend_from_slice(b"stale-handle");

        assert!(
            !two.index_iter().valid(),
            "precondition: index iterator should start out invalid"
        );
        assert!(
            two.data_iter().has_iterator(),
            "precondition: data iterator should be present before init_data_block"
        );
        assert!(
            !two.data_block_handle().is_empty(),
            "precondition: cached handle should be non-empty before init_data_block"
        );

        two.init_data_block();

        assert_eq!(
            block_function_calls, 0,
            "block_function must not be invoked when index iterator is invalid"
        );
        assert!(
            two.data_iter().iter().is_none(),
            "data iterator should be cleared when index iterator is invalid"
        );
        assert!(
            two.data_block_handle().is_empty(),
            "cached data_block_handle should be cleared when index iterator is invalid"
        );
        assert!(
            two.status().is_ok(),
            "init_data_block should not introduce an error when clearing state"
        );
    }

    #[traced_test]
    fn init_data_block_reuses_existing_iterator_when_handle_matches() {
        let call_counter = AtomicUsize::new(0);
        let arg: *mut c_void =
            &call_counter as *const AtomicUsize as *mut c_void;

        fn counting_block_function(
            arg: *mut c_void,
            _options: &ReadOptions,
            _handle: &Slice,
        ) -> Option<Box<dyn LevelDBIteratorInterface>> {
            assert!(
                !arg.is_null(),
                "counting_block_function requires non-null arg"
            );
            let counter = unsafe { &*(arg as *const AtomicUsize) };
            counter.fetch_add(1, Ordering::SeqCst);
            Some(Box::new(MockStubIterator::new_empty()))
        }

        let pairs: [(&[u8], &[u8]); 1] = [(b"index0", b"handle0")];
        let mut raw_index = MockStubIterator::new_with_entries(&pairs);
        raw_index.seek_to_first();

        let index_iter: Box<dyn LevelDBIteratorInterface> = Box::new(raw_index);
        let options = ReadOptions::default();

        let mut two = TwoLevelIterator::new(
            index_iter,
            counting_block_function,
            arg,
            options,
        );

        let initial_data_iter = MockStubIterator::new_empty();
        two.set_data_iterator(Some(Box::new(initial_data_iter)));

        let handle = two.index_iter().value();
        let handle_bytes = slice_as_bytes(&handle).to_vec();
        two.data_block_handle_mut().clear();
        two.data_block_handle_mut().extend_from_slice(&handle_bytes);

        assert!(
            two.data_iter().has_iterator(),
            "precondition: data iterator should be present before init_data_block"
        );
        assert_eq!(
            call_counter.load(Ordering::SeqCst),
            0,
            "block_function should not have been called before init_data_block"
        );
        let cached_handle_before = two.data_block_handle().clone();

        two.init_data_block();

        assert_eq!(
            call_counter.load(Ordering::SeqCst),
            0,
            "block_function must not be invoked when the cached handle matches the index handle"
        );
        assert!(
            two.data_iter().has_iterator(),
            "data iterator must still be present after init_data_block when the handle matches"
        );
        assert_eq!(
            two.data_block_handle(),
            &cached_handle_before,
            "data_block_handle must remain unchanged when reusing the existing iterator"
        );
    }

    #[traced_test]
    fn init_data_block_clears_iterator_when_block_function_returns_none() {
        let mut block_function_calls: usize = 0;
        let arg: *mut c_void = &mut block_function_calls as *mut usize as *mut c_void;

        let raw_index_pairs: [(&[u8], &[u8]); 1] = [(b"index0", b"handle0")];
        let mut raw_index = MockStubIterator::new_with_entries(&raw_index_pairs);
        raw_index.seek_to_first();

        let index_iter: Box<dyn LevelDBIteratorInterface> = Box::new(raw_index);
        let options = ReadOptions::default();

        let mut two = TwoLevelIterator::new(
            index_iter,
            counting_none_block_function,
            arg,
            options,
        );

        two.set_data_iterator(Some(Box::new(MockStubIterator::new_empty())));
        two.data_block_handle_mut().extend_from_slice(b"stale-handle");

        two.init_data_block();

        assert_eq!(
            block_function_calls, 1,
            "block_function should be invoked exactly once when the handle changes"
        );
        assert!(
            two.data_iter().iter().is_none(),
            "data iterator should be cleared when block_function returns None"
        );
        assert!(
            two.data_block_handle().is_empty(),
            "data_block_handle must be cleared when block_function returns None"
        );
        assert!(
            two.status().is_ok(),
            "internal status must remain OK when block_function returns None"
        );
    }

    #[traced_test]
    fn init_data_block_constructs_new_iterator_and_caches_handle_bytes() {
        let mut blocks: Vec<Vec<(Vec<u8>, Vec<u8>)>> = Vec::new();
        blocks.push(vec![(b"k1".to_vec(), b"v1".to_vec())]);

        let mut blocks_holder = blocks;
        let arg: *mut c_void =
            &mut blocks_holder as *mut Vec<Vec<(Vec<u8>, Vec<u8>)>> as *mut c_void;

        let index_iter = make_simple_index_iterator(blocks_holder.len());
        let options = ReadOptions::default();

        let mut two = TwoLevelIterator::new(
            index_iter,
            test_block_function_for_blocks,
            arg,
            options,
        );

        two.index_iter_mut().seek_to_first();
        assert!(
            two.index_iter().valid(),
            "index iterator should be valid after seek_to_first on the wrapper"
        );
        assert!(
            two.data_iter().iter().is_none(),
            "data iterator should start out empty prior to init_data_block"
        );

        two.init_data_block();

        assert!(
            two.data_iter().iter().is_some(),
            "init_data_block must attach a new data iterator when block_function returns Some"
        );

        let handle = two.index_iter().value();
        let expected_bytes = slice_as_bytes(&handle).to_vec();
        assert_eq!(
            two.data_block_handle().as_slice(),
            expected_bytes.as_slice(),
            "data_block_handle must contain an exact copy of the index handle bytes"
        );
    }
}
