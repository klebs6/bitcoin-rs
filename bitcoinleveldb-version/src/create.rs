// ---------------- [ File: bitcoinleveldb-version/src/create.rs ]
crate::ix!();

impl Version {
    
    pub fn new_concatenating_iterator(
        &self,
        options: &ReadOptions,
        level:   i32,
    ) -> *mut LevelDBIterator {
        trace!(
            "Version::new_concatenating_iterator: enter; level={}",
            level
        );

        assert!(
            level >= 0 && (level as usize) < NUM_LEVELS,
            "Version::new_concatenating_iterator: level {} out of range",
            level
        );

        unsafe {
            let vset_ptr = self.vset();
            assert!(
                !vset_ptr.is_null(),
                "Version::new_concatenating_iterator: vset pointer must not be null"
            );

            let icmp: &InternalKeyComparator = (*vset_ptr).icmp();
            let files_level: &Vec<*mut FileMetaData> =
                &self.files()[level as usize];

            trace!(
                "Version::new_concatenating_iterator: level {} file count={}",
                level,
                files_level.len()
            );

            // Index iterator over the files in this level.
            let index_iter_impl = LevelFileNumIterator::new(icmp, files_level);
            let index_iter_iface: Box<dyn LevelDBIteratorInterface> =
                Box::new(index_iter_impl);

            let table_cache_ptr = (*vset_ptr).table_cache();
            assert!(
                !table_cache_ptr.is_null(),
                "Version::new_concatenating_iterator: table_cache pointer must not be null"
            );

            let arg: *mut c_void = table_cache_ptr as *mut c_void;

            // Two‑level iterator: outer iterates over files, inner over file contents.
            let block_fn: BlockFunction = |arg_ptr, read_opts, index_value| {
                let raw_iter = get_file_iterator(arg_ptr, read_opts, index_value);
                if raw_iter.is_null() {
                    trace!(
                        "Version::new_concatenating_iterator:block_fn: get_file_iterator returned null"
                    );
                    None
                } else {
                    let boxed: Box<LevelDBIterator> = Box::from_raw(raw_iter);
                    let iface: Box<dyn LevelDBIteratorInterface> = boxed;
                    Some(iface)
                }
            };

            let two_level_iface: Box<dyn LevelDBIteratorInterface> =
                bitcoinleveldb_duplex::new_two_level_iterator(
                    index_iter_iface,
                    block_fn,
                    arg,
                    options,
                );

            let wrapper = LevelDBIterator::new(Some(two_level_iface));
            let raw_ptr = Box::into_raw(Box::new(wrapper));

            trace!(
                "Version::new_concatenating_iterator: returning iterator @ {:p}",
                raw_ptr
            );

            raw_ptr
        }
    }
}

#[cfg(test)]
mod version_create_concatenating_iterator_tests {
    use super::*;
    use super::version_test_helpers as helpers;

    #[test]
    #[should_panic(expected = "level -1 out of range")]
    fn new_concatenating_iterator_panics_on_negative_level() {
        let version = helpers::build_empty_version();
        let options = ReadOptions::default();
        let _ = version.new_concatenating_iterator(&options, -1);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn new_concatenating_iterator_panics_on_level_at_or_above_num_levels() {
        let version = helpers::build_empty_version();
        let options = ReadOptions::default();
        let _ = version.new_concatenating_iterator(&options, NUM_LEVELS as i32);
    }

    #[traced_test]
    fn new_concatenating_iterator_signature_is_stable() {
        let _fn_ptr: fn(&Version, &ReadOptions, i32) -> *mut LevelDBIterator =
            Version::new_concatenating_iterator;
        let _ = _fn_ptr;
    }
}


