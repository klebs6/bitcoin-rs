// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/key_converting_iterator.rs ]
crate::ix!();

impl LevelDBIteratorInterface for KeyConvertingIterator {}

/**
  | A helper class that converts internal
  | format keys into user keys.
  |
  | This is a faithful port of LevelDB's
  | `KeyConvertingIterator`:
  |
  |   * It owns an underlying internal iterator.
  |   * It presents user‑keys at the API boundary.
  |   * It tracks a cached Status used to surface
  |     malformed internal keys as `Corruption`.
  |
  */
#[derive(Getters,MutGetters,Setters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct KeyConvertingIterator {

    /**
      | In the original C++ code this is the
      | `Iterator` base subobject, which owns
      | the cleanup chain and destructor behaviour.
      |
      | Here we keep an explicit `LevelDBIterator`
      | instance that manages the cleanup list;
      | this ensures that any cleanup handlers
      | registered against this iterator are
      | executed when we are dropped.
      |
      */
    base:   LevelDBIterator,

    /**
      | Cached status; if this is non‑OK we stop
      | consulting the underlying iterator and
      | return a copy of this value instead.
      |
      */
    status: RefCell<Status>,

    /**
      | Pointer to the underlying internal iterator.
      | 
      | Ownership:
      |   * `KeyConvertingIterator` takes ownership
      |     of `iter` and will destroy it in `Drop`
      |     using `Box::from_raw`.
      |
      | Requirements:
      |   * `iter` must be obtained from
      |     `Box::<dyn LevelDBIteratorInterface>::into_raw`.
      |
      | Safety:
      |   * All uses of `self.iter` are wrapped in
      |     `unsafe` blocks and guarded by explicit
      |     `is_null` checks where appropriate.
      |
      */
    iter:   *mut dyn LevelDBIteratorInterface,
}

