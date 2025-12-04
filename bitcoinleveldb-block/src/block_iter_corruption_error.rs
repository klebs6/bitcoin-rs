// ---------------- [ File: bitcoinleveldb-block/src/block_iter_corruption_error.rs ]
crate::ix!();

impl BlockIter {
    
    pub fn corruption_error(&mut self) {
        trace!("BlockIter::corruption_error: marking iterator invalid");
        self.mark_invalid();

        let msg_bytes = b"bad entry in block"[..].to_vec();
        let msg_slice = Slice::from(msg_bytes.as_slice());
        self.set_status(Status::corruption(&msg_slice, None));

        self.key_buffer_mut().clear();
        self.value_slice_mut().clear();
    }
}

#[cfg(test)]
mod block_iter_corruption_behavior_tests {
    use super::*;

    #[derive(Clone, Default)]
    struct DummyComparator;

    impl Compare for DummyComparator {
        fn compare(&self, _a: &Slice, _b: &Slice) -> i32 {
            0
        }
    }

    impl Named for DummyComparator {
        fn name(&self) -> &str {
            "dummy-comparator"
        }
    }

    impl FindShortestSeparator for DummyComparator {
        fn find_shortest_separator(&self, _start: &mut String, _limit: &Slice) {}
    }

    impl FindShortSuccessor for DummyComparator {
        fn find_short_successor(&self, _key: &mut String) {}
    }

    impl SliceComparator for DummyComparator {}

    #[traced_test]
    fn corruption_error_marks_iterator_invalid_and_clears_key_and_value() {
        let buf = vec![0u8; 8];
        let cmp = bitcoinleveldb_comparator::BytewiseComparatorImpl::default();
        let cmp_ref: &dyn SliceComparator = &cmp;
        let cmp_ptr: *const dyn SliceComparator = cmp_ref as *const dyn SliceComparator;

        let mut iter = BlockIter::new(
            cmp_ptr,
            buf.as_ptr(),
            4,
            1,
        );

        iter.corruption_error();

        trace!(
            "after corruption_error: valid={}, key_len={}, value_len={}",
            iter.valid(),
            iter.key_buffer().len(),
            *iter.value_slice().size()
        );

        assert!(!iter.valid());
        assert_eq!(iter.key_buffer().len(), 0);
        assert_eq!(*iter.value_slice().size(), 0);

        let status = iter.status();
        debug!("status after corruption_error is_ok={}", status.is_ok());
        assert!(!status.is_ok());
    }
}
