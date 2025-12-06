// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/recording_internal_iterator.rs ]
crate::ix!();

pub fn user_key_from_internal_slice(internal: &Slice) -> String {
    trace!(
        "user_key_from_internal_slice: decoding internal key slice (len={})",
        *internal.size()
    );

    let mut parsed = ParsedInternalKey::default();
    let ok = parse_internal_key(internal, &mut parsed as *mut ParsedInternalKey);
    assert!(
        ok,
        "user_key_from_internal_slice: expected valid internal key encoding in RecordingInternalIterator"
    );
    parsed.user_key().to_string()
}

#[derive(Default,Getters,MutGetters,Setters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct RecordingInternalIteratorState {
    seek_targets:           Vec<Vec<u8>>,
    seek_to_first_calls:    usize,
    seek_to_last_calls:     usize,
    next_calls:             usize,
    prev_calls:             usize,
    status_calls:           usize,
}

pub struct RecordingInternalIterator {
    entries:    Vec<(InternalKey, String)>,
    index:      Option<usize>,
    status:     Status,
    shared:     Rc<RefCell<RecordingInternalIteratorState>>,
    drop_flag:  Rc<RefCell<bool>>,
}

impl RecordingInternalIterator {
    pub fn new(
        entries:   Vec<(InternalKey, String)>,
        status:    Status,
        shared:    Rc<RefCell<RecordingInternalIteratorState>>,
        drop_flag: Rc<RefCell<bool>>,
    ) -> Self {
        RecordingInternalIterator {
            entries,
            index: None,
            status,
            shared,
            drop_flag,
        }
    }

    pub fn single_entry(
        user_key:  &str,
        sequence:  SequenceNumber,
        ty:        ValueType,
        value:     &str,
        shared:    Rc<RefCell<RecordingInternalIteratorState>>,
        drop_flag: Rc<RefCell<bool>>,
    ) -> Self {
        let user_slice = Slice::from(user_key);
        let internal   = InternalKey::new(&user_slice, sequence, ty);
        RecordingInternalIterator::new(
            vec![(internal, value.to_owned())],
            Status::ok(),
            shared,
            drop_flag,
        )
    }
}

impl Drop for RecordingInternalIterator {
    fn drop(&mut self) {
        trace!("RecordingInternalIterator::drop");
        *self.drop_flag.borrow_mut() = true;
    }
}

impl LevelDBIteratorInterface for RecordingInternalIterator {}

impl Valid for RecordingInternalIterator {
    fn valid(&self) -> bool {
        let is_valid = self
            .index
            .map(|i| i < self.entries.len())
            .unwrap_or(false);
        trace!(
            "RecordingInternalIterator::valid -> {} (index={:?}, len={})",
            is_valid,
            self.index,
            self.entries.len()
        );
        is_valid
    }
}

impl SeekToFirst for RecordingInternalIterator {
    fn seek_to_first(&mut self) {
        trace!("RecordingInternalIterator::seek_to_first");
        {
            let mut state = self.shared.borrow_mut();
            state.seek_to_first_calls += 1;
        }
        if self.entries.is_empty() {
            self.index = None;
        } else {
            self.index = Some(0);
        }
    }
}

impl SeekToLast for RecordingInternalIterator {
    fn seek_to_last(&mut self) {
        trace!("RecordingInternalIterator::seek_to_last");
        {
            let mut state = self.shared.borrow_mut();
            state.seek_to_last_calls += 1;
        }
        if self.entries.is_empty() {
            self.index = None;
        } else {
            self.index = Some(self.entries.len() - 1);
        }
    }
}

impl Seek for RecordingInternalIterator {
    fn seek(&mut self, target: &Slice) {
        trace!(
            "RecordingInternalIterator::seek: target_len={}",
            *target.size()
        );
        let bytes = slice_as_bytes(target).to_vec();
        {
            let mut state = self.shared.borrow_mut();
            state.seek_targets.push(bytes);
        }
        if self.entries.is_empty() {
            self.index = None;
        } else {
            self.index = Some(0);
        }
    }
}

impl Next for RecordingInternalIterator {
    fn next(&mut self) {
        trace!(
            "RecordingInternalIterator::next: index_before={:?}",
            self.index
        );
        {
            let mut state = self.shared.borrow_mut();
            state.next_calls += 1;
        }
        if let Some(i) = self.index {
            if i + 1 < self.entries.len() {
                self.index = Some(i + 1);
            } else {
                self.index = None;
            }
        }
    }
}

impl Prev for RecordingInternalIterator {
    fn prev(&mut self) {
        trace!(
            "RecordingInternalIterator::prev: index_before={:?}",
            self.index
        );
        {
            let mut state = self.shared.borrow_mut();
            state.prev_calls += 1;
        }
        if let Some(i) = self.index {
            if i > 0 {
                self.index = Some(i - 1);
            } else {
                self.index = None;
            }
        }
    }
}

impl Key for RecordingInternalIterator {
    fn key(&self) -> Slice {
        trace!(
            "RecordingInternalIterator::key: index={:?}, len={}",
            self.index,
            self.entries.len()
        );
        let idx = self
            .index
            .expect("RecordingInternalIterator::key called while invalid");
        self.entries[idx].0.encode()
    }
}

impl Value for RecordingInternalIterator {
    fn value(&self) -> Slice {
        trace!(
            "RecordingInternalIterator::value: index={:?}, len={}",
            self.index,
            self.entries.len()
        );
        let idx = self
            .index
            .expect("RecordingInternalIterator::value called while invalid");
        let v = &self.entries[idx].1;
        Slice::from(v.as_str())
    }
}

