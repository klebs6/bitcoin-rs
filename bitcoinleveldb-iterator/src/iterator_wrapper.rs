// ---------------- [ File: bitcoinleveldb-iterator/src/iterator_wrapper.rs ]
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
pub struct LevelDBIteratorWrapper {
    iter:  *mut LevelDBIterator,
    valid: bool,
    key_:   Slice,
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

impl Drop for LevelDBIteratorWrapper {

    fn drop(&mut self) {
        todo!();
        /*
            delete iter_;
        */
    }
}

impl LevelDBIteratorWrapper {

    pub fn new(iter: *mut LevelDBIterator) -> Self {
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
    
    pub fn iter(&self) -> *mut LevelDBIterator {
        trace!(
            "LevelDBIteratorWrapper::iter: returning iter pointer={:?}",
            self.iter
        );
        self.iter
    }

    /**
      | Takes ownership of "iter" and will delete
      | it when destroyed, or when Set() is invoked
      | again.
      |
      */
    pub fn set(&mut self, iter: *mut LevelDBIterator)  {
        
        todo!();
        /*
            delete iter_;
        iter_ = iter;
        if (iter_ == nullptr) {
          valid_ = false;
        } else {
          Update();
        }
        */
    }

    /**
      | Iterator interface methods
      |
      */
    pub fn valid(&self) -> bool {
        trace!(
            "LevelDBIteratorWrapper::valid: cached_valid={}, iter={:?}",
            self.valid,
            self.iter
        );
        self.valid
    }
   
    pub fn key(&self) -> Slice {
        trace!(
            "LevelDBIteratorWrapper::key: requested; cached_valid={}, iter={:?}",
            self.valid,
            self.iter
        );

        assert!(
            self.valid(),
            "LevelDBIteratorWrapper::key requires the iterator to be valid"
        );

        // We must not move out of `self.key_`, so recreate an equivalent Slice
        // pointing at the same underlying bytes.
        let data = self.key_.data();
        let size = self.key_.size();
        let result = Slice::from_ptr_len(*data, *size);

        trace!(
            "LevelDBIteratorWrapper::key: returning cached key slice (data={:?}, size={})",
            data,
            size
        );

        result
    }

    pub fn value(&self) -> Slice {
        trace!(
            "LevelDBIteratorWrapper::value: requested; cached_valid={}, iter={:?}",
            self.valid,
            self.iter
        );

        assert!(
            self.valid(),
            "LevelDBIteratorWrapper::value requires the iterator to be valid"
        );

        unsafe {
            assert!(
                !self.iter.is_null(),
                "LevelDBIteratorWrapper::value: underlying iterator pointer is null"
            );
            let value = (*self.iter).value();
            trace!(
                "LevelDBIteratorWrapper::value: delegated to underlying iterator; value={:?}",
                value
            );
            value
        }
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

    pub fn next(&mut self) {
        trace!(
            "LevelDBIteratorWrapper::next: advancing; iter={:?}, before_valid={}",
            self.iter,
            self.valid
        );

        unsafe {
            assert!(
                !self.iter.is_null(),
                "LevelDBIteratorWrapper::next: underlying iterator pointer is null"
            );
            (*self.iter).next();
        }

        self.update();
    }

    pub fn prev(&mut self) {
        trace!(
            "LevelDBIteratorWrapper::prev: moving backwards; iter={:?}, before_valid={}",
            self.iter,
            self.valid
        );

        unsafe {
            assert!(
                !self.iter.is_null(),
                "LevelDBIteratorWrapper::prev: underlying iterator pointer is null"
            );
            (*self.iter).prev();
        }

        self.update();
    }

    pub fn seek(&mut self, k: &Slice) {
        trace!(
            "LevelDBIteratorWrapper::seek: seeking to target={:?}, iter={:?}",
            k,
            self.iter
        );

        unsafe {
            assert!(
                !self.iter.is_null(),
                "LevelDBIteratorWrapper::seek: underlying iterator pointer is null"
            );
            (*self.iter).seek(k);
        }

        self.update();
    }

    pub fn seek_to_first(&mut self) {
        trace!(
            "LevelDBIteratorWrapper::seek_to_first: iter={:?}",
            self.iter
        );

        unsafe {
            assert!(
                !self.iter.is_null(),
                "LevelDBIteratorWrapper::seek_to_first: underlying iterator pointer is null"
            );
            (*self.iter).seek_to_first();
        }

        self.update();
    }

    pub fn seek_to_last(&mut self) {
        trace!(
            "LevelDBIteratorWrapper::seek_to_last: iter={:?}",
            self.iter
        );

        unsafe {
            assert!(
                !self.iter.is_null(),
                "LevelDBIteratorWrapper::seek_to_last: underlying iterator pointer is null"
            );
            (*self.iter).seek_to_last();
        }

        self.update();
    }

    pub fn update(&mut self) {
        trace!(
            "LevelDBIteratorWrapper::update: refreshing cached valid/key; iter={:?}",
            self.iter
        );

        if self.iter.is_null() {
            trace!(
                "LevelDBIteratorWrapper::update: iter is null; marking invalid and clearing key cache"
            );
            self.valid = false;
            self.key_ = Slice::default();
            return;
        }

        unsafe {
            self.valid = (*self.iter).valid();
            trace!(
                "LevelDBIteratorWrapper::update: underlying valid={}",
                self.valid
            );

            if self.valid {
                let k = (*self.iter).key();
                let data = k.data();
                let size = k.size();

                self.key_ = Slice::from_ptr_len(data, size);

                trace!(
                    "LevelDBIteratorWrapper::update: cached key from underlying iterator (data={:?}, size={})",
                    data,
                    size
                );
            }
        }
    }
}
