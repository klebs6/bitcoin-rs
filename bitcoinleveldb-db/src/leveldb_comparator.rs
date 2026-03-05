// ---------------- [ File: bitcoinleveldb-db/src/leveldb_comparator.rs ]
crate::ix!();

pub fn leveldb_comparator_create(
    state: *mut core::ffi::c_void,
    destructor: fn(_0: *mut core::ffi::c_void),
    compare: fn(_0: *mut core::ffi::c_void, a: *const u8, alen: usize, b: *const u8, blen: usize) -> i32,
    name: fn(_0: *mut core::ffi::c_void) -> *const u8,
) -> *mut LevelDBComparator {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        state_is_null = state.is_null(),
        "leveldb_comparator_create entry"
    );

    let result = Arc::new(LevelDBComparatorBuilder::default()
        .state(state)
        .destructor(destructor)
        .compare(compare)
        .name(name)
        .build()
        .unwrap()
    );

    let p = Arc::into_raw(result) as *mut LevelDBComparator;

    trace!(
        target: "bitcoinleveldb_db::c_api",
        ptr = (p as usize),
        "leveldb_comparator_create exit"
    );
    p
}

pub fn leveldb_comparator_destroy(cmp: *mut LevelDBComparator) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        cmp_is_null = cmp.is_null(),
        "leveldb_comparator_destroy entry"
    );

    unsafe {
        if cmp.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_comparator_destroy called with null cmp"
            );
            return;
        }

        drop(Arc::from_raw(cmp as *const LevelDBComparator));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_comparator_destroy exit");
}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_comparator_rs__exhaustive_test_suite {
    use super::*;

    struct BitcoinleveldbDbLeveldbComparatorRsDropProbeState {
        dropped_count: Rc<RefCell<usize>>,
    }

    fn bitcoinleveldb_db__leveldb_comparator_rs__probe_destructor(state: *mut c_void) {
        if state.is_null() {
            return;
        }

        unsafe {
            let boxed: Box<BitcoinleveldbDbLeveldbComparatorRsDropProbeState> =
                Box::from_raw(state as *mut BitcoinleveldbDbLeveldbComparatorRsDropProbeState);

            {
                let mut dropped = boxed.dropped_count.borrow_mut();
                *dropped = *dropped + 1;
            }

            drop(boxed);
        }
    }

    fn bitcoinleveldb_db__leveldb_comparator_rs__probe_compare(
        _state: *mut c_void,
        _a: *const u8,
        _alen: usize,
        _b: *const u8,
        _blen: usize,
    ) -> i32 {
        0
    }

    fn bitcoinleveldb_db__leveldb_comparator_rs__probe_name(_state: *mut c_void) -> *const u8 {
        b"bitcoinleveldb_db__leveldb_comparator_rs__probe\0".as_ptr()
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_comparator_rs__create_destroy_null_is_safe() {
        unsafe {
            leveldb_comparator_destroy(core::ptr::null_mut());
        }
        assert!(true);
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_comparator_rs__comparator_is_dropped_only_after_last_arc_reference() {
        unsafe {
            let dropped_count: Rc<RefCell<usize>> = Rc::new(RefCell::new(0usize));

            let state = Box::new(BitcoinleveldbDbLeveldbComparatorRsDropProbeState {
                dropped_count: dropped_count.clone(),
            });

            let state_ptr: *mut c_void = Box::into_raw(state) as *mut c_void;

            let cmp: *mut LevelDBComparator = leveldb_comparator_create(
                state_ptr,
                bitcoinleveldb_db__leveldb_comparator_rs__probe_destructor,
                bitcoinleveldb_db__leveldb_comparator_rs__probe_compare,
                bitcoinleveldb_db__leveldb_comparator_rs__probe_name,
            );

            assert!(!cmp.is_null());

            let opt: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!opt.is_null());

            crate::leveldb_options::leveldb_options_set_comparator(opt, cmp);

            leveldb_comparator_destroy(cmp);

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
