// ---------------- [ File: bitcoinleveldb-version/src/level_file_num_iterator.rs ]
crate::ix!();

pub struct LevelFileNumIterator {
    icmp:      *const InternalKeyComparator,
    files:     *const *mut FileMetaData,
    num_files: usize,
    index:     usize,
    value_buf: UnsafeCell<[u8; 16]>,
}

impl LevelFileNumIterator {

    pub fn new(icmp: *const InternalKeyComparator, files: &[*mut FileMetaData]) -> Self {
        let num_files = files.len();
        let files_ptr = if num_files == 0 { core::ptr::null() } else { files.as_ptr() };

        Self {
            icmp,
            files: files_ptr,
            num_files,
            index: num_files,
            value_buf: UnsafeCell::new([0; 16]),
        }
    }

    fn current_file(&self) -> Option<*mut FileMetaData> {
        if self.index < self.num_files && !self.files.is_null() {
            unsafe {
                Some(*self.files.add(self.index))
            }
        } else {
            None
        }
    }

    fn files_slice(&self) -> &[*mut FileMetaData] {
        if self.num_files == 0 || self.files.is_null() {
            &[]
        } else {
            unsafe {
                core::slice::from_raw_parts(self.files, self.num_files)
            }
        }
    }
}

impl LevelDBIteratorInterface for LevelFileNumIterator {}

impl LevelDBIteratorValid for LevelFileNumIterator {
    fn valid(&self) -> bool {
        self.index < self.num_files
    }
}

impl LevelDBIteratorSeekToFirst for LevelFileNumIterator {
    fn seek_to_first(&mut self) {
        debug!("LevelFileNumIterator_seek_to_first");
        if self.num_files == 0 {
            self.index = self.num_files;
        } else {
            self.index = 0;
        }
    }
}

impl LevelDBIteratorSeekToLast for LevelFileNumIterator {
    fn seek_to_last(&mut self) {
        debug!("LevelFileNumIterator_seek_to_last");
        if self.num_files == 0 {
            self.index = self.num_files;
        } else {
            self.index = self.num_files - 1;
        }
    }
}

impl LevelDBIteratorSeek for LevelFileNumIterator {
    fn seek(&mut self, target: &Slice) {
        debug!("LevelFileNumIterator_seek target={:?}", target);

        if self.num_files == 0 {
            self.index = self.num_files;
            return;
        }

        let icmp_ref: &InternalKeyComparator = unsafe {
            &*self.icmp
        };

        let files = self.files_slice();
        let idx = find_file(icmp_ref, files, target);
        self.index = core::cmp::min(idx as usize, self.num_files);
    }
}

impl LevelDBIteratorNext for LevelFileNumIterator {
    fn next(&mut self) {
        debug!("LevelFileNumIterator_next");
        if self.valid() {
            self.index += 1;
        }
    }
}

impl LevelDBIteratorPrev for LevelFileNumIterator {
    fn prev(&mut self) {
        debug!("LevelFileNumIterator_prev");
        if !self.valid() {
            return;
        }

        if self.index == 0 {
            // Past-the-beginning becomes invalid
            self.index = self.num_files;
        } else {
            self.index -= 1;
        }
    }
}

impl LevelDBIteratorKey for LevelFileNumIterator {
    fn key(&self) -> Slice {
        debug!("LevelFileNumIterator_key");

        let f_ptr = match self.current_file() {
            Some(f) => f,
            None => {
                error!("LevelFileNumIterator_key on invalid iterator");
                return Slice::default();
            }
        };

        unsafe {
            // Use the encoded largest internal key for the level entry key,
            // matching the C++ LevelDB implementation.
            (*f_ptr).largest().encode()
        }
    }
}

impl LevelDBIteratorValue for LevelFileNumIterator {
    fn value(&self) -> Slice {
        let f_ptr = match self.current_file() {
            Some(f) => f,
            None => {
                error!("LevelFileNumIterator_value on invalid iterator");
                return Slice::default();
            }
        };

        unsafe {
            let f = &*f_ptr;
            let buf = &mut *self.value_buf.get();

            buf[0..8].copy_from_slice(&encode_fixed64_le(*f.number()));
            buf[8..16].copy_from_slice(&encode_fixed64_le(*f.file_size()));

            Slice::from_ptr_len(buf.as_ptr(), 16)
        }
    }
}

impl LevelDBIteratorStatus for LevelFileNumIterator {
    fn status(&self) -> crate::Status {
        crate::Status::ok()
    }
}

#[cfg(test)]
mod version_level_file_num_iterator_tests {
    use super::*;
    use super::version_test_helpers as helpers;

    fn build_icmp() -> InternalKeyComparator {
        let user_cmp = bitcoinleveldb_comparator::bytewise_comparator();
        InternalKeyComparator::new(user_cmp)
    }

