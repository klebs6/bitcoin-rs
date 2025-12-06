// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/status.rs ]
crate::ix!();

impl LevelDBIteratorStatus for KeyConvertingIterator {

    fn status(&self) -> crate::Status {
        let iter_ptr = self.iter_raw();

        trace!(
            "KeyConvertingIterator::status: evaluating cached vs underlying status; iter={:?}",
            iter_ptr
        );

        let cached = self.status().borrow();

        if cached.is_ok() {
            unsafe {
                assert!(
                    !iter_ptr.is_null(),
                    "KeyConvertingIterator::status: underlying iterator pointer is null"
                );
                let st = (*iter_ptr).status();
                trace!(
                    "KeyConvertingIterator::status: cached OK; returning underlying status_code={:?}",
                    st.code()
                );
                st
            }
        } else {
            trace!(
                "KeyConvertingIterator::status: cached non‑OK status_code={:?}; returning cached status",
                cached.code()
            );
            Status::new_from_other_copy(&*cached)
        }
    }
}

#[cfg(test)]
mod key_converting_iterator_status_tests {
    use super::*;
    use crate::{
        MalformedInternalKeyIterator,
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
    fn status_uses_underlying_status_when_cached_ok() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let underlying_status = Status::not_found(&Slice::from("missing"), None);

        let raw = boxed_internal_iterator(RecordingInternalIterator::new(
            Vec::new(),
            Status::new_from_other_copy(&underlying_status),
            shared.clone(),
            drop_flag.clone(),
        ));

        let kc = KeyConvertingIterator::new(raw);

        let st = <KeyConvertingIterator as LevelDBIteratorStatus>::status(&kc);
        assert!(
            st.is_not_found(),
            "when cached status is OK, LevelDBIteratorStatus::status must forward the underlying iterator status"
        );

        let state = shared.borrow();
        assert_eq!(
            *state.status_calls(),
            1,
            "underlying status() must be called exactly once when cached status is OK"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator in underlying-status test must drop iterator"
        );
    }

    #[traced_test]
    fn status_prefers_cached_status_when_non_ok() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let raw = boxed_internal_iterator(MalformedInternalKeyIterator::new(
            b"short".to_vec(),
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        ));

        let kc = KeyConvertingIterator::new(raw);

        let _ = kc.key();

        {
            let state = shared.borrow();
            assert_eq!(
                *state.status_calls(),
                0,
                "key() must not invoke underlying status()"
            );
        }

        let st_cached = <KeyConvertingIterator as LevelDBIteratorStatus>::status(&kc);
        assert!(
            st_cached.is_corruption(),
            "LevelDBIteratorStatus::status must return cached Corruption after parse failure"
        );

        {
            let state = shared.borrow();
            assert_eq!(
                *state.status_calls(),
                0,
                "when cached status is non‑OK, underlying iterator status() must not be called"
            );
        }

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator in cached-status test must drop iterator"
        );
    }
}

