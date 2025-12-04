// ---------------- [ File: bitcoinleveldb-iteratorwrapper/src/iterator_wrapper.rs ]
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
pub struct LevelDBIteratorWrapper {
    iter:  Option<Box<dyn LevelDBIteratorInterface>>,
    valid: bool,
    key_:  Slice,
}

impl Default for LevelDBIteratorWrapper {

    fn default() -> Self {
        trace!(
            "LevelDBIteratorWrapper::default: initializing with no iterator and invalid state"
        );
        LevelDBIteratorWrapper {
            iter:  None,
            valid: false,
            key_:  Slice::default(),
        }
    }
}

impl LevelDBIteratorWrapper {

    /**
      | Construct a wrapper that optionally owns
      | an iterator. Passing `None` corresponds
      | to the default C++ constructor with
      | a null `Iterator*`.
      */
    pub fn new(iter: Option<Box<dyn LevelDBIteratorInterface>>) -> Self {
        trace!(
            "LevelDBIteratorWrapper::new: constructing wrapper; has_iter={}",
            iter.is_some()
        );

        let mut wrapper = LevelDBIteratorWrapper::default();
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
      | Cached valid() result from the most recent
      | Update() or positioning call.
      */
    pub fn valid(&self) -> bool {
        trace!(
            "LevelDBIteratorWrapper::valid: cached_valid={}, has_iter={}",
            self.valid,
            self.has_iterator()
        );
        self.valid
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
      | Iterator status: returns the underlying
      | iterator's Status.
      |
      | REQUIRES: an iterator is attached.
      */
    pub fn status(&self) -> crate::Status {
        trace!(
            "LevelDBIteratorWrapper::status: querying underlying iterator; has_iter={}",
            self.has_iterator()
        );

        let iter = self
            .iter()
            .expect("LevelDBIteratorWrapper::status: underlying iterator is missing");

        let st = iter.status();
        trace!(
            "LevelDBIteratorWrapper::status: underlying status_code={:?}",
            st.code()
        );
        st
    }

    /**
      | Replace the currently owned iterator with
      | a new one (or detach it if `None`).
      |
      | This matches the C++ semantics of
      | IteratorWrapper::Set, which deletes the
      | previous iterator, installs the new one,
      | and refreshes the cached state.
      */
    pub fn reset_iterator(&mut self, iter: Option<Box<dyn LevelDBIteratorInterface>>) {
        trace!(
            "LevelDBIteratorWrapper::reset_iterator: replacing iterator; had_iter={}, new_has_iter={}",
            self.has_iterator(),
            iter.is_some()
        );

        if let Some(old_iter) = self.take_iter() {
            let raw: *const dyn LevelDBIteratorInterface = &*old_iter;
            trace!(
                "LevelDBIteratorWrapper::reset_iterator: deallocating previously owned iterator at {:p}",
                raw
            );
            drop(old_iter);
        } else {
            trace!(
                "LevelDBIteratorWrapper::reset_iterator: no existing iterator to deallocate"
            );
        }

        self.replace_iter(iter);

        if !self.has_iterator() {
            trace!(
                "LevelDBIteratorWrapper::reset_iterator: new iterator is None; marking wrapper as invalid"
            );
            self.set_valid_flag(false);
        } else {
            trace!(
                "LevelDBIteratorWrapper::reset_iterator: new iterator attached; updating cached state"
            );
            self.update();
        }
    }
}

#[cfg(test)]
mod iterator_wrapper_behavior_tests {
    use super::*;

    #[traced_test]
    fn default_wrapper_starts_without_iterator_and_is_invalid() {
        trace!("default_wrapper_starts_without_iterator_and_is_invalid: start");

        let wrapper = LevelDBIteratorWrapper::default();

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

        let mut wrapper = LevelDBIteratorWrapper::new(Some(Box::new(stub)));

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

        let wrapper = LevelDBIteratorWrapper::default();
        let _ = wrapper.key();
    }

    #[test]
    #[should_panic]
    fn value_panics_when_wrapper_invalid() {
        trace!("value_panics_when_wrapper_invalid: start");

        let wrapper = LevelDBIteratorWrapper::default();
        let _ = wrapper.value();
    }

    #[test]
    #[should_panic]
    fn status_panics_when_no_iterator_attached() {
        trace!("status_panics_when_no_iterator_attached: start");

        let wrapper = LevelDBIteratorWrapper::default();
        let _ = wrapper.status();
    }

    #[test]
    #[should_panic]
    fn next_panics_when_no_iterator_attached() {
        trace!("next_panics_when_no_iterator_attached: start");

        let mut wrapper = LevelDBIteratorWrapper::default();
        wrapper.next();
    }

    #[test]
    #[should_panic]
    fn prev_panics_when_no_iterator_attached() {
        trace!("prev_panics_when_no_iterator_attached: start");

        let mut wrapper = LevelDBIteratorWrapper::default();
        wrapper.prev();
    }

    #[test]
    #[should_panic]
    fn seek_panics_when_no_iterator_attached() {
        trace!("seek_panics_when_no_iterator_attached: start");

        let mut wrapper = LevelDBIteratorWrapper::default();
        let target = Slice::from("abc");
        wrapper.seek(&target);
    }

    #[test]
    #[should_panic]
    fn seek_to_first_panics_when_no_iterator_attached() {
        trace!("seek_to_first_panics_when_no_iterator_attached: start");

        let mut wrapper = LevelDBIteratorWrapper::default();
        wrapper.seek_to_first();
    }

    #[test]
    #[should_panic]
    fn seek_to_last_panics_when_no_iterator_attached() {
        trace!("seek_to_last_panics_when_no_iterator_attached: start");

        let mut wrapper = LevelDBIteratorWrapper::default();
        wrapper.seek_to_last();
    }

    #[test]
    #[should_panic]
    fn update_panics_when_no_iterator_attached() {
        trace!("update_panics_when_no_iterator_attached: start");

        let mut wrapper = LevelDBIteratorWrapper::default();
        wrapper.update();
    }
}
