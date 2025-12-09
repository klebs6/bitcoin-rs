// ---------------- [ File: bitcoinleveldb-iterator/src/mock_stub_iterator.rs ]
crate::ix!();

use std::sync::atomic::{AtomicUsize, Ordering};

pub struct MockStubIterator {
    entries:    Vec<(Vec<u8>, Vec<u8>)>,
    index:      Option<usize>,
    status:     crate::Status,
    next_calls: AtomicUsize,
    prev_calls: AtomicUsize,
}

impl LevelDBIteratorInterface for MockStubIterator {}

impl MockStubIterator {

    pub fn new_empty() -> Self {
        trace!("MockStubIterator::new_empty");
        MockStubIterator {
            entries:    Vec::new(),
            index:      None,
            status:     crate::Status::ok(),
            next_calls: AtomicUsize::new(0),
            prev_calls: AtomicUsize::new(0),
        }
    }

    pub fn new_with_entries(pairs: &[(&[u8], &[u8])]) -> Self {
        trace!(
            "MockStubIterator::new_with_entries: count={}",
            pairs.len()
        );
        let entries = pairs
            .iter()
            .map(|(k, v)| (k.to_vec(), v.to_vec()))
            .collect();
        MockStubIterator {
            entries,
            index:      None,
            status:     crate::Status::ok(),
            next_calls: AtomicUsize::new(0),
            prev_calls: AtomicUsize::new(0),
        }
    }

    pub fn current_index(&self) -> usize {
        self.index
            .expect("MockStubIterator::current_index: iterator not positioned")
    }

    pub fn next_call_count(&self) -> usize {
        self.next_calls.load(Ordering::SeqCst)
    }

    pub fn prev_call_count(&self) -> usize {
        self.prev_calls.load(Ordering::SeqCst)
    }
}

impl LevelDBIteratorValid for MockStubIterator {

    fn valid(&self) -> bool {
        self.index.is_some()
    }
}

impl LevelDBIteratorSeekToFirst for MockStubIterator {

    fn seek_to_first(&mut self) {
        trace!(
            "MockStubIterator::seek_to_first: entries_len={}",
            self.entries.len()
        );
        if self.entries.is_empty() {
            self.index = None;
        } else {
            self.index = Some(0);
        }
    }
}

impl LevelDBIteratorSeekToLast for MockStubIterator {

    fn seek_to_last(&mut self) {
        trace!(
            "MockStubIterator::seek_to_last: entries_len={}",
            self.entries.len()
        );
        if self.entries.is_empty() {
            self.index = None;
        } else {
            self.index = Some(self.entries.len() - 1);
        }
    }
}

impl LevelDBIteratorSeek for MockStubIterator {

    fn seek(&mut self, target: &Slice) {
        trace!(
            "MockStubIterator::seek: target=\"{}\", entries_len={}",
            target.to_string(),
            self.entries.len()
        );

        if self.entries.is_empty() {
            self.index = None;
            return;
        }

        let target_len = *target.size();
        let target_bytes = if target_len == 0 {
            Vec::new()
        } else {
            unsafe {
                let data_ptr = *target.data();
                std::slice::from_raw_parts(data_ptr, target_len).to_vec()
            }
        };

        let mut pos: Option<usize> = None;
        for (i, (k, _)) in self.entries.iter().enumerate() {
            if k.as_slice() >= target_bytes.as_slice() {
                pos = Some(i);
                break;
            }
        }

        trace!("MockStubIterator::seek: resolved index={:?}", pos);

        self.index = pos;
    }
}

impl LevelDBIteratorNext for MockStubIterator {

    fn next(&mut self) {
        let before = self.index;
        self.next_calls.fetch_add(1, Ordering::SeqCst);
        trace!(
            "MockStubIterator::next: before_index={:?}, entries_len={}",
            before,
            self.entries.len()
        );

        if let Some(i) = before {
            if i + 1 < self.entries.len() {
                self.index = Some(i + 1);
            } else {
                self.index = None;
            }
        }
    }
}

impl LevelDBIteratorPrev for MockStubIterator {

