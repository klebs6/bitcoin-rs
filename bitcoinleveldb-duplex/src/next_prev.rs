// ---------------- [ File: bitcoinleveldb-duplex/src/next_prev.rs ]
crate::ix!();

impl LevelDBIteratorNext for TwoLevelIterator {

    fn next(&mut self) {
        trace!(
            "TwoLevelIterator::next: begin; data_valid_before={}",
            self.valid()
        );

        assert!(
            self.valid(),
            "TwoLevelIterator::next requires iterator to be valid"
        );

        self.data_iter_mut().next();
        self.skip_empty_data_blocks_forward();

        trace!(
            "TwoLevelIterator::next: end; data_valid_after={}",
            self.valid()
        );
    }
}

impl LevelDBIteratorPrev for TwoLevelIterator {

    fn prev(&mut self) {
        trace!(
            "TwoLevelIterator::prev: begin; data_valid_before={}",
            self.valid()
        );

        assert!(
            self.valid(),
            "TwoLevelIterator::prev requires iterator to be valid"
        );

        self.data_iter_mut().prev();
        self.skip_empty_data_blocks_backward();

        trace!(
            "TwoLevelIterator::prev: end; data_valid_after={}",
            self.valid()
        );
    }
}

#[cfg(test)]
mod two_level_iterator_next_prev_tests {
    use super::*;
    use core::ffi::c_void;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    fn unused_block_function(
        _arg: *mut c_void,
        _options: &ReadOptions,
        _handle: &Slice,
    ) -> Option<Box<dyn LevelDBIteratorInterface>> {
        None
    }

    #[traced_test]
    fn next_panics_when_iterator_is_not_valid() {
        let result = catch_unwind(AssertUnwindSafe(|| {
            let index_iter: Box<dyn LevelDBIteratorInterface> =
                Box::new(MockStubIterator::new_empty());
            let options = ReadOptions::default();
            let mut two = TwoLevelIterator::new(
                index_iter,
                unused_block_function,
                core::ptr::null_mut(),
                options,
            );

            assert!(
                !two.valid(),
                "precondition: newly constructed TwoLevelIterator must be invalid"
            );

            two.next();
        }));

        assert!(
            result.is_err(),
            "TwoLevelIterator::next should panic when called on an invalid iterator"
        );
    }

    #[traced_test]
    fn prev_panics_when_iterator_is_not_valid() {
        let result = catch_unwind(AssertUnwindSafe(|| {
            let index_iter: Box<dyn LevelDBIteratorInterface> =
                Box::new(MockStubIterator::new_empty());
            let options = ReadOptions::default();
            let mut two = TwoLevelIterator::new(
                index_iter,
                unused_block_function,
                core::ptr::null_mut(),
                options,
            );

            assert!(
                !two.valid(),
                "precondition: newly constructed TwoLevelIterator must be invalid"
            );

            two.prev();
        }));

        assert!(
            result.is_err(),
            "TwoLevelIterator::prev should panic when called on an invalid iterator"
        );
    }

    fn single_block_function(
        arg: *mut c_void,
        _options: &ReadOptions,
        _handle: &Slice,
    ) -> Option<Box<dyn LevelDBIteratorInterface>> {
        assert!(
            !arg.is_null(),
            "single_block_function expects a non-null arg pointing to a Vec<(Vec<u8>, Vec<u8>)>"
        );
        let entries = unsafe { &mut *(arg as *mut Vec<(Vec<u8>, Vec<u8>)>) };
        if entries.is_empty() {
            Some(Box::new(MockStubIterator::new_empty()))
        } else {
            let pairs: Vec<(&[u8], &[u8])> = entries
                .iter()
                .map(|(k, v)| (k.as_slice(), v.as_slice()))
                .collect();
            Some(Box::new(MockStubIterator::new_with_entries(&pairs)))
        }
    }

    #[traced_test]
    fn next_moves_iterator_to_invalid_after_last_element() {
        let mut block_entries: Vec<(Vec<u8>, Vec<u8>)> =
            vec![(b"k".to_vec(), b"v".to_vec())];
        let arg: *mut c_void =
            &mut block_entries as *mut Vec<(Vec<u8>, Vec<u8>)> as *mut c_void;

        let index_pairs: [(&[u8], &[u8]); 1] = [(b"index0", b"0")];
        let mut raw_index = MockStubIterator::new_with_entries(&index_pairs);
        raw_index.seek_to_first();
        let index_iter: Box<dyn LevelDBIteratorInterface> = Box::new(raw_index);

        let options = ReadOptions::default();
        let mut two = TwoLevelIterator::new(
            index_iter,
            single_block_function,
            arg,
            options,
        );

        two.seek_to_first();
        assert!(
            two.valid(),
            "iterator must be valid after seek_to_first when a single entry is present"
        );
        assert_eq!(two.key().to_string(), "k");
        assert_eq!(two.value().to_string(), "v");

        two.next();

        assert!(
            !two.valid(),
            "iterator should become invalid after advancing past the last entry in the only data block"
        );
    }

}
