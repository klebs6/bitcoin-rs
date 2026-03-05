// ---------------- [ File: bitcoinleveldb-db/src/comparator.rs ]
crate::ix!();

#[derive(Builder,Getters)]
#[getset(get="pub")]
pub struct LevelDBComparator {
    state:      *mut c_void,
    destructor: fn(_0: *mut c_void),
    compare:    fn(
        _0:   *mut c_void,
        a:    *const u8,
        alen: usize,
        b:    *const u8,
        blen: usize,
    ) -> i32,
    name:       fn(_0: *mut c_void) -> *const u8,
}

impl bitcoinleveldb_comparator::Compare for LevelDBComparator {
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        (self.compare)(
            self.state,
            *a.data(),
            *a.size(),
            *b.data(),
            *b.size(),
        )
    }
}

impl bitcoinleveldb_comparator::FindShortSuccessor for LevelDBComparator {
    fn find_short_successor(&self, _key: &mut Vec<u8>) {
        trace!(
            target: "bitcoinleveldb_db::c_api",
            "LevelDBComparator::find_short_successor noop"
        );
    }
}

impl Comparator<Slice> for LevelDBComparator {
    fn compare(&self, a: &Slice, b: &Slice) -> core::cmp::Ordering {
        trace!(
            target: "bitcoinleveldb_db::c_api",
            alen = *a.size(),
            blen = *b.size(),
            "LevelDBComparator::compare entry"
        );

        let r = (self.compare)(
            self.state,
            *a.data(),
            *a.size(),
            *b.data(),
            *b.size(),
        );

        let ord = if r < 0 {
            core::cmp::Ordering::Less
        } else if r > 0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        };

        trace!(
            target: "bitcoinleveldb_db::c_api",
            result = r,
            ordering = ?ord,
            "LevelDBComparator::compare exit"
        );

        ord
    }

}

impl FindShortestSeparator for LevelDBComparator {
    fn find_shortest_separator(&self, _0: &mut Vec<u8>, _1: &[u8]) {
        trace!(
            target: "bitcoinleveldb_db::c_api",
            "LevelDBComparator::find_shortest_separator noop"
        );
    }
}

impl Drop for LevelDBComparator {
    fn drop(&mut self) {
        trace!(target: "bitcoinleveldb_db::c_api", "LevelDBComparator::drop entry");
        (self.destructor)(self.state);
        trace!(target: "bitcoinleveldb_db::c_api", "LevelDBComparator::drop exit");
    }
}

