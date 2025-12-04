// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/recording_internal_iterator.rs ]
crate::ix!();

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
