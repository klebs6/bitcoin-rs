// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/seek.rs ]
crate::ix!();

impl Seek for KeyConvertingIterator {

    fn seek(&mut self, target: &Slice) {
        let iter_ptr = self.iter_raw_mut();

        trace!(
            "KeyConvertingIterator::seek: target={:?}, iter={:?}",
            target,
            iter_ptr
        );

        let max_sequence: SequenceNumber = ((0x1u64 << 56) - 1);
        let ikey = ParsedInternalKey::new(target, &max_sequence, ValueType::TypeValue);

        let mut encoded = String::new();
        unsafe {
            append_internal_key(&mut encoded as *mut String, &ikey);
        }

        let encoded_slice = Slice::from(&encoded);

        unsafe {
            assert!(
                !iter_ptr.is_null(),
                "KeyConvertingIterator::seek: underlying iterator pointer is null"
            );
            (*iter_ptr).seek(&encoded_slice);
        }
    }
}

#[cfg(test)]
mod key_converting_iterator_seek_tests {
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
    fn seek_encodes_target_as_internal_key_and_forwards_to_underlying_iterator() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let raw = boxed_internal_iterator(RecordingInternalIterator::new(
            Vec::new(),
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        ));

        let mut kc = KeyConvertingIterator::new(raw);

        let user_key_str = "seek-user-key";
        let user_key     = Slice::from(user_key_str);

        kc.seek(&user_key);

        let state = shared.borrow();
        assert_eq!(
            state.seek_targets().len(),
            1,
            "seek() must result in exactly one underlying seek() call"
        );

        let encoded = state.seek_targets()[0].clone();
        let encoded_slice = Slice::from(encoded.as_slice());

        let mut parsed = ParsedInternalKey::default();
        let ok = parse_internal_key(&encoded_slice, &mut parsed as *mut ParsedInternalKey);
        assert!(
            ok,
            "encoded seek target must decode as valid internal key"
        );

        assert_eq!(
            parsed.user_key().to_string(),
            user_key_str,
            "encoded internal key must contain original user key"
        );

        let max_sequence: SequenceNumber = ((0x1u64 << 56) - 1);
        assert_eq!(
            *parsed.sequence(),
            max_sequence,
            "seek must use kMaxSequenceNumber sequence"
        );
        assert_eq!(
            *parsed.ty(),
            ValueType::TypeValue,
            "seek must encode ValueType::TypeValue"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator in seek() tests must drop underlying iterator"
        );
    }
}
