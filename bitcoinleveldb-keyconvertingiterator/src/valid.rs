// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/valid.rs ]
crate::ix!();

impl LevelDBIteratorValid for KeyConvertingIterator {

    fn valid(&self) -> bool {
        let iter_ptr = self.iter_raw();

        trace!(
            "KeyConvertingIterator::valid: delegating to underlying iter={:?}",
            iter_ptr
        );

        unsafe {
            if iter_ptr.is_null() {
                trace!(
                    "KeyConvertingIterator::valid: underlying iterator pointer is null -> false"
                );
                false
            } else {
                let v = (*iter_ptr).valid();
                trace!(
                    "KeyConvertingIterator::valid: underlying valid={}",
                    v
                );
                v
            }
        }
    }
}

#[cfg(test)]
mod key_converting_iterator_validity_tests {
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
    fn valid_reflects_underlying_iterator_state() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let entries = vec![(
            InternalKey::new(&Slice::from("vk"), 1, ValueType::TypeValue),
            "vv".to_owned(),
        )];

        let raw = boxed_internal_iterator(RecordingInternalIterator::new(
            entries,
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        ));

        let mut kc = KeyConvertingIterator::new(raw);

        assert!(
            !kc.valid(),
            "before any seek, underlying iterator should be invalid and valid() must return false"
        );

        kc.seek_to_first();
        assert!(
            kc.valid(),
            "after seek_to_first, valid() must reflect underlying valid state"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator in valid() tests must drop underlying iterator"
        );
    }
}
