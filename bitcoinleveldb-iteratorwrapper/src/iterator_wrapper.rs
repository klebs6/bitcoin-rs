// ---------------- [ File: bitcoinleveldb-iteratorwrapper/src/iterator_wrapper.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/iterator_wrapper.h]

/**
  | A internal wrapper class with an interface
  | similar to Iterator that caches the valid() and
  | key() results for an underlying iterator.
  |
  | This can help avoid virtual function calls and
  | also gives better cache locality.
  */
#[derive(Getters,MutGetters,Setters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct LevelDBIteratorWrapper {
    // Raw owning pointer to the polymorphic iterator interface.
    // Must have been allocated via Box<dyn LevelDBIteratorInterface>.
    iter:  *mut dyn LevelDBIteratorInterface,
    valid: bool,
    key_:  Slice,
}

impl Default for LevelDBIteratorWrapper {
    fn default() -> Self {
        trace!(
            "LevelDBIteratorWrapper::default: initializing with null iterator and invalid state"
        );
        LevelDBIteratorWrapper {
            iter:  core::ptr::null_mut(),
            valid: false,
            key_:  Slice::default(),
        }
    }
}

impl LevelDBIteratorWrapper {
    pub fn new(iter: *mut dyn LevelDBIteratorInterface) -> Self {
        trace!(
            "LevelDBIteratorWrapper::new: constructing wrapper for iter={:?}",
            iter
        );

        let mut wrapper = LevelDBIteratorWrapper {
            iter:  core::ptr::null_mut(),
            valid: false,
            key_:  Slice::default(),
        };

        wrapper.set(iter);
        wrapper
    }

    /**
      | Methods below require iter() != nullptr
      |
      */
    pub fn status(&self) -> crate::Status {
        trace!(
            "LevelDBIteratorWrapper::status: querying underlying iterator; iter={:?}",
            self.iter
        );

        unsafe {
            assert!(
                !self.iter.is_null(),
                "LevelDBIteratorWrapper::status: underlying iterator pointer is null"
            );
            let st = (*self.iter).status();
            trace!(
                "LevelDBIteratorWrapper::status: underlying status_code={:?}",
                st.code()
            );
            st
        }
    }
}

#[cfg(test)]
mod iterator_wrapper_contract_tests {
    use super::*;
    use core::ptr;

    #[traced_test]
    fn iterator_wrapper_default_state_is_invalid_and_null() {
        trace!("iterator_wrapper_default_state_is_invalid_and_null: start");

        let wrapper = LevelDBIteratorWrapper::default();

        trace!(
            "iterator_wrapper_default_state_is_invalid_and_null: iter={:?}, valid={}",
            wrapper.iter(),
            wrapper.valid()
        );

        assert!(
            wrapper.iter().is_null(),
            "default LevelDBIteratorWrapper must start with a null iterator pointer"
        );
        assert!(
            !wrapper.valid(),
            "default LevelDBIteratorWrapper must start in an invalid state"
        );
    }

    #[traced_test]
    fn iterator_wrapper_new_with_null_produces_invalid_wrapper() {
        trace!("iterator_wrapper_new_with_null_produces_invalid_wrapper: start");

        let default_wrapper = LevelDBIteratorWrapper::default();
        let new_wrapper     = LevelDBIteratorWrapper::new(ptr::null_mut());

        trace!(
            "iterator_wrapper_new_with_null_produces_invalid_wrapper: default_iter={:?}, new_iter={:?}",
            default_wrapper.iter(),
            new_wrapper.iter()
        );

        assert!(
            new_wrapper.iter().is_null(),
            "new wrapper constructed with null iterator must hold a null pointer"
        );
        assert_eq!(
            default_wrapper.valid(),
            new_wrapper.valid(),
            "new wrapper constructed with null iterator must be invalid, matching Default"
        );
    }

    #[test]
    #[should_panic]
    fn iterator_wrapper_key_panics_when_invalid() {
        trace!("iterator_wrapper_key_panics_when_invalid: start");

        let wrapper = LevelDBIteratorWrapper::default();

        trace!(
            "iterator_wrapper_key_panics_when_invalid: about to call key() with valid={}",
            wrapper.valid()
        );

        // Must panic because the iterator is not valid.
        let _ = wrapper.key();
    }

