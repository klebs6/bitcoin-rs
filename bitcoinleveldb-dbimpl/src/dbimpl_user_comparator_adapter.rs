// ---------------- [ File: bitcoinleveldb-dbimpl/src/dbimpl_user_comparator_adapter.rs ]
crate::ix!();

#[derive(Clone, Copy)]
pub struct DbImplUserComparatorAdapter {
    ptr: *const dyn SliceComparator,
}

impl DbImplUserComparatorAdapter {
    #[inline]
    pub fn new(ptr: *const dyn SliceComparator) -> Self {
        Self { ptr }
    }

    pub fn as_ref(&self) -> &dyn SliceComparator {
        if self.ptr.is_null() {
            tracing::error!(
                "DBImpl user comparator pointer had null data; falling back to null_slice_comparator()"
            );

            let fallback_ptr: *const dyn SliceComparator = null_slice_comparator();

            if fallback_ptr.is_null() {
                tracing::error!(
                    "null_slice_comparator() returned a null comparator pointer; cannot provide fallback"
                );
                panic!();
            }

            unsafe { &*fallback_ptr }
        } else {
            unsafe { &*self.ptr }
        }
    }
}

impl Named for DbImplUserComparatorAdapter {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        self.as_ref().name()
    }
}

impl Compare for DbImplUserComparatorAdapter {
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        self.as_ref().compare(a, b)
    }
}

impl FindShortestSeparator for DbImplUserComparatorAdapter {
    fn find_shortest_separator(&self, start: &mut Vec<u8>, limit: &[u8]) {
        self.as_ref().find_shortest_separator(start, limit)
    }
}

impl FindShortSuccessor for DbImplUserComparatorAdapter {
    fn find_short_successor(&self, key: &mut Vec<u8>) {
        self.as_ref().find_short_successor(key)
    }
}

impl SliceComparator for DbImplUserComparatorAdapter {
    fn bytewise_comparator(&self) -> *const dyn SliceComparator {
        self.as_ref().bytewise_comparator()
    }
}

#[cfg(test)]
mod user_comparator_adapter_interface_and_delegation_suite {
    use super::*;

    #[traced_test]
    fn user_comparator_adapter_new_signature_is_stable() {
        tracing::info!("Asserting DbImplUserComparatorAdapter::new signature is stable");
        let _f: fn(*const dyn SliceComparator) -> DbImplUserComparatorAdapter =
            DbImplUserComparatorAdapter::new;
        let _ = _f;
    }

    #[traced_test]
    fn user_comparator_adapter_delegates_name_and_compare_to_inner_comparator() {
        let inner: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());
        let inner_ptr: *const dyn SliceComparator = (&*inner) as *const dyn SliceComparator;

        let adapter: DbImplUserComparatorAdapter = DbImplUserComparatorAdapter::new(inner_ptr);

        let a = Slice::from_str("a");
        let b = Slice::from_str("b");

        let expected = inner.compare(&a, &b);
        let got = adapter.compare(&a, &b);

        tracing::debug!(
            expected,
            got,
            inner_name = %inner.name(),
            adapter_name = %adapter.name(),
            "Comparator compare/name delegation"
        );

        assert_eq!(got, expected);
        assert_eq!(adapter.name(), inner.name());
        assert!(adapter.bytewise_comparator().is_null() == inner.bytewise_comparator().is_null());
    }

    #[traced_test]
    fn user_comparator_adapter_delegates_separator_and_successor_mutations() {
        let inner: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());
        let inner_ptr: *const dyn SliceComparator = (&*inner) as *const dyn SliceComparator;

        let adapter: DbImplUserComparatorAdapter = DbImplUserComparatorAdapter::new(inner_ptr);

        let limit: Vec<u8> = b"foobar".to_vec();

        let mut start_inner: Vec<u8> = b"foo".to_vec();
        let mut start_adapter: Vec<u8> = start_inner.clone();

        inner.find_shortest_separator(&mut start_inner, &limit);
        adapter.find_shortest_separator(&mut start_adapter, &limit);

        tracing::debug!(
            start_inner = ?start_inner,
            start_adapter = ?start_adapter,
            limit = ?limit,
            "find_shortest_separator delegation"
        );

        assert_eq!(start_adapter, start_inner);

        let mut key_inner: Vec<u8> = b"foo".to_vec();
        let mut key_adapter: Vec<u8> = key_inner.clone();

        inner.find_short_successor(&mut key_inner);
        adapter.find_short_successor(&mut key_adapter);

        tracing::debug!(
            key_inner = ?key_inner,
            key_adapter = ?key_adapter,
            "find_short_successor delegation"
        );

        assert_eq!(key_adapter, key_inner);
    }

    #[traced_test]
    fn user_comparator_adapter_falls_back_to_null_slice_comparator_when_ptr_is_null() {
        let null_concrete: *const BytewiseComparatorImpl = core::ptr::null();
        let null_trait: *const dyn SliceComparator = null_concrete as *const dyn SliceComparator;

        let adapter: DbImplUserComparatorAdapter = DbImplUserComparatorAdapter::new(null_trait);

        let a = Slice::from_str("a");
        let b = Slice::from_str("b");

        let fallback: &dyn SliceComparator = unsafe {
            null_slice_comparator()
                .as_ref()
                .unwrap_or_else(|| {
                    tracing::error!("null_slice_comparator() returned a null pointer");
                    panic!();
                })
        };

        let expected = fallback.compare(&a, &b);
        let got = adapter.compare(&a, &b);

        tracing::debug!(
            expected,
            got,
            fallback_name = %fallback.name(),
            adapter_name = %adapter.name(),
            "Null-pointer fallback delegation"
        );

        assert_eq!(got, expected);
        assert_eq!(adapter.name(), fallback.name());
    }
}