    #[traced_test]
    fn iterator_is_initially_invalid_and_becomes_valid_after_seek_to_first() {
        let icmp = build_icmp();

        let files_vec = vec![
            helpers::build_file_meta_boxed(1, 10, "a", "c"),
            helpers::build_file_meta_boxed(2, 20, "d", "f"),
            helpers::build_file_meta_boxed(3, 30, "g", "i"),
        ];

        let mut iter =
            LevelFileNumIterator::new(&icmp as *const InternalKeyComparator, &files_vec);

        assert!(
            !iter.valid(),
            "Fresh LevelFileNumIterator must start in invalid state"
        );

        iter.seek_to_first();
        assert!(iter.valid(), "Iterator must be valid after seek_to_first");

        unsafe {
            let fptr = files_vec[0];
            let expected = (*fptr).largest().encode();
            let key = iter.key();
            assert_eq!(
                slice_as_bytes(&expected),
                slice_as_bytes(&key),
                "First key should come from the first file's largest key"
            );
        }

        unsafe {
            helpers::free_file_meta_slice(&files_vec);
        }
    }

    #[traced_test]
    fn iterator_traverses_all_files_with_next_and_becomes_invalid_at_end() {
        let icmp = build_icmp();

        let files_vec = vec![
            helpers::build_file_meta_boxed(10, 100, "a", "b"),
            helpers::build_file_meta_boxed(20, 200, "c", "d"),
            helpers::build_file_meta_boxed(30, 300, "e", "f"),
        ];

        let mut iter =
            LevelFileNumIterator::new(&icmp as *const InternalKeyComparator, &files_vec);

        iter.seek_to_first();
        assert!(iter.valid(), "Iterator must be valid after seek_to_first");

        unsafe {
            let f = &*files_vec[0];

            // key() must be the encoded largest internal key
            let expected_key = f.largest().encode();
            let key = iter.key();
            assert_eq!(
                slice_as_bytes(&expected_key),
                slice_as_bytes(&key),
                "Key slice for first file must match its largest key"
            );

            // value() must be the 16-byte file handle
            let value = iter.value();
            let raw = slice_as_bytes(&value);

            assert_eq!(raw.len(), 16, "Value must encode file number and file size");

            assert_eq!(
                decode_fixed64_le(raw.as_ptr()),
                *f.number(),
                "First 8 bytes of value must be file number"
            );

            assert_eq!(
                decode_fixed64_le(raw.as_ptr().add(8)),
                *f.file_size(),
                "Second 8 bytes of value must be file size"
            );
        }

        iter.next();
        assert!(iter.valid(), "Iterator must be valid for second entry");
        unsafe {
            let expected = (*files_vec[1]).largest().encode();
            let key = iter.key();
            assert_eq!(
                slice_as_bytes(&expected),
                slice_as_bytes(&key),
                "Second file key mismatch"
            );
        }

        iter.next();
        assert!(iter.valid(), "Iterator must be valid for third entry");
        unsafe {
            let expected = (*files_vec[2]).largest().encode();
            let key = iter.key();
            assert_eq!(
                slice_as_bytes(&expected),
                slice_as_bytes(&key),
                "Third file key mismatch"
            );
        }

        iter.next();
        assert!(
            !iter.valid(),
            "Iterator must become invalid after moving past last file"
        );

        unsafe {
            helpers::free_file_meta_slice(&files_vec);
        }
    }

    #[traced_test]
    fn seek_positions_to_first_file_with_largest_key_ge_target() {
        let icmp = build_icmp();

        let files_vec = vec![
            helpers::build_file_meta_boxed(100, 10, "a", "c"),
            helpers::build_file_meta_boxed(200, 10, "d", "f"),
            helpers::build_file_meta_boxed(300, 10, "g", "i"),
        ];

        let mut iter =
            LevelFileNumIterator::new(&icmp as *const InternalKeyComparator, &files_vec);

        let target_key = helpers::build_internal_key_from_str("e");
        let target_slice = target_key.encode();

        iter.seek(&target_slice);
        assert!(
            iter.valid(),
            "Iterator must be valid after seeking to key inside known ranges"
        );

        unsafe {
            let expected = (*files_vec[1]).largest().encode();
            let key = iter.key();
            assert_eq!(
                slice_as_bytes(&expected),
                slice_as_bytes(&key),
                "Seek should land on the first file whose largest key >= target"
            );
        }

        unsafe {
            helpers::free_file_meta_slice(&files_vec);
        }
    }

    #[traced_test]
    fn seek_past_all_files_results_in_invalid_iterator() {
        let icmp = build_icmp();

        let files_vec = vec![
            helpers::build_file_meta_boxed(5, 10, "a", "c"),
            helpers::build_file_meta_boxed(6, 10, "d", "f"),
        ];

        let mut iter =
            LevelFileNumIterator::new(&icmp as *const InternalKeyComparator, &files_vec);

        let target_key = helpers::build_internal_key_from_str("z");
        let target_slice = target_key.encode();

        iter.seek(&target_slice);
        assert!(
            !iter.valid(),
            "Seeking past the largest key must produce an invalid iterator"
        );

        unsafe {
            helpers::free_file_meta_slice(&files_vec);
        }
    }