    #[test]
    #[should_panic]
    fn iterator_wrapper_value_panics_when_invalid() {
        trace!("iterator_wrapper_value_panics_when_invalid: start");

        let wrapper = LevelDBIteratorWrapper::default();

        trace!(
            "iterator_wrapper_value_panics_when_invalid: about to call value() with valid={}",
            wrapper.valid()
        );

        // Must panic because the iterator is not valid.
        let _ = wrapper.value();
    }

    #[test]
    #[should_panic]
    fn iterator_wrapper_status_panics_when_iter_pointer_is_null() {
        trace!("iterator_wrapper_status_panics_when_iter_pointer_is_null: start");

        let wrapper = LevelDBIteratorWrapper::default();

        trace!(
            "iterator_wrapper_status_panics_when_iter_pointer_is_null: about to call status() with iter={:?}",
            wrapper.iter()
        );

        // Must panic because the underlying iterator pointer is null.
        let _ = wrapper.status();
    }

    #[test]
    #[should_panic]
    fn iterator_wrapper_next_panics_when_iter_pointer_is_null() {
        trace!("iterator_wrapper_next_panics_when_iter_pointer_is_null: start");

        let mut wrapper = LevelDBIteratorWrapper::default();

        trace!(
            "iterator_wrapper_next_panics_when_iter_pointer_is_null: about to call next() with iter={:?}",
            wrapper.iter()
        );

        // Must panic because the underlying iterator pointer is null.
        wrapper.next();
    }

    #[test]
    #[should_panic]
    fn iterator_wrapper_prev_panics_when_iter_pointer_is_null() {
        trace!("iterator_wrapper_prev_panics_when_iter_pointer_is_null: start");

        let mut wrapper = LevelDBIteratorWrapper::default();

        trace!(
            "iterator_wrapper_prev_panics_when_iter_pointer_is_null: about to call prev() with iter={:?}",
            wrapper.iter()
        );

        // Must panic because the underlying iterator pointer is null.
        wrapper.prev();
    }

    #[test]
    #[should_panic]
    fn iterator_wrapper_seek_panics_when_iter_pointer_is_null() {
        trace!("iterator_wrapper_seek_panics_when_iter_pointer_is_null: start");

        let mut wrapper = LevelDBIteratorWrapper::default();
        let target      = Slice::default();

        trace!(
            "iterator_wrapper_seek_panics_when_iter_pointer_is_null: about to call seek() with iter={:?}, target={:?}",
            wrapper.iter(),
            target
        );

        // Must panic because the underlying iterator pointer is null.
        wrapper.seek(&target);
    }

    #[test]
    #[should_panic]
    fn iterator_wrapper_seek_to_first_panics_when_iter_pointer_is_null() {
        trace!("iterator_wrapper_seek_to_first_panics_when_iter_pointer_is_null: start");

        let mut wrapper = LevelDBIteratorWrapper::default();

        trace!(
            "iterator_wrapper_seek_to_first_panics_when_iter_pointer_is_null: about to call seek_to_first() with iter={:?}",
            wrapper.iter()
        );

        // Must panic because the underlying iterator pointer is null.
        wrapper.seek_to_first();
    }

    #[test]
    #[should_panic]
    fn iterator_wrapper_seek_to_last_panics_when_iter_pointer_is_null() {
        trace!("iterator_wrapper_seek_to_last_panics_when_iter_pointer_is_null: start");

        let mut wrapper = LevelDBIteratorWrapper::default();

        trace!(
            "iterator_wrapper_seek_to_last_panics_when_iter_pointer_is_null: about to call seek_to_last() with iter={:?}",
            wrapper.iter()
        );

        // Must panic because the underlying iterator pointer is null.
        wrapper.seek_to_last();
    }

    #[test]
    #[should_panic]
    fn iterator_wrapper_update_panics_when_iter_pointer_is_null() {
        trace!("iterator_wrapper_update_panics_when_iter_pointer_is_null: start");

        let mut wrapper = LevelDBIteratorWrapper::default();

        trace!(
            "iterator_wrapper_update_panics_when_iter_pointer_is_null: about to call update() with iter={:?}",
            wrapper.iter()
        );

        // Must panic because the underlying iterator pointer is null.
        wrapper.update();
    }
}
