// ---------------- [ File: bitcoinleveldb-block/src/block_iter.rs ]
crate::ix!();

pub struct BlockIter {

    base:          LevelDBIterator,

    comparator:    *const dyn SliceComparator,

    /**
      | underlying block contents
      |
      */
    data:          *const u8,

    /**
      | Offset of restart array (list of fixed32)
      |
      */
    restarts:      u32,

    /**
      | Number of uint32_t entries in restart
      | array
      |
      */
    num_restarts:  u32,

    /**
      | current_ is offset in data_ of current
      | entry. >= restarts_ if !Valid
      |
      */
    current:       u32,

    /**
      | Index of restart block in which current_
      | falls
      |
      */
    restart_index: u32,

    key_:          String,
    value:         Slice,
    status:        Status,
}

impl BlockIter {
  
    pub fn new(
        comparator:   *const dyn SliceComparator,
        data:         *const u8,
        restarts:     u32,
        num_restarts: u32,
    ) -> Self {
        assert!(
            num_restarts > 0,
            "BlockIter::new requires num_restarts > 0"
        );

        trace!(
            "BlockIter::new: data={:?}, restarts_offset={}, num_restarts={}, comparator_ptr={:p}",
            data,
            restarts,
            num_restarts,
            comparator
        );

        BlockIter {
            base:          LevelDBIterator::new(),
            comparator,
            data,
            restarts,
            num_restarts,
            current:       restarts,
            restart_index: num_restarts,
            key_:          String::new(),
            value:         Slice::default(),
            status:        Status::default(),
        }
    }

    #[inline]
    pub fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        trace!(
            "BlockIter::compare called: a_len={}, b_len={}",
            *a.size(),
            *b.size()
        );
        unsafe {
            (*self.comparator).compare(a, b)
        }
    }
 
    pub fn valid(&self) -> bool {
        let v = self.current < self.restarts;
        trace!(
            "BlockIter::valid called => {} (current={}, restarts={})",
            v,
            self.current,
            self.restarts
        );
        v
    }

    pub fn status(&self) -> crate::Status {
        trace!("BlockIter::status called");
        Status::new_from_other_copy(&self.status)
    }
 
    pub fn next(&mut self) {
        assert!(
            self.valid(),
            "BlockIter::next called on invalid iterator"
        );
        trace!("BlockIter::next: current={}", self.current);
        self.parse_next_key();
    }

    #[inline]
    pub fn data_ptr(&self) -> *const u8 {
        self.data
    }

    #[inline]
    pub fn restarts_offset(&self) -> u32 {
        self.restarts
    }

    #[inline]
    pub fn num_restarts(&self) -> u32 {
        self.num_restarts
    }

    #[inline]
    pub fn current_offset(&self) -> u32 {
        self.current
    }

    #[inline]
    pub fn set_current_offset(&mut self, offset: u32) {
        self.current = offset;
    }

    #[inline]
    pub fn restart_index(&self) -> u32 {
        self.restart_index
    }

    #[inline]
    pub fn set_restart_index(&mut self, index: u32) {
        self.restart_index = index;
    }

    #[inline]
    pub fn mark_invalid(&mut self) {
        self.current       = self.restarts;
        self.restart_index = self.num_restarts;
    }

    #[inline]
    pub fn key_buffer(&self) -> &String {
        &self.key_
    }

    #[inline]
    pub fn key_buffer_mut(&mut self) -> &mut String {
        &mut self.key_
    }

    #[inline]
    pub fn value_slice(&self) -> &Slice {
        &self.value
    }

    #[inline]
    pub fn value_slice_mut(&mut self) -> &mut Slice {
        &mut self.value
    }

    #[inline]
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    #[inline]
    pub fn base_mut_ptr(&mut self) -> *mut LevelDBIterator {
        &mut self.base as *mut LevelDBIterator
    }
}

#[cfg(test)]
mod block_iter_construction_and_validity_tests {
    use super::*;

    #[derive(Clone, Default)]
    struct DummyComparator;

    impl Compare for DummyComparator {
        fn compare(&self, a: &Slice, b: &Slice) -> i32 {
            let a_bytes = unsafe { core::slice::from_raw_parts(*a.data(), *a.size()) };
            let b_bytes = unsafe { core::slice::from_raw_parts(*b.data(), *b.size()) };
            for (aa, bb) in a_bytes.iter().zip(b_bytes.iter()) {
                if aa < bb {
                    return -1;
                }
                if aa > bb {
                    return 1;
                }
            }
            a_bytes.len().cmp(&b_bytes.len()) as i32
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

    fn build_simple_two_entry_block_bytes() -> Vec<u8> {
        let mut options = Box::new(Options::default());
        let opts_ptr: *const Options = &*options;

        let mut builder = BlockBuilder::new(opts_ptr);
        builder.add(
            &Slice::from("a".as_bytes()),
            &Slice::from("v1".as_bytes()),
        );
        builder.add(
            &Slice::from("b".as_bytes()),
            &Slice::from("v2".as_bytes()),
        );

        let block_slice = builder.finish();
        unsafe {
            let ptr = *block_slice.data();
            let len = *block_slice.size();
            core::slice::from_raw_parts(ptr, len).to_vec()
        }
    }

    #[traced_test]
    fn block_iter_new_starts_at_restart_region_and_is_initially_invalid() {
        let block_bytes = build_simple_two_entry_block_bytes();
        let len         = block_bytes.len();
        assert!(len > 8);

        let num_restarts =
            u32::from_le_bytes(block_bytes[len - 4..].try_into().unwrap());
        let restart_offset = (len - (1 + num_restarts as usize) * 4) as u32;

        let cmp = bitcoinleveldb_comparator::BytewiseComparatorImpl::default();
        let cmp_ref: &dyn SliceComparator = &cmp;
        let cmp_ptr: *const dyn SliceComparator = cmp_ref as *const dyn SliceComparator;

        let iter = BlockIter::new(
            cmp_ptr,
            block_bytes.as_ptr(),
            restart_offset,
            num_restarts,
        );

        trace!(
            "constructed BlockIter in test: restarts_offset={}, num_restarts={}",
            iter.restarts_offset(),
            iter.num_restarts()
        );

        assert!(!iter.valid());
        assert_eq!(iter.current_offset(), iter.restarts_offset());
    }

    #[traced_test]
    fn block_iter_status_defaults_to_ok() {
        let block_bytes = build_simple_two_entry_block_bytes();
        let len         = block_bytes.len();
        let num_restarts =
            u32::from_le_bytes(block_bytes[len - 4..].try_into().unwrap());
        let restart_offset = (len - (1 + num_restarts as usize) * 4) as u32;

        let cmp = bitcoinleveldb_comparator::BytewiseComparatorImpl::default();
        let cmp_ref: &dyn SliceComparator = &cmp;
        let cmp_ptr: *const dyn SliceComparator = cmp_ref as *const dyn SliceComparator;

        let iter = BlockIter::new(
            cmp_ptr,
            block_bytes.as_ptr(),
            restart_offset,
            num_restarts,
        );

        let status = iter.status();
        debug!("initial BlockIter status is_ok={}", status.is_ok());
        assert!(status.is_ok());
    }
}
