// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/key_converting_iterator.rs ]
crate::ix!();

/// Owned raw pointer to an internal iterator implementation.
///
/// Lifetime and ownership semantics:
///   * The pointer must be obtained from
///     `Box::<dyn InternalLevelDBIterator>::into_raw`.
///   * It is owned by whoever holds it and must be
///     eventually reclaimed with `Box::from_raw`.
///   * `KeyConvertingIterator` takes ownership of the
///     pointer passed to `new` and drops it in `Drop`.
pub type RawInternalLevelDBIterator = *mut dyn LevelDBIteratorInterface;

/// A helper class that converts internal
/// format keys into user keys.
/// 
/// This is a faithful port of LevelDB's
/// `KeyConvertingIterator`:
/// 
///   * It owns an underlying internal iterator.
///   * It presents user‑keys at the API boundary.
///   * It tracks a cached Status used to surface
///     malformed internal keys as `Corruption`.
/// 
#[derive(Getters,MutGetters,Setters)]
pub struct KeyConvertingIterator {

    /// In the original C++ code this is the
    /// `Iterator` base subobject, which owns
    /// the cleanup chain and destructor behaviour.
    /// 
    /// Here we keep an explicit `LevelDBIterator`
    /// instance that manages the cleanup list;
    /// this ensures that any cleanup handlers
    /// registered against this iterator are
    /// executed when we are dropped.
    /// 
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    base:   LevelDBIterator,

    /// Cached status; if this is non‑OK we stop
    /// consulting the underlying iterator and
    /// return a copy of this value instead.
    /// 
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    status: RefCell<Status>,

    /// Pointer to the underlying internal iterator.
    /// 
    /// Ownership:
    ///   * `KeyConvertingIterator` takes ownership
    ///     of `iter` and will destroy it in `Drop`
    ///     using `Box::from_raw`.
    /// 
    /// Requirements:
    ///   * `iter` must be obtained from
    ///     `Box::<dyn InternalLevelDBIterator>::into_raw`.
    /// 
    /// Safety:
    ///   * All uses of this pointer are wrapped in
    ///     `unsafe` blocks and guarded by explicit
    ///     `is_null` checks where appropriate.
    /// 
    #[getset(skip)]
    iter:   RawInternalLevelDBIterator,
}

impl KeyConvertingIterator {

    /// Internal constructor used within this crate to build a
    /// key‑converting iterator from an owned raw internal iterator
    /// pointer.
    ///
    /// This function is defined in the same module as the struct so
    /// that it can initialize private fields directly; the public
    /// constructor in `create.rs` delegates to this function.
    #[inline]
    pub(crate) fn new_internal(iter: RawInternalLevelDBIterator) -> Self {
        KeyConvertingIterator {
            base:   LevelDBIterator::new(),
            status: RefCell::new(Status::ok()),
            iter,
        }
    }

    /// Return the raw internal iterator pointer (by value).
    ///
    /// This does not transfer ownership; it is purely a
    /// convenience for internal delegation code.
    #[inline]
    pub(crate) fn iter_raw(&self) -> RawInternalLevelDBIterator {
        self.iter
    }

    /// Return the raw internal iterator pointer (by value),
    /// for call sites that hold `&mut self`.
    ///
    /// The pointer itself is `Copy` so this does not
    /// borrow the internal iterator mutably; the caller
    /// must still respect aliasing rules when using it.
    #[inline]
    pub(crate) fn iter_raw_mut(&mut self) -> RawInternalLevelDBIterator {
        self.iter
    }

}

