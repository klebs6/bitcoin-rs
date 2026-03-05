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
