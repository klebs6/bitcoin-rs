// ---------------- [ File: bitcoinleveldb-iterator/src/iterator.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/iterator_wrapper.h]

/**
  | An internal wrapper class with an interface
  | similar to Iterator that caches the valid()
  | and key() results for an underlying iterator.
  |
  | This can help avoid virtual function calls and
  | also gives better cache locality.
  */
#[derive(Getters,MutGetters)]
pub struct LevelDBIterator {
    iter:  Option<Box<dyn LevelDBIteratorInterface>>,
    valid: bool,
    key_:  Slice,

    /// Shared cleanup list for this iterator base.
    ///
    /// Multiple `LevelDBIterator` instances may share the same
    /// cleanup list via `Rc`. The last one to drop triggers the
    /// execution of the registered cleanup handlers.
    #[getset(skip)]
    rep:   Rc<RefCell<LevelDBIteratorCleanupNodeList>>,
}


impl LevelDBIteratorInterface for LevelDBIterator {}

impl Default for LevelDBIterator {

    fn default() -> Self {
        trace!(
            "LevelDBIterator::default: initializing with no iterator and invalid state"
        );
        LevelDBIterator {
            iter:  None,
            valid: false,
            key_:  Slice::default(),
            rep:   Rc::new(RefCell::new(LevelDBIteratorCleanupNodeList::new())),
        }
    }
}

impl LevelDBIteratorValid for LevelDBIterator {

    /**
      | Cached valid() result from the most recent
      | Update() or positioning call.
      */
    fn valid(&self) -> bool {
        trace!(
            "LevelDBIterator::valid: cached_valid={}, has_iter={}",
            self.valid,
            self.has_iterator()
        );
        self.valid
    }
}

impl LevelDBIteratorStatus for LevelDBIterator {

    /**
      | Iterator status: returns the underlying
      | iterator's Status.
      |
      | REQUIRES: an iterator is attached.
      */
    fn status(&self) -> crate::Status {
        trace!(
            "LevelDBIterator::status: querying underlying iterator; has_iter={}",
            self.has_iterator()
        );

        let iter = self
            .iter()
            .expect("LevelDBIterator::status: underlying iterator is missing");

        let st = iter.status();
        trace!(
            "LevelDBIterator::status: underlying status_code={:?}",
            st.code()
        );
        st
    }
}

impl LevelDBIterator {

    /**
      | Construct a wrapper that optionally owns
      | an iterator. Passing `None` corresponds
      | to the default C++ constructor with
      | a null `Iterator*`.
      */
    pub fn new(iter: Option<Box<dyn LevelDBIteratorInterface>>) -> Self {
        trace!(
            "LevelDBIterator::new: constructing wrapper; has_iter={}",
            iter.is_some()
        );

        let mut wrapper = LevelDBIterator::default();
        wrapper.reset_iterator(iter);
        wrapper
    }

    /**
      | Returns true iff an underlying iterator
      | is currently attached.
      */
    pub fn has_iterator(&self) -> bool {
        self.iter.is_some()
    }

    /**
      | Read‑only access to the underlying iterator,
      | if present.
      */
    pub fn iter(&self) -> Option<&dyn LevelDBIteratorInterface> {
        self.iter
            .as_ref()
            .map(|boxed| boxed.as_ref() as &dyn LevelDBIteratorInterface)
    }

    /**
      | Mutable access to the underlying iterator,
      | if present.
      */
    pub fn iter_mut(&mut self) -> Option<&mut dyn LevelDBIteratorInterface> {
        self.iter
            .as_mut()
            .map(|boxed| boxed.as_mut() as &mut dyn LevelDBIteratorInterface)
    }

    /**
      | Internal helper used by Drop and reset logic
      | to take ownership of the iterator box.
      */
    pub(crate) fn take_iter(&mut self) -> Option<Box<dyn LevelDBIteratorInterface>> {
        self.iter.take()
    }

    /**
      | Internal helper that installs a new iterator
      | box and returns the previous one (if any).
      */
    pub(crate) fn replace_iter(
        &mut self,
        new_iter: Option<Box<dyn LevelDBIteratorInterface>>,
    ) -> Option<Box<dyn LevelDBIteratorInterface>> {
        core::mem::replace(&mut self.iter, new_iter)
    }

