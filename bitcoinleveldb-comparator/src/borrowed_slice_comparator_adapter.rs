crate::ix!();

struct BorrowedSliceComparatorAdapter {
    comparator: *const dyn SliceComparator,
}

impl Named for BorrowedSliceComparatorAdapter {
    fn name(&self) -> Cow<'_,str> {
        trace!(
            target: "bitcoinleveldb_comparator::borrowed_slice_comparator_adapter",
            label = "borrowed_slice_comparator_adapter.name",
            comparator_ptr = ?self.comparator
        );

        assert!(
            !self.comparator.is_null(),
            "BorrowedSliceComparatorAdapter::name: comparator pointer must not be null"
        );

        unsafe { (&*self.comparator).name() }
    }
}

impl Compare for BorrowedSliceComparatorAdapter {
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        trace!(
            target: "bitcoinleveldb_comparator::borrowed_slice_comparator_adapter",
            label = "borrowed_slice_comparator_adapter.compare",
            comparator_ptr = ?self.comparator,
            a_len = *a.size(),
            b_len = *b.size()
        );

        assert!(
            !self.comparator.is_null(),
            "BorrowedSliceComparatorAdapter::compare: comparator pointer must not be null"
        );

        unsafe { (&*self.comparator).compare(a, b) }
    }
}

impl FindShortestSeparator for BorrowedSliceComparatorAdapter {
    fn find_shortest_separator(&self, start: &mut Vec<u8>, limit: &[u8]) {
        trace!(
            target: "bitcoinleveldb_comparator::borrowed_slice_comparator_adapter",
            label = "borrowed_slice_comparator_adapter.find_shortest_separator",
            comparator_ptr = ?self.comparator,
            start_len = start.len(),
            limit_len = limit.len()
        );

        assert!(
            !self.comparator.is_null(),
            "BorrowedSliceComparatorAdapter::find_shortest_separator: comparator pointer must not be null"
        );

        unsafe { (&*self.comparator).find_shortest_separator(start, limit) }
    }
}

impl FindShortSuccessor for BorrowedSliceComparatorAdapter {
    fn find_short_successor(&self, key: &mut Vec<u8>) {
        trace!(
            target: "bitcoinleveldb_comparator::borrowed_slice_comparator_adapter",
            label = "borrowed_slice_comparator_adapter.find_short_successor",
            comparator_ptr = ?self.comparator,
            key_len = key.len()
        );

        assert!(
            !self.comparator.is_null(),
            "BorrowedSliceComparatorAdapter::find_short_successor: comparator pointer must not be null"
        );

        unsafe { (&*self.comparator).find_short_successor(key) }
    }
}

impl SliceComparator for BorrowedSliceComparatorAdapter {
    fn bytewise_comparator(&self) -> *const dyn SliceComparator {
        trace!(
            target: "bitcoinleveldb_comparator::borrowed_slice_comparator_adapter",
            label = "borrowed_slice_comparator_adapter.bytewise_comparator",
            comparator_ptr = ?self.comparator
        );

        assert!(
            !self.comparator.is_null(),
            "BorrowedSliceComparatorAdapter::bytewise_comparator: comparator pointer must not be null"
        );

        unsafe { (&*self.comparator).bytewise_comparator() }
    }
}

/// Returns a boxed comparator that forwards every operation to `comparator`
/// without taking ownership.
///
/// Invariant:
/// the comparator pointer must remain non-null and valid for the entire lifetime
/// of the returned boxed trait object. This adapter preserves comparator identity
/// across subsystem boundaries and forbids accidental fallback to bytewise order.
pub fn borrowed_slice_comparator_adapter_box(
    comparator: *const dyn SliceComparator,
) -> Option<Box<dyn SliceComparator>> {
    trace!(
        target: "bitcoinleveldb_comparator::borrowed_slice_comparator_adapter",
        label = "borrowed_slice_comparator_adapter_box.entry",
        comparator_ptr = ?comparator
    );

    if comparator.is_null() {
        error!(
            target: "bitcoinleveldb_comparator::borrowed_slice_comparator_adapter",
            label = "borrowed_slice_comparator_adapter_box.null_pointer",
            comparator_ptr = ?comparator
        );
        return None;
    }

    let boxed: Box<dyn SliceComparator> =
        Box::new(BorrowedSliceComparatorAdapter { comparator });

    trace!(
        target: "bitcoinleveldb_comparator::borrowed_slice_comparator_adapter",
        label = "borrowed_slice_comparator_adapter_box.exit",
        comparator_ptr = ?comparator
    );

    Some(boxed)
}

#[cfg(test)]
mod borrowed_slice_comparator_adapter_behavior_tests {
    use super::*;

    fn build_null_borrowed_slice_comparator_adapter_test_ptr() -> *const dyn SliceComparator {
        let null_impl_ptr: *const BytewiseComparatorImpl = core::ptr::null();
        null_impl_ptr as *const dyn SliceComparator
    }

    #[traced_test]
    fn borrowed_slice_comparator_adapter_box_returns_none_for_null_pointer() {
        let null_ptr = build_null_borrowed_slice_comparator_adapter_test_ptr();

        let maybe_adapter = borrowed_slice_comparator_adapter_box(null_ptr);

        assert!(
            maybe_adapter.is_none(),
            "borrowed_slice_comparator_adapter_box must reject null comparator pointers"
        );
    }

    #[traced_test]
    fn borrowed_slice_comparator_adapter_box_forwards_full_slice_comparator_surface() {
        let comparator_ptr = bytewise_comparator();
        let maybe_adapter = borrowed_slice_comparator_adapter_box(comparator_ptr);

        match maybe_adapter {
            Some(adapter) => {
                assert_eq!(
                    adapter.name().as_ref(),
                    "leveldb.BytewiseComparator",
                    "Forwarded comparator name must match the underlying comparator"
                );

                let a_bytes = b"a";
                let b_bytes = b"b";
                let a = Slice::from(&a_bytes[..]);
                let b = Slice::from(&b_bytes[..]);

                assert!(
                    adapter.compare(&a, &b) < 0,
                    "Forwarded comparator compare() must preserve ordering"
                );

                let mut separator = b"foo1".to_vec();
                let limit = b"foo9".to_vec();
                adapter.find_shortest_separator(&mut separator, &limit[..]);

                assert_eq!(
                    separator,
                    b"foo2".to_vec(),
                    "Forwarded find_shortest_separator() must preserve underlying shortening logic"
                );

                let mut successor = b"foo\xff\xff".to_vec();
                adapter.find_short_successor(&mut successor);

                assert_eq!(
                    successor,
                    b"g".to_vec(),
                    "Forwarded find_short_successor() must preserve underlying successor logic"
                );

                let forwarded_bytewise = adapter.bytewise_comparator();

                assert!(
                    !forwarded_bytewise.is_null(),
                    "Forwarded bytewise_comparator() must return a valid comparator pointer"
                );

                unsafe {
                    assert_eq!(
                        (&*forwarded_bytewise).name().as_ref(),
                        "leveldb.BytewiseComparator",
                        "Forwarded bytewise comparator must preserve the canonical bytewise identity"
                    );
                }
            }
            None => {
                assert!(
                    false,
                    "borrowed_slice_comparator_adapter_box must succeed for a valid comparator pointer"
                );
            }
        }
    }
}
