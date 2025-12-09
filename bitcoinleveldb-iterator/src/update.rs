// ---------------- [ File: bitcoinleveldb-iterator/src/update.rs ]
crate::ix!();

impl LevelDBIterator {

    pub fn update(&mut self) {
        trace!(
            "LevelDBIterator::update: refreshing cached valid/key; has_iter={}",
            self.has_iterator()
        );

        // Limit the immutable borrow of self to this block so we can
        // safely mutate self afterwards.
        let (is_valid, maybe_key) = {
            let iter_ref = self
                .iter()
                .expect("LevelDBIterator::update: underlying iterator is missing");

            let is_valid = iter_ref.valid();

            trace!(
                "LevelDBIterator::update: underlying valid={}",
                is_valid
            );

            if is_valid {
                let k = iter_ref.key();
                (true, Some(k))
            } else {
                (false, None)
            }
        };

        self.set_valid_flag(is_valid);

        if let Some(k) = maybe_key {
            self.set_cached_key_from_slice(&k);

            let data = k.data();
            let size = k.size();

            trace!(
                "LevelDBIterator::update: cached key from underlying iterator (data={:?}, size={})",
                data,
                size
            );
        }
    }
}

#[cfg(test)]
mod iterator_wrapper_update_tests {
    use super::*;

    #[traced_test]
    fn update_marks_wrapper_invalid_when_underlying_iterator_is_invalid() {
        trace!("update_marks_wrapper_invalid_when_underlying_iterator_is_invalid: start");

        let stub = MockStubIterator::new_with_entries(&[(b"a".as_ref(), b"1".as_ref())]);
        let mut wrapper = LevelDBIterator::new(Some(Box::new(stub)));

        wrapper.seek_to_first();
        assert!(
            wrapper.valid(),
            "wrapper must be valid after seeking to first on non-empty iterator"
        );

        {
            let iter_mut = wrapper
                .iter_mut()
                .expect("iterator must be present for invalidation test");
            iter_mut.next();
        }

        assert!(
            wrapper.valid(),
            "before update, wrapper still has stale cached valid flag"
        );

        wrapper.update();

        assert!(
            !wrapper.valid(),
            "update must clear valid flag when underlying iterator is invalid"
        );
    }

    #[traced_test]
    fn update_caches_key_from_underlying_iterator() {
        trace!("update_caches_key_from_underlying_iterator: start");

        let stub = MockStubIterator::new_with_entries(&[
            (b"a".as_ref(), b"1".as_ref()),
            (b"b".as_ref(), b"2".as_ref()),
        ]);

        let mut wrapper = LevelDBIterator::new(Some(Box::new(stub)));

        {
            let iter_mut = wrapper
                .iter_mut()
                .expect("iterator must be present for key cache test");
            iter_mut.seek_to_last();
        }

        wrapper.update();

        assert!(
            wrapper.valid(),
            "wrapper must be valid after update when underlying iterator is positioned"
        );

        let key = wrapper.key().to_string();
        assert_eq!(
            key, "b",
            "update must cache the key from the current underlying iterator position"
        );
    }
}
