// ---------------- [ File: bitcoinleveldb-db/src/copy_string.rs ]
crate::ix!();

pub fn copy_string(str_: &String) -> *mut u8 {
    let len = str_.as_bytes().len();
    trace!(target: "bitcoinleveldb_db::c_api", len = len, "CopyString entry");

    unsafe {
        let result = libc::malloc(len) as *mut u8;

        if result.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                len = len,
                "CopyString malloc failed"
            );
            return core::ptr::null_mut();
        }

        if len > 0 {
            core::ptr::copy_nonoverlapping(str_.as_bytes().as_ptr(), result, len);
        }

        trace!(
            target: "bitcoinleveldb_db::c_api",
            ptr_is_null = result.is_null(),
            "CopyString exit"
        );
        result
    }
}

#[cfg(test)]
mod bitcoinleveldb_db__copy_string_rs__exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn bitcoinleveldb_db__copy_string_rs__copy_string_copies_exact_bytes_for_non_empty_input() {
        let mut s: String = String::from("hello\0world");
        s.retain(|c| c != '\0');

        let len: usize = s.as_bytes().len();
        assert!(len > 0);

        let out: *mut u8 = copy_string(&s);
        assert!(!out.is_null());

        unsafe {
            let bytes: &[u8] = core::slice::from_raw_parts(out as *const u8, len);
            assert_eq!(bytes, s.as_bytes());
        }

        crate::leveldb_free::leveldb_free(out as *mut core::ffi::c_void);

        s.push('!');
    }

    #[traced_test]
    fn bitcoinleveldb_db__copy_string_rs__copy_string_empty_string_is_safe_and_freeable_if_non_null() {
        let s: String = String::new();
        let out: *mut u8 = copy_string(&s);

        if out.is_null() {
            assert!(true);
        } else {
            crate::leveldb_free::leveldb_free(out as *mut core::ffi::c_void);
            assert!(true);
        }
    }
}
