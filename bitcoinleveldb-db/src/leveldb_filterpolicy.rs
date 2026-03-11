// ---------------- [ File: bitcoinleveldb-db/src/leveldb_filterpolicy.rs ]
crate::ix!();

pub fn leveldb_filterpolicy_create(
    state: *mut core::ffi::c_void,
    destructor: fn(_0: *mut core::ffi::c_void),
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
    trace!(
        target: "bitcoinleveldb_db::c_api",
        state_is_null = state.is_null(),
        "leveldb_filterpolicy_create entry"
    );

    let mut binding = LevelDBFilterPolicyBuilder::default();

    let builder = binding
        .state(state)
        .destructor(destructor)
        .name(name)
        .create(create_filter)
        .key_match(key_may_match);

    let built = match builder.build() {
        Ok(policy) => policy,
        Err(_) => {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_filterpolicy_create builder failed"
            );
            return core::ptr::null_mut();
        }
    };

    let result = Arc::new(built);
    let p = Arc::into_raw(result) as *mut LevelDBFilterPolicy;

    trace!(
        target: "bitcoinleveldb_db::c_api",
        ptr = (p as usize),
        "leveldb_filterpolicy_create exit"
    );
    p
}

pub fn leveldb_filterpolicy_destroy(filter: *mut LevelDBFilterPolicy) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        filter_is_null = filter.is_null(),
        "leveldb_filterpolicy_destroy entry"
    );

    unsafe {
        if filter.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_filterpolicy_destroy called with null filter"
            );
            return;
        }

        drop(Arc::from_raw(filter as *const LevelDBFilterPolicy));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_filterpolicy_destroy exit");
}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_filterpolicy_create_bloom_rs__exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_filterpolicy_create_bloom_rs__builtin_bloom_policy_is_constructible_and_usable() {
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

            leveldb_filterpolicy_destroy(policy);
        }
    }
}
