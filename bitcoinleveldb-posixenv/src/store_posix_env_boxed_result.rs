// ---------------- [ File: bitcoinleveldb-posixenv/src/store_posix_env_boxed_result.rs ]
crate::ix!();

pub fn store_posix_env_boxed_result<T: ?Sized>(
    caller: &str,
    result: *mut *mut Box<T>,
    value: Box<T>,
) -> crate::Status {
    trace!(
        caller = caller,
        result_ptr = ?result,
        "store_posix_env_boxed_result: installing boxed trait object into output pointer"
    );

    let outer: Box<Box<T>> = Box::new(value);
    let raw: *mut Box<T> = Box::into_raw(outer);

    unsafe {
        *result = raw;
    }

    debug!(
        caller = caller,
        result_ptr = ?raw,
        "store_posix_env_boxed_result: stored boxed trait object"
    );

    crate::Status::ok()
}
