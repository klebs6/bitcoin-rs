// ---------------- [ File: bitcoinleveldb-block/src/block_iter_seek.rs ]
crate::ix!();

impl BlockIter {

    pub fn seek(&mut self, target: &Slice) {
        trace!(
            "BlockIter::seek: target_len={}, restarts_offset={}, num_restarts={}",
            *target.size(),
            self.restarts_offset(),
            self.num_restarts()
        );

        if self.num_restarts() == 0 {
            self.mark_invalid();
            return;
        }

        let mut left:  u32 = 0;
        let mut right: u32 = self.num_restarts() - 1;

        while left < right {
            let mid           = (left + right + 1) / 2;
            let region_offset = self.get_restart_point(mid);

            unsafe {
                let p     = self.data_ptr().add(region_offset as usize);
                let limit = self.data_ptr().add(self.restarts_offset() as usize);

                let mut shared:       u32 = 0;
                let mut non_shared:   u32 = 0;
                let mut value_length: u32 = 0;

                let key_ptr = crate::decode_entry(
                    p,
                    limit,
                    &mut shared,
                    &mut non_shared,
                    &mut value_length,
                );

                if key_ptr.is_null() || shared != 0 {
                    warn!(
                        "BlockIter::seek: corruption while scanning restart array (mid={}, shared={})",
                        mid,
                        shared
                    );
                    self.corruption_error();
                    return;
                }

                let mid_key = Slice::from_ptr_len(key_ptr, non_shared as usize);
                let cmp     = self.compare(&mid_key, target);

                trace!(
                    "BlockIter::seek: mid={}, region_offset={}, cmp={}",
                    mid,
                    region_offset,
                    cmp
                );

                if cmp < 0 {
                    left = mid;
                } else {
                    if mid == 0 {
                        right = 0;
                    } else {
                        right = mid - 1;
                    }
                }
            }
        }

        trace!(
            "BlockIter::seek: restart linear scan starting at {}",
            left
        );

        self.seek_to_restart_point(left);

        loop {
            if !self.parse_next_key() {
                trace!(
                    "BlockIter::seek: ParseNextKey returned false during linear scan"
                );
                return;
            }

            let k   = self.key();
            let cmp = self.compare(&k, target);
            trace!(
                "BlockIter::seek: linear scan compare result={} (key_len={})",
                cmp,
                *k.size()
            );

            if cmp >= 0 {
                return;
            }
        }
    }
}

#[cfg(test)]
mod block_iter_seek_tests {
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
            &Slice::from("c".as_bytes()),
            &Slice::from("v3".as_bytes()),
        );
        builder.add(
            &Slice::from("e".as_bytes()),
            &Slice::from("v5".as_bytes()),
        );

        let slice = builder.finish();
        unsafe {
            let ptr = *slice.data();
            let len = *slice.size();
            core::slice::from_raw_parts(ptr, len).to_vec()
        }
    }

    #[traced_test]
    fn seek_positions_on_exact_key_or_next_greater() {
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

        let target = Slice::from("b".as_bytes());
        trace!("seeking to target 'b'");
        iter.seek(&target);

        if iter.valid() {
            let k = iter.key();
            let key_bytes =
                unsafe { core::slice::from_raw_parts(*k.data(), *k.size()) };
            let key_str =
                String::from_utf8(key_bytes.to_vec()).expect("valid utf8");
            debug!("seek('b') landed on key='{}'", key_str);
            assert_eq!(key_str, "c");
        } else {
            panic!("iterator invalid after seek");
        }

        let target_exact = Slice::from("e".as_bytes());
        trace!("seeking to exact key 'e'");
        iter.seek(&target_exact);

        if iter.valid() {
            let k = iter.key();
            let key_bytes =
                unsafe { core::slice::from_raw_parts(*k.data(), *k.size()) };
            let key_str =
                String::from_utf8(key_bytes.to_vec()).expect("valid utf8");
            debug!("seek('e') landed on key='{}'", key_str);
            assert_eq!(key_str, "e");
        } else {
            panic!("iterator invalid after seek to exact key");
        }
    }
}
