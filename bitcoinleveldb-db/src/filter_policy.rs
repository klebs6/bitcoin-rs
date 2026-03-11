// ---------------- [ File: bitcoinleveldb-db/src/filter_policy.rs ]
crate::ix!();

#[derive(Builder,Getters)]
#[getset(get = "pub")]
pub struct LevelDBFilterPolicy {
    state:      *mut c_void,
    destructor: fn(_0: *mut c_void),
    name:       fn(_0: *mut c_void) -> *const u8,
    create:     fn(
        _0:               *mut c_void,
        key_array:        *const *const u8,
        key_length_array: *const usize,
        num_keys:         i32,
        filter_length:    *mut usize,
    ) -> *mut u8,
    key_match:  fn(
        _0:            *mut c_void,
        key_:          *const u8,
        length:        usize,
        filter:        *const u8,
        filter_length: usize,
    ) -> u8,
}


impl FilterPolicy for LevelDBFilterPolicy {}

impl Drop for LevelDBFilterPolicy {
    fn drop(&mut self) {
        trace!(target: "bitcoinleveldb_db::c_api", "LevelDBFilterPolicy::drop entry");
        (self.destructor)(self.state);
        trace!(target: "bitcoinleveldb_db::c_api", "LevelDBFilterPolicy::drop exit");
    }
}

impl Named for LevelDBFilterPolicy {
    fn name(&self) -> Cow<'_, str> {
        trace!(target: "bitcoinleveldb_db::c_api", "LevelDBFilterPolicy::name entry");
        let p = (self.name)(self.state);
        trace!(
            target: "bitcoinleveldb_db::c_api",
            ptr_is_null = p.is_null(),
            "LevelDBFilterPolicy::name exit"
        );

        if p.is_null() {
            return Cow::Borrowed("");
        }

        unsafe {
            let cstr = std::ffi::CStr::from_ptr(p as *const core::ffi::c_char);
            Cow::Owned(cstr.to_string_lossy().into_owned())
        }
    }

}

impl CreateFilter for LevelDBFilterPolicy {
    fn create_filter(&self, keys: *const Slice, n: i32, dst: &mut Vec<u8>) {
        trace!(
            target: "bitcoinleveldb_db::c_api",
            n = n,
            "LevelDBFilterPolicy::create_filter entry"
        );

        if n <= 0 {
            trace!(
                target: "bitcoinleveldb_db::c_api",
                "LevelDBFilterPolicy::create_filter early-exit (n<=0)"
            );
            return;
        }

        let n_usize: usize = n as usize;

        // Mirror the C++ allocation/loop structure.
        let mut key_pointers: Vec<*const u8> = vec![core::ptr::null(); n_usize];
        let mut key_sizes: Vec<usize> = vec![0; n_usize];

        unsafe {
            for i in 0..n_usize {
                let kref: &Slice = &*keys.add(i);
                key_pointers[i] = *kref.data();
                key_sizes[i] = *kref.size();
            }

            let mut len: usize = 0;
            let filter: *mut u8 = (self.create)(
                self.state,
                key_pointers.as_ptr(),
                key_sizes.as_ptr(),
                n,
                (&mut len) as *mut usize,
            );

            trace!(
                target: "bitcoinleveldb_db::c_api",
                len = len,
                filter_is_null = filter.is_null(),
                "LevelDBFilterPolicy::create_filter callback returned"
            );

            if len > 0 {
                if filter.is_null() {
                    error!(
                        target: "bitcoinleveldb_db::c_api",
                        len = len,
                        "LevelDBFilterPolicy::create_filter callback returned null with nonzero length"
                    );
                } else {
                    let bytes: &[u8] = core::slice::from_raw_parts(filter as *const u8, len);
                    dst.extend_from_slice(bytes);
                }
            }

            if !filter.is_null() {
                libc::free(filter as *mut core::ffi::c_void);
            }
        }

        trace!(
            target: "bitcoinleveldb_db::c_api",
            dst_len = dst.len(),
            "LevelDBFilterPolicy::create_filter exit"
        );
    }
}

impl KeyMayMatch for LevelDBFilterPolicy {
    fn key_may_match(&self, key_: &Slice, filter: &Slice) -> bool {
        trace!(
            target: "bitcoinleveldb_db::c_api",
            key_len = *key_.size(),
            filter_len = *filter.size(),
            "LevelDBFilterPolicy::key_may_match entry"
        );

        let r = (self.key_match)(
            self.state,
            *key_.data(),
            *key_.size(),
            *filter.data(),
            *filter.size(),
        );

        let ok = r != 0;

        trace!(
            target: "bitcoinleveldb_db::c_api",
            result_u8 = r,
            ok = ok,
            "LevelDBFilterPolicy::key_may_match exit"
        );

        ok
    }
}

