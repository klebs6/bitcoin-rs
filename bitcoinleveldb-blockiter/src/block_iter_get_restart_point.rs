// ---------------- [ File: bitcoinleveldb-blockiter/src/block_iter_get_restart_point.rs ]
crate::ix!();

impl BlockIter {

    pub fn get_restart_point(&mut self, index: u32) -> u32 {
        let num_restarts = self.num_restarts();

        assert!(
            (index as usize) < num_restarts as usize,
            "BlockIter::get_restart_point: index {} out of range {}",
            index,
            num_restarts
        );

        let offset = self.restarts_offset() as usize
            + (index as usize) * core::mem::size_of::<u32>();

        unsafe {
            let slice = core::slice::from_raw_parts(
                self.data_ptr().add(offset),
                core::mem::size_of::<u32>(),
            );
            let value = bitcoinleveldb_coding::decode_fixed32(slice.as_ptr());
            trace!(
                "BlockIter::get_restart_point: index={}, offset={}, value={}",
                index,
                offset,
                value
            );
            value
        }
    }
}

#[cfg(test)]
mod block_iter_restart_point_tests {
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
    fn get_restart_point_decodes_correct_offsets() {
        let mut bytes = vec![0u8; 12];
        bytes[4..8].copy_from_slice(&4u32.to_le_bytes());
        bytes[8..12].copy_from_slice(&2u32.to_le_bytes()); // num_restarts=2

        let cmp = bitcoinleveldb_comparator::BytewiseComparatorImpl::default();
        let cmp_ref: &dyn SliceComparator = &cmp;
        let cmp_ptr: *const dyn SliceComparator = cmp_ref as *const dyn SliceComparator;

        let mut iter = BlockIter::new(
            cmp_ptr,
            bytes.as_ptr(),
            4,
            2,
        );

        let first = iter.get_restart_point(0);
        let second = iter.get_restart_point(1);

        trace!(
            "decoded restart points: first={}, second={}",
            first,
            second
        );

        debug!("raw backing bytes={:?}", bytes);

        assert_eq!(first, 4);
        assert_eq!(second, 2u32);
    }
}
