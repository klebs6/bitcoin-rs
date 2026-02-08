// ---------------- [ File: bitcoinleveldb-db/src/leveldb_filterpolicy.rs ]
crate::ix!();

pub fn leveldb_filterpolicy_create(
    state: *mut core::ffi::c_void,
    destructor: fn(_0: *mut core::ffi::c_void) -> core::ffi::c_void,
    create_filter: fn(
        _0: *mut core::ffi::c_void,
        key_array: *const *const u8,
        key_length_array: *const usize,
        num_keys: i32,
        filter_length: *mut usize,
    ) -> *mut u8,
    key_may_match: fn(
        _0: *mut core::ffi::c_void,
        key_: *const u8,
        length: usize,
        filter: *const u8,
        filter_length: usize,
    ) -> u8,
    name: fn(_0: *mut core::ffi::c_void) -> *const u8,
) -> *mut LevelDBFilterPolicy {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_filterpolicy_create entry"; "state_is_null" => state.is_null());

    let result = Arc::new(LevelDBFilterPolicy {
        state,
        destructor,
        name,
        create: create_filter,
        key_match: key_may_match,
    });

    let p = Arc::into_raw(result) as *mut LevelDBFilterPolicy;

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_filterpolicy_create exit"; "ptr" => (p as usize));
    p
}

pub fn leveldb_filterpolicy_destroy(filter: *mut LevelDBFilterPolicy) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_filterpolicy_destroy entry"; "filter_is_null" => filter.is_null());

    unsafe {
        if filter.is_null() {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_filterpolicy_destroy called with null filter");
            return;
        }

        drop(Arc::from_raw(filter as *const LevelDBFilterPolicy));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_filterpolicy_destroy exit");
}
