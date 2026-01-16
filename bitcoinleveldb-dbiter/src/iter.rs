// ---------------- [ File: bitcoinleveldb-dbiter/src/iter.rs ]
crate::ix!();

/// Memtables and sstables that make the DB representation contain
/// (userkey,seq,type) => uservalue entries.  
///
/// DBIter combines multiple entries for the same userkey found in the DB
/// representation into a single entry while accounting for sequence numbers,
/// deletion markers, overwrites, etc.
///
#[derive(Getters, CopyGetters, MutGetters, Setters)]
pub struct DBIter {
    base: LevelDBIterator,

    #[getset(get="pub(crate)")]
    db: Rc<RefCell<dyn DBIterReadSample>>,

    #[getset(get="pub(crate)")]
    user_comparator: Box<dyn SliceComparator>,

    #[getset(get="pub(crate)")]
    iter: Rc<RefCell<LevelDBIterator>>,

    #[getset(get_copy="pub(crate)")]
    sequence: SequenceNumber,

    #[getset(get="pub(crate)", set="pub(crate)")]
    internal_status: Status,

    /**
      | == current key when direction_==kReverse
      |
      */
    #[getset(get="pub(crate)", get_mut="pub(crate)")]
    saved_key_: String,

    /**
      | == current raw value when direction_==kReverse
      |
      */
    #[getset(get="pub(crate)", get_mut="pub(crate)")]
    saved_value: String,

    #[getset(get_copy="pub(crate)", set="pub(crate)")]
    direction: DBIterDirection,

    #[getset(get_copy="pub", set="pub(crate)")]
    valid: bool,

    #[getset(get_mut="pub(crate)")]
    rnd: Random,

    #[getset(get_copy="pub(crate)", set="pub(crate)")]
    bytes_until_read_sampling: usize,
}

impl DBIter {
    pub(crate) fn new_inner(
        db:   Rc<RefCell<dyn DBIterReadSample>>,
        cmp:  Box<dyn SliceComparator>,
        iter: Rc<RefCell<LevelDBIterator>>,
        s:    SequenceNumber,
        seed: u32,
    ) -> Self {
        Self {
            base:                      LevelDBIterator::default(),
            db:                        db,
            user_comparator:           cmp,
            iter:                      iter,
            sequence:                  s,
            internal_status:           Status::ok(),
            saved_key_:                String::new(),
            saved_value:               String::new(),
            direction:                 DBIterDirection::Forward,
            valid:                     false,
            rnd:                       Random::new(seed),
            bytes_until_read_sampling: 0,
        }
    }
}

/// Which direction is the iterator currently
/// moving?
/// 
/// (1) When moving forward, the internal
///     iterator is positioned at the exact entry
///     that yields this->key(), this->value()
/// 
/// (2) When moving backwards, the internal
///     iterator is positioned just before all
///     entries whose user key == this->key().
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DBIterDirection { Forward, Reverse }

/// Return a new iterator that converts internal keys (yielded by
/// "*internal_iter") that were live at the specified "sequence" number into
/// appropriate user keys.
///
pub fn new_db_iterator(
    db:                  Rc<RefCell<dyn DBIterReadSample>>,
    user_key_comparator: Box<dyn SliceComparator>,
    internal_iter:       Rc<RefCell<LevelDBIterator>>,
    sequence:            SequenceNumber,
    seed:                u32,
) -> Rc<RefCell<LevelDBIterator>> {
    trace!(
        "new_db_iterator: sequence={} seed={} (wrapping internal iterator)",
        sequence,
        seed
    );

    let db_iter = DBIter::new(db, user_key_comparator, internal_iter, sequence, seed);

    Rc::new(RefCell::new(LevelDBIterator::new(Some(Box::new(db_iter)))))
}
