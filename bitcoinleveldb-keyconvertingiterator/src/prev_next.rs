// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/prev_next.rs ]
crate::ix!();

impl LevelDBIteratorNext for KeyConvertingIterator {

    fn next(&mut self) {
        let iter_ptr = self.iter_raw_mut();

        trace!(
            "KeyConvertingIterator::next: delegating to iter={:?}",
            iter_ptr
        );

        unsafe {
            assert!(
                !iter_ptr.is_null(),
                "KeyConvertingIterator::next: underlying iterator pointer is null"
            );
            (*iter_ptr).next();
        }
    }
}

impl LevelDBIteratorPrev for KeyConvertingIterator {

    fn prev(&mut self) {
        let iter_ptr = self.iter_raw_mut();

        trace!(
            "KeyConvertingIterator::prev: delegating to iter={:?}",
            iter_ptr
        );

        unsafe {
            assert!(
                !iter_ptr.is_null(),
                "KeyConvertingIterator::prev: underlying iterator pointer is null"
            );
            (*iter_ptr).prev();
        }
    }
}

#[cfg(test)]
mod key_converting_iterator_prev_next_tests {
    use super::*;
    use crate::{
        RecordingInternalIterator,
        RecordingInternalIteratorState,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    fn boxed_internal_iterator<T>(iter: T) -> crate::RawInternalLevelDBIterator
    where
        T: LevelDBIteratorInterface + 'static,
    {
        let boxed: Box<dyn LevelDBIteratorInterface> = Box::new(iter);
        Box::into_raw(boxed)
    }

    #[traced_test]
    fn next_and_prev_are_forwarded_to_underlying_iterator() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let entries = vec![
            (
                InternalKey::new(&Slice::from("a"), 1, ValueType::TypeValue),
                "va".to_owned(),
            ),
            (
                InternalKey::new(&Slice::from("b"), 2, ValueType::TypeValue),
                "vb".to_owned(),
            ),
        ];

        let raw = boxed_internal_iterator(RecordingInternalIterator::new(
            entries,
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        ));

        let mut kc = KeyConvertingIterator::new(raw);

        kc.seek_to_first();
        kc.next();
        kc.next();
        kc.prev();

        let state = shared.borrow();
        assert_eq!(
            *state.next_calls(),
            2,
            "two calls to next() must be forwarded to underlying iterator"
        );
        assert_eq!(
            *state.prev_calls(),
            1,
            "one call to prev() must be forwarded to underlying iterator"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator in next/prev tests must drop underlying iterator"
        );
    }
}
