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

#[cfg(test)]
mod initialize_posix_env_result_slot_tests {
    use super::*;

    #[traced_test]
    fn initialize_posix_env_result_slot_overwrites_existing_pointer_value() {
        let mut result: *mut Box<dyn WritableFile> =
            0x1 as *mut Box<dyn WritableFile>;

        initialize_posix_env_result_slot::<dyn WritableFile>(
            "initialize_posix_env_result_slot_tests",
            &mut result as *mut *mut Box<dyn WritableFile>,
        );

        assert!(
            result.is_null(),
            "initialize_posix_env_result_slot must zero out the provided pointer"
        );
    }
}
