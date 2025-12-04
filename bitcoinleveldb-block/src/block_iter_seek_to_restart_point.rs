// ---------------- [ File: bitcoinleveldb-block/src/block_iter_seek_to_restart_point.rs ]
crate::ix!();

impl BlockIter {

    pub fn seek_to_restart_point(&mut self, index: u32) {
        trace!(
            "BlockIter::seek_to_restart_point: index={}, num_restarts={}",
            index,
            self.num_restarts()
        );

        self.key_buffer_mut().clear();
        self.set_restart_index(index);

        let offset = self.get_restart_point(index);
        unsafe {
            let ptr   = self.data_ptr().add(offset as usize);
            let slice = Slice::from_ptr_len(ptr, 0);
            *self.value_slice_mut() = slice;
        }

        self.set_current_offset(offset);
        trace!(
            "BlockIter::seek_to_restart_point: new current={}, value_len=0",
            self.current_offset()
        );
    }
}

#[cfg(test)]
mod block_iter_seek_to_restart_point_tests {
    use super::*;

    #[derive(Clone, Default)]
    struct DummyComparator;

    impl Compare for DummyComparator {
        fn compare(&self, _a: &Slice, _b: &Slice) -> i32 { 0 }
    }
    impl Named for DummyComparator {
        fn name(&self) -> &str { "dummy-comparator" }
    }
    impl FindShortestSeparator for DummyComparator {
        fn find_shortest_separator(&self, _start: &mut String, _limit: &Slice) {}
    }
    impl FindShortSuccessor for DummyComparator {
        fn find_short_successor(&self, _key: &mut String) {}
    }
    impl SliceComparator for DummyComparator {}

    #[traced_test]
    fn seek_to_restart_point_resets_key_and_sets_current_offset() {
        let mut backing = vec![0u8; 16];
        backing[8..12].copy_from_slice(&2u32.to_le_bytes());
        backing[12..16].copy_from_slice(&2u32.to_le_bytes()); // num_restarts=2

        let cmp = bitcoinleveldb_comparator::BytewiseComparatorImpl::default();
        let cmp_ref: &dyn SliceComparator = &cmp;
        let cmp_ptr: *const dyn SliceComparator = cmp_ref as *const dyn SliceComparator;

        let mut iter = BlockIter::new(
            cmp_ptr,
            backing.as_ptr(),
            8,
            2,
        );

        iter.key_buffer_mut().push_str("prefix");
        iter.seek_to_restart_point(1);

        trace!(
            "after seek_to_restart_point: current={}, restart_index={}, key_len={}, value_len={}",
            iter.current_offset(),
            iter.restart_index(),
            iter.key_buffer().len(),
            *iter.value_slice().size()
        );

        debug!("backing data for restart-point test: {:?}", backing);

        assert_eq!(iter.restart_index(), 1);
        assert_eq!(iter.key_buffer().len(), 0);
        assert_eq!(*iter.value_slice().size(), 0);
    }
}
