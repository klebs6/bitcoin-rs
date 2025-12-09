// ---------------- [ File: bitcoinleveldb-duplex/src/valid.rs ]
crate::ix!();

impl LevelDBIteratorValid for TwoLevelIterator {
    
    fn valid(&self) -> bool {
        let is_valid = self.data_iter().valid();
        trace!(
            "TwoLevelIterator::valid: data_iter_has_iter={}, data_iter_valid={}",
            self.data_iter().has_iterator(),
            is_valid,
        );
        is_valid
    }
}

#[cfg(test)]
mod two_level_iterator_valid_tests {
    use super::*;
    use core::ffi::c_void;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn unused_block_function(
        _arg: *mut c_void,
        _options: &ReadOptions,
        _handle: &Slice,
    ) -> Option<Box<dyn LevelDBIteratorInterface>> {
        None
    }

    #[traced_test]
    fn two_level_iterator_is_invalid_when_no_data_iterator_is_installed() {
        let index_iter: Box<dyn LevelDBIteratorInterface> =
            Box::new(MockStubIterator::new_empty());
        let options = ReadOptions::default();

        let two = TwoLevelIterator::new(
            index_iter,
            unused_block_function,
            core::ptr::null_mut(),
            options,
        );

        assert!(
            !two.valid(),
            "TwoLevelIterator should report invalid when there is no data iterator"
        );
    }

    #[traced_test]
    fn two_level_iterator_valid_state_tracks_data_iterator_validity() {
        let index_iter: Box<dyn LevelDBIteratorInterface> =
            Box::new(MockStubIterator::new_empty());
        let options = ReadOptions::default();

        let mut two = TwoLevelIterator::new(
            index_iter,
            unused_block_function,
            core::ptr::null_mut(),
            options,
        );

        let key_calls = Arc::new(AtomicUsize::new(0));
        let value_calls = Arc::new(AtomicUsize::new(0));
        let counting_iter = MockCountingKVIterator::new_single(
            b"k",
            b"v",
            key_calls.clone(),
            value_calls.clone(),
        );

        two.set_data_iterator(Some(Box::new(counting_iter)));

        assert!(
            two.valid(),
            "TwoLevelIterator should become valid after installing a valid data iterator"
        );
        assert_eq!(
            key_calls.load(Ordering::SeqCst),
            1,
            "underlying key() should have been called once while caching the key"
        );

        two.data_iter_mut().next();

        assert!(
            !two.valid(),
            "TwoLevelIterator should become invalid after the data iterator moves past its only entry"
        );
    }
}
