// ---------------- [ File: bitcoinleveldb-emptyiterator/src/empty_iterator.rs ]
crate::ix!();

#[derive(Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct EmptyIterator {
    base:   LevelDBIterator,
    status: Status,
}

/**
  | Return an empty iterator (yields nothing).
  |
  */
pub fn new_empty_iterator() -> *mut LevelDBIterator {
    trace!("new_empty_iterator: creating OK empty iterator");

    let ok   = Status::ok();
    let iter = EmptyIterator::new(&ok);
    let boxed = Box::new(iter);
    let raw: *mut EmptyIterator = Box::into_raw(boxed);

    unsafe {
        // Because `base` is the first field of `EmptyIterator`, the address
        // of `base` is the same as the start of the allocation. We return
        // the pointer to the embedded `LevelDBIterator` subobject, which is
        // what the C++ API exposed as the base iterator type.
        let base_ptr: *mut LevelDBIterator = &mut *(*raw).base_mut();
        trace!(
            "new_empty_iterator: allocated EmptyIterator at {:?}, base_ptr={:?}",
            raw,
            base_ptr
        );
        base_ptr
    }
}

impl EmptyIterator {
    pub fn new(s: &Status) -> Self {
        trace!(
            "EmptyIterator::new: status_is_ok={}",
            s.is_ok()
        );

        EmptyIterator {
            base:   LevelDBIterator::default(),
            status: Status::new_from_other_copy(s),
        }
    }

    pub fn valid(&self) -> bool {
        trace!("EmptyIterator::valid called -> false");
        false
    }

    pub fn seek(&mut self, _target: &Slice) {
        trace!(
            "EmptyIterator::seek called (no-op; iterator is always invalid)"
        );
    }

    pub fn seek_to_first(&mut self) {
        trace!(
            "EmptyIterator::seek_to_first called (no-op; iterator is always invalid)"
        );
    }

    pub fn seek_to_last(&mut self) {
        trace!(
            "EmptyIterator::seek_to_last called (no-op; iterator is always invalid)"
        );
    }

    pub fn next(&mut self) {
        trace!(
            "EmptyIterator::next called; panicking because iterator is invalid"
        );
        panic!("EmptyIterator::next called on empty iterator");
    }

    pub fn prev(&mut self) {
        trace!(
            "EmptyIterator::prev called; panicking because iterator is invalid"
        );
        panic!("EmptyIterator::prev called on empty iterator");
    }

    pub fn key(&self) -> Slice {
        trace!(
            "EmptyIterator::key called; panicking because iterator is invalid"
        );
        panic!("EmptyIterator::key called on empty iterator");
    }

    pub fn value(&self) -> Slice {
        trace!(
            "EmptyIterator::value called; panicking because iterator is invalid"
        );
        panic!("EmptyIterator::value called on empty iterator");
    }
}

impl LevelDBIteratorStatus for EmptyIterator {
    fn status(&self) -> crate::Status {
        Status::new_from_other_copy(&self.status)
    }
}

#[cfg(test)]
mod tests_empty_iterator_behavior {
    use super::*;

    #[traced_test]
    fn empty_iterator_reports_invalid_and_ok_status() {
        let ok  = Status::ok();
        let mut iter = EmptyIterator::new(&ok);

        assert!(
            !iter.valid(),
            "EmptyIterator should always report invalid"
        );

        let st = iter.status();
        assert!(
            st.is_ok(),
            "EmptyIterator created with Status::ok should report ok status"
        );

        let dummy_key = Slice::default();

        // These are all no-ops and must not panic.
        iter.seek(&dummy_key);
        iter.seek_to_first();
        iter.seek_to_last();
    }

    #[test]
    #[should_panic(expected = "EmptyIterator::next called on empty iterator")]
    fn empty_iterator_next_panics_when_invalid() {
        let ok = Status::ok();
        let mut iter = EmptyIterator::new(&ok);
        iter.next();
    }

    #[test]
    #[should_panic(expected = "EmptyIterator::prev called on empty iterator")]
    fn empty_iterator_prev_panics_when_invalid() {
        let ok = Status::ok();
        let mut iter = EmptyIterator::new(&ok);
        iter.prev();
    }

    #[test]
    #[should_panic(expected = "EmptyIterator::key called on empty iterator")]
    fn empty_iterator_key_panics_when_invalid() {
        let ok = Status::ok();
        let iter = EmptyIterator::new(&ok);
        let _ = iter.key();
    }

    #[test]
    #[should_panic(expected = "EmptyIterator::value called on empty iterator")]
    fn empty_iterator_value_panics_when_invalid() {
        let ok = Status::ok();
        let iter = EmptyIterator::new(&ok);
        let _ = iter.value();
    }

    #[traced_test]
    fn new_empty_iterator_allocates_distinct_non_null_pointers() {
        let p1: *mut LevelDBIterator = new_empty_iterator();
        let p2: *mut LevelDBIterator = new_empty_iterator();

        assert!(
            !p1.is_null(),
            "new_empty_iterator must never return a null pointer (p1)"
        );
        assert!(
            !p2.is_null(),
            "new_empty_iterator must never return a null pointer (p2)"
        );
        assert_ne!(
            p1, p2,
            "Each call to new_empty_iterator should allocate a distinct iterator base pointer"
        );

        // Ownership and reclamation of these raw pointers is handled at the
        // higher layers that know the concrete iterator type. We only ensure
        // that the factory returns sensible, unique base pointers.
    }
}
