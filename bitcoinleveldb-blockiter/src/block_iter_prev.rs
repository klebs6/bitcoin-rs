// ---------------- [ File: bitcoinleveldb-blockiter/src/block_iter_prev.rs ]
crate::ix!();

impl BlockIter {
   
    pub fn prev(&mut self) {
        assert!(
            self.valid(),
            "BlockIter::prev called on invalid iterator"
        );

        trace!(
            "BlockIter::prev: starting from current={}, restart_index={}",
            self.current_offset(),
            self.restart_index()
        );

        let original = self.current_offset();

        while self.restart_index() > 0
            && self.get_restart_point(self.restart_index()) >= original
        {
            let new_index = self.restart_index() - 1;
            self.set_restart_index(new_index);
            trace!(
                "BlockIter::prev: scanning backwards, restart_index now={}",
                new_index
            );
        }

        if self.restart_index() == 0 && self.get_restart_point(0) >= original {
            trace!(
                "BlockIter::prev: reached beginning; iterator becomes invalid"
            );
            self.mark_invalid();
            return;
        }

        let idx = self.restart_index();
        self.seek_to_restart_point(idx);

        loop {
            if !self.parse_next_key() {
                trace!(
                    "BlockIter::prev: ParseNextKey returned false while scanning"
                );
                return;
            }

            let next_off = self.next_entry_offset();
            if next_off >= original {
                trace!(
                    "BlockIter::prev: positioned at entry starting before original (next_off={}, original={})",
                    next_off,
                    original
                );
                return;
            }
        }
    }
}

#[cfg(test)]
mod block_iter_prev_tests {
    use super::*;

    #[derive(Clone, Default)]
    struct DummyComparator;

    impl Compare for DummyComparator {
        fn compare(&self, a: &Slice, b: &Slice) -> i32 {
            let a_bytes = unsafe { core::slice::from_raw_parts(*a.data(), *a.size()) };
            let b_bytes = unsafe { core::slice::from_raw_parts(*b.data(), *b.size()) };
            for (aa, bb) in a_bytes.iter().zip(b_bytes.iter()) {
                if aa < bb { return -1; }
                if aa > bb { return 1; }
            }
            a_bytes.len().cmp(&b_bytes.len()) as i32
        }
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
    fn prev_moves_iterator_backwards_one_entry() {
        let block_bytes = build_block_bytes();
        let len         = block_bytes.len();
        let num_restarts =
            u32::from_le_bytes(block_bytes[len - 4..].try_into().unwrap());
        let restart_offset = (len - (1 + num_restarts as usize) * 4) as u32;

        let cmp = bitcoinleveldb_comparator::BytewiseComparatorImpl::default();
        let cmp_ref: &dyn SliceComparator = &cmp;
        let cmp_ptr: *const dyn SliceComparator = cmp_ref as *const dyn SliceComparator;

        let mut iter = BlockIter::new(
            cmp_ptr,
            block_bytes.as_ptr(),
            restart_offset,
            num_restarts,
        );

        iter.seek_to_restart_point(0);
        iter.parse_next_key();
        iter.parse_next_key();
        iter.parse_next_key();

        let current_key = {
            let k = iter.key();
            let bytes =
                unsafe { core::slice::from_raw_parts(*k.data(), *k.size()) };
            String::from_utf8(bytes.to_vec()).expect("valid utf8 in test")
        };
        assert_eq!(current_key, "c");

        trace!("calling prev() from key 'c'");
        iter.prev();

        if iter.valid() {
            let k = iter.key();
            let bytes =
                unsafe { core::slice::from_raw_parts(*k.data(), *k.size()) };
            let key_str =
                String::from_utf8(bytes.to_vec()).expect("valid utf8 in test");
            debug!("after prev(), key='{}'", key_str);
            assert_eq!(key_str, "b");
        } else {
            panic!("iterator became invalid after prev()");
        }
    }
}
