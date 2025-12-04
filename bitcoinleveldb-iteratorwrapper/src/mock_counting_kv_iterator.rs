crate::ix!();

pub struct MockCountingKVIterator {
    entries:     Vec<(Vec<u8>, Vec<u8>)>,
    index:       Option<usize>,
    key_calls:   Arc<AtomicUsize>,
    value_calls: Arc<AtomicUsize>,
}

impl LevelDBIteratorInterface for MockCountingKVIterator {}

impl MockCountingKVIterator {

    pub fn new_single(
        key: &[u8],
        value: &[u8],
        key_calls: Arc<AtomicUsize>,
        value_calls: Arc<AtomicUsize>,
    ) -> Self {
        MockCountingKVIterator {
            entries:     vec![(key.to_vec(), value.to_vec())],
            index:       Some(0),
            key_calls,
            value_calls,
        }
    }

    pub fn current_entry(&self) -> (&[u8], &[u8]) {
        let idx = self.index.expect("MockCountingKVIterator not positioned");
        let (ref k, ref v) = self.entries[idx];
        (k.as_slice(), v.as_slice())
    }
}

impl Valid for MockCountingKVIterator {

    fn valid(&self) -> bool {
        self.index.is_some()
    }
}

impl SeekToFirst for MockCountingKVIterator {

    fn seek_to_first(&mut self) {
        trace!("MockCountingKVIterator::seek_to_first (no-op for single entry)");
        self.index = Some(0);
    }
}

impl SeekToLast for MockCountingKVIterator {

    fn seek_to_last(&mut self) {
        trace!("MockCountingKVIterator::seek_to_last (no-op for single entry)");
        self.index = Some(0);
    }
}

impl Seek for MockCountingKVIterator {

    fn seek(&mut self, _target: &Slice) {
        trace!("MockCountingKVIterator::seek (no-op for KV tests)");
    }
}

impl Next for MockCountingKVIterator {

    fn next(&mut self) {
        trace!("MockCountingKVIterator::next (invalidating position)");
        self.index = None;
    }
}

impl Prev for MockCountingKVIterator {

    fn prev(&mut self) {
        trace!("MockCountingKVIterator::prev (invalidating position)");
        self.index = None;
    }
}

impl LevelDBIteratorStatus for MockCountingKVIterator {

    fn status(&self) -> crate::Status {
        crate::Status::ok()
    }
}

impl Key for MockCountingKVIterator {

    fn key(&self) -> Slice {
        self.key_calls.fetch_add(1, atomic::Ordering::SeqCst);
        let (k, _) = self.current_entry();
        Slice::from(k)
    }
}

impl Value for MockCountingKVIterator {

    fn value(&self) -> Slice {
        self.value_calls.fetch_add(1, atomic::Ordering::SeqCst);
        let (_, v) = self.current_entry();
        Slice::from(v)
    }
}
