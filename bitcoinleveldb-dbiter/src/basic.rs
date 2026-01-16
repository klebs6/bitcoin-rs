// ---------------- [ File: bitcoinleveldb-dbiter/src/basic.rs ]
crate::ix!();

impl LevelDBIteratorInterface for DBIter {}

impl LevelDBIteratorValid for DBIter {
    fn valid(&self) -> bool {
        DBIter::valid(self)
    }
}

impl LevelDBIteratorKey for DBIter {
    fn key(&self) -> Slice {
        DBIter::key(self)
    }
}

impl LevelDBIteratorValue for DBIter {
    fn value(&self) -> Slice {
        DBIter::value(self)
    }
}

impl LevelDBIteratorStatus for DBIter {
    fn status(&self) -> crate::Status {
        DBIter::status(self)
    }
}

impl LevelDBIteratorNext for DBIter {
    fn next(&mut self) {
        DBIter::next(self)
    }
}

impl LevelDBIteratorPrev for DBIter {
    fn prev(&mut self) {
        DBIter::prev(self)
    }
}

impl LevelDBIteratorSeek for DBIter {
    fn seek(&mut self, target: &Slice) {
        DBIter::seek(self, target)
    }
}

impl LevelDBIteratorSeekToFirst for DBIter {
    fn seek_to_first(&mut self) {
        DBIter::seek_to_first(self)
    }
}

impl LevelDBIteratorSeekToLast for DBIter {
    fn seek_to_last(&mut self) {
        DBIter::seek_to_last(self)
    }
}
