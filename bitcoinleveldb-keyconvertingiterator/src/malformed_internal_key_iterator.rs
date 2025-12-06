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

#[cfg(test)]
mod malformed_internal_key_iterator_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[traced_test]
    fn malformed_iterator_reports_valid_and_exposes_configured_key() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let payload = b"bad-key".to_vec();
        let it = MalformedInternalKeyIterator::new(
            payload.clone(),
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        );

        assert!(
            it.valid(),
            "MalformedInternalKeyIterator is constructed as logically valid"
        );

        let key_slice = it.key();
        assert_eq!(
            key_slice.to_string(),
            String::from_utf8_lossy(&payload),
            "key() must expose the malformed internal key bytes as-is"
        );

        drop(it);
        assert!(
            *drop_flag.borrow(),
            "dropping MalformedInternalKeyIterator must flip drop_flag"
        );
    }

    #[traced_test]
    fn malformed_iterator_records_seek_and_next_prev_status_calls() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let mut it = MalformedInternalKeyIterator::new(
            b"bad".to_vec(),
            Status::io_error(&Slice::from("io"), None),
            shared.clone(),
            drop_flag.clone(),
        );

        let target = Slice::from("t");
        it.seek(&target);
        it.seek_to_first();
        it.seek_to_last();
        it.next();
        it.prev();

        let _ = <MalformedInternalKeyIterator as crate::LevelDBIteratorStatus>::status(&it);

        let state = shared.borrow();
        assert_eq!(state.seek_targets().len(), 1);
        assert_eq!(*state.seek_to_first_calls(), 1);
        assert_eq!(*state.seek_to_last_calls(), 1);
        assert_eq!(*state.next_calls(), 1);
        assert_eq!(*state.prev_calls(), 1);
        assert_eq!(*state.status_calls(), 1);

        drop(it);
        assert!(
            *drop_flag.borrow(),
            "dropping MalformedInternalKeyIterator in behavior test must flip drop_flag"
        );
    }
}
