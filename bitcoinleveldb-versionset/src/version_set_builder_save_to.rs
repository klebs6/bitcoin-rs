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

                // Merge the set of added files with the set of pre-existing files.
                // Drop any deleted files. Store the result in *v.
                let base_files: &Vec<*mut FileMetaData> = &(*base_ptr).files()[level];

                let added_ptr: *mut VersionSetBuilderFileSet =
                    self.level_state_ref(level).added_files_ptr();

                assert!(
                    !added_ptr.is_null(),
                    "VersionSetBuilder::save_to: added_files ptr is null at level {}",
                    level
                );

                let added_set: &VersionSetBuilderFileSet = &*added_ptr;

                let mut added_vec: Vec<*mut FileMetaData> = added_set.iter().copied().collect();
                added_vec.sort_by(|a_ref, b_ref| cmp_files(*a_ref, *b_ref));

                {
                    let files_level: &mut Vec<*mut FileMetaData> = &mut (*(*v).files_mut())[level];
                    files_level.reserve(base_files.len() + added_vec.len());
                }

                trace!(
                    level = level,
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
                        level,
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
                            level
                        );

                        if cmp_files(added_file, mid_ptr) == core::cmp::Ordering::Less {
                            hi = mid;
                        } else {
                            lo = mid + 1;
                        }
                    }

                    let bpos: usize = lo;

                    for i in base_index..bpos {
                        self.maybe_add_file(v, level, base_files[i]);
                    }
                    base_index = bpos;

                    self.maybe_add_file(v, level, added_file);
                }

                // Add remaining base files
                for i in base_index..base_end {
                    self.maybe_add_file(v, level, base_files[i]);
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

#[cfg(test)]
mod version_set_builder_save_to_exhaustive_test_suite {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};
    use tracing::{debug, error, info, trace, warn};

    fn make_unique_temp_db_dir(prefix: &str) -> PathBuf {
        let pid = std::process::id();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);

        let mut p = std::env::temp_dir();
        p.push(format!("{prefix}_{pid}_{nanos}"));
        p
    }

    fn remove_dir_all_best_effort(dir: &Path) {
        match std::fs::remove_dir_all(dir) {
            Ok(()) => trace!(dir = %dir.display(), "removed temp db dir"),
            Err(e) => warn!(dir = %dir.display(), error = ?e, "failed to remove temp db dir (best effort)"),
        }
    }

    fn assert_status_ok(st: &Status, context: &'static str) {
        if !st.is_ok() {
            error!(?st, context, "unexpected non-ok Status");
            panic!("unexpected non-ok Status in {context}");
        }
        trace!(context, "Status OK");
    }

    fn make_ikey(user_key: &str, seq: u64) -> InternalKey {
        InternalKey::new(&Slice::from(user_key), seq, ValueType::TypeValue)
    }

    fn make_internal_key_comparator_from_options(options: &Options) -> InternalKeyComparator {
        let ucmp_ptr: *const dyn SliceComparator =
            options.comparator().as_ref() as *const dyn SliceComparator;
        InternalKeyComparator::new(ucmp_ptr)
    }

    struct RawMutexTestGuard {
        mu: *mut RawMutex,
    }

    impl RawMutexTestGuard {
        fn lock(mu: *mut RawMutex) -> Self {
            trace!(mu_ptr = %format!("{:p}", mu), "RawMutexTestGuard::lock");
            unsafe { (*mu).lock() };
            Self { mu }
        }
    }

    impl Drop for RawMutexTestGuard {
        fn drop(&mut self) {
            trace!(mu_ptr = %format!("{:p}", self.mu), "RawMutexTestGuard::drop (unlock)");
            unsafe { (*self.mu).unlock() };
        }
    }

    #[traced_test]
    fn save_to_merges_base_and_added_files_in_key_order_and_respects_deletions() {
        let dir = make_unique_temp_db_dir("versionset_builder_save_to_merge");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 128));
        let mut mu = Box::new(RawMutex::INIT);

        let mut vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st0 = vs.recover(&mut save_manifest as *mut bool);
        assert_status_ok(&st0, "recover");

        let _guard = RawMutexTestGuard::lock(mu.as_mut() as *mut RawMutex);

        // Build a base state with two L1 files: A=[a,c], C=[g,i]
        let f_a = vs.new_file_number();
        let mut e_a = VersionEdit::default();
        e_a.add_file(1, f_a, 10, &make_ikey("a", 1), &make_ikey("c", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e_a as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply base A",
        );

        let f_c = vs.new_file_number();
        let mut e_c = VersionEdit::default();
        e_c.add_file(1, f_c, 10, &make_ikey("g", 1), &make_ikey("i", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e_c as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply base C",
        );

        let base = vs.current();
        assert!(!base.is_null(), "base must not be null");

        // Builder on top of base: add B=[d,f]
        let mut builder = VersionSetBuilder::new(&mut vs as *mut VersionSet, base);

        let f_b = vs.new_file_number();
        let mut edit = VersionEdit::default();
        edit.add_file(1, f_b, 10, &make_ikey("d", 1), &make_ikey("f", 1));
        builder.apply(&mut edit as *mut VersionEdit);

        let vs_ptr: *mut VersionSet = &mut vs as *mut VersionSet;
        let out_v = Version::from(VersionSetPtr::new(vs_ptr));
        let out_ptr: *mut Version = Box::into_raw(Box::new(out_v));

        builder.save_to(out_ptr);

        unsafe {
            let l1 = &(*out_ptr).files()[1];
            let nums: Vec<u64> = l1.iter().copied().filter(|p| !p.is_null()).map(|p| *(*p).number()).collect();
            debug!(?nums, "L1 file numbers after merge");
            assert_eq!(
                nums,
                vec![f_a, f_b, f_c],
                "expected A,B,C in key order regardless of file number allocation order"
            );
        }

        // Now test deletion: delete A, keep B and C.
        let mut builder2 = VersionSetBuilder::new(&mut vs as *mut VersionSet, base);

        let mut edit2 = VersionEdit::default();
        edit2.delete_file(1, f_a);
        builder2.apply(&mut edit2 as *mut VersionEdit);

        let out_v2 = Version::from(VersionSetPtr::new(vs_ptr));
        let out_ptr2: *mut Version = Box::into_raw(Box::new(out_v2));

        builder2.save_to(out_ptr2);

        unsafe {
            let l1 = &(*out_ptr2).files()[1];
            let nums: Vec<u64> = l1.iter().copied().filter(|p| !p.is_null()).map(|p| *(*p).number()).collect();
            debug!(?nums, "L1 file numbers after deletion");
            assert!(
                !nums.contains(&f_a),
                "deleted base file must not appear in saved version"
            );
        }

        // Best-effort cleanup: avoid assuming Version drop/unref details in unit tests.
        let _ = (out_ptr, out_ptr2);

        remove_dir_all_best_effort(&dir);
    }
}
