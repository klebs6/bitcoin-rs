// ---------------- [ File: bitcoinleveldb-posixenv/src/initialize_posix_env_result_slot.rs ]
crate::ix!();

pub fn initialize_posix_env_result_slot<T: ?Sized>(
    caller: &str,
    result: *mut *mut Box<T>,
) {
    trace!(
        caller = caller,
        result_ptr = ?result,
        "initialize_posix_env_result_slot: preparing output pointer"
    );

    assert!(
        !result.is_null(),
        "initialize_posix_env_result_slot: result pointer must not be null"
    );

    unsafe {
        *result = std::ptr::null_mut();
    }
}
