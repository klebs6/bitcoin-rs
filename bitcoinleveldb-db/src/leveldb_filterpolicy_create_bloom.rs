// ---------------- [ File: bitcoinleveldb-db/src/leveldb_filterpolicy_create_bloom.rs ]
crate::ix!();

struct BitcoinleveldbDbBuiltinBloomFilterPolicyState {
    policy: Box<dyn bitcoinleveldb_filter::FilterPolicy>,
    name_z: Vec<u8>,
}

fn bitcoinleveldb_db__builtin_bloom_filterpolicy_destructor(state: *mut c_void) {
    if state.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(
            state as *mut BitcoinleveldbDbBuiltinBloomFilterPolicyState
        ));
    }
}

fn bitcoinleveldb_db__builtin_bloom_filterpolicy_name(state: *mut c_void) -> *const u8 {
    if state.is_null() {
        return core::ptr::null();
    }

    unsafe {
        let st =
            &*(state as *const BitcoinleveldbDbBuiltinBloomFilterPolicyState);
        st.name_z.as_ptr()
    }
}

fn bitcoinleveldb_db__builtin_bloom_filterpolicy_create(
    state: *mut c_void,
    key_array: *const *const u8,
    key_length_array: *const usize,
    num_keys: i32,
    filter_length: *mut usize,
) -> *mut u8 {
    unsafe {
        if !filter_length.is_null() {
            *filter_length = 0usize;
        }

        if state.is_null() {
            return core::ptr::null_mut();
        }

        if num_keys > 0 && (key_array.is_null() || key_length_array.is_null()) {
            error!(
                target: "bitcoinleveldb_db::c_api",
                num_keys = num_keys,
                "builtin bloom create received null key arrays with nonzero num_keys"
            );
            return core::ptr::null_mut();
        }

        let st =
            &mut *(state as *mut BitcoinleveldbDbBuiltinBloomFilterPolicyState);

        let n = if num_keys <= 0 { 0usize } else { num_keys as usize };
        let mut keys: Vec<Slice> = Vec::with_capacity(n);

        for i in 0..n {
            let kp: *const u8 = *key_array.add(i);
            let kl: usize = *key_length_array.add(i);
            keys.push(Slice::from_ptr_len(kp, kl));
        }

        let mut dst: Vec<u8> = Vec::new();
        st.policy.create_filter(keys.as_ptr(), num_keys, &mut dst);

        if !filter_length.is_null() {
            *filter_length = dst.len();
        }

        if dst.is_empty() {
            return core::ptr::null_mut();
        }

        let out = libc::malloc(dst.len()) as *mut u8;
        if out.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                len = dst.len(),
                "builtin bloom create malloc failed"
            );
            if !filter_length.is_null() {
                *filter_length = 0usize;
            }
            return core::ptr::null_mut();
        }

        core::ptr::copy_nonoverlapping(dst.as_ptr(), out, dst.len());
        out
    }
}

fn bitcoinleveldb_db__builtin_bloom_filterpolicy_key_may_match(
    state: *mut c_void,
    key_: *const u8,
    length: usize,
    filter: *const u8,
    filter_length: usize,
) -> u8 {
    if state.is_null() {
        return 0u8;
    }

    unsafe {
        let st =
            &*(state as *const BitcoinleveldbDbBuiltinBloomFilterPolicyState);

        let key_slice = Slice::from_ptr_len(key_, length);
        let filter_slice = Slice::from_ptr_len(filter, filter_length);

        if st.policy.key_may_match(&key_slice, &filter_slice) {
            1u8
        } else {
            0u8
        }
    }
}

pub fn leveldb_filterpolicy_create_bloom(bits_per_key_: i32) -> *mut LevelDBFilterPolicy {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        bits_per_key = bits_per_key_,
        "leveldb_filterpolicy_create_bloom entry"
    );

    let policy: Box<dyn FilterPolicy> =
        new_bloom_filter_policy(bits_per_key_);

    let mut name_z = policy.name().into_owned().into_bytes();
    name_z.push(0u8);

    let state = Box::new(BitcoinleveldbDbBuiltinBloomFilterPolicyState {
        policy,
        name_z,
    });

    let state_ptr = Box::into_raw(state) as *mut c_void;

    let p = crate::leveldb_filterpolicy::leveldb_filterpolicy_create(
        state_ptr,
        bitcoinleveldb_db__builtin_bloom_filterpolicy_destructor,
        bitcoinleveldb_db__builtin_bloom_filterpolicy_create,
        bitcoinleveldb_db__builtin_bloom_filterpolicy_key_may_match,
        bitcoinleveldb_db__builtin_bloom_filterpolicy_name,
    );

    if p.is_null() {
        unsafe {
            drop(Box::from_raw(
                state_ptr as *mut BitcoinleveldbDbBuiltinBloomFilterPolicyState
            ));
        }
        error!(
            target: "bitcoinleveldb_db::c_api",
            "leveldb_filterpolicy_create_bloom failed"
        );
        return core::ptr::null_mut();
    }

    trace!(
        target: "bitcoinleveldb_db::c_api",
        ptr = (p as usize),
        "leveldb_filterpolicy_create_bloom exit"
    );

    p
}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_filterpolicy_create_bloom_rs__exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_filterpolicy_create_bloom_rs__create_roundtrip_has_builtin_name_and_matches_inserted_key() {
        unsafe {
            let policy: *mut LevelDBFilterPolicy = leveldb_filterpolicy_create_bloom(10);
            assert!(!policy.is_null());

            let pref: &LevelDBFilterPolicy = &*policy;
            let name = Named::name(pref);
            assert_eq!(name.as_ref(), "leveldb.BuiltinBloomFilter2");

            let key_buf = b"alpha";
            let key = Slice::from_ptr_len(key_buf.as_ptr(), key_buf.len());
            let key2 = Slice::from_ptr_len(key_buf.as_ptr(), key_buf.len());
            let keys = vec![key];

            let mut filter = Vec::<u8>::new();
            CreateFilter::create_filter(pref, keys.as_ptr(), 1, &mut filter);
            assert!(!filter.is_empty());

            let filter_slice = Slice::from_ptr_len(filter.as_ptr(), filter.len());
            assert!(KeyMayMatch::key_may_match(pref, &key2, &filter_slice));

            crate::leveldb_filterpolicy::leveldb_filterpolicy_destroy(policy);
        }
    }
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
