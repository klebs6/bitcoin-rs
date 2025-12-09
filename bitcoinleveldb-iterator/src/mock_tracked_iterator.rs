// ---------------- [ File: bitcoinleveldb-iterator/src/mock_tracked_iterator.rs ]
crate::ix!();

pub struct MockTrackedIterator {
    drops: Arc<AtomicUsize>,
}

impl MockTrackedIterator {

    pub fn new(counter: Arc<AtomicUsize>) -> Self {
        MockTrackedIterator { drops: counter }
    }
}

impl Drop for MockTrackedIterator {

    fn drop(&mut self) {
        trace!("MockTrackedIterator(set-tests)::drop");
        self.drops.fetch_add(1, atomic::Ordering::SeqCst);
    }
}

impl LevelDBIteratorValid for MockTrackedIterator {

    fn valid(&self) -> bool {
        false
    }
}

impl LevelDBIteratorSeekToFirst for MockTrackedIterator {

    fn seek_to_first(&mut self) {}
}

impl LevelDBIteratorSeekToLast for MockTrackedIterator {

    fn seek_to_last(&mut self) {}
}

impl LevelDBIteratorSeek for MockTrackedIterator {

    fn seek(&mut self, _target: &Slice) {}
}

impl LevelDBIteratorNext for MockTrackedIterator {

    fn next(&mut self) {}
}

impl LevelDBIteratorPrev for MockTrackedIterator {

    fn prev(&mut self) {}
}

impl LevelDBIteratorStatus for MockTrackedIterator {

    fn status(&self) -> crate::Status {
        crate::Status::ok()
    }
}

impl LevelDBIteratorKey for MockTrackedIterator {

    fn key(&self) -> Slice {
        panic!("MockTrackedIterator(set-tests)::key should not be called");
    }
}

impl LevelDBIteratorValue for MockTrackedIterator {

    fn value(&self) -> Slice {
        panic!("MockTrackedIterator(set-tests)::value should not be called");
    }
}

impl LevelDBIteratorInterface for MockTrackedIterator {}
