// ---------------- [ File: bitcoinleveldb-db/src/leveldb_comparator.rs ]
crate::ix!();

pub fn leveldb_comparator_create(
    state: *mut core::ffi::c_void,
    destructor: fn(_0: *mut core::ffi::c_void) -> core::ffi::c_void,
    compare: fn(_0: *mut core::ffi::c_void, a: *const u8, alen: usize, b: *const u8, blen: usize) -> i32,
    name: fn(_0: *mut core::ffi::c_void) -> *const u8,
) -> *mut LevelDBComparator {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_comparator_create entry"; "state_is_null" => state.is_null());

    let result = Arc::new(LevelDBComparator {
        state,
        destructor,
        compare,
        name,
    });

    let p = Arc::into_raw(result) as *mut LevelDBComparator;

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_comparator_create exit"; "ptr" => (p as usize));
    p

    /*
        leveldb_comparator_t* result = new leveldb_comparator_t;
      result->state_ = state;
      result->destructor_ = destructor;
      result->compare_ = compare;
      result->name_ = name;
      return result;
    */
}

pub fn leveldb_comparator_destroy(cmp: *mut LevelDBComparator) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_comparator_destroy entry"; "cmp_is_null" => cmp.is_null());

    unsafe {
        if cmp.is_null() {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_comparator_destroy called with null cmp");
            return;
        }

        drop(Arc::from_raw(cmp as *const LevelDBComparator));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_comparator_destroy exit");

    /*
        delete cmp;
    */
}
