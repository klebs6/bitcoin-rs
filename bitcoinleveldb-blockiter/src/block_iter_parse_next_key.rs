// ---------------- [ File: bitcoinleveldb-blockiter/src/block_iter_parse_next_key.rs ]
crate::ix!();

impl BlockIter {

    pub fn parse_next_key(&mut self) -> bool {
        trace!(
            "BlockIter::parse_next_key: current={}, restart_index={}",
            self.current_offset(),
            self.restart_index()
        );

        let next_off = self.next_entry_offset();
        self.set_current_offset(next_off);

        unsafe {
            let p     = self.data_ptr().add(self.current_offset() as usize);
            let limit = self.data_ptr().add(self.restarts_offset() as usize);

            if p >= limit {
                trace!(
                    "BlockIter::parse_next_key: reached end (p >= limit); marking iterator invalid"
                );
                self.mark_invalid();
                return false;
            }

            let mut shared:       u32 = 0;
            let mut non_shared:   u32 = 0;
            let mut value_length: u32 = 0;

            let key_ptr = decode_entry(
                p, 
                limit, 
                &mut shared, 
                &mut non_shared, 
                &mut value_length
            );

            let key_len = self.key_buffer().len();

            if key_ptr.is_null() || (key_len as u32) < shared {
                warn!(
                    "BlockIter::parse_next_key: corruption (key_ptr_null={}, key_len={}, shared={})",
                    key_ptr.is_null(),
                    key_len,
                    shared
                );
                self.corruption_error();
                return false;
            }

            let key_buf = self.key_buffer_mut();
            key_buf.truncate(shared as usize);

            let key_delta =
                core::slice::from_raw_parts(key_ptr, non_shared as usize);
            key_buf.push_str(core::str::from_utf8_unchecked(key_delta));

            let value_ptr = key_ptr.add(non_shared as usize);
            let new_value =
                Slice::from_ptr_len(value_ptr, value_length as usize);
            *self.value_slice_mut() = new_value;

            trace!(
                "BlockIter::parse_next_key: new key_len={}, value_len={}, current_offset={}",
                self.key_buffer().len(),
                *self.value_slice().size(),
                self.current_offset()
            );

            while self.restart_index() + 1 < self.num_restarts()
                && self.get_restart_point(self.restart_index() + 1) < self.current_offset()
            {
                let new_index = self.restart_index() + 1;
                self.set_restart_index(new_index);
                trace!(
                    "BlockIter::parse_next_key: advance restart_index to {}",
                    new_index
                );
            }

            true
        }
    }
}

#[cfg(test)]
mod block_iter_parse_next_key_tests {
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

        let slice = builder.finish();
        unsafe {
            let ptr = *slice.data();
            let len = *slice.size();
            core::slice::from_raw_parts(ptr, len).to_vec()
        }
    }

    #[traced_test]
    fn parse_next_key_iterates_over_all_entries() {
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

        let mut seen = Vec::<(String, String)>::new();

        while iter.parse_next_key() && iter.next_entry_offset() < iter.restarts_offset()
        {
            if !iter.valid() {
                break;
            }

            let k = iter.key();
            let v = iter.value();

            let k_str = unsafe {
                let bytes = core::slice::from_raw_parts(*k.data(), *k.size());
                core::str::from_utf8_unchecked(bytes).to_string()
            };
            let v_str = unsafe {
                let bytes = core::slice::from_raw_parts(*v.data(), *v.size());
                core::str::from_utf8_unchecked(bytes).to_string()
            };

            trace!("iterated entry key='{}', value='{}'", k_str, v_str);
            seen.push((k_str, v_str));
        }

        debug!("seen entries: {:?}", seen);
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0], ("a".to_string(), "v1".to_string()));
        assert_eq!(seen[1], ("b".to_string(), "v2".to_string()));
    }
}
