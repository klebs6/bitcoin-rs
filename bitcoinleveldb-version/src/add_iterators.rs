// ---------------- [ File: bitcoinleveldb-version/src/add_iterators.rs ]
crate::ix!();

impl Version {

    /// Append to *iters a sequence of iterators that will yield the contents of
    /// this Version when merged together.
    /// 
    /// REQUIRES: This version has been saved (see VersionSet::SaveTo)
    ///
    pub fn add_iterators(
        &mut self,
        options: &ReadOptions,
        iters:   *mut Vec<*mut LevelDBIterator>,
    ) {
        trace!(
            "Version::add_iterators: enter; iters_ptr={:?}, fill_cache={}, verify_checksums={}",
            iters,
            *options.fill_cache(),
            *options.verify_checksums()
        );

        assert!(
            !iters.is_null(),
            "Version::add_iterators: iters pointer must not be null"
        );

        unsafe {
            let iters_ref: &mut Vec<*mut LevelDBIterator> = &mut *iters;

            // Merge all level‑0 files together since they may overlap.
            let level0_files = &self.files()[0];

            trace!(
                "Version::add_iterators: level 0 has {} files",
                level0_files.len()
            );

            let vset_ptr = self.vset();
            assert!(
                !vset_ptr.is_null(),
                "Version::add_iterators: vset pointer must not be null"
            );

            let table_cache_ptr = (*vset_ptr).table_cache();
            assert!(
                !table_cache_ptr.is_null(),
                "Version::add_iterators: table_cache pointer must not be null"
            );

            // Merge all level zero files together since they may overlap
            for &fptr in level0_files.iter() {
                if fptr.is_null() {
                    warn!(
                        "Version::add_iterators: null FileMetaData pointer encountered at level 0"
                    );
                    continue;
                }

                let f: &FileMetaData = &*fptr;
                let number           = *f.number();
                let file_size        = *f.file_size();

                trace!(
                    "Version::add_iterators: adding iterator for L0 file {} (size={})",
                    number,
                    file_size
                );

                let iter_ptr: *mut LevelDBIterator =
                    (*table_cache_ptr).new_iterator(options, number, file_size, core::ptr::null_mut());

                iters_ref.push(iter_ptr);
            }

            // For levels > 0, use concatenating iterators that walk through
            // the non‑overlapping files in the level, opening them lazily.
            for level in 1..(NUM_LEVELS as i32) {
                let files_level = &self.files()[level as usize];
                if files_level.is_empty() {
                    continue;
                }

                trace!(
                    "Version::add_iterators: level {} has {} files; adding concatenating iterator",
                    level,
                    files_level.len()
                );

                let iter_ptr = self.new_concatenating_iterator(options, level);
                iters_ref.push(iter_ptr);
            }
        }

        trace!("Version::add_iterators: completed");
    }
}

#[cfg(test)]
mod version_add_iterators_behavior_tests {
    use super::*;
    use super::version_test_helpers as helpers;

    #[test]
    #[should_panic(expected = "iters pointer must not be null")]
    fn add_iterators_panics_when_iters_pointer_is_null() {
        let mut version = helpers::build_empty_version();
        let options = ReadOptions::default();
        let null_vec_ptr: *mut Vec<*mut LevelDBIterator> = core::ptr::null_mut();

        version.add_iterators(&options, null_vec_ptr);
    }

    #[traced_test]
    fn add_iterators_method_signature_remains_stable() {
        let _fn_ptr: fn(&mut Version, &ReadOptions, *mut Vec<*mut LevelDBIterator>) =
            Version::add_iterators;
        let _ = _fn_ptr;
    }
}