#[cfg(test)]
mod bitcoinleveldb_db__filter_policy_rs__exhaustive_test_suite {
    use super::*;

    struct BitcoinleveldbDbFilterPolicyRsCallbackState {
        dropped_count: Rc<RefCell<usize>>,
        name_calls:    Rc<RefCell<usize>>,
        create_calls:  Rc<RefCell<usize>>,
        match_calls:   Rc<RefCell<usize>>,
        last_keys:     Rc<RefCell<Vec<Vec<u8>>>>,
        return_null:   bool,
    }

    fn bitcoinleveldb_db__filter_policy_rs__callback_destructor(state: *mut c_void) {
        if state.is_null() {
            return;
        }

        unsafe {
            let boxed: Box<BitcoinleveldbDbFilterPolicyRsCallbackState> =
                Box::from_raw(state as *mut BitcoinleveldbDbFilterPolicyRsCallbackState);

            {
                let mut dropped = boxed.dropped_count.borrow_mut();
                *dropped = *dropped + 1;
            }

            drop(boxed);
        }
    }

    fn bitcoinleveldb_db__filter_policy_rs__callback_name(state: *mut c_void) -> *const u8 {
        if state.is_null() {
            return core::ptr::null();
        }

        unsafe {
            let st: &mut BitcoinleveldbDbFilterPolicyRsCallbackState =
                &mut *(state as *mut BitcoinleveldbDbFilterPolicyRsCallbackState);

            {
                let mut calls = st.name_calls.borrow_mut();
                *calls = *calls + 1;
            }

            b"bitcoinleveldb_db__filter_policy_rs__test_policy\0".as_ptr()
        }
    }

    fn bitcoinleveldb_db__filter_policy_rs__callback_create_filter(
        state: *mut c_void,
        key_array: *const *const u8,
        key_length_array: *const usize,
        num_keys: i32,
        filter_length: *mut usize,
    ) -> *mut u8 {
        if state.is_null() {
            unsafe {
                if !filter_length.is_null() {
                    *filter_length = 0;
                }
            }
            return core::ptr::null_mut();
        }

        unsafe {
            let st: &mut BitcoinleveldbDbFilterPolicyRsCallbackState =
                &mut *(state as *mut BitcoinleveldbDbFilterPolicyRsCallbackState);

            {
                let mut calls = st.create_calls.borrow_mut();
                *calls = *calls + 1;
            }

            let mut observed: Vec<Vec<u8>> = Vec::new();

            if num_keys > 0 {
                let n: usize = num_keys as usize;
                for i in 0..n {
                    let kp: *const u8 = *key_array.add(i);
                    let kl: usize = *key_length_array.add(i);

                    let bytes: Vec<u8> = if kp.is_null() && kl == 0 {
                        Vec::new()
                    } else {
                        core::slice::from_raw_parts(kp, kl).to_vec()
                    };

                    observed.push(bytes);
                }
            }

            {
                let mut lk = st.last_keys.borrow_mut();
                lk.clear();
                lk.extend_from_slice(observed.as_slice());
            }

            if st.return_null {
                if !filter_length.is_null() {
                    *filter_length = 2usize;
                }
                return core::ptr::null_mut();
            }

            let payload: [u8; 2] = [0xABu8, 0xCDu8];

            if !filter_length.is_null() {
                *filter_length = payload.len();
            }

            let out: *mut u8 = libc::malloc(payload.len()) as *mut u8;
            if out.is_null() {
                if !filter_length.is_null() {
                    *filter_length = 0usize;
                }
                return core::ptr::null_mut();
            }

            core::ptr::copy_nonoverlapping(payload.as_ptr(), out, payload.len());
            out
        }
    }

    fn bitcoinleveldb_db__filter_policy_rs__callback_key_may_match(
        state: *mut c_void,
        key_: *const u8,
        length: usize,
        _filter: *const u8,
        _filter_length: usize,
    ) -> u8 {
        if state.is_null() {
            return 0u8;
        }

        unsafe {
            let st: &mut BitcoinleveldbDbFilterPolicyRsCallbackState =
                &mut *(state as *mut BitcoinleveldbDbFilterPolicyRsCallbackState);

            {
                let mut calls = st.match_calls.borrow_mut();
                *calls = *calls + 1;
            }

            let key_bytes: &[u8] = if key_.is_null() && length == 0 {
                &[]
            } else {
                core::slice::from_raw_parts(key_, length)
            };

            if key_bytes.is_empty() {
                0u8
            } else {
                1u8
            }
        }
    }

