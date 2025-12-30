// ---------------- [ File: bitcoinleveldb-versionsetbuilder/src/version_set_builder_apply.rs ]
crate::ix!();

impl VersionSetBuilder {

    /**
      | Apply all of the edits in *edit to the
      | current state.
      |
      */
    pub fn apply(&mut self, edit: *mut VersionEdit) {
        trace!(
            edit_ptr = ?edit,
            "VersionSetBuilder::apply: applying VersionEdit"
        );

        if edit.is_null() {
            error!("VersionSetBuilder::apply: edit is null; skipping");
            return;
        }

        let vset_ptr: *mut VersionSet = self.vset_ptr();
        if vset_ptr.is_null() {
            error!("VersionSetBuilder::apply: vset is null; skipping");
            return;
        }

        unsafe {
            // Update compaction pointers.
            for (level, key) in (*edit).compact_pointers().iter() {
                let lvl_i32 = *level;
                if lvl_i32 < 0 {
                    warn!(
                        level = lvl_i32,
                        "VersionSetBuilder::apply: negative level in compact_pointers; skipping"
                    );
                    continue;
                }

                let lvl = lvl_i32 as usize;
                if lvl >= NUM_LEVELS {
                    warn!(
                        level = lvl_i32,
                        "VersionSetBuilder::apply: out-of-range level in compact_pointers; skipping"
                    );
                    continue;
                }

                let encoded = key.encode();
                (*vset_ptr).compact_pointer_mut()[lvl] = encoded.to_string();

                debug!(
                    level = lvl_i32,
                    "VersionSetBuilder::apply: updated compact pointer"
                );
            }

            // Mark deleted files.
            for (level, file_num) in (*edit).deleted_files().iter() {
                let lvl_i32 = *level;
                if lvl_i32 < 0 {
                    warn!(
                        level = lvl_i32,
                        file_num = *file_num,
                        "VersionSetBuilder::apply: negative level in deleted_files; skipping"
                    );
                    continue;
                }

                let lvl = lvl_i32 as usize;
                if lvl >= NUM_LEVELS {
                    warn!(
                        level = lvl_i32,
                        file_num = *file_num,
                        "VersionSetBuilder::apply: out-of-range level in deleted_files; skipping"
                    );
                    continue;
                }

                self.level_state_mut_ref(lvl)
                    .deleted_files_mut_ref()
                    .insert(*file_num);

                debug!(
                    level = lvl_i32,
                    file_num = *file_num,
                    "VersionSetBuilder::apply: recorded deletion"
                );
            }

            // Add new files.
            for (level, file_meta) in (*edit).new_files().iter() {
                let lvl_i32 = *level;
                if lvl_i32 < 0 {
                    warn!(
                        level = lvl_i32,
                        "VersionSetBuilder::apply: negative level in new_files; skipping"
                    );
                    continue;
                }

                let lvl = lvl_i32 as usize;
                if lvl >= NUM_LEVELS {
                    warn!(
                        level = lvl_i32,
                        "VersionSetBuilder::apply: out-of-range level in new_files; skipping"
                    );
                    continue;
                }

                let file_num = *file_meta.number();
                let file_size = *file_meta.file_size();

                let mut f = Box::new(FileMetaData::default());

                *f.number_mut() = file_num;
                *f.file_size_mut() = file_size;
                *f.smallest_mut() = (*file_meta.smallest()).clone();
                *f.largest_mut() = (*file_meta.largest()).clone();

                *f.refs_mut() = 1;

                // We arrange to automatically compact this file after
                // a certain number of seeks.  Let's assume:
                //   (1) One seek costs 10ms
                //   (2) Writing or reading 1MB costs 10ms (100MB/s)
                //   (3) A compaction of 1MB does 25MB of IO:
                //         1MB read from this level
                //         10-12MB read from next level (boundaries may be misaligned)
                //         10-12MB written to next level
                // This implies that 25 seeks cost the same as the compaction
                // of 1MB of data.  I.e., one seek costs approximately the
                // same as the compaction of 40KB of data.  We are a little
                // conservative and allow approximately one seek for every 16KB
                // of data before triggering a compaction.
                let mut allowed_seeks = (file_size / 16384) as i32;
                if allowed_seeks < 100 {
                    allowed_seeks = 100;
                }
                *f.allowed_seeks_mut() = allowed_seeks;

                // If we are re-adding a file that was previously deleted at this level,
                // clear the deletion marker.
                self.level_state_mut_ref(lvl)
                    .deleted_files_mut_ref()
                    .remove(&file_num);

                let f_ptr = Box::into_raw(f);

                let added_ptr = self.level_state_ref(lvl).added_files_ptr();
                if added_ptr.is_null() {
                    error!(
                        level = lvl_i32,
                        file_num,
                        "VersionSetBuilder::apply: added_files set pointer is null; dropping file metadata"
                    );
                    drop(Box::from_raw(f_ptr));
                    continue;
                }

                (*added_ptr).insert(f_ptr);

                info!(
                    level = lvl_i32,
                    file_num,
                    file_size,
                    allowed_seeks,
                    "VersionSetBuilder::apply: queued new file for inclusion"
                );
            }
        }
    }
}
