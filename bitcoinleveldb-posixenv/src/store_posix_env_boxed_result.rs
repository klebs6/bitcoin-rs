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

#[cfg(test)]
mod store_posix_env_boxed_result_tests {
    use super::*;

    #[derive(Debug)]
    struct DummyBoxedValue {
        value: i32,
    }

    #[traced_test]
    fn store_posix_env_boxed_result_installs_boxed_value_into_raw_pointer() {
        let mut result: *mut Box<DummyBoxedValue> = std::ptr::null_mut();

        let value = Box::new(DummyBoxedValue { value: 7 });

        let status = store_posix_env_boxed_result::<DummyBoxedValue>(
            "store_posix_env_boxed_result_tests",
            &mut result as *mut *mut Box<DummyBoxedValue>,
            value,
        );

        assert!(
            status.is_ok(),
            "store_posix_env_boxed_result should always return Status::ok(): {}",
            status.to_string()
        );

        assert!(
            !result.is_null(),
            "store_posix_env_boxed_result must populate the out-parameter with a non-null pointer"
        );

        unsafe {
            let outer: Box<Box<DummyBoxedValue>> = Box::from_raw(result);
            let inner: Box<DummyBoxedValue> = *outer;

            assert_eq!(
                inner.value, 7,
                "the stored boxed value must round-trip through the raw pointer intact"
            );
        }
    }
}

