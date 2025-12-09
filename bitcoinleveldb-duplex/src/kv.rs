// ---------------- [ File: bitcoinleveldb-duplex/src/kv.rs ]
crate::ix!();

impl LevelDBIteratorKey for TwoLevelIterator {
   
    fn key(&self) -> Slice {
        trace!(
            "TwoLevelIterator::key: requested; data_iter_has_iter={}, data_iter_valid={}",
            self.data_iter().has_iterator(),
            self.data_iter().valid(),
        );

        assert!(
            self.valid(),
            "TwoLevelIterator::key requires iterator to be valid"
        );

        let k = self.data_iter().key();

        trace!(
            "TwoLevelIterator::key: delegated to data_iter; key_size={}",
            k.size()
        );

        k
    }
}

impl LevelDBIteratorValue for TwoLevelIterator {
   
    fn value(&self) -> Slice {
        trace!(
            "TwoLevelIterator::value: requested; data_iter_has_iter={}, data_iter_valid={}",
            self.data_iter().has_iterator(),
            self.data_iter().valid(),
        );

        assert!(
            self.valid(),
            "TwoLevelIterator::value requires iterator to be valid"
        );

        let v = self.data_iter().value();

        trace!(
            "TwoLevelIterator::value: delegated to data_iter; value_size={}",
            v.size()
        );

        v
    }
}

#[cfg(test)]
mod two_level_iterator_key_value_tests {
    use super::*;
    use core::ffi::c_void;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn unused_block_function(
        _arg: *mut c_void,
        _options: &ReadOptions,
        _handle: &Slice,
    ) -> Option<Box<dyn LevelDBIteratorInterface>> {
        None
    }

    fn make_two_level_with_single_counting_kv() -> (
        TwoLevelIterator,
        Arc<AtomicUsize>,
        Arc<AtomicUsize>,
    ) {
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
            b"key-1",
            b"value-1",
            key_calls.clone(),
            value_calls.clone(),
        );

        two.set_data_iterator(Some(Box::new(counting_iter)));

        (two, key_calls, value_calls)
    }

    #[traced_test]
    fn key_and_value_delegate_to_data_iterator_and_use_cached_key() {
        let (mut two, key_calls, value_calls) =
            make_two_level_with_single_counting_kv();

        assert!(
            two.valid(),
            "iterator must be valid after installing the counting data iterator"
        );
        assert_eq!(
            key_calls.load(Ordering::SeqCst),
            1,
            "underlying key() should be called once when the wrapper caches the key"
        );
        assert_eq!(
            value_calls.load(Ordering::SeqCst),
            0,
            "underlying value() must not be called before value() is requested"
        );

        let k = two.key().to_string();
        let v = two.value().to_string();

        assert_eq!(k, "key-1");
        assert_eq!(v, "value-1");

        assert_eq!(
            key_calls.load(Ordering::SeqCst),
            1,
            "TwoLevelIterator::key must use the cached key and avoid extra underlying calls"
        );
        assert_eq!(
            value_calls.load(Ordering::SeqCst),
            1,
            "TwoLevelIterator::value must call the underlying iterator exactly once"
        );
    }

    #[traced_test]
    fn key_and_value_panic_when_iterator_is_not_valid() {
        let (mut two, _key_calls, _value_calls) =
            make_two_level_with_single_counting_kv();

        two.data_iter_mut().next();
        assert!(
            !two.valid(),
            "iterator should be invalid after advancing past the single data entry"
        );

        let key_result = catch_unwind(AssertUnwindSafe(|| {
            let _ = two.key();
        }));
        assert!(
            key_result.is_err(),
            "TwoLevelIterator::key is expected to panic when called on an invalid iterator"
        );

        let value_result = catch_unwind(AssertUnwindSafe(|| {
            let _ = two.value();
        }));
        assert!(
            value_result.is_err(),
            "TwoLevelIterator::value is expected to panic when called on an invalid iterator"
        );
    }

}