    fn prev(&mut self) {
        let before = self.index;
        self.prev_calls.fetch_add(1, Ordering::SeqCst);
        trace!(
            "MockStubIterator::prev: before_index={:?}, entries_len={}",
            before,
            self.entries.len()
        );

        if let Some(i) = before {
            if i > 0 {
                self.index = Some(i - 1);
            } else {
                self.index = None;
            }
        }
    }
}

impl LevelDBIteratorStatus for MockStubIterator {

    fn status(&self) -> crate::Status {
        trace!("MockStubIterator::status: returning stored status");
        crate::Status::new_from_other_copy(&self.status)
    }
}

impl LevelDBIteratorKey for MockStubIterator {

    fn key(&self) -> Slice {
        assert!(
            self.valid(),
            "MockStubIterator::key requires iterator to be valid"
        );
        let idx = self.current_index();
        let (ref key_bytes, _) = self.entries[idx];
        Slice::from(key_bytes.as_slice())
    }
}

impl LevelDBIteratorValue for MockStubIterator {

    fn value(&self) -> Slice {
        assert!(
            self.valid(),
            "MockStubIterator::value requires iterator to be valid"
        );
        let idx = self.current_index();
        let (_, ref value_bytes) = self.entries[idx];
        Slice::from(value_bytes.as_slice())
    }
}

#[cfg(test)]
mod stub_iterator_behavior_tests {
    use super::*;

    #[traced_test]
    fn new_empty_is_initially_invalid() {
        trace!("new_empty_is_initially_invalid: start");

        let it = MockStubIterator::new_empty();
        assert!(
            !it.valid(),
            "empty stub iterator must start in an invalid state"
        );
    }

    #[traced_test]
    fn seek_to_first_on_non_empty_positions_at_first_entry() {
        trace!("seek_to_first_on_non_empty_positions_at_first_entry: start");

        let mut it = MockStubIterator::new_with_entries(&[
            (b"a".as_ref(), b"1".as_ref()),
            (b"b".as_ref(), b"2".as_ref()),
        ]);

        it.seek_to_first();

        assert!(it.valid());
        assert_eq!(it.key().to_string(), "a");
        assert_eq!(it.value().to_string(), "1");
    }

    #[traced_test]
    fn seek_to_last_on_non_empty_positions_at_last_entry() {
        trace!("seek_to_last_on_non_empty_positions_at_last_entry: start");

        let mut it = MockStubIterator::new_with_entries(&[
            (b"a".as_ref(), b"1".as_ref()),
            (b"b".as_ref(), b"2".as_ref()),
        ]);

        it.seek_to_last();

        assert!(it.valid());
        assert_eq!(it.key().to_string(), "b");
        assert_eq!(it.value().to_string(), "2");
    }

    #[traced_test]
    fn next_and_prev_move_within_bounds() {
        trace!("next_and_prev_move_within_bounds: start");

        let mut it = MockStubIterator::new_with_entries(&[
            (b"a".as_ref(), b"1".as_ref()),
            (b"b".as_ref(), b"2".as_ref()),
            (b"c".as_ref(), b"3".as_ref()),
        ]);

        it.seek_to_first();
        assert_eq!(it.key().to_string(), "a");

        it.next();
        assert!(it.valid());
        assert_eq!(it.key().to_string(), "b");

        it.next();
        assert!(it.valid());
        assert_eq!(it.key().to_string(), "c");

        it.next();
        assert!(
            !it.valid(),
            "next past the last element must invalidate the stub iterator"
        );

        it.seek_to_last();
        it.prev();
        assert!(it.valid());
        assert_eq!(it.key().to_string(), "b");
    }

    #[traced_test]
    fn seek_to_middle_key_behaves_like_lower_bound() {
        trace!("seek_to_middle_key_behaves_like_lower_bound: start");

        let mut it = MockStubIterator::new_with_entries(&[
            (b"a".as_ref(), b"1".as_ref()),
            (b"c".as_ref(), b"3".as_ref()),
        ]);

        let target = Slice::from("b");
        it.seek(&target);

        assert!(it.valid());
        assert_eq!(it.key().to_string(), "c");
    }
}
