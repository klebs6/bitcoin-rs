// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/malformed_internal_key_iterator.rs ]
crate::ix!();

#[derive(Default,Getters,MutGetters,Setters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct MalformedInternalKeyIterator {
    internal_key_bytes: Vec<u8>,
    status:             Status,
    shared:             Rc<RefCell<RecordingInternalIteratorState>>,
    drop_flag:          Rc<RefCell<bool>>,
}

impl MalformedInternalKeyIterator {
    pub fn new(
        internal_key_bytes: Vec<u8>,
        status:             Status,
        shared:             Rc<RefCell<RecordingInternalIteratorState>>,
        drop_flag:          Rc<RefCell<bool>>,
    ) -> Self {
        MalformedInternalKeyIterator {
            internal_key_bytes,
            status,
            shared,
            drop_flag,
        }
    }
}

impl Drop for MalformedInternalKeyIterator {
    fn drop(&mut self) {
        trace!("MalformedInternalKeyIterator::drop");
        *self.drop_flag.borrow_mut() = true;
    }
}

impl LevelDBIteratorInterface for MalformedInternalKeyIterator {}

impl Valid for MalformedInternalKeyIterator {
    fn valid(&self) -> bool {
        trace!("MalformedInternalKeyIterator::valid -> true");
        true
    }
}

impl SeekToFirst for MalformedInternalKeyIterator {
    fn seek_to_first(&mut self) {
        trace!("MalformedInternalKeyIterator::seek_to_first (no-op)");
        let mut state = self.shared.borrow_mut();
        *state.seek_to_first_calls_mut() += 1;
    }
}

impl SeekToLast for MalformedInternalKeyIterator {
    fn seek_to_last(&mut self) {
        trace!("MalformedInternalKeyIterator::seek_to_last (no-op)");
        let mut state = self.shared.borrow_mut();
        *state.seek_to_last_calls_mut() += 1;
    }
}

impl Seek for MalformedInternalKeyIterator {
    fn seek(&mut self, target: &Slice) {
        trace!(
            "MalformedInternalKeyIterator::seek: target_len={}",
            *target.size()
        );
        let mut state = self.shared.borrow_mut();
        state.seek_targets_mut().push(slice_as_bytes(target).to_vec());
    }
}

impl Next for MalformedInternalKeyIterator {
    fn next(&mut self) {
        trace!("MalformedInternalKeyIterator::next (no-op)");
        let mut state = self.shared.borrow_mut();
        *state.next_calls_mut() += 1;
    }
}

impl Prev for MalformedInternalKeyIterator {
    fn prev(&mut self) {
        trace!("MalformedInternalKeyIterator::prev (no-op)");
        let mut state = self.shared.borrow_mut();
        *state.prev_calls_mut() += 1;
    }
}

impl Key for MalformedInternalKeyIterator {
    fn key(&self) -> Slice {
        trace!(
            "MalformedInternalKeyIterator::key: returning malformed key len={}",
            self.internal_key_bytes.len()
        );
        Slice::from(self.internal_key_bytes.as_slice())
    }
}

impl Value for MalformedInternalKeyIterator {
    fn value(&self) -> Slice {
        trace!("MalformedInternalKeyIterator::value: returning empty value");
        Slice::default()
    }
}

impl LevelDBIteratorStatus for MalformedInternalKeyIterator {
    fn status(&self) -> Status {
        trace!("MalformedInternalKeyIterator::status");
        {
            let mut state = self.shared.borrow_mut();
            *state.status_calls_mut() += 1;
        }
        Status::new_from_other_copy(&self.status)
    }
}
