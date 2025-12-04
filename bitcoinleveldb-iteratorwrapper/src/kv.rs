// ---------------- [ File: bitcoinleveldb-iteratorwrapper/src/kv.rs ]
crate::ix!();

impl LevelDBIteratorWrapper {

    pub fn key(&self) -> Slice {
        trace!(
            "LevelDBIteratorWrapper::key: requested; cached_valid={}, has_iter={}",
            self.valid(),
            self.has_iterator()
        );

        assert!(
            self.valid(),
            "LevelDBIteratorWrapper::key requires the iterator to be valid"
        );

        let cached = self.cached_key();
        let data   = cached.data();
        let size   = cached.size();
        let result = Slice::from_ptr_len(*data, *size);

        trace!(
            "LevelDBIteratorWrapper::key: returning cached key slice (data={:?}, size={})",
            data,
            size
        );

        result
    }

    pub fn value(&self) -> Slice {
        trace!(
            "LevelDBIteratorWrapper::value: requested; cached_valid={}, has_iter={}",
            self.valid(),
            self.has_iterator()
        );

        assert!(
            self.valid(),
            "LevelDBIteratorWrapper::value requires the iterator to be valid"
        );

        let iter = self
            .iter()
            .expect("LevelDBIteratorWrapper::value: underlying iterator is missing");

        let value = iter.value();

        trace!(
            "LevelDBIteratorWrapper::value: delegated to underlying iterator; value={:?}",
            value
        );

        value
    }
}

#[cfg(test)]
mod iterator_wrapper_kv_tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[traced_test]
    fn key_uses_cached_slice_without_requerying_underlying_iterator() {
        trace!("key_uses_cached_slice_without_requerying_underlying_iterator: start");

        let key_calls   = Arc::new(AtomicUsize::new(0));
        let value_calls = Arc::new(AtomicUsize::new(0));

        let iter = MockCountingKVIterator::new_single(
            b"k1",
            b"v1",
            key_calls.clone(),
            value_calls.clone(),
        );

        // new(Some(iter)) will call reset_iterator(), which for a non‑empty
        // iterator invokes update() once and thus calls key() exactly once.
        let wrapper = LevelDBIteratorWrapper::new(Some(Box::new(iter)));

        assert_eq!(
            key_calls.load(Ordering::SeqCst),
            1,
            "construction/reset_iterator must call key() exactly once to populate the cache"
        );
        assert_eq!(
            value_calls.load(Ordering::SeqCst),
            0,
            "no value() calls are expected during initial caching"
        );

        let k1 = wrapper.key().to_string();
        assert_eq!(k1, "k1");

        assert_eq!(
            key_calls.load(Ordering::SeqCst),
            1,
            "wrapper.key() must use cached key and not re-invoke underlying key()"
        );
        assert_eq!(
            value_calls.load(Ordering::SeqCst),
            0,
            "value() must not be called when only key() is queried"
        );
    }

    #[traced_test]
    fn value_delegates_to_underlying_iterator_every_call() {
        trace!("value_delegates_to_underlying_iterator_every_call: start");

        let key_calls   = Arc::new(AtomicUsize::new(0));
        let value_calls = Arc::new(AtomicUsize::new(0));

        let iter = MockCountingKVIterator::new_single(
            b"k1",
            b"v1",
            key_calls.clone(),
            value_calls.clone(),
        );

        // As above, construction does one key() call via reset_iterator()->update().
        let mut wrapper = LevelDBIteratorWrapper::new(Some(Box::new(iter)));

        assert_eq!(
            key_calls.load(Ordering::SeqCst),
            1,
            "initial construction must fetch key exactly once"
        );

        // Each value() call should delegate to the underlying iterator.
        let v1 = wrapper.value().to_string();
        let v2 = wrapper.value().to_string();

        assert_eq!(v1, "v1");
        assert_eq!(v2, "v1");
        assert_eq!(
            value_calls.load(Ordering::SeqCst),
            2,
            "each wrapper.value() call must delegate to the underlying iterator"
        );
        assert_eq!(
            key_calls.load(Ordering::SeqCst),
            1,
            "key must only be fetched once during initial update; value() must not re-fetch key()"
        );
    }
}
