// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/value.rs ]
crate::ix!();

impl LevelDBIteratorValue for KeyConvertingIterator {

    fn value(&self) -> Slice {
        let iter_ptr = self.iter_raw();

        trace!(
            "KeyConvertingIterator::value: delegating to iter={:?}",
            iter_ptr
        );

        unsafe {
            assert!(
                !iter_ptr.is_null(),
                "KeyConvertingIterator::value: underlying iterator pointer is null"
            );
            let v = (*iter_ptr).value();
            trace!(
                "KeyConvertingIterator::value: underlying value slice={:?}",
                v
            );
            v
        }
    }
}

#[cfg(test)]
mod key_converting_iterator_value_tests {
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
    fn value_forwards_underlying_value_slice() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let entries = vec![(
            InternalKey::new(&Slice::from("key"), 1, ValueType::TypeValue),
            "value-123".to_owned(),
        )];

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
            "iterator must be valid before calling value()"
        );

        let value_slice = kc.value();
        assert_eq!(
            value_slice.to_string(),
            "value-123",
            "value() must forward the underlying value slice unchanged"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator in value() tests must drop underlying iterator"
        );
    }
}