impl LevelDBIteratorStatus for RecordingInternalIterator {
    fn status(&self) -> Status {
        trace!("RecordingInternalIterator::status");
        {
            let mut state = self.shared.borrow_mut();
            state.status_calls += 1;
        }
        Status::new_from_other_copy(&self.status)
    }
}

#[cfg(test)]
mod recording_internal_iterator_behavior_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[traced_test]
    fn initial_state_is_invalid_and_counters_zero() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let it = RecordingInternalIterator::new(
            Vec::new(),
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        );

        assert!(
            !it.valid(),
            "iterator over empty entries must start invalid"
        );

        let state = shared.borrow();
        assert_eq!(*state.seek_to_first_calls(), 0);
        assert_eq!(*state.seek_to_last_calls(), 0);
        assert_eq!(*state.next_calls(), 0);
        assert_eq!(*state.prev_calls(), 0);
        assert_eq!(*state.status_calls(), 0);

        drop(it);
        assert!(
            *drop_flag.borrow(),
            "dropping RecordingInternalIterator must flip drop_flag"
        );
    }

    #[traced_test]
    fn seek_to_first_and_seek_to_last_position_and_increment_counters() {
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

        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let mut it = RecordingInternalIterator::new(
            entries,
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        );

        it.seek_to_first();
        assert!(
            it.valid(),
            "after seek_to_first iterator must be valid"
        );

        let first_key_slice = it.key();
        let first_user_key  = user_key_from_internal_slice(&first_key_slice);
        assert_eq!(
            first_user_key,
            "a",
            "seek_to_first must position on first user key"
        );

        it.seek_to_last();
        assert!(
            it.valid(),
            "after seek_to_last iterator must still be valid"
        );

        let last_key_slice = it.key();
        let last_user_key  = user_key_from_internal_slice(&last_key_slice);
        assert_eq!(
            last_user_key,
            "b",
            "seek_to_last must position on last user key"
        );

        let state = shared.borrow();
        assert_eq!(*state.seek_to_first_calls(), 1);
        assert_eq!(*state.seek_to_last_calls(), 1);

        drop(it);
        assert!(
            *drop_flag.borrow(),
            "dropping RecordingInternalIterator in seek tests must flip drop_flag"
        );
    }

    #[traced_test]
    fn seek_records_target_bytes_and_resets_index() {
        let entries = vec![(
            InternalKey::new(&Slice::from("x"), 1, ValueType::TypeValue),
            "vx".to_owned(),
        )];

        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let mut it = RecordingInternalIterator::new(
            entries,
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        );

        let target = Slice::from("target-key");
        it.seek(&target);

        assert!(
            it.valid(),
            "after seek() iterator should be valid when entries exist"
        );

        let key_slice  = it.key();
        let user_key   = user_key_from_internal_slice(&key_slice);
        assert_eq!(
            user_key,
            "x",
            "seek() in the recording iterator always positions on first user key when non-empty"
        );

        let state = shared.borrow();
        assert_eq!(state.seek_targets().len(), 1);
        assert_eq!(
            String::from_utf8_lossy(&state.seek_targets()[0]),
            "target-key"
        );

        drop(it);
        assert!(
            *drop_flag.borrow(),
            "dropping RecordingInternalIterator in seek record test must flip drop_flag"
        );
    }

    #[traced_test]
    fn next_and_prev_walk_entries_and_update_counters() {
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

        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let mut it = RecordingInternalIterator::new(
            entries,
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        );

        it.seek_to_first();

        let first_key_slice = it.key();
        let first_user_key  = user_key_from_internal_slice(&first_key_slice);
        assert_eq!(
            first_user_key,
            "a",
            "after seek_to_first, iterator must expose first user key"
        );

        it.next();
        assert!(
            it.valid(),
            "iterator must remain valid after advancing to second entry"
        );

        let second_key_slice = it.key();
        let second_user_key  = user_key_from_internal_slice(&second_key_slice);
        assert_eq!(
            second_user_key,
            "b",
            "after one next(), iterator must expose second user key"
        );

        it.next();
        assert!(
            !it.valid(),
            "iterator must become invalid after moving past the last entry"
        );

        it.prev();
        assert!(
            !it.valid(),
            "prev() from invalid state leaves iterator invalid in recording implementation"
        );

        let state = shared.borrow();
        assert_eq!(*state.next_calls(), 2);
        assert_eq!(*state.prev_calls(), 1);

        drop(it);
        assert!(
            *drop_flag.borrow(),
            "dropping RecordingInternalIterator in next/prev tests must flip drop_flag"
        );
    }


    #[traced_test]
    fn status_returns_copy_of_configured_status_and_updates_counter() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let configured = Status::io_error(&Slice::from("io"), None);

        let it = RecordingInternalIterator::new(
            Vec::new(),
            Status::new_from_other_copy(&configured),
            shared.clone(),
            drop_flag.clone(),
        );

        let st = it.status();
        assert!(
            st.is_io_error(),
            "status() must return a copy of the configured status"
        );

        let state = shared.borrow();
        assert_eq!(*state.status_calls(), 1);

        drop(it);
        assert!(
            *drop_flag.borrow(),
            "dropping RecordingInternalIterator in status tests must flip drop_flag"
        );
    }
}
