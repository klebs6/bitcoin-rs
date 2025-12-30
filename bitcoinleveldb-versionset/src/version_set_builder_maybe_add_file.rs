// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_builder_maybe_add_file.rs ]
crate::ix!();

impl VersionSetBuilder {

    pub fn maybe_add_file(&mut self, v: *mut Version, level: i32, f: *mut FileMetaData) {
        unsafe {
            debug_assert!(!v.is_null());
            debug_assert!(!f.is_null());

            if level < 0 {
                warn!(
                    level,
                    file_num = *(*f).number(),
                    "VersionSetBuilder::maybe_add_file: negative level; skipping"
                );
                return;
            }

            let lvl = level as usize;
            if lvl >= NUM_LEVELS {
                warn!(
                    level,
                    file_num = *(*f).number(),
                    "VersionSetBuilder::maybe_add_file: out-of-range level; skipping"
                );
                return;
            }

            let file_num = *(*f).number();
            if self
                .level_state_ref(lvl)
                .deleted_files_ref()
                .contains(&file_num)
            {
                trace!(
                    level,
                    file_num,
                    "VersionSetBuilder::maybe_add_file: file is deleted for this level; skipping"
                );
                return;
            }

            let files_vec = &mut (*(*v).files_mut())[lvl];

            if level > 0 && !files_vec.is_empty() {
                // Must not overlap
                let last = *files_vec.last().expect("checked non-empty");

                let vset_ptr: *mut VersionSet = self.vset_ptr();
                debug_assert!(
                    !vset_ptr.is_null(),
                    "VersionSetBuilder::maybe_add_file: vset is null"
                );

                let icmp_ref = (*vset_ptr).icmp();

                let last_largest = (*last).largest().encode();
                let f_smallest = (*f).smallest().encode();

                let r = icmp_ref.compare(&last_largest, &f_smallest);

                assert!(
                    r < 0,
                    "VersionSetBuilder::maybe_add_file: overlapping files in level > 0"
                );
            }

            *(*f).refs_mut() += 1;
            files_vec.push(f);

            trace!(
                level,
                file_num,
                "VersionSetBuilder::maybe_add_file: added file to output Version"
            );
        }
    }
}