impl Named for LevelDBComparator {
    fn name(&self) -> Cow<'_, str> {
        trace!(target: "bitcoinleveldb_db::c_api", "LevelDBComparator::name entry");
        let p = (self.name)(self.state);
        trace!(
            target: "bitcoinleveldb_db::c_api",
            ptr_is_null = p.is_null(),
            "LevelDBComparator::name exit"
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


impl bitcoinleveldb_comparator::SliceComparator for LevelDBComparator {
    fn bytewise_comparator(&self) -> *const dyn bitcoinleveldb_comparator::SliceComparator {
        bitcoinleveldb_comparator::bytewise_comparator()
    }
}

#[cfg(test)]
mod bitcoinleveldb_db__comparator_rs__exhaustive_test_suite {
    use super::*;

    struct BitcoinleveldbDbComparatorRsCallbackState {
        dropped_count:    Rc<RefCell<usize>>,
        compare_calls:    Rc<RefCell<usize>>,
        last_a:           Rc<RefCell<Vec<u8>>>,
        last_b:           Rc<RefCell<Vec<u8>>>,
        name_calls:       Rc<RefCell<usize>>,
        return_null_name: bool,
    }

    fn bitcoinleveldb_db__comparator_rs__callback_destructor(state: *mut c_void) {
        if state.is_null() {
            return;
        }

        unsafe {
            let boxed: Box<BitcoinleveldbDbComparatorRsCallbackState> =
                Box::from_raw(state as *mut BitcoinleveldbDbComparatorRsCallbackState);

            {
                let mut dropped = boxed.dropped_count.borrow_mut();
                *dropped = *dropped + 1;
            }

            drop(boxed);
        }
    }

    fn bitcoinleveldb_db__comparator_rs__callback_compare(
        state: *mut c_void,
        a: *const u8,
        alen: usize,
        b: *const u8,
        blen: usize,
    ) -> i32 {
        if state.is_null() {
            return 0;
        }

        unsafe {
            let st: &mut BitcoinleveldbDbComparatorRsCallbackState =
                &mut *(state as *mut BitcoinleveldbDbComparatorRsCallbackState);

            {
                let mut calls = st.compare_calls.borrow_mut();
                *calls = *calls + 1;
            }

            let a_bytes: &[u8] = if a.is_null() && alen == 0 {
                &[]
            } else {
                core::slice::from_raw_parts(a, alen)
            };

            let b_bytes: &[u8] = if b.is_null() && blen == 0 {
                &[]
            } else {
                core::slice::from_raw_parts(b, blen)
            };

            {
                let mut last_a = st.last_a.borrow_mut();
                last_a.clear();
                last_a.extend_from_slice(a_bytes);
            }

            {
                let mut last_b = st.last_b.borrow_mut();
                last_b.clear();
                last_b.extend_from_slice(b_bytes);
            }

            let ord: core::cmp::Ordering = a_bytes.cmp(b_bytes);

            match ord {
                core::cmp::Ordering::Less => -1,
                core::cmp::Ordering::Equal => 0,
                core::cmp::Ordering::Greater => 1,
            }
        }
    }

    fn bitcoinleveldb_db__comparator_rs__callback_name(state: *mut c_void) -> *const u8 {
        if state.is_null() {
            return core::ptr::null();
        }

        unsafe {
            let st: &mut BitcoinleveldbDbComparatorRsCallbackState =
                &mut *(state as *mut BitcoinleveldbDbComparatorRsCallbackState);

            {
                let mut calls = st.name_calls.borrow_mut();
                *calls = *calls + 1;
            }

            if st.return_null_name {
                core::ptr::null()
            } else {
                b"bitcoinleveldb_db__comparator_rs__test_comparator\0".as_ptr()
            }
        }
    }

    fn bitcoinleveldb_db__comparator_rs__build_comparator_or_panic(
        state_ptr: *mut c_void,
    ) -> LevelDBComparator {
        let mut binding = LevelDBComparatorBuilder::default();

        let builder = binding
            .state(state_ptr)
            .destructor(bitcoinleveldb_db__comparator_rs__callback_destructor)
            .compare(bitcoinleveldb_db__comparator_rs__callback_compare)
            .name(bitcoinleveldb_db__comparator_rs__callback_name);

        match builder.build() {
            Ok(v) => v,
            Err(_) => {
                panic!();
            }
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__comparator_rs__compare_records_inputs_and_maps_sign_to_ordering() {
        let dropped_count: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let compare_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let last_a: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let last_b: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let name_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));

        let state = Box::new(BitcoinleveldbDbComparatorRsCallbackState {
            dropped_count: dropped_count.clone(),
            compare_calls: compare_calls.clone(),
            last_a: last_a.clone(),
            last_b: last_b.clone(),
            name_calls: name_calls.clone(),
            return_null_name: false,
        });

        let state_ptr: *mut c_void = Box::into_raw(state) as *mut c_void;

        let cmp: LevelDBComparator =
            bitcoinleveldb_db__comparator_rs__build_comparator_or_panic(state_ptr);

        let a_bytes: Vec<u8> = vec![b'a', b'1'];
        let b_bytes: Vec<u8> = vec![b'a', b'2'];

        let a_slice: Slice = Slice::from_ptr_len(a_bytes.as_ptr(), a_bytes.len());
        let b_slice: Slice = Slice::from_ptr_len(b_bytes.as_ptr(), b_bytes.len());

        let r_i32: i32 = bitcoinleveldb_comparator::Compare::compare(&cmp, &a_slice, &b_slice);
        assert!(r_i32 < 0);

        let ord: core::cmp::Ordering = Comparator::<Slice>::compare(&cmp, &a_slice, &b_slice);
        match ord {
            core::cmp::Ordering::Less => {}
            core::cmp::Ordering::Equal => {
                panic!();
            }
            core::cmp::Ordering::Greater => {
                panic!();
            }
        }

        {
            let calls = compare_calls.borrow();
            assert_eq!(*calls, 1usize);
        }

        {
            let la = last_a.borrow();
            assert_eq!(la.as_slice(), a_bytes.as_slice());
        }

        {
            let lb = last_b.borrow();
            assert_eq!(lb.as_slice(), b_bytes.as_slice());
        }

        drop(cmp);

        {
            let dropped = dropped_count.borrow();
            assert_eq!(*dropped, 1usize);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__comparator_rs__named_returns_empty_when_callback_returns_null_ptr() {
        let dropped_count: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let compare_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let last_a: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let last_b: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let name_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));

        let state = Box::new(BitcoinleveldbDbComparatorRsCallbackState {
            dropped_count: dropped_count.clone(),
            compare_calls: compare_calls.clone(),
            last_a: last_a.clone(),
            last_b: last_b.clone(),
            name_calls: name_calls.clone(),
            return_null_name: true,
        });

        let state_ptr: *mut c_void = Box::into_raw(state) as *mut c_void;

        let cmp: LevelDBComparator =
            bitcoinleveldb_db__comparator_rs__build_comparator_or_panic(state_ptr);

        let name: Cow<'_, str> = Named::name(&cmp);
        assert_eq!(name.as_ref(), "");

        {
            let calls = name_calls.borrow();
            assert_eq!(*calls, 1usize);
        }

        drop(cmp);

        {
            let dropped = dropped_count.borrow();
            assert_eq!(*dropped, 1usize);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__comparator_rs__named_reads_c_string_when_callback_returns_non_null_ptr() {
        let dropped_count: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let compare_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let last_a: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let last_b: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let name_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));

        let state = Box::new(BitcoinleveldbDbComparatorRsCallbackState {
            dropped_count: dropped_count.clone(),
            compare_calls: compare_calls.clone(),
            last_a: last_a.clone(),
            last_b: last_b.clone(),
            name_calls: name_calls.clone(),
            return_null_name: false,
        });

        let state_ptr: *mut c_void = Box::into_raw(state) as *mut c_void;

        let cmp: LevelDBComparator =
            bitcoinleveldb_db__comparator_rs__build_comparator_or_panic(state_ptr);

        let name: Cow<'_, str> = Named::name(&cmp);
        assert_eq!(name.as_ref(), "bitcoinleveldb_db__comparator_rs__test_comparator");

        {
            let calls = name_calls.borrow();
            assert_eq!(*calls, 1usize);
        }

        drop(cmp);

        {
            let dropped = dropped_count.borrow();
            assert_eq!(*dropped, 1usize);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__comparator_rs__find_short_successor_and_separator_are_noops() {
        let dropped_count: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let compare_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));
        let last_a: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let last_b: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let name_calls: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));

        let state = Box::new(BitcoinleveldbDbComparatorRsCallbackState {
            dropped_count: dropped_count.clone(),
            compare_calls: compare_calls.clone(),
            last_a: last_a.clone(),
            last_b: last_b.clone(),
            name_calls: name_calls.clone(),
            return_null_name: false,
        });

        let state_ptr: *mut c_void = Box::into_raw(state) as *mut c_void;

        let cmp: LevelDBComparator =
            bitcoinleveldb_db__comparator_rs__build_comparator_or_panic(state_ptr);

        let mut key: Vec<u8> = vec![b'k', b'e', b'y'];
        let before_key: Vec<u8> = key.clone();

        bitcoinleveldb_comparator::FindShortSuccessor::find_short_successor(&cmp, &mut key);
        assert_eq!(key.as_slice(), before_key.as_slice());

        let mut sep_key: Vec<u8> = vec![b'a', b'a'];
        let before_sep_key: Vec<u8> = sep_key.clone();
        let limit: &[u8] = &[b'z', b'z'];

        FindShortestSeparator::find_shortest_separator(&cmp, &mut sep_key, limit);
        assert_eq!(sep_key.as_slice(), before_sep_key.as_slice());

        let p = bitcoinleveldb_comparator::SliceComparator::bytewise_comparator(&cmp);
        assert!(!p.is_null());

        drop(cmp);

        {
            let dropped = dropped_count.borrow();
            assert_eq!(*dropped, 1usize);
        }
    }
}
