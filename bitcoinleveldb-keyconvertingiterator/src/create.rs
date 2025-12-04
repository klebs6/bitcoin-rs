// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/create.rs ]
crate::ix!();

impl KeyConvertingIterator {

    /**
      | Construct a key‑converting iterator that
      | takes ownership of an underlying internal
      | iterator.
      |
      | Safety:
      |   * `iter` must come from
      |     `Box::<dyn LevelDBIteratorInterface>::into_raw`.
      |   * It must not be freed elsewhere; this
      |     type is responsible for calling
      |     `Box::from_raw` in `Drop`.
      |
      */
    pub fn new(iter: *mut dyn LevelDBIteratorInterface) -> Self {
        trace!(
            "KeyConvertingIterator::new: constructing with underlying iter={:?}",
            iter
        );

        KeyConvertingIterator {
            base:   LevelDBIterator::new(),
            status: RefCell::new(Status::ok()),
            iter,
        }
    }
}