    fn bitcoinleveldb_db__filter_policy_rs__build_policy_or_panic(
        state_ptr: *mut c_void,
    ) -> LevelDBFilterPolicy {
        let mut binding = LevelDBFilterPolicyBuilder::default();
        let builder = binding
            .state(state_ptr)
            .destructor(bitcoinleveldb_db__filter_policy_rs__callback_destructor)
            .name(bitcoinleveldb_db__filter_policy_rs__callback_name)
            .create(bitcoinleveldb_db__filter_policy_rs__callback_create_filter)
            .key_match(bitcoinleveldb_db__filter_policy_rs__callback_key_may_match);

        match builder.build() {
            Ok(v) => v,
            Err(_) => {
                panic!();
            }
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__filter_policy_rs__name_is_stable_and_calls_callback() {
        let dropped_count: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let name_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let create_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let match_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let last_keys: Rc<RefCell<Vec<Vec<u8>>>> = Rc::new(RefCell::new(Vec::new()));

        let state = Box::new(BitcoinleveldbDbFilterPolicyRsCallbackState {
            dropped_count: dropped_count.clone(),
            name_calls: name_calls.clone(),
            create_calls: create_calls.clone(),
            match_calls: match_calls.clone(),
            last_keys: last_keys.clone(),
            return_null: false,
        });

        let state_ptr: *mut c_void = Box::into_raw(state) as *mut c_void;
        let policy: LevelDBFilterPolicy =
            bitcoinleveldb_db__filter_policy_rs__build_policy_or_panic(state_ptr);

        let n: Cow<'_, str> = Named::name(&policy);
        assert_eq!(n.as_ref(), "bitcoinleveldb_db__filter_policy_rs__test_policy");

        {
            let calls = name_calls.borrow();
            assert_eq!(*calls, 1usize);
        }

        drop(policy);

        {
            let dropped = dropped_count.borrow();
            assert_eq!(*dropped, 1usize);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__filter_policy_rs__create_filter_populates_dst_and_records_keys() {
        let dropped_count: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let name_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let create_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let match_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let last_keys: Rc<RefCell<Vec<Vec<u8>>>> = Rc::new(RefCell::new(Vec::new()));

        let state = Box::new(BitcoinleveldbDbFilterPolicyRsCallbackState {
            dropped_count: dropped_count.clone(),
            name_calls: name_calls.clone(),
            create_calls: create_calls.clone(),
            match_calls: match_calls.clone(),
            last_keys: last_keys.clone(),
            return_null: false,
        });

        let state_ptr: *mut c_void = Box::into_raw(state) as *mut c_void;
        let policy: LevelDBFilterPolicy =
            bitcoinleveldb_db__filter_policy_rs__build_policy_or_panic(state_ptr);

        let k1_buf: Vec<u8> = vec![1u8, 2u8];
        let k2_buf: Vec<u8> = vec![3u8];

        let k1: Slice = Slice::from_ptr_len(k1_buf.as_ptr(), k1_buf.len());
        let k2: Slice = Slice::from_ptr_len(k2_buf.as_ptr(), k2_buf.len());

        let keys_vec: Vec<Slice> = vec![k1, k2];
        let keys_ptr: *const Slice = keys_vec.as_ptr();

        let mut dst: Vec<u8> = Vec::new();
        CreateFilter::create_filter(&policy, keys_ptr, 2, &mut dst);

        assert_eq!(dst.as_slice(), &[0xABu8, 0xCDu8]);

        {
            let calls = create_calls.borrow();
            assert_eq!(*calls, 1usize);
        }

        {
            let observed = last_keys.borrow();
            assert_eq!(observed.len(), 2usize);
            assert_eq!(observed[0].as_slice(), k1_buf.as_slice());
            assert_eq!(observed[1].as_slice(), k2_buf.as_slice());
        }

        drop(policy);

        {
            let dropped = dropped_count.borrow();
            assert_eq!(*dropped, 1usize);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__filter_policy_rs__create_filter_early_exits_for_non_positive_n() {
        let dropped_count: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let name_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let create_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let match_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let last_keys: Rc<RefCell<Vec<Vec<u8>>>> = Rc::new(RefCell::new(Vec::new()));

        let state = Box::new(BitcoinleveldbDbFilterPolicyRsCallbackState {
            dropped_count: dropped_count.clone(),
            name_calls: name_calls.clone(),
            create_calls: create_calls.clone(),
            match_calls: match_calls.clone(),
            last_keys: last_keys.clone(),
            return_null: false,
        });

        let state_ptr: *mut c_void = Box::into_raw(state) as *mut c_void;
        let policy: LevelDBFilterPolicy =
            bitcoinleveldb_db__filter_policy_rs__build_policy_or_panic(state_ptr);

        let mut dst: Vec<u8> = vec![9u8, 9u8];
        let before: Vec<u8> = dst.clone();

        CreateFilter::create_filter(&policy, core::ptr::null(), 0, &mut dst);
        assert_eq!(dst.as_slice(), before.as_slice());

        {
            let calls = create_calls.borrow();
            assert_eq!(*calls, 0usize);
        }

        drop(policy);

        {
            let dropped = dropped_count.borrow();
            assert_eq!(*dropped, 1usize);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__filter_policy_rs__create_filter_does_not_append_when_callback_returns_null_with_nonzero_len() {
        let dropped_count: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let name_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let create_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let match_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let last_keys: Rc<RefCell<Vec<Vec<u8>>>> = Rc::new(RefCell::new(Vec::new()));

        let state = Box::new(BitcoinleveldbDbFilterPolicyRsCallbackState {
            dropped_count: dropped_count.clone(),
            name_calls: name_calls.clone(),
            create_calls: create_calls.clone(),
            match_calls: match_calls.clone(),
            last_keys: last_keys.clone(),
            return_null: true,
        });

        let state_ptr: *mut c_void = Box::into_raw(state) as *mut c_void;
        let policy: LevelDBFilterPolicy =
            bitcoinleveldb_db__filter_policy_rs__build_policy_or_panic(state_ptr);

        let k1_buf: Vec<u8> = vec![7u8];
        let k1: Slice = Slice::from_ptr_len(k1_buf.as_ptr(), k1_buf.len());
        let keys_vec: Vec<Slice> = vec![k1];

        let mut dst: Vec<u8> = vec![1u8, 2u8, 3u8];
        let before: Vec<u8> = dst.clone();

        CreateFilter::create_filter(&policy, keys_vec.as_ptr(), 1, &mut dst);
        assert_eq!(dst.as_slice(), before.as_slice());

        {
            let calls = create_calls.borrow();
            assert_eq!(*calls, 1usize);
        }

        drop(policy);

        {
            let dropped = dropped_count.borrow();
            assert_eq!(*dropped, 1usize);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__filter_policy_rs__key_may_match_maps_nonzero_to_true_and_zero_to_false() {
        let dropped_count: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let name_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let create_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let match_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let last_keys: Rc<RefCell<Vec<Vec<u8>>>> = Rc::new(RefCell::new(Vec::new()));

        let state = Box::new(BitcoinleveldbDbFilterPolicyRsCallbackState {
            dropped_count: dropped_count.clone(),
            name_calls: name_calls.clone(),
            create_calls: create_calls.clone(),
            match_calls: match_calls.clone(),
            last_keys: last_keys.clone(),
            return_null: false,
        });

        let state_ptr: *mut c_void = Box::into_raw(state) as *mut c_void;
        let policy: LevelDBFilterPolicy =
            bitcoinleveldb_db__filter_policy_rs__build_policy_or_panic(state_ptr);

        let filter_buf: Vec<u8> = vec![0u8, 1u8];
        let filter: Slice = Slice::from_ptr_len(filter_buf.as_ptr(), filter_buf.len());

        let empty_key_buf: Vec<u8> = Vec::new();
        let empty_key: Slice = Slice::from_ptr_len(empty_key_buf.as_ptr(), empty_key_buf.len());

        let nonempty_key_buf: Vec<u8> = vec![9u8];
        let nonempty_key: Slice = Slice::from_ptr_len(nonempty_key_buf.as_ptr(), nonempty_key_buf.len());

        let r0: bool = KeyMayMatch::key_may_match(&policy, &empty_key, &filter);
        assert!(!r0);

        let r1: bool = KeyMayMatch::key_may_match(&policy, &nonempty_key, &filter);
        assert!(r1);

        {
            let calls = match_calls.borrow();
            assert_eq!(*calls, 2usize);
        }

        drop(policy);

        {
            let dropped = dropped_count.borrow();
            assert_eq!(*dropped, 1usize);
        }
    }
}
