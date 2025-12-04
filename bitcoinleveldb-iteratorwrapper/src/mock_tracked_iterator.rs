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

impl Valid for MockTrackedIterator {

    fn valid(&self) -> bool {
        false
    }
}

impl SeekToFirst for MockTrackedIterator {

    fn seek_to_first(&mut self) {}
}

impl SeekToLast for MockTrackedIterator {

    fn seek_to_last(&mut self) {}
}

impl Seek for MockTrackedIterator {

    fn seek(&mut self, _target: &Slice) {}
}

impl Next for MockTrackedIterator {

    fn next(&mut self) {}
}

impl Prev for MockTrackedIterator {

    fn prev(&mut self) {}
}

impl LevelDBIteratorStatus for MockTrackedIterator {

    fn status(&self) -> crate::Status {
        crate::Status::ok()
    }
}

impl Key for MockTrackedIterator {

    fn key(&self) -> Slice {
        panic!("MockTrackedIterator(set-tests)::key should not be called");
    }
}

impl Value for MockTrackedIterator {

    fn value(&self) -> Slice {
        panic!("MockTrackedIterator(set-tests)::value should not be called");
    }
}

impl LevelDBIteratorInterface for MockTrackedIterator {}
