// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/create.rs ]
crate::ix!();

impl KeyConvertingIterator {

    /**
      | Construct a key‑converting iterator that
      | takes ownership of an underlying internal
      | iterator.
      |
      | Safety:
      |   * `iter` must come from
      |     `Box::<dyn LevelDBIteratorInterface>::into_raw`.
      |   * It must not be freed elsewhere; this
      |     type is responsible for calling
      |     `Box::from_raw` in `Drop`.
      |
      */
    pub fn new(iter: crate::RawInternalLevelDBIterator) -> Self {
        trace!(
            "KeyConvertingIterator::new: constructing with underlying iter={:?}",
            iter
        );

        KeyConvertingIterator::new_internal(iter)
    }
}

#[cfg(test)]
mod key_converting_iterator_construction_tests {
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
    fn new_sets_status_ok_and_stores_pointer() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let raw = boxed_internal_iterator(RecordingInternalIterator::new(
            Vec::new(),
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        ));

        let kc = KeyConvertingIterator::new(raw);

        assert!(
            kc.status().borrow().is_ok(),
            "KeyConvertingIterator::new must initialize cached status to OK"
        );

        assert!(
            kc.iter_raw() == raw,
            "KeyConvertingIterator::new must store the exact raw iterator pointer"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator constructed via new() must drop underlying iterator"
        );
    }
}
