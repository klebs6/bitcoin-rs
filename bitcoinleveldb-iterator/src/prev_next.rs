// ---------------- [ File: bitcoinleveldb-iterator/src/prev_next.rs ]
crate::ix!();

impl LevelDBIteratorPrev for LevelDBIterator {

    fn prev(&mut self) {
        trace!(
            "LevelDBIterator::prev: moving backwards; has_iter={}, before_valid={}",
            self.has_iterator(),
            self.valid()
        );

        {
            let iter = self
                .iter_mut()
                .expect("LevelDBIterator::prev: underlying iterator is missing");
            iter.prev();
        }

        self.update();
    }
}

impl LevelDBIteratorNext for LevelDBIterator {

    fn next(&mut self) {
        trace!(
            "LevelDBIterator::next: advancing; has_iter={}, before_valid={}",
            self.has_iterator(),
            self.valid()
        );

        {
            let iter = self
                .iter_mut()
                .expect("LevelDBIterator::next: underlying iterator is missing");
            iter.next();
        }

        self.update();
    }
}

#[cfg(test)]
mod iterator_wrapper_prev_next_tests {
    use super::*;

    #[traced_test]
    fn next_advances_and_invalidates_at_end() {
        trace!("next_advances_and_invalidates_at_end: start");

        let stub = MockStubIterator::new_with_entries(&[
            (b"k1".as_ref(), b"v1".as_ref()),
            (b"k2".as_ref(), b"v2".as_ref()),
        ]);

        let mut wrapper = LevelDBIterator::new(Some(Box::new(stub)));

        wrapper.seek_to_first();
        assert!(wrapper.valid());
        assert_eq!(wrapper.key().to_string(), "k1");

        wrapper.next();
        assert!(wrapper.valid());
        assert_eq!(wrapper.key().to_string(), "k2");

        wrapper.next();
        assert!(
            !wrapper.valid(),
            "next past the last element must invalidate the wrapper"
        );
    }

    #[traced_test]
    fn prev_moves_backward_and_invalidates_before_first() {
        trace!("prev_moves_backward_and_invalidates_before_first: start");

        let stub = MockStubIterator::new_with_entries(&[
            (b"k1".as_ref(), b"v1".as_ref()),
            (b"k2".as_ref(), b"v2".as_ref()),
        ]);

        let mut wrapper = LevelDBIterator::new(Some(Box::new(stub)));

        wrapper.seek_to_last();
        assert!(wrapper.valid());
        assert_eq!(wrapper.key().to_string(), "k2");

        wrapper.prev();
        assert!(wrapper.valid());
        assert_eq!(wrapper.key().to_string(), "k1");

        wrapper.prev();
        assert!(
            !wrapper.valid(),
            "prev before the first element must invalidate the wrapper"
        );
    }
}