    #[traced_test]
    fn key_and_value_on_invalid_iterator_return_empty_slice() {
        let icmp = build_icmp();

        let files_vec: Vec<*mut FileMetaData> = Vec::new();
        let iter = LevelFileNumIterator::new(&icmp as *const InternalKeyComparator, &files_vec);

        assert!(
            !iter.valid(),
            "Iterator built with zero files must start invalid"
        );

        let key_slice = iter.key();
        let value_slice = iter.value();

        assert_eq!(
            *key_slice.size(),
            0,
            "Key from invalid iterator must be an empty slice"
        );
        assert_eq!(
            *value_slice.size(),
            0,
            "Value from invalid iterator must be an empty slice"
        );
    }
}

#[cfg(test)]
mod version_create_concatenating_iterator_pipeline_tests {
    use super::*;
    use super::version_test_helpers as helpers;

    struct SingleEntryIter {
        key: Vec<u8>,
        value: Vec<u8>,
        valid: bool,
    }

    impl LevelDBIteratorInterface for SingleEntryIter {}

    impl LevelDBIteratorValid for SingleEntryIter {
        fn valid(&self) -> bool {
            self.valid
        }
    }

    impl LevelDBIteratorSeekToFirst for SingleEntryIter {
        fn seek_to_first(&mut self) {
            self.valid = true;
        }
    }

    impl LevelDBIteratorSeekToLast for SingleEntryIter {
        fn seek_to_last(&mut self) {
            self.valid = true;
        }
    }

    impl LevelDBIteratorSeek for SingleEntryIter {
        fn seek(&mut self, _target: &Slice) {
            self.valid = true;
        }
    }

    impl LevelDBIteratorNext for SingleEntryIter {
        fn next(&mut self) {
            self.valid = false;
        }
    }

    impl LevelDBIteratorPrev for SingleEntryIter {
        fn prev(&mut self) {
            self.valid = false;
        }
    }

    impl LevelDBIteratorStatus for SingleEntryIter {
        fn status(&self) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl LevelDBIteratorKey for SingleEntryIter {
        fn key(&self) -> Slice {
            if self.valid {
                Slice::from(self.key.as_slice())
            } else {
                Slice::default()
            }
        }
    }

    impl LevelDBIteratorValue for SingleEntryIter {
        fn value(&self) -> Slice {
            if self.valid {
                Slice::from(self.value.as_slice())
            } else {
                Slice::default()
            }
        }
    }

    fn build_icmp() -> InternalKeyComparator {
        let user_cmp = bitcoinleveldb_comparator::bytewise_comparator();
        InternalKeyComparator::new(user_cmp)
    }

    #[traced_test]
    fn two_level_iterator_pipeline_decodes_file_handle_and_yields_expected_entries() {
        let icmp = build_icmp();

        let files_vec = vec![
            helpers::build_file_meta_boxed(10, 100, "a", "b"),
            helpers::build_file_meta_boxed(20, 200, "c", "d"),
            helpers::build_file_meta_boxed(30, 300, "e", "f"),
        ];

        let index_iter: Box<dyn LevelDBIteratorInterface> =
            Box::new(LevelFileNumIterator::new(
                &icmp as *const InternalKeyComparator,
                &files_vec,
            ));

        let block_fn: BlockFunction = |_arg, _opts, handle| {
            let raw = handle.as_bytes();

            assert_eq!(
                raw.len(),
                16,
                "LevelFileNumIterator value() must be a 16-byte file handle"
            );

            let file_number = unsafe { decode_fixed64_le(raw.as_ptr()) };
            let file_size = unsafe { decode_fixed64_le(unsafe { raw.as_ptr().add(8) }) };

            let key = format!("file:{file_number}").into_bytes();
            let value = format!("size:{file_size}").into_bytes();

            Some(Box::new(SingleEntryIter {
                key,
                value,
                valid: false,
            }))
        };

        let two_level: Box<dyn LevelDBIteratorInterface> =
            new_two_level_iterator(
                index_iter,
                block_fn,
                core::ptr::null_mut(),
                &ReadOptions::default(),
            );

        let mut iter = LevelDBIterator::new(Some(two_level));

        iter.seek_to_first();
        assert!(iter.valid());
        assert_eq!(iter.key().to_string(), "file:10");
        assert_eq!(iter.value().to_string(), "size:100");

        iter.next();
        assert!(iter.valid());
        assert_eq!(iter.key().to_string(), "file:20");
        assert_eq!(iter.value().to_string(), "size:200");

        iter.next();
        assert!(iter.valid());
        assert_eq!(iter.key().to_string(), "file:30");
        assert_eq!(iter.value().to_string(), "size:300");

        iter.next();
        assert!(!iter.valid());

        unsafe {
            helpers::free_file_meta_slice(&files_vec);
        }
    }
}
