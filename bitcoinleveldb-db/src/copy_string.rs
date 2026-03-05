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
