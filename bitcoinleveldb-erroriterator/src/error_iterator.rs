// ---------------- [ File: bitcoinleveldb-erroriterator/src/error_iterator.rs ]
crate::ix!();

/**
  | Return an empty iterator with the specified
  | status.
  |
  */
pub fn new_error_iterator(status: &Status) -> *mut LevelDBIterator {
    trace!(
        "new_error_iterator: creating error iterator (status_is_ok={})",
        status.is_ok()
    );

    let iter = EmptyIterator::new(status);
    let boxed = Box::new(iter);
    let raw: *mut EmptyIterator = Box::into_raw(boxed);

    unsafe {
        // As with `new_empty_iterator`, the address of `base` is the start of
        // the allocation, so we can safely return it as the abstract iterator.
        let base_ptr: *mut LevelDBIterator = &mut *(*raw).base_mut();
        trace!(
            "new_error_iterator: allocated EmptyIterator at {:?}, base_ptr={:?}",
            raw,
            base_ptr
        );
        base_ptr
    }
}

#[cfg(test)]
mod error_iterator_behavior_tests {
    use super::*;

    #[traced_test]
    fn error_iterator_propagates_non_ok_status_via_concrete_type() {
        let msg_bytes = b"synthetic iterator error".to_vec();
        let msg_slice = Slice::from(msg_bytes.as_slice());
        let err_status = Status::corruption(&msg_slice, None);

        let iter = EmptyIterator::new(&err_status);
        let st = iter.status();

        assert!(
            !st.is_ok(),
            "EmptyIterator constructed with an error status must report non‑OK"
        );
    }

    #[traced_test]
    fn new_error_iterator_returns_non_null_pointer() {
        let msg_bytes = b"synthetic iterator error".to_vec();
        let msg_slice = Slice::from(msg_bytes.as_slice());
        let err_status = Status::corruption(&msg_slice, None);

        let base_ptr: *mut LevelDBIterator = new_error_iterator(&err_status);
        assert!(
            !base_ptr.is_null(),
            "new_error_iterator must never return null"
        );

        // As with new_empty_iterator, we intentionally leak here; we cannot
        // safely reconstruct the owning EmptyIterator box from the base pointer.
    }
}
