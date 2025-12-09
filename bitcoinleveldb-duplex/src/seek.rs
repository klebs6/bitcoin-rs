// ---------------- [ File: bitcoinleveldb-duplex/src/seek.rs ]
crate::ix!();

impl LevelDBIteratorSeek for TwoLevelIterator {

    fn seek(&mut self, target: &Slice) {
        trace!(
            "TwoLevelIterator::seek: target_size={}, index_valid_before={}, data_valid_before={}",
            target.size(),
            self.index_iter().valid(),
            self.data_iter().valid(),
        );

        self.index_iter_mut().seek(target);
        self.init_data_block();

        if self.data_iter().iter().is_some() {
            self.data_iter_mut().seek(target);
        }

        self.skip_empty_data_blocks_forward();

        trace!(
            "TwoLevelIterator::seek: after; index_valid={}, data_valid={}",
            self.index_iter().valid(),
            self.data_iter().valid(),
        );
    }
}

impl LevelDBIteratorSeekToFirst for TwoLevelIterator {
   
    fn seek_to_first(&mut self) {
        trace!(
            "TwoLevelIterator::seek_to_first: index_valid_before={}, data_valid_before={}",
            self.index_iter().valid(),
            self.data_iter().valid(),
        );

        self.index_iter_mut().seek_to_first();
        self.init_data_block();

        if self.data_iter().iter().is_some() {
            self.data_iter_mut().seek_to_first();
        }

        self.skip_empty_data_blocks_forward();

        trace!(
            "TwoLevelIterator::seek_to_first: after; index_valid={}, data_valid={}",
            self.index_iter().valid(),
            self.data_iter().valid(),
        );
    }
}

impl LevelDBIteratorSeekToLast for TwoLevelIterator {
   
    fn seek_to_last(&mut self) {
        trace!(
            "TwoLevelIterator::seek_to_last: index_valid_before={}, data_valid_before={}",
            self.index_iter().valid(),
            self.data_iter().valid(),
        );

        self.index_iter_mut().seek_to_last();
        self.init_data_block();

        if self.data_iter().iter().is_some() {
            self.data_iter_mut().seek_to_last();
        }

        self.skip_empty_data_blocks_backward();

        trace!(
            "TwoLevelIterator::seek_to_last: after; index_valid={}, data_valid={}",
            self.index_iter().valid(),
            self.data_iter().valid(),
        );
    }
}

#[cfg(test)]
mod two_level_iterator_seek_tests {
    use super::*;
    use core::ffi::c_void;

    fn test_block_function(
        arg: *mut c_void,
        _options: &ReadOptions,
        handle: &Slice,
    ) -> Option<Box<dyn LevelDBIteratorInterface>> {
        let handle_str = handle.to_string();
        let block_index: usize = handle_str
            .parse()
            .expect("handle should be a valid ASCII usize");

        let blocks_ptr = arg as *mut Vec<Vec<(Vec<u8>, Vec<u8>)>>;
        assert!(
            !blocks_ptr.is_null(),
            "blocks pointer passed to test_block_function must not be null"
        );

        let blocks = unsafe { &mut *blocks_ptr };

        if block_index >= blocks.len() {
            return None;
        }

        let entries_ref = &blocks[block_index];
        if entries_ref.is_empty() {
            let iter = MockStubIterator::new_empty();
            return Some(Box::new(iter));
        }

        let mut pairs: Vec<(&[u8], &[u8])> = Vec::with_capacity(entries_ref.len());
        for (k, v) in entries_ref.iter() {
            pairs.push((k.as_slice(), v.as_slice()));
        }

        let iter = MockStubIterator::new_with_entries(&pairs);
        Some(Box::new(iter))
    }

    fn make_index_iterator_for_blocks(
        blocks: &Vec<Vec<(Vec<u8>, Vec<u8>)>>,
    ) -> Box<dyn LevelDBIteratorInterface> {
        let mut pairs: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();

        for (i, block) in blocks.iter().enumerate() {
            if block.is_empty() {
                continue;
            }
            let (last_key, _) = block
                .last()
                .expect("non-empty block must have at least one entry");
            let handle = i.to_string().into_bytes();
            pairs.push((last_key.clone(), handle));
        }

        let pairs_slice: Vec<(&[u8], &[u8])> = pairs
            .iter()
            .map(|(k, v)| (k.as_slice(), v.as_slice()))
            .collect();

        let mut iter = MockStubIterator::new_with_entries(&pairs_slice);
        iter.seek_to_first();
        Box::new(iter)
    }

