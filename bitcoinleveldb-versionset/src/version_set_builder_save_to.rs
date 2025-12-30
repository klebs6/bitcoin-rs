// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_builder_save_to.rs ]
crate::ix!();

impl VersionSetBuilder {

    /// Save the current state in *v.
    /// 
    pub fn save_to(&mut self, v: *mut Version) {
        trace!(
            v_ptr = ?v,
            vset_ptr = ?self.vset_ptr(),
            base_ptr = ?self.base_ptr(),
            "VersionSetBuilder::save_to: enter"
        );

        assert!(!v.is_null(), "VersionSetBuilder::save_to: v is null");

        let vset_ptr: *mut VersionSet = self.vset_ptr();
        let base_ptr: *mut Version = self.base_ptr();

        assert!(
            !vset_ptr.is_null(),
            "VersionSetBuilder::save_to: vset is null"
        );
        assert!(
            !base_ptr.is_null(),
            "VersionSetBuilder::save_to: base is null"
        );

        let icmp_ptr: *const InternalKeyComparator =
            unsafe { (*vset_ptr).icmp() as *const InternalKeyComparator };

        assert!(
            !icmp_ptr.is_null(),
            "VersionSetBuilder::save_to: icmp_ptr is null"
        );

        let cmp_files = |a: *mut FileMetaData, b: *mut FileMetaData| -> core::cmp::Ordering {
            unsafe {
                debug_assert!(!a.is_null());
                debug_assert!(!b.is_null());

                let icmp_ref: &InternalKeyComparator = &*icmp_ptr;

                let r = icmp_ref.compare_internal_key((*a).smallest(), (*b).smallest());
                if r < 0 {
                    return core::cmp::Ordering::Less;
                }
                if r > 0 {
                    return core::cmp::Ordering::Greater;
                }

                let a_num = *(*a).number();
                let b_num = *(*b).number();
                a_num.cmp(&b_num)
            }
        };

        unsafe {
            let icmp_ref: &InternalKeyComparator = &*icmp_ptr;

            for level in 0..NUM_LEVELS {
                let level_i32: i32 = level as i32;

                // Merge the set of added files with the set of pre-existing files.
                // Drop any deleted files. Store the result in *v.
                let base_files: &Vec<*mut FileMetaData> = &(*base_ptr).files()[level];

                let added_ptr: *mut VersionSetBuilderFileSet =
                    self.level_state_ref(level).added_files_ptr();

                assert!(
                    !added_ptr.is_null(),
                    "VersionSetBuilder::save_to: added_files ptr is null at level {}",
                    level_i32
                );

                let added_set: &VersionSetBuilderFileSet = &*added_ptr;

                let mut added_vec: Vec<*mut FileMetaData> = added_set.iter().copied().collect();
                added_vec.sort_by(|a_ref, b_ref| cmp_files(*a_ref, *b_ref));

                {
                    let files_level: &mut Vec<*mut FileMetaData> = &mut (*(*v).files_mut())[level];
                    files_level.reserve(base_files.len() + added_vec.len());
                }

                trace!(
                    level = level_i32,
                    base_files_len = base_files.len(),
                    added_files_len = added_vec.len(),
                    deleted_files_len = self.level_state_ref(level).deleted_files_ref().len(),
                    "VersionSetBuilder::save_to: merging"
                );

                let mut base_index: usize = 0;
                let base_end: usize = base_files.len();

                for (aidx, &added_file) in added_vec.iter().enumerate() {
                    assert!(
                        !added_file.is_null(),
                        "VersionSetBuilder::save_to: null added_file at level {} index {}",
                        level_i32,
                        aidx
                    );

                    // upper_bound(base_index..base_end, added_file, cmp_files):
                    // first base_files[pos] such that added_file < base_files[pos]
                    let mut lo: usize = base_index;
                    let mut hi: usize = base_end;

                    while lo < hi {
                        let mid: usize = (lo + hi) / 2;
                        let mid_ptr: *mut FileMetaData = base_files[mid];

                        assert!(
                            !mid_ptr.is_null(),
                            "VersionSetBuilder::save_to: null base_files[{}] at level {}",
                            mid,
                            level_i32
                        );

                        if cmp_files(added_file, mid_ptr) == core::cmp::Ordering::Less {
                            hi = mid;
                        } else {
                            lo = mid + 1;
                        }
                    }

                    let bpos: usize = lo;

                    for i in base_index..bpos {
                        self.maybe_add_file(v, level_i32, base_files[i]);
                    }
                    base_index = bpos;

                    self.maybe_add_file(v, level_i32, added_file);
                }

                // Add remaining base files
                for i in base_index..base_end {
                    self.maybe_add_file(v, level_i32, base_files[i]);
                }

                // Debug-only: verify no overlap in levels > 0
                #[cfg(debug_assertions)]
                {
                    if level > 0 {
                        let files_level: &Vec<*mut FileMetaData> = &(*v).files()[level];

                        for i in 1..files_level.len() {
                            let prev_ptr: *mut FileMetaData = files_level[i - 1];
                            let cur_ptr: *mut FileMetaData = files_level[i];

                            assert!(
                                !prev_ptr.is_null() && !cur_ptr.is_null(),
                                "VersionSetBuilder::save_to: null FileMetaData pointer in v.files()[{}] at i={}",
                                level,
                                i
                            );

                            let prev: &FileMetaData = &*prev_ptr;
                            let cur: &FileMetaData = &*cur_ptr;

                            let r: i32 =
                                icmp_ref.compare_internal_key(prev.largest(), cur.smallest());

                            if r >= 0 {
                                error!(
                                    level,
                                    i,
                                    prev_largest = %prev.largest().debug_string(),
                                    cur_smallest = %cur.smallest().debug_string(),
                                    prev_file_number = *prev.number(),
                                    cur_file_number = *cur.number(),
                                    "VersionSetBuilder::save_to: overlapping ranges in same level"
                                );
                                panic!("overlapping ranges in same level");
                            }
                        }
                    }
                }
            }
        }

        trace!("VersionSetBuilder::save_to: exit");

        /*
            BySmallestKeyComparator cmp;
        cmp.internal_comparator = &vset_->icmp_;
        for (int level = 0; level < config::NUM_LEVELS; level++) {
          // Merge the set of added files with the set of pre-existing files.
          // Drop any deleted files.  Store the result in *v.
          const std::vector<FileMetaData*>& base_files = base_->files_[level];
          std::vector<FileMetaData*>::const_iterator base_iter = base_files.begin();
          std::vector<FileMetaData*>::const_iterator base_end = base_files.end();
          const VersionSetBuilderFileSet* added_files = levels_[level].added_files;
          v->files_[level].reserve(base_files.size() + added_files->size());
          for (const auto& added_file : *added_files) {
            // Add all smaller files listed in base_
            for (std::vector<FileMetaData*>::const_iterator bpos =
                     std::upper_bound(base_iter, base_end, added_file, cmp);
                 base_iter != bpos; ++base_iter) {
              MaybeAddFile(v, level, *base_iter);
            }

            MaybeAddFile(v, level, added_file);
          }

          // Add remaining base files
          for (; base_iter != base_end; ++base_iter) {
            MaybeAddFile(v, level, *base_iter);
          }

    #ifndef NDEBUG
          // Make sure there is no overlap in levels > 0
          if (level > 0) {
            for (uint32_t i = 1; i < v->files_[level].size(); i++) {
              const InternalKey& prev_end = v->files_[level][i - 1]->largest;
              const InternalKey& this_begin = v->files_[level][i]->smallest;
              if (vset_->icmp_.Compare(prev_end, this_begin) >= 0) {
                fprintf(stderr, "overlapping ranges in same level %s vs. %s\n",
                        prev_end.DebugString().c_str(),
                        this_begin.DebugString().c_str());
                abort();
              }
            }
          }
    #endif
        }
        */
    }
}