impl LevelDBIteratorInterface for KeyConvertingIterator {}

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
            *state.seek_to_first_calls(),
            1,
            "underlying seek_to_first should have been called exactly once"
        );
        assert_eq!(
            *state.next_calls(),
            0,
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

        let value_slice = kc.value();
        let value_str   = value_slice.to_string();
        assert_eq!(
            value_str, "v0",
            "KeyConvertingIterator::value must forward the underlying value slice"
        );

        let st = <KeyConvertingIterator as LevelDBIteratorStatus>::status(&kc);
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

        let st = <KeyConvertingIterator as LevelDBIteratorStatus>::status(&kc);
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

        let st = <KeyConvertingIterator as LevelDBIteratorStatus>::status(&kc);
        assert!(
            st.is_not_found(),
            "when cached status is OK, status() must return the underlying iterator status"
        );

        {
            let state = shared.borrow();
            assert_eq!(
                *state.status_calls(),
                1,
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
                *state.status_calls(),
                0,
                "key() must not invoke underlying status()"
            );
        }

        let st_cached = <KeyConvertingIterator as LevelDBIteratorStatus>::status(&kc);
        assert!(
            st_cached.is_corruption(),
            "status() must return cached corruption status after parse failure"
        );

        {
            let state = shared.borrow();
            assert_eq!(
                *state.status_calls(),
                0,
                "when cached status is non‑OK, status() must NOT call the underlying iterator"
            );
        }

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator must drop underlying malformed iterator in status test as well"
        );
    }

    #[traced_test]
    fn seek_to_last_positions_on_last_entry_and_increments_counter() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let entries = vec![
            (
                InternalKey::new(&Slice::from("k0"), 1, ValueType::TypeValue),
                "v0".to_owned(),
            ),
            (
                InternalKey::new(&Slice::from("k1"), 2, ValueType::TypeValue),
                "v1".to_owned(),
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
            "iterator should be valid after seek_to_last on non-empty input"
        );

        let key = kc.key().to_string();
        assert_eq!(
            key, "k1",
            "seek_to_last should position iterator on last entry"
        );

        let state = shared.borrow();
        assert_eq!(
            *state.seek_to_last_calls(),
            1,
            "underlying seek_to_last should have been called exactly once"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator must drop underlying iterator in seek_to_last test"
        );
    }

    #[traced_test]
    fn next_and_prev_calls_propagate_to_underlying_counters() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let entries = vec![
            (
                InternalKey::new(&Slice::from("k0"), 1, ValueType::TypeValue),
                "v0".to_owned(),
            ),
            (
                InternalKey::new(&Slice::from("k1"), 2, ValueType::TypeValue),
                "v1".to_owned(),
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
            "two calls to next() should be recorded on underlying iterator"
        );
        assert_eq!(
            *state.prev_calls(),
            1,
            "one call to prev() should be recorded on underlying iterator"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator must drop underlying iterator in next/prev test"
        );
    }

    #[traced_test]
    fn internal_leveldb_iterator_blanket_impl_covers_recording_iterator() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let entries = vec![(
            InternalKey::new(&Slice::from("key0"), 1, ValueType::TypeValue),
            "value0".to_owned(),
        )];

        let iter = RecordingInternalIterator::new(
            entries,
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        );

        let _boxed: Box<dyn LevelDBIteratorInterface> = Box::new(iter);

        assert!(
            !*drop_flag.borrow(),
            "iterator should not have been dropped yet"
        );
    }

    #[traced_test]
    fn raw_internal_iterator_pointer_roundtrip_via_box() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let entries = vec![(
            InternalKey::new(&Slice::from("key-rt"), 7, ValueType::TypeValue),
            "v-rt".to_owned(),
        )];

        let raw: RawInternalLevelDBIterator = boxed_internal_iterator(
            RecordingInternalIterator::new(
                entries,
                Status::ok(),
                shared.clone(),
                drop_flag.clone(),
            )
        );

        unsafe {
            assert!(
                !raw.is_null(),
                "boxed_internal_iterator must never return a null pointer"
            );

            let boxed: Box<dyn LevelDBIteratorInterface> = Box::from_raw(raw);

            assert!(
                !*drop_flag.borrow(),
                "taking Box::from_raw must not drop the iterator yet"
            );

            drop(boxed);
        }

        assert!(
            *drop_flag.borrow(),
            "dropping the reconstructed Box must drop the underlying iterator"
        );
    }


    #[traced_test]
    fn key_converting_iterator_holds_base_status_and_iter_pointer() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let raw = boxed_internal_iterator(RecordingInternalIterator::new(
            Vec::new(),
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        ));

        let kc = KeyConvertingIterator::new(raw);

        let inner_rc = kc.base().inner();
        assert_eq!(
            Rc::strong_count(&inner_rc),
            2,
            "KeyConvertingIterator::base should retain exactly one inner Rc clone"
        );

        {
            let st_ref = kc.status().borrow();
            assert!(
                st_ref.is_ok(),
                "newly constructed KeyConvertingIterator must start with OK status cache"
            );
        }

        assert!(
            kc.iter_raw() == raw,
            "iter_raw must round‑trip the underlying pointer"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator must drop underlying iterator through Drop impl"
        );
    }

    #[traced_test]
    fn key_converting_iterator_can_wrap_null_pointer_for_testing() {
        let raw_null: RawInternalLevelDBIterator = unsafe {
            core::mem::transmute::<(usize, usize), RawInternalLevelDBIterator>((0usize, 0usize))
        };

        let kc = KeyConvertingIterator::new(raw_null);
        assert!(
            !kc.valid(),
            "valid() must be false if the underlying iterator pointer is null"
        );
    }

    #[traced_test]
    fn new_internal_initializes_status_and_stores_pointer() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let raw = boxed_internal_iterator(RecordingInternalIterator::new(
            Vec::new(),
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        ));

        let kc = KeyConvertingIterator::new_internal(raw);

        assert!(
            kc.status().borrow().is_ok(),
            "new_internal must initialize cached status to OK"
        );

        assert!(
            kc.iter_raw() == raw,
            "iter_raw must return the exact raw iterator pointer passed to new_internal"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator constructed via new_internal must drop the underlying iterator"
        );
    }

    #[traced_test]
    fn iter_raw_mut_allows_mutating_underlying_iterator_through_key_converting_iterator() {
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

        let mut kc = KeyConvertingIterator::new_internal(raw);

        assert!(
            !kc.valid(),
            "before any positioning, iterator must report invalid"
        );

        kc.seek_to_first();
        assert!(
            kc.valid(),
            "after seek_to_first through KeyConvertingIterator, iterator must become valid"
        );

        let key_str = kc.key().to_string();
        assert_eq!(
            key_str, "key0",
            "key obtained through KeyConvertingIterator must match underlying iterator key"
        );

        drop(kc);
        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator after using iter_raw_mut delegation must drop underlying iterator"
        );
    }

}
