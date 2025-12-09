// ---------------- [ File: bitcoinleveldb-blockiter/src/block_iter_seek_to.rs ]
crate::ix!();

impl LevelDBIteratorSeekToFirst for BlockIter {
     
    fn seek_to_first(&mut self) {
        trace!("BlockIter::seek_to_first called");
        self.seek_to_restart_point(0);
        self.parse_next_key();
    }
}

impl LevelDBIteratorSeekToLast for BlockIter {
    
    fn seek_to_last(&mut self) {
        trace!("BlockIter::seek_to_last called");
        if self.num_restarts() == 0 {
            self.mark_invalid();
            return;
        }

        self.seek_to_restart_point(self.num_restarts() - 1);

        while self.parse_next_key()
            && self.next_entry_offset() < self.restarts_offset()
        {
            trace!(
                "BlockIter::seek_to_last: skipping entry at offset {}",
                self.current_offset()
            );
        }
    }
}

#[cfg(test)]
mod block_iter_seek_to_first_last_tests {
    use super::*;

    fn build_block_bytes() -> Vec<u8> {
        let opts_box = Box::new(Options::default());
        let opts_ptr: *const Options = &*opts_box;

        let mut builder = BlockBuilder::new(opts_ptr);
        builder.add(
            &Slice::from("a".as_bytes()),
            &Slice::from("v1".as_bytes()),
        );
        builder.add(
            &Slice::from("b".as_bytes()),
            &Slice::from("v2".as_bytes()),
        );
        builder.add(
            &Slice::from("c".as_bytes()),
            &Slice::from("v3".as_bytes()),
        );

        let slice = builder.finish();
        unsafe {
            let ptr = *slice.data();
            let len = *slice.size();
            core::slice::from_raw_parts(ptr, len).to_vec()
        }
    }

    #[traced_test]
    fn seek_to_first_positions_at_smallest_key() {
        let block_bytes = build_block_bytes();
        let len         = block_bytes.len();
        let num_restarts =
            u32::from_le_bytes(block_bytes[len - 4..].try_into().unwrap());
        let restart_offset = (len - (1 + num_restarts as usize) * 4) as u32;

        let cmp = BytewiseComparatorImpl::default();
        let cmp_ref: &dyn SliceComparator = &cmp;
        let cmp_ptr: *const dyn SliceComparator = cmp_ref as *const dyn SliceComparator;

        let mut iter = BlockIter::new(
            cmp_ptr,
            block_bytes.as_ptr(),
            restart_offset,
            num_restarts,
        );

        iter.seek_to_first();

        assert!(iter.valid());
        let k = iter.key();
        let key_str = unsafe {
            let bytes = core::slice::from_raw_parts(*k.data(), *k.size());
            core::str::from_utf8_unchecked(bytes).to_string()
        };
        debug!("seek_to_first landed on key='{}'", key_str);
        assert_eq!(key_str, "a");
    }

    #[traced_test]
    fn seek_to_last_positions_at_largest_key() {
        let block_bytes = build_block_bytes();
        let len         = block_bytes.len();
        let num_restarts =
            u32::from_le_bytes(block_bytes[len - 4..].try_into().unwrap());
        let restart_offset = (len - (1 + num_restarts as usize) * 4) as u32;

        let cmp = BytewiseComparatorImpl::default();
        let cmp_ref: &dyn SliceComparator = &cmp;
        let cmp_ptr: *const dyn SliceComparator = cmp_ref as *const dyn SliceComparator;

        let mut iter = BlockIter::new(
            cmp_ptr,
            block_bytes.as_ptr(),
            restart_offset,
            num_restarts,
        );

        iter.seek_to_last();

        assert!(iter.valid());
        let k = iter.key();
        let key_str = unsafe {
            let bytes = core::slice::from_raw_parts(*k.data(), *k.size());
            core::str::from_utf8_unchecked(bytes).to_string()
        };
        trace!("seek_to_last landed on key='{}'", key_str);
        assert_eq!(key_str, "c");
    }
}
