// ---------------- [ File: bitcoinleveldb-erroriterator/src/error_iterator.rs ]
crate::ix!();

/// Construct a new "error iterator" that always yields a non‑OK status.
///
/// This is used by helpers like `TableCache::new_iterator` when the
/// underlying table cannot be opened. The returned `LevelDBIterator`
/// wraps an `EmptyIterator` carrying the provided `status`.
pub fn new_error_iterator(status: &Status) -> *mut LevelDBIterator {
    trace!(
        "new_error_iterator: creating error iterator (status_is_ok={})",
        status.is_ok()
    );

    // Copy the incoming status so the iterator owns its error state.
    let copied_status =
        bitcoinleveldb_status::Status::new_from_other_copy(status);

    let empty_iface: Box<dyn LevelDBIteratorInterface> =
        Box::new(EmptyIterator::new(copied_status));

    let wrapper = LevelDBIterator::new(Some(empty_iface));

    let raw_wrapper: *mut LevelDBIterator = Box::into_raw(Box::new(wrapper));

    trace!(
        "new_error_iterator: allocated EmptyIterator-backed wrapper at {:?}",
        raw_wrapper
    );

    raw_wrapper
}


#[cfg(test)]
mod error_iterator_behavior_tests {
    use super::*;

    #[traced_test]
    fn error_iterator_propagates_non_ok_status_via_concrete_type() {
        let msg_bytes = b"synthetic iterator error".to_vec();
        let msg_slice = Slice::from(msg_bytes.as_slice());
        let err_status = Status::corruption(&msg_slice, None);

        let iter = EmptyIterator::new(err_status);
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
