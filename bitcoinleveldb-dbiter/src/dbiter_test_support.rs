// ---------------- [ File: bitcoinleveldb-dbiter/src/dbiter_test_support.rs ]
#![cfg(test)]

crate::ix!();

use core::borrow::Borrow;
use std::borrow::Cow;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

static TEST_BYTEWISE_COMPARATOR: TestBytewiseComparator = TestBytewiseComparator {};

#[derive(Clone, Default)]
pub(crate) struct TestBytewiseComparator {}

impl Named for TestBytewiseComparator {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("dbiter-test-bytewise-comparator")
    }
}

impl Compare for TestBytewiseComparator {
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        bytewise_compare(a.as_bytes(), b.as_bytes())
    }
}

impl FindShortSuccessor for TestBytewiseComparator {
    fn find_short_successor(&self, _k: &mut Vec<u8>) {}
}

impl FindShortestSeparator for TestBytewiseComparator {
    fn find_shortest_separator(&self, _start: &mut Vec<u8>, _limit: &[u8]) {}
}

impl SliceComparator for TestBytewiseComparator {
    fn bytewise_comparator(&self) -> *const (dyn SliceComparator + 'static) {
        &TEST_BYTEWISE_COMPARATOR as *const TestBytewiseComparator as *const dyn SliceComparator
    }
}

pub(crate) struct ReadSampleCountingDb {
    read_sample_calls: Arc<AtomicUsize>,
    last_key_len:      Arc<AtomicUsize>,
}

impl ReadSampleCountingDb {
    pub(crate) fn new(read_sample_calls: Arc<AtomicUsize>, last_key_len: Arc<AtomicUsize>) -> Self {
        Self {
            read_sample_calls,
            last_key_len,
        }
    }

    pub(crate) fn read_sample_calls(&self) -> usize {
        self.read_sample_calls.load(Ordering::SeqCst)
    }

    pub(crate) fn last_key_len(&self) -> usize {
        self.last_key_len.load(Ordering::SeqCst)
    }
}

impl DBIterReadSample for ReadSampleCountingDb {
    fn record_read_sample(&mut self, key: Slice) {
        trace!(
            "ReadSampleCountingDb::record_read_sample: key_len={}",
            key.as_bytes().len()
        );
        self.read_sample_calls.fetch_add(1, Ordering::SeqCst);
        self.last_key_len
            .store(key.as_bytes().len(), Ordering::SeqCst);
    }
}

pub(crate) fn make_read_sample_counting_db(
) -> (
    Rc<RefCell<dyn DBIterReadSample>>,
    Arc<AtomicUsize>,
    Arc<AtomicUsize>,
) {
    let calls = Arc::new(AtomicUsize::new(0));
    let last_len = Arc::new(AtomicUsize::new(0));

    let db: Rc<RefCell<dyn DBIterReadSample>> =
        Rc::new(RefCell::new(ReadSampleCountingDb::new(
            calls.clone(),
            last_len.clone(),
        )));

    (db, calls, last_len)
}

pub(crate) fn make_user_comparator() -> Box<dyn SliceComparator> {
    Box::new(TestBytewiseComparator::default())
}

pub(crate) fn encode_internal_key_bytes(
    user_key: &[u8],
    seq:      SequenceNumber,
    ty:       ValueType,
) -> Vec<u8> {
    let user = Slice::from_bytes(user_key);
    let pik = ParsedInternalKey::new(&user, &seq, ty);

    let mut dst = String::new();
    append_internal_key(&mut dst as *mut String, &pik);

    dst.as_bytes().to_vec()
}

pub(crate) fn make_entry(
    user_key: &[u8],
    seq:      SequenceNumber,
    ty:       ValueType,
    value:    &[u8],
) -> (Vec<u8>, Vec<u8>) {
    (encode_internal_key_bytes(user_key, seq, ty), value.to_vec())
}

pub(crate) fn make_corrupt_entry(key_bytes: &[u8], value: &[u8]) -> (Vec<u8>, Vec<u8>) {
    (key_bytes.to_vec(), value.to_vec())
}

pub(crate) fn make_internal_stub_iterator(
    entries: Vec<(Vec<u8>, Vec<u8>)>,
) -> Rc<RefCell<LevelDBIterator>> {
    let refs: Vec<(&[u8], &[u8])> = entries
        .iter()
        .map(|(k, v)| (k.as_slice(), v.as_slice()))
        .collect();

    let stub = MockStubIterator::new_with_entries(&refs);

    Rc::new(RefCell::new(LevelDBIterator::new(Some(Box::new(stub)))))
}

pub(crate) fn build_dbiter_direct(
    sequence: SequenceNumber,
    seed:     u32,
    entries:  Vec<(Vec<u8>, Vec<u8>)>,
) -> (DBIter, Arc<AtomicUsize>, Arc<AtomicUsize>) {
    let (db, calls, last_len) = make_read_sample_counting_db();
    let cmp = make_user_comparator();
    let internal_iter = make_internal_stub_iterator(entries);

    let dbiter = DBIter::new(db, cmp, internal_iter, sequence, seed);

    (dbiter, calls, last_len)
}

