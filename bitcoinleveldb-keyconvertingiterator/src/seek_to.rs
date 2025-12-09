// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/seek_to.rs ]
crate::ix!();

impl LevelDBIteratorSeekToFirst for KeyConvertingIterator {

    fn seek_to_first(&mut self) {
        let iter_ptr = self.iter_raw_mut();

        trace!(
            "KeyConvertingIterator::seek_to_first: delegating to iter={:?}",
            iter_ptr
        );

        unsafe {
            assert!(
                !iter_ptr.is_null(),
                "KeyConvertingIterator::seek_to_first: underlying iterator pointer is null"
            );
            (*iter_ptr).seek_to_first();
        }
    }
}

impl LevelDBIteratorSeekToLast for KeyConvertingIterator {

    fn seek_to_last(&mut self) {
        let iter_ptr = self.iter_raw_mut();

        trace!(
            "KeyConvertingIterator::seek_to_last: delegating to iter={:?}",
            iter_ptr
        );

        unsafe {
            assert!(
                !iter_ptr.is_null(),
                "KeyConvertingIterator::seek_to_last: underlying iterator pointer is null"
            );
            (*iter_ptr).seek_to_last();
        }
    }
}

#[cfg(test)]
mod key_converting_iterator_seek_to_tests {
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
    fn seek_to_first_positions_on_first_entry() {
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
        assert!(
            kc.valid(),
            "seek_to_first must leave iterator valid when entries exist"
        );
        assert_eq!(
            kc.key().to_string(),
            "a",
            "seek_to_first must position on first logical key"
        );

        let state = shared.borrow();
        assert_eq!(
            *state.seek_to_first_calls(),
            1,
            "underlying seek_to_first must have been called exactly once"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator in seek_to_first test must drop underlying iterator"
        );
    }

    #[traced_test]
    fn seek_to_last_positions_on_last_entry() {
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

        kc.seek_to_last();
        assert!(
            kc.valid(),
            "seek_to_last must leave iterator valid when entries exist"
        );
        assert_eq!(
            kc.key().to_string(),
            "b",
            "seek_to_last must position on last logical key"
        );

        let state = shared.borrow();
        assert_eq!(
            *state.seek_to_last_calls(),
            1,
            "underlying seek_to_last must have been called exactly once"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator in seek_to_last test must drop underlying iterator"
        );
    }
}