    /**
      | Internal helper to mutate the cached valid flag.
      */
    pub(crate) fn set_valid_flag(&mut self, flag: bool) {
        self.valid = flag;
    }

    /**
      | Internal helper to read the cached key slice.
      */
    pub(crate) fn cached_key(&self) -> &Slice {
        &self.key_
    }

    /**
      | Internal helper to overwrite the cached key
      | from another Slice.
      */
    pub(crate) fn set_cached_key_from_slice(&mut self, src: &Slice) {
        let data = src.data();
        let size = src.size();
        self.key_ = Slice::from_ptr_len(*data, *size);
    }

    /**
      | Replace the currently owned iterator with
      | a new one (or detach it if `None`).
      |
      | This matches the C++ semantics of
      | LevelDBIterator::Set, which deletes the
      | previous iterator, installs the new one,
      | and refreshes the cached state.
      */
    pub fn reset_iterator(&mut self, iter: Option<Box<dyn LevelDBIteratorInterface>>) {
        trace!(
            "LevelDBIterator::reset_iterator: replacing iterator; had_iter={}, new_has_iter={}",
            self.has_iterator(),
            iter.is_some()
        );

        if let Some(old_iter) = self.take_iter() {
            let raw: *const dyn LevelDBIteratorInterface = &*old_iter;
            trace!(
                "LevelDBIterator::reset_iterator: deallocating previously owned iterator at {:p}",
                raw
            );
            drop(old_iter);
        } else {
            trace!(
                "LevelDBIterator::reset_iterator: no existing iterator to deallocate"
            );
        }

        self.replace_iter(iter);

        if !self.has_iterator() {
            trace!(
                "LevelDBIterator::reset_iterator: new iterator is None; marking wrapper as invalid"
            );
            self.set_valid_flag(false);
        } else {
            trace!(
                "LevelDBIterator::reset_iterator: new iterator attached; updating cached state"
            );
            self.update();
        }
    }

    /// Return a cloned handle to the shared cleanup list.
    ///
    /// This mirrors the original design where callers could
    /// hold an extra `Rc` to the iterator's cleanup state and
    /// observe the reference count.
    pub fn rep(&self) -> Rc<RefCell<LevelDBIteratorCleanupNodeList>> {
        self.rep.clone()
    }
}

impl RegisterCleanup for LevelDBIterator {

    /**
      | Register a cleanup function that will be
      | invoked when the iterator is eventually
      | destroyed.
      |
      */
    fn register_cleanup(
        &self,
        func:  LevelDBIteratorCleanupFunction,
        arg1:  *mut c_void,
        arg2:  *mut c_void
    )
    {
        trace!(
            "LevelDBIteratorInner::register_cleanup: delegating to inner; func={:p}, arg1={:?}, arg2={:?}",
            func as *const (),
            arg1,
            arg2
        );
        self.rep.borrow_mut().register_cleanup(func, arg1, arg2);
    }
}

#[cfg(test)]
mod iterator_wrapper_behavior_tests {
    use super::*;

    #[traced_test]
    fn default_wrapper_starts_without_iterator_and_is_invalid() {
        trace!("default_wrapper_starts_without_iterator_and_is_invalid: start");

        let wrapper = LevelDBIterator::default();

        assert!(
            !wrapper.has_iterator(),
            "default wrapper must not own an iterator"
        );
        assert!(
            !wrapper.valid(),
            "default wrapper must start in an invalid state"
        );
    }

    #[traced_test]
    fn wrapper_constructed_with_iterator_starts_invalid_until_positioned() {
        trace!("wrapper_constructed_with_iterator_starts_invalid_until_positioned: start");

        let stub = MockStubIterator::new_with_entries(&[(
            b"key1".as_ref(),
            b"value1".as_ref(),
        )]);

        let mut wrapper = LevelDBIterator::new(Some(Box::new(stub)));

        assert!(
            wrapper.has_iterator(),
            "wrapper constructed with an iterator must report has_iterator()"
        );
        assert!(
            !wrapper.valid(),
            "wrapper should be invalid until the underlying iterator is positioned"
        );

        wrapper.seek_to_first();

        assert!(
            wrapper.valid(),
            "wrapper must become valid after seek_to_first on non-empty iterator"
        );

        let key = wrapper.key().to_string();
        let value = wrapper.value().to_string();

        assert_eq!(key, "key1");
        assert_eq!(value, "value1");
    }