    fn build_two_level_from_blocks(
        blocks: &mut Vec<Vec<(Vec<u8>, Vec<u8>)>>,
    ) -> TwoLevelIterator {
        let arg: *mut c_void =
            blocks as *mut Vec<Vec<(Vec<u8>, Vec<u8>)>> as *mut c_void;
        let index_iter = make_index_iterator_for_blocks(blocks);
        let options = ReadOptions::default();

        TwoLevelIterator::new(index_iter, test_block_function, arg, options)
    }

    #[traced_test]
    fn seek_returns_first_entry_when_target_precedes_all_keys() {
        let mut blocks: Vec<Vec<(Vec<u8>, Vec<u8>)>> = vec![
            vec![
                (b"a".to_vec(), b"va".to_vec()),
                (b"b".to_vec(), b"vb".to_vec()),
            ],
            vec![
                (b"d".to_vec(), b"vd".to_vec()),
                (b"e".to_vec(), b"ve".to_vec()),
            ],
        ];

            let mut two = build_two_level_from_blocks(&mut blocks);

            let target = Slice::from("0");
            two.seek(&target);

            assert!(
                two.valid(),
                "iterator should be valid when seeking to a key before the first block"
            );
            assert_eq!(two.key().to_string(), "a");
            assert_eq!(two.value().to_string(), "va");
    }

    #[traced_test]
    fn seek_finds_exact_and_next_keys_across_blocks() {
        let mut blocks: Vec<Vec<(Vec<u8>, Vec<u8>)>> = vec![
            vec![
                (b"a".to_vec(), b"va".to_vec()),
                (b"b".to_vec(), b"vb".to_vec()),
            ],
            vec![
                (b"d".to_vec(), b"vd".to_vec()),
                (b"e".to_vec(), b"ve".to_vec()),
            ],
        ];

            let mut two = build_two_level_from_blocks(&mut blocks);

            let target_exact = Slice::from("b");
            two.seek(&target_exact);
            assert!(
                two.valid(),
                "iterator should be valid when seeking to an exact key present in the first block"
            );
            assert_eq!(two.key().to_string(), "b");
            assert_eq!(two.value().to_string(), "vb");

            let target_between = Slice::from("c");
            two.seek(&target_between);
            assert!(
                two.valid(),
                "iterator should be valid when seeking between blocks"
            );
            assert_eq!(two.key().to_string(), "d");
            assert_eq!(two.value().to_string(), "vd");
    }

    #[traced_test]
    fn seek_past_last_key_yields_invalid_iterator() {
        let mut blocks: Vec<Vec<(Vec<u8>, Vec<u8>)>> = vec![
            vec![
                (b"a".to_vec(), b"va".to_vec()),
                (b"b".to_vec(), b"vb".to_vec()),
            ],
            vec![
                (b"d".to_vec(), b"vd".to_vec()),
                (b"e".to_vec(), b"ve".to_vec()),
            ],
        ];

            let mut two = build_two_level_from_blocks(&mut blocks);

            let target = Slice::from("z");
            two.seek(&target);

            assert!(
                !two.valid(),
                "iterator must become invalid when seeking past the last key in the last block"
            );
    }

    #[traced_test]
    fn seek_to_first_and_seek_to_last_position_on_extreme_entries() {
        let mut blocks: Vec<Vec<(Vec<u8>, Vec<u8>)>> = vec![
            vec![
                (b"a".to_vec(), b"va".to_vec()),
                (b"b".to_vec(), b"vb".to_vec()),
            ],
            vec![
                (b"d".to_vec(), b"vd".to_vec()),
                (b"e".to_vec(), b"ve".to_vec()),
            ],
        ];

            let mut two = build_two_level_from_blocks(&mut blocks);

            two.seek_to_first();
            assert!(
                two.valid(),
                "seek_to_first should produce a valid iterator when any entries are present"
            );
            assert_eq!(two.key().to_string(), "a");
            assert_eq!(two.value().to_string(), "va");

            two.seek_to_last();
            assert!(
                two.valid(),
                "seek_to_last should produce a valid iterator when any entries are present"
            );
            assert_eq!(two.key().to_string(), "e");
            assert_eq!(two.value().to_string(), "ve");
    }
}