#[cfg(test)]
mod key_converting_iterator_behavior {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn boxed_internal_iterator<T>(iter: T) -> *mut dyn LevelDBIteratorInterface
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
            InternalKey::new(&Slice::from("key0"), 1, ValueType::TypeValue),
            "value0".to_owned(),
        )];

        let raw = boxed_internal_iterator(RecordingInternalIterator::new(
            entries,
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        ));

        let mut kc = KeyConvertingIterator::new(raw);

        // Initially invalid because we have not positioned the iterator.
        assert!(
            !kc.valid(),
            "KeyConvertingIterator should report invalid before any seek"
        );

        kc.seek_to_first();
        assert!(
            kc.valid(),
            "KeyConvertingIterator should report valid after seek_to_first"
        );

        let state = shared.borrow();
        assert_eq!(
            *state.seek_to_first_calls(), 1,
            "underlying seek_to_first should have been called exactly once"
        );
        assert_eq!(
            *state.next_calls(), 0,
            "no next() calls expected in this scenario"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator must drop underlying iterator exactly once"
        );
    }

    #[traced_test]
    fn seek_encodes_user_key_as_internal_key_with_max_sequence_and_type_value() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let entries: Vec<(InternalKey, String)> = Vec::new();

        let raw = boxed_internal_iterator(RecordingInternalIterator::new(
            entries,
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        ));

        let mut kc = KeyConvertingIterator::new(raw);

        let user_key_str = "user-key-for-seek";
        let user_key     = Slice::from(user_key_str);

        kc.seek(&user_key);

        let state = shared.borrow();
        assert_eq!(
            state.seek_targets().len(),
            1,
            "exactly one underlying seek() call expected"
        );
        let encoded = state.seek_targets()[0].clone();

        // Decode the internal key we passed to the underlying iterator.
        let encoded_slice = Slice::from(encoded.as_slice());

        let mut parsed = ParsedInternalKey::default();
        let ok = parse_internal_key(&encoded_slice, &mut parsed as *mut ParsedInternalKey);
        assert!(
            ok,
            "encoded seek target should decode as a valid internal key"
        );

        let decoded_user = parsed.user_key().to_string();
        assert_eq!(
            decoded_user, user_key_str,
            "user key embedded in internal key must match original"
        );

        let max_sequence: SequenceNumber = ((0x1u64 << 56) - 1);
        assert_eq!(
            *parsed.sequence(),
            max_sequence,
            "seek must use kMaxSequenceNumber"
        );
        assert_eq!(
            *parsed.ty(),
            ValueType::TypeValue,
            "seek must use ValueType::TypeValue"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator must drop underlying iterator"
        );
    }

    #[traced_test]
    fn key_returns_user_key_and_keeps_status_ok_on_successful_parse() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let user_key_str = "hello-user-key";
        let user_key     = Slice::from(user_key_str);

        let raw = boxed_internal_iterator(RecordingInternalIterator::single_entry(
            user_key_str,
            42,
            ValueType::TypeValue,
            "v0",
            shared.clone(),
            drop_flag.clone(),
        ));

        let mut kc = KeyConvertingIterator::new(raw);

        kc.seek_to_first();
        assert!(kc.valid(), "iterator should be valid after seek_to_first");

        let result_key = kc.key();
        let result_str = result_key.to_string();
        assert_eq!(
            result_str, user_key_str,
            "KeyConvertingIterator::key must expose the user key portion of the internal key"
        );

        let st = kc.status();
        assert!(
            st.is_ok(),
            "status must remain OK after successfully parsing internal key"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator must drop underlying iterator"
        );
    }

    #[traced_test]
    fn key_on_malformed_internal_key_sets_corruption_and_returns_synthetic_key() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        // Any slice shorter than 8 bytes is an invalid internal key.
        let malformed_internal = b"short".to_vec();

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
        let key_str   = key_slice.to_string();
        assert_eq!(
            key_str, "corrupted key",
            "KeyConvertingIterator::key must return a synthetic 'corrupted key' slice on parse failure"
        );

        let st = kc.status();
        assert!(
            st.is_corruption(),
            "status must be set to Corruption after malformed internal key is observed"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator must drop underlying malformed iterator"
        );
    }

    #[traced_test]
    fn status_uses_underlying_when_cached_ok_and_prefers_cached_when_not_ok() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        // First scenario: cached OK -> delegate to underlying.
        let underlying_status = Status::not_found(&Slice::from("missing"), None);

        let raw = boxed_internal_iterator(RecordingInternalIterator::new(
            Vec::new(),
            Status::new_from_other_copy(&underlying_status),
            shared.clone(),
            drop_flag.clone(),
        ));

        let kc = KeyConvertingIterator::new(raw);

        let st = kc.status();
        assert!(
            st.is_not_found(),
            "when cached status is OK, status() must return the underlying iterator status"
        );

        {
            let state = shared.borrow();
            assert_eq!(
                *state.status_calls(), 1,
                "underlying status() should have been called exactly once so far"
            );
        }

        drop(kc);

        // Second scenario: cached non‑OK -> do NOT call underlying status().
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let raw = boxed_internal_iterator(MalformedInternalKeyIterator::new(
            b"short".to_vec(),
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        ));

        let kc = KeyConvertingIterator::new(raw);

        // Calling key() will force a parse failure and set cached status to Corruption.
        let _ = kc.key();

        {
            let state = shared.borrow();
            assert_eq!(
                *state.status_calls(), 0,
                "key() must not invoke underlying status()"
            );
        }

        let st_cached = kc.status();
        assert!(
            st_cached.is_corruption(),
            "status() must return cached corruption status after parse failure"
        );

        {
            let state = shared.borrow();
            assert_eq!(
                *state.status_calls(), 0,
                "when cached status is non‑OK, status() must NOT call the underlying iterator"
            );
        }

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator must drop underlying malformed iterator in status test as well"
        );
    }
}