    #[test]
    #[should_panic]
    fn key_panics_when_wrapper_invalid() {
        trace!("key_panics_when_wrapper_invalid: start");

        let wrapper = LevelDBIterator::default();
        let _ = wrapper.key();
    }

    #[test]
    #[should_panic]
    fn value_panics_when_wrapper_invalid() {
        trace!("value_panics_when_wrapper_invalid: start");

        let wrapper = LevelDBIterator::default();
        let _ = wrapper.value();
    }

    #[test]
    #[should_panic]
    fn status_panics_when_no_iterator_attached() {
        trace!("status_panics_when_no_iterator_attached: start");

        let wrapper = LevelDBIterator::default();
        let _ = wrapper.status();
    }

    #[test]
    #[should_panic]
    fn next_panics_when_no_iterator_attached() {
        trace!("next_panics_when_no_iterator_attached: start");

        let mut wrapper = LevelDBIterator::default();
        wrapper.next();
    }

    #[test]
    #[should_panic]
    fn prev_panics_when_no_iterator_attached() {
        trace!("prev_panics_when_no_iterator_attached: start");

        let mut wrapper = LevelDBIterator::default();
        wrapper.prev();
    }

    #[test]
    #[should_panic]
    fn seek_panics_when_no_iterator_attached() {
        trace!("seek_panics_when_no_iterator_attached: start");

        let mut wrapper = LevelDBIterator::default();
        let target = Slice::from("abc");
        wrapper.seek(&target);
    }

    #[test]
    #[should_panic]
    fn seek_to_first_panics_when_no_iterator_attached() {
        trace!("seek_to_first_panics_when_no_iterator_attached: start");

        let mut wrapper = LevelDBIterator::default();
        wrapper.seek_to_first();
    }

    #[test]
    #[should_panic]
    fn seek_to_last_panics_when_no_iterator_attached() {
        trace!("seek_to_last_panics_when_no_iterator_attached: start");

        let mut wrapper = LevelDBIterator::default();
        wrapper.seek_to_last();
    }

    #[test]
    #[should_panic]
    fn update_panics_when_no_iterator_attached() {
        trace!("update_panics_when_no_iterator_attached: start");

        let mut wrapper = LevelDBIterator::default();
        wrapper.update();
    }
}


#[cfg(test)]
mod tests_leveldb_iterator_lifecycle {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static ITERATOR_CLEANUP_CALLED: AtomicUsize = AtomicUsize::new(0);

    fn iterator_cleanup(_arg1: *mut c_void, _arg2: *mut c_void) -> c_void {
        debug!("iterator_cleanup invoked");
        ITERATOR_CLEANUP_CALLED.fetch_add(1, Ordering::SeqCst);
        unsafe { std::mem::zeroed() }
    }

    #[traced_test]
    fn leveldb_iterator_new_creates_inner_without_cleanup() {
        let iter = LevelDBIterator::default();
        let inner_rc = iter.rep().clone();

        // We have two references now: `iter` and `inner_rc`.
        assert_eq!(Rc::strong_count(&inner_rc), 2);

        // Dropping `iter` should not panic, and the inner will be dropped
        // once the last Rc goes away.
        drop(iter);
        drop(inner_rc);
    }

    #[traced_test]
    fn leveldb_iterator_register_cleanup_delegates_to_inner() {
        ITERATOR_CLEANUP_CALLED.store(0, Ordering::SeqCst);

        {
            let iter = LevelDBIterator::default();
            iter.register_cleanup(
                iterator_cleanup,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            );
        } // iterator (and its inner) dropped here

        assert_eq!(
            ITERATOR_CLEANUP_CALLED.load(Ordering::SeqCst),
            1,
            "Cleanup registered via LevelDBIterator::register_cleanup must run exactly once on drop"
        );
    }
}
