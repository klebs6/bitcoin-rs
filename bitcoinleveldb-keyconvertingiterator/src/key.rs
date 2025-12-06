// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/key.rs ]
crate::ix!();

impl Key for KeyConvertingIterator {

    fn key(&self) -> Slice {
        let iter_ptr = self.iter_raw();

        trace!(
            "KeyConvertingIterator::key: called; iter={:?}",
            iter_ptr
        );

        assert!(
            self.valid(),
            "KeyConvertingIterator::key requires the iterator to be valid"
        );

        unsafe {
            assert!(
                !iter_ptr.is_null(),
                "KeyConvertingIterator::key: underlying iterator pointer is null"
            );

            let internal_key = (*iter_ptr).key();
            trace!(
                "KeyConvertingIterator::key: underlying internal key slice={:?}",
                internal_key
            );

            let mut parsed = ParsedInternalKey::default();
            let ok = parse_internal_key(
                &internal_key,
                &mut parsed as *mut ParsedInternalKey,
            );

            if !ok {
                trace!(
                    "KeyConvertingIterator::key: ParseInternalKey failed; marking status as corruption"
                );

                let msg_slice = Slice::from("malformed internal key");
                let st = Status::corruption(&msg_slice, None);
                *self.status().borrow_mut() = st;

                let corrupted = Slice::from("corrupted key");
                trace!(
                    "KeyConvertingIterator::key: returning synthetic corrupted key slice={:?}",
                    corrupted
                );
                corrupted
            } else {
                let user_key_ref: &Slice = parsed.user_key();
                let data = *user_key_ref.data();
                let size = *user_key_ref.size();
                let user_key = Slice::from_ptr_len(data, size);

                trace!(
                    "KeyConvertingIterator::key: returning parsed user key slice (data={:?}, size={})",
                    data,
                    size
                );

                user_key
            }
        }
    }
}

#[cfg(test)]
mod key_conversion_tests {
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
    fn key_returns_user_key_for_well_formed_internal_key() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let user_key_str = "user-key-ok";

        let raw = boxed_internal_iterator(RecordingInternalIterator::single_entry(
            user_key_str,
            123,
            ValueType::TypeValue,
            "v",
            shared.clone(),
            drop_flag.clone(),
        ));

        let mut kc = KeyConvertingIterator::new(raw);

        kc.seek_to_first();
        assert!(
            kc.valid(),
            "iterator must be valid after seek_to_first"
        );

        let key_slice = kc.key();
        assert_eq!(
            key_slice.to_string(),
            user_key_str,
            "key() must return the logical user key"
        );

        assert!(
            kc.status().borrow().is_ok(),
            "status cache must remain OK after successful key parsing"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator in key() success test must drop underlying iterator"
        );
    }

    #[traced_test]
    fn key_sets_corruption_status_and_returns_synthetic_key_on_parse_failure() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let malformed_internal = b"bad".to_vec();

        let raw = boxed_internal_iterator(MalformedInternalKeyIterator::new(
            malformed_internal,
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        ));

        let kc = KeyConvertingIterator::new(raw);

        assert!(
            kc.valid(),
            "MalformedInternalKeyIterator is constructed as logically valid"
        );

        let key_slice = kc.key();
        assert_eq!(
            key_slice.to_string(),
            "corrupted key",
            "key() must synthesize a 'corrupted key' slice on parse failure"
        );

        {
            let st_ref = kc.status().borrow();
            assert!(
                st_ref.is_corruption(),
                "status cache must become Corruption after malformed key"
            );
        }

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator in malformed key test must drop underlying iterator"
        );
    }
}
