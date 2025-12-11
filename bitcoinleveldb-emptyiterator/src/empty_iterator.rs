// ---------------- [ File: bitcoinleveldb-emptyiterator/src/empty_iterator.rs ]
crate::ix!();

pub fn new_empty_iterator() -> *mut LevelDBIterator {
    trace!("new_empty_iterator: creating OK empty iterator");

    let ok_status = Status::ok();

    // Build the underlying EmptyIterator as a dynamic LevelDBIteratorInterface.
    let empty_iface: Box<dyn LevelDBIteratorInterface> =
        Box::new(EmptyIterator::new(ok_status));

    // Wrap it in a LevelDBIterator so that the wrapper owns the interface
    // and all methods delegate correctly (has_iter == true).
    let wrapper = LevelDBIterator::new(Some(empty_iface));

    let raw_wrapper: *mut LevelDBIterator = Box::into_raw(Box::new(wrapper));

    trace!(
        "new_empty_iterator: allocated LevelDBIterator wrapper at {:?}",
        raw_wrapper
    );

    raw_wrapper
}

/// An iterator representing either an empty data set or a terminal error.
///
/// When constructed with `Status::ok()`, this behaves as an "empty but OK"
/// iterator: `valid()` is always false and `status()` is OK.
///
/// When constructed with a non‑OK status (e.g. via `new_error_iterator`),
/// this behaves as a terminal error iterator: `valid()` is still always
/// false, but `status()` returns the stored non‑OK status.
pub struct EmptyIterator {
    status_: Status,
}

impl EmptyIterator {
    /// Construct a new EmptyIterator with the provided status.
    ///
    /// If `status` is OK, this behaves as an "empty but OK" iterator.
    /// If `status` is non-OK, this behaves as a terminal error iterator.
    pub fn new(status: Status) -> Self {
        trace!(
            "EmptyIterator::new: status_is_ok={}",
            status.is_ok()
        );
        EmptyIterator { status_: status }
    }
}

impl LevelDBIteratorValid for EmptyIterator {
    fn valid(&self) -> bool {
        let ok = self.status_.is_ok();
        trace!(
            "EmptyIterator::valid -> false (status_is_ok={})",
            ok
        );
        false
    }
}

impl LevelDBIteratorSeekToFirst for EmptyIterator {
    fn seek_to_first(&mut self) {
        trace!("EmptyIterator::seek_to_first: no-op");
    }
}

impl LevelDBIteratorSeekToLast for EmptyIterator {
    fn seek_to_last(&mut self) {
        trace!("EmptyIterator::seek_to_last: no-op");
    }
}

impl LevelDBIteratorSeek for EmptyIterator {
    fn seek(&mut self, target: &Slice) {
        trace!(
            "EmptyIterator::seek: no-op, target_len={}",
            *target.size()
        );
    }
}

impl LevelDBIteratorNext for EmptyIterator {
    fn next(&mut self) {
        trace!("EmptyIterator::next: no-op");
    }
}

impl LevelDBIteratorPrev for EmptyIterator {
    fn prev(&mut self) {
        trace!("EmptyIterator::prev: no-op");
    }
}

impl LevelDBIteratorKey for EmptyIterator {
    fn key(&self) -> Slice {
        trace!("EmptyIterator::key: returning empty Slice");
        Slice::default()
    }
}

impl LevelDBIteratorValue for EmptyIterator {
    fn value(&self) -> Slice {
        trace!("EmptyIterator::value: returning empty Slice");
        Slice::default()
    }
}

impl LevelDBIteratorStatus for EmptyIterator {
    fn status(&self) -> Status {
        trace!(
            "EmptyIterator::status: status_is_ok={}",
            self.status_.is_ok()
        );
        bitcoinleveldb_status::Status::new_from_other_copy(
            &self.status_,
        )
    }
}

impl LevelDBIteratorInterface for EmptyIterator {}

#[cfg(test)]
mod empty_iterator_behavior_tests {
    use super::*;

    #[traced_test]
    fn empty_iterator_ok_status_is_always_invalid() {
        let ok = Status::ok();
        let mut iter = EmptyIterator::new(ok);

        assert!(
            !iter.valid(),
            "EmptyIterator should always report invalid"
        );

        let st = iter.status();
        assert!(
            st.is_ok(),
            "EmptyIterator created with Status::ok must report OK status"
        );

        let dummy_key = Slice::default();

        // All navigation methods are no-ops and must not panic.
        iter.seek(&dummy_key);
        iter.seek_to_first();
        iter.seek_to_last();
        iter.next();
        iter.prev();

        // key/value must be empty slices.
        let k = iter.key();
        let v = iter.value();
        assert_eq!(*k.size(), 0);
        assert_eq!(*v.size(), 0);
    }

    #[traced_test]
    fn empty_iterator_error_status_is_propagated() {
        let msg = Slice::from("empty-iter-corruption");
        let err_status = Status::corruption(&msg, None);

        let iter = EmptyIterator::new(err_status);
        let st = iter.status();

        assert!(
            !st.is_ok(),
            "EmptyIterator created with non-OK status must report non-OK status"
        );
    }

    #[traced_test]
    fn new_empty_iterator_returns_ok_empty_wrapper() {
        let ptr1: *mut LevelDBIterator = new_empty_iterator();
        let ptr2: *mut LevelDBIterator = new_empty_iterator();

        assert!(
            !ptr1.is_null(),
            "new_empty_iterator must never return a null pointer (ptr1)"
        );
        assert!(
            !ptr2.is_null(),
            "new_empty_iterator must never return a null pointer (ptr2)"
        );
        assert_ne!(
            ptr1, ptr2,
            "Each call to new_empty_iterator should allocate a distinct iterator wrapper"
        );

        unsafe {
            // Interface-level behavior: always invalid, OK status.
            assert!(
                !(*ptr1).valid(),
                "wrapper from new_empty_iterator must report invalid"
            );
            assert!(
                (*ptr1).status().is_ok(),
                "wrapper from new_empty_iterator must report OK status"
            );

            assert!(
                !(*ptr2).valid(),
                "second wrapper from new_empty_iterator must report invalid"
            );
            assert!(
                (*ptr2).status().is_ok(),
                "second wrapper from new_empty_iterator must report OK status"
            );

            // Avoid leaks by reclaiming the wrappers.
            drop(Box::from_raw(ptr1));
            drop(Box::from_raw(ptr2));
        }
    }
}