pub(crate) fn build_leveldb_iterator_via_new_db_iterator(
    sequence: SequenceNumber,
    seed:     u32,
    entries:  Vec<(Vec<u8>, Vec<u8>)>,
) -> (Rc<RefCell<LevelDBIterator>>, Arc<AtomicUsize>, Arc<AtomicUsize>) {
    let (db, calls, last_len) = make_read_sample_counting_db();
    let cmp = make_user_comparator();
    let internal_iter = make_internal_stub_iterator(entries);

    let it = new_db_iterator(db, cmp, internal_iter, sequence, seed);

    (it, calls, last_len)
}

pub(crate) fn collect_forward_visible_entries(
    it: &Rc<RefCell<LevelDBIterator>>,
) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut out: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();

    it.borrow_mut().seek_to_first();

    loop {
        let it_ref: std::cell::Ref<'_, LevelDBIterator> = it.as_ref().borrow();

        let is_valid = it_ref.valid();
        if !is_valid {
            break;
        }

        let k = it_ref.key();
        let v = it_ref.value();

        out.push((k.as_bytes().to_vec(), v.as_bytes().to_vec()));

        drop(it_ref);
        it.borrow_mut().next();
    }

    out
}

pub(crate) fn collect_reverse_visible_entries(
    it: &Rc<RefCell<LevelDBIterator>>,
) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut out: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();

    it.borrow_mut().seek_to_last();

    loop {
        let it_ref: std::cell::Ref<'_, LevelDBIterator> = it.as_ref().borrow();
        let is_valid = it_ref.valid();
        if !is_valid {
            break;
        }

        let k = it_ref.key();
        let v = it_ref.value();

        out.push((k.as_bytes().to_vec(), v.as_bytes().to_vec()));

        drop(it_ref);
        it.borrow_mut().prev();
    }

    out
}

pub(crate) struct FixedStatusSingleEntryIterator {
    key:    Vec<u8>,
    value:  Vec<u8>,
    status: Status,
    index:  Option<usize>,
}

impl FixedStatusSingleEntryIterator {
    pub(crate) fn new(key: Vec<u8>, value: Vec<u8>, status: Status) -> Self {
        Self {
            key,
            value,
            status,
            index: None,
        }
    }
}

impl LevelDBIteratorInterface for FixedStatusSingleEntryIterator {}

impl LevelDBIteratorValid for FixedStatusSingleEntryIterator {
    fn valid(&self) -> bool {
        self.index.is_some()
    }
}

impl LevelDBIteratorSeekToFirst for FixedStatusSingleEntryIterator {
    fn seek_to_first(&mut self) {
        self.index = Some(0);
    }
}

impl LevelDBIteratorSeekToLast for FixedStatusSingleEntryIterator {
    fn seek_to_last(&mut self) {
        self.index = Some(0);
    }
}

impl LevelDBIteratorSeek for FixedStatusSingleEntryIterator {
    fn seek(&mut self, _target: &Slice) {
        self.index = Some(0);
    }
}

impl LevelDBIteratorNext for FixedStatusSingleEntryIterator {
    fn next(&mut self) {
        self.index = None;
    }
}

impl LevelDBIteratorPrev for FixedStatusSingleEntryIterator {
    fn prev(&mut self) {
        self.index = None;
    }
}

impl LevelDBIteratorStatus for FixedStatusSingleEntryIterator {
    fn status(&self) -> crate::Status {
        self.status.clone()
    }
}

impl LevelDBIteratorKey for FixedStatusSingleEntryIterator {
    fn key(&self) -> Slice {
        Slice::from_bytes(self.key.as_slice())
    }
}

impl LevelDBIteratorValue for FixedStatusSingleEntryIterator {
    fn value(&self) -> Slice {
        Slice::from_bytes(self.value.as_slice())
    }
}

pub(crate) fn make_fixed_status_internal_iter(
    key:    Vec<u8>,
    value:  Vec<u8>,
    status: Status,
) -> Rc<RefCell<LevelDBIterator>> {
    let inner = FixedStatusSingleEntryIterator::new(key, value, status);
    Rc::new(RefCell::new(LevelDBIterator::new(Some(Box::new(inner)))))
}

pub(crate) fn build_dbiter_direct_with_internal_iter(
    sequence:     SequenceNumber,
    seed:         u32,
    internal_iter: Rc<RefCell<LevelDBIterator>>,
) -> (DBIter, Arc<AtomicUsize>, Arc<AtomicUsize>) {
    let (db, calls, last_len) = make_read_sample_counting_db();
    let cmp = make_user_comparator();

    let dbiter = DBIter::new(db, cmp, internal_iter, sequence, seed);

    (dbiter, calls, last_len)
}

pub(crate) fn status_code(s: &Status) -> StatusCode {
    s.code()
}

pub(crate) fn slice_bytes(s: &Slice) -> Vec<u8> {
    s.as_bytes().to_vec()
}
