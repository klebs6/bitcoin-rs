// ---------------- [ File: bitcoinleveldb-iterator/src/key_converting_iterator.rs ]
crate::ix!();

/**
  | A helper class that converts internal
  | format keys into user keys
  |
  */
pub struct KeyConvertingIterator {
    base:   LevelDBIterator,
    status: RefCell<Status>,
    iter:   *mut LevelDBIterator,
}

impl Drop for KeyConvertingIterator {
    fn drop(&mut self) {
        todo!();
        /*
            delete iter_;
        */
    }
}

impl KeyConvertingIterator {

    pub fn new(iter: *mut LevelDBIterator) -> Self {
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
   
    pub fn valid(&self) -> bool {
        trace!(
            "KeyConvertingIterator::valid: delegating to underlying iter={:?}",
            self.iter
        );

        unsafe {
            if self.iter.is_null() {
                trace!(
                    "KeyConvertingIterator::valid: underlying iterator pointer is null -> false"
                );
                false
            } else {
                let v = (*self.iter).valid();
                trace!(
                    "KeyConvertingIterator::valid: underlying valid={}",
                    v
                );
                v
            }
        }
    }
   
    pub fn seek(&mut self, target: &Slice) {
        trace!(
            "KeyConvertingIterator::seek: target={:?}, iter={:?}",
            target,
            self.iter
        );

        // This mirrors:
        //
        //   ParsedInternalKey ikey(target, kMaxSequenceNumber, kTypeValue);
        //   std::string encoded;
        //   AppendInternalKey(&encoded, ikey);
        //   iter_->Seek(encoded);
        //
        let max_sequence: SequenceNumber = ((0x1u64 << 56) - 1);
        let ikey = ParsedInternalKey::new(target, &max_sequence, ValueType::TypeValue);

        let mut encoded = String::new();
        unsafe {
            append_internal_key(&mut encoded as *mut String, &ikey);
        }

        let encoded_slice = Slice::from(&encoded);

        unsafe {
            assert!(
                !self.iter.is_null(),
                "KeyConvertingIterator::seek: underlying iterator pointer is null"
            );
            (*self.iter).seek(&encoded_slice);
        }
    }

    pub fn seek_to_first(&mut self) {
        trace!(
            "KeyConvertingIterator::seek_to_first: delegating to iter={:?}",
            self.iter
        );

        unsafe {
            assert!(
                !self.iter.is_null(),
                "KeyConvertingIterator::seek_to_first: underlying iterator pointer is null"
            );
            (*self.iter).seek_to_first();
        }
    }

    pub fn seek_to_last(&mut self) {
        trace!(
            "KeyConvertingIterator::seek_to_last: delegating to iter={:?}",
            self.iter
        );

        unsafe {
            assert!(
                !self.iter.is_null(),
                "KeyConvertingIterator::seek_to_last: underlying iterator pointer is null"
            );
            (*self.iter).seek_to_last();
        }
    }
 
    pub fn next(&mut self) {
        trace!(
            "KeyConvertingIterator::next: delegating to iter={:?}",
            self.iter
        );

        unsafe {
            assert!(
                !self.iter.is_null(),
                "KeyConvertingIterator::next: underlying iterator pointer is null"
            );
            (*self.iter).next();
        }
    }

    pub fn prev(&mut self) {
        trace!(
            "KeyConvertingIterator::prev: delegating to iter={:?}",
            self.iter
        );

        unsafe {
            assert!(
                !self.iter.is_null(),
                "KeyConvertingIterator::prev: underlying iterator pointer is null"
            );
            (*self.iter).prev();
        }
    }

    pub fn key(&self) -> Slice {
        trace!(
            "KeyConvertingIterator::key: called; iter={:?}",
            self.iter
        );

        assert!(
            self.valid(),
            "KeyConvertingIterator::key requires the iterator to be valid"
        );

        unsafe {
            assert!(
                !self.iter.is_null(),
                "KeyConvertingIterator::key: underlying iterator pointer is null"
            );

            let internal_key = (*self.iter).key();
            trace!(
                "KeyConvertingIterator::key: underlying internal key slice={:?}",
                internal_key
            );

            let mut parsed = ParsedInternalKey::default();
            let ok = parse_internal_key(
                &internal_key,
                &mut parsed as *mut ParsedInternalKey,
            );

            if !ok {
                trace!(
                    "KeyConvertingIterator::key: ParseInternalKey failed; marking status as corruption"
                );

                let msg_slice = Slice::from("malformed internal key");
                let st = Status::corruption(&msg_slice, None);
                *self.status.borrow_mut() = st;

                let corrupted = Slice::from("corrupted key");
                trace!(
                    "KeyConvertingIterator::key: returning synthetic corrupted key slice={:?}",
                    corrupted
                );
                corrupted
            } else {
                let user_key_ref: &Slice = parsed.user_key();
                let data = user_key_ref.data();
                let size = user_key_ref.size();
                let user_key = Slice::from_ptr_len(data, size);

                trace!(
                    "KeyConvertingIterator::key: returning parsed user key slice (data={:?}, size={})",
                    data,
                    size
                );

                user_key
            }
        }
    }

    pub fn value(&self) -> Slice {
        trace!(
            "KeyConvertingIterator::value: delegating to iter={:?}",
            self.iter
        );

        unsafe {
            assert!(
                !self.iter.is_null(),
                "KeyConvertingIterator::value: underlying iterator pointer is null"
            );
            let v = (*self.iter).value();
            trace!(
                "KeyConvertingIterator::value: underlying value slice={:?}",
                v
            );
            v
        }
    }
   
    pub fn status(&self) -> crate::Status {
        trace!(
            "KeyConvertingIterator::status: evaluating cached vs underlying status; iter={:?}",
            self.iter
        );

        let cached = self.status.borrow();

        if cached.is_ok() {
            unsafe {
                assert!(
                    !self.iter.is_null(),
                    "KeyConvertingIterator::status: underlying iterator pointer is null"
                );
                let st = (*self.iter).status();
                trace!(
                    "KeyConvertingIterator::status: cached OK; returning underlying status_code={:?}",
                    st.code()
                );
                st
            }
        } else {
            trace!(
                "KeyConvertingIterator::status: cached non‑OK status_code={:?}; returning cached status",
                cached.code()
            );
            Status::new_from_other_copy(&*cached)
        }
    }
}
