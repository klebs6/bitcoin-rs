// ---------------- [ File: bitcoinleveldb-db/src/leveldb_free.rs ]
crate::ix!();

pub fn leveldb_free(ptr: *mut core::ffi::c_void) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        ptr_is_null = ptr.is_null(),
        "leveldb_free entry"
    );

    unsafe {
        libc::free(ptr);
    }
}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_free_rs__exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_free_rs__free_null_is_safe() {
        unsafe {
            leveldb_free(core::ptr::null_mut());
        }
        assert!(true);
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_free_rs__free_malloced_pointer_is_safe() {
        unsafe {
            let p: *mut u8 = libc::malloc(8usize) as *mut u8;
            assert!(!p.is_null());

            *p.add(0) = 1u8;
            *p.add(7) = 2u8;

            leveldb_free(p as *mut core::ffi::c_void);
        }
    }
}
