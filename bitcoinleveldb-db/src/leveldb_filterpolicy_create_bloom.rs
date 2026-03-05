// ---------------- [ File: bitcoinleveldb-db/src/leveldb_filterpolicy_create_bloom.rs ]
crate::ix!();

pub fn leveldb_filterpolicy_create_bloom(bits_per_key_: i32) -> *mut LevelDBFilterPolicy {
    
    todo!();
        /*
            // Make a leveldb_filterpolicy_t, but override all of its methods so
          // they delegate to a NewBloomFilterPolicy() instead of user
          // supplied C functions.
          struct Wrapper : public leveldb_filterpolicy_t {
            static c_void DoNothing(c_void*) {}

            ~Wrapper() { delete rep_; }
            const char* Name() const { return rep_->Name(); }
            c_void CreateFilter(const Slice* keys, int n, std::string* dst) const {
              return rep_->CreateFilter(keys, n, dst);
            }
            bool KeyMayMatch(const Slice& key, const Slice& filter) const {
              return rep_->KeyMayMatch(key, filter);
            }

            const FilterPolicy* rep_;
          };
          Wrapper* wrapper = new Wrapper;
          wrapper->rep_ = NewBloomFilterPolicy(bits_per_key);
          wrapper->state_ = nullptr;
          wrapper->destructor_ = &Wrapper::DoNothing;
          return wrapper;
        */
}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_filterpolicy_rs__exhaustive_test_suite {
    use super::*;

    struct BitcoinleveldbDbLeveldbFilterpolicyRsDropProbeState {
        dropped_count: Rc<RefCell<usize>>,
    }

    fn bitcoinleveldb_db__leveldb_filterpolicy_rs__probe_destructor(state: *mut c_void) {
        if state.is_null() {
            return;
        }

        unsafe {
            let boxed: Box<BitcoinleveldbDbLeveldbFilterpolicyRsDropProbeState> =
                Box::from_raw(state as *mut BitcoinleveldbDbLeveldbFilterpolicyRsDropProbeState);

            {
                let mut dropped = boxed.dropped_count.borrow_mut();
                *dropped = *dropped + 1;
            }

            drop(boxed);
        }
    }

    fn bitcoinleveldb_db__leveldb_filterpolicy_rs__probe_name(_state: *mut c_void) -> *const u8 {
        b"bitcoinleveldb_db__leveldb_filterpolicy_rs__probe\0".as_ptr()
    }

    fn bitcoinleveldb_db__leveldb_filterpolicy_rs__probe_create(
        _state: *mut c_void,
        _key_array: *const *const u8,
        _key_length_array: *const usize,
        _num_keys: i32,
        filter_length: *mut usize,
    ) -> *mut u8 {
        unsafe {
            if !filter_length.is_null() {
                *filter_length = 0usize;
            }
        }
        core::ptr::null_mut()
    }

    fn bitcoinleveldb_db__leveldb_filterpolicy_rs__probe_key_may_match(
        _state: *mut c_void,
        _key_: *const u8,
        _length: usize,
        _filter: *const u8,
        _filter_length: usize,
    ) -> u8 {
        1u8
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_filterpolicy_rs__destroy_null_is_safe() {
        unsafe {
            leveldb_filterpolicy_destroy(core::ptr::null_mut());
        }
        assert!(true);
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_filterpolicy_rs__filter_policy_is_dropped_only_after_last_arc_reference() {
        unsafe {
            let dropped_count: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));

            let state = Box::new(BitcoinleveldbDbLeveldbFilterpolicyRsDropProbeState {
                dropped_count: dropped_count.clone(),
            });

            let state_ptr: *mut c_void = Box::into_raw(state) as *mut c_void;

            let policy: *mut LevelDBFilterPolicy = leveldb_filterpolicy_create(
                state_ptr,
                bitcoinleveldb_db__leveldb_filterpolicy_rs__probe_destructor,
                bitcoinleveldb_db__leveldb_filterpolicy_rs__probe_create,
                bitcoinleveldb_db__leveldb_filterpolicy_rs__probe_key_may_match,
                bitcoinleveldb_db__leveldb_filterpolicy_rs__probe_name,
            );

            assert!(!policy.is_null());

            let opt: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!opt.is_null());

            crate::leveldb_options::leveldb_options_set_filter_policy(opt, policy);

            leveldb_filterpolicy_destroy(policy);

            {
                let dropped = dropped_count.borrow();
                assert_eq!(*dropped, 0usize);
            }

            crate::leveldb_options::leveldb_options_destroy(opt);

            {
                let dropped = dropped_count.borrow();
                assert_eq!(*dropped, 1usize);
            }
        }
    }
}
