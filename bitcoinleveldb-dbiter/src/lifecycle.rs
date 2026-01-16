// ---------------- [ File: bitcoinleveldb-dbiter/src/lifecycle.rs ]
crate::ix!();

pub trait DBIterReadSample: 'static {
    fn record_read_sample(&mut self, key: Slice);
}

impl Drop for DBIter {
    fn drop(&mut self) {
        // C++: delete iter_;
        // Rust: owned Rc<RefCell<LevelDBIterator>> is released automatically.
        trace!("DBIter::drop");
    }
}

impl DBIter {
    pub fn new(
        db:   Rc<RefCell<dyn DBIterReadSample>>,
        cmp:  Box<dyn SliceComparator>,
        iter: Rc<RefCell<LevelDBIterator>>,
        s:    SequenceNumber,
        seed: u32,
    ) -> Self {
        trace!(
            "DBIter::new: seed={} sequence={} (initial valid=false, direction=Forward)",
            seed,
            s
        );

        let mut this = Self::new_inner(db, cmp, iter, s, seed);

        let initial_period = this.random_compaction_period();
        this.set_bytes_until_read_sampling(initial_period);

        debug!(
            "DBIter::new: bytes_until_read_sampling={}",
            this.bytes_until_read_sampling()
        );

        this
    }
}

#[cfg(test)]
mod dbiter_new_db_iterator_suite {
    use super::*;

    #[traced_test]
    fn new_db_iterator_starts_invalid_until_positioned() {
        info!("new_db_iterator starts invalid until a positioning call occurs");

        let entries = vec![make_entry(b"a", 1, ValueType::TypeValue, b"va")];
        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 123, entries);

        assert!(!it.borrow().valid());
    }

    #[traced_test]
    fn new_db_iterator_can_iterate_single_visible_entry() {
        info!("new_db_iterator yields a single visible entry after seek_to_first");

        let entries = vec![make_entry(b"a", 7, ValueType::TypeValue, b"va7")];
        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(7, 999, entries);

        it.borrow_mut().seek_to_first();

        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"a");
        assert_eq!(it.borrow().value().as_bytes(), b"va7");

        it.borrow_mut().next();
        assert!(!it.borrow().valid());
    }

    #[traced_test]
    fn new_db_iterator_applies_snapshot_sequence_and_deletion_semantics() {
        info!("new_db_iterator filters by sequence and respects deletion markers");

        // Internal order:
        // a: seq 7 value (hidden by snapshot=5), seq 5 value (visible)
        // b: seq 6 value (hidden), no older versions => b absent
        // c: seq 4 deletion (visible), seq 3 value (hidden) => c absent
        // d: seq 2 value (visible)
        let entries = vec![
            make_entry(b"a", 7, ValueType::TypeValue, b"a7"),
            make_entry(b"a", 5, ValueType::TypeValue, b"a5"),
            make_entry(b"b", 6, ValueType::TypeValue, b"b6"),
            make_entry(b"c", 4, ValueType::TypeDeletion, b""),
            make_entry(b"c", 3, ValueType::TypeValue, b"c3"),
            make_entry(b"d", 2, ValueType::TypeValue, b"d2"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(5, 42, entries);

        let got = collect_forward_visible_entries(&it);

        assert_eq!(got.len(), 2);
        assert_eq!(got[0].0.as_slice(), b"a");
        assert_eq!(got[0].1.as_slice(), b"a5");
        assert_eq!(got[1].0.as_slice(), b"d");
        assert_eq!(got[1].1.as_slice(), b"d2");
    }
}
