// ---------------- [ File: bitcoinleveldb-iteratorwrapper/src/seek.rs ]
crate::ix!();

impl LevelDBIteratorWrapper {

    pub fn seek(&mut self, k: &Slice) {
        trace!(
            "LevelDBIteratorWrapper::seek: seeking to target={:?}, has_iter={}",
            k,
            self.has_iterator()
        );

        {
            let iter = self
                .iter_mut()
                .expect("LevelDBIteratorWrapper::seek: underlying iterator is missing");
            iter.seek(k);
        }

        self.update();
    }

    pub fn seek_to_first(&mut self) {
        trace!(
            "LevelDBIteratorWrapper::seek_to_first: has_iter={}",
            self.has_iterator()
        );

        {
            let iter = self
                .iter_mut()
                .expect("LevelDBIteratorWrapper::seek_to_first: underlying iterator is missing");
            iter.seek_to_first();
        }

        self.update();
    }

    pub fn seek_to_last(&mut self) {
        trace!(
            "LevelDBIteratorWrapper::seek_to_last: has_iter={}",
            self.has_iterator()
        );

        {
            let iter = self
                .iter_mut()
                .expect("LevelDBIteratorWrapper::seek_to_last: underlying iterator is missing");
            iter.seek_to_last();
        }

        self.update();
    }
}

#[cfg(test)]
mod iterator_wrapper_seek_tests {
    use super::*;

    #[traced_test]
    fn seek_positions_at_first_key_greater_or_equal_to_target() {
        trace!("seek_positions_at_first_key_greater_or_equal_to_target: start");

        let stub = MockStubIterator::new_with_entries(&[
            (b"a".as_ref(), b"1".as_ref()),
            (b"c".as_ref(), b"3".as_ref()),
        ]);

        let mut wrapper = LevelDBIteratorWrapper::new(Some(Box::new(stub)));

        let target = Slice::from("b");
        wrapper.seek(&target);

        assert!(
            wrapper.valid(),
            "seek to a middle key must result in a valid position when a larger key exists"
        );
        assert_eq!(
            wrapper.key().to_string(),
            "c",
            "seek(target='b') should land on 'c'"
        );
    }

    #[traced_test]
    fn seek_on_empty_iterator_marks_invalid() {
        trace!("seek_on_empty_iterator_marks_invalid: start");

        let stub = MockStubIterator::new_empty();
        let mut wrapper = LevelDBIteratorWrapper::new(Some(Box::new(stub)));

        let target = Slice::from("anything");
        wrapper.seek(&target);

        assert!(
            !wrapper.valid(),
            "seek on an empty iterator must leave the wrapper invalid"
        );
    }

    #[traced_test]
    fn seek_to_first_and_last_on_non_empty_iterator() {
        trace!("seek_to_first_and_last_on_non_empty_iterator: start");

        let stub = MockStubIterator::new_with_entries(&[
            (b"a".as_ref(), b"1".as_ref()),
            (b"b".as_ref(), b"2".as_ref()),
            (b"c".as_ref(), b"3".as_ref()),
        ]);

        let mut wrapper = LevelDBIteratorWrapper::new(Some(Box::new(stub)));

        wrapper.seek_to_first();
        assert!(wrapper.valid());
        assert_eq!(wrapper.key().to_string(), "a");

        wrapper.seek_to_last();
        assert!(wrapper.valid());
        assert_eq!(wrapper.key().to_string(), "c");
    }
}
