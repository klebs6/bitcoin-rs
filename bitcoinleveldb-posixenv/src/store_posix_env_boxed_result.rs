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

#[cfg(test)]
mod store_posix_env_boxed_result_trait_object_drop_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[derive(Debug)]
    struct DropCountingDebug {
        drops: Arc<AtomicUsize>,
        id:    u64,
    }

    impl Drop for DropCountingDebug {
        fn drop(&mut self) {
            let prev = self.drops.fetch_add(1, Ordering::SeqCst);
            debug!(
                id = self.id,
                previous = prev,
                current  = prev + 1,
                "DropCountingDebug::drop: incremented drop counter"
            );
        }
    }

    #[traced_test]
    fn store_posix_env_boxed_result_round_trips_dyn_trait_object_and_drops_exactly_once() {
        trace!("store_posix_env_boxed_result_round_trips_dyn_trait_object_and_drops_exactly_once: start");

        let drops = Arc::new(AtomicUsize::new(0));

        let inner: Box<dyn core::fmt::Debug> = Box::new(DropCountingDebug {
            drops: drops.clone(),
            id:    7,
        });

        let mut result: *mut Box<dyn core::fmt::Debug> = core::ptr::null_mut();

        let st = store_posix_env_boxed_result::<dyn core::fmt::Debug>(
            "store_posix_env_boxed_result_trait_object_drop_tests",
            &mut result as *mut *mut Box<dyn core::fmt::Debug>,
            inner,
        );

        assert!(
            st.is_ok(),
            "store_posix_env_boxed_result must return Status::ok(): {}",
            st.to_string()
        );
        assert!(
            !result.is_null(),
            "store_posix_env_boxed_result must populate out-parameter with non-null pointer"
        );

        debug!(
            stored_ptr = ?result,
            "reconstructing Box<Box<dyn Debug>> from raw pointer and dropping it"
        );

        unsafe {
            let outer: Box<Box<dyn core::fmt::Debug>> = Box::from_raw(result);
            drop(outer);
        }

        let observed = drops.load(Ordering::SeqCst);

        assert_eq!(
            observed, 1,
            "dropping the outer Box<Box<dyn Debug>> must drop the inner value exactly once"
        );

        trace!("store_posix_env_boxed_result_round_trips_dyn_trait_object_and_drops_exactly_once: done");
    }
}
