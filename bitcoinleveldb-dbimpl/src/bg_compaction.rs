// ---------------- [ File: bitcoinleveldb-dbimpl/src/bg_compaction.rs ]
crate::ix!();

impl DBImpl {
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn background_compaction(&mut self) {
        self.mutex.assert_held();

        if !self.imm.is_null() {
            self.compact_mem_table();
            return;
        }

        let mut c: *mut Compaction = core::ptr::null_mut();
        let is_manual: bool = !self.manual_compaction_.is_null();
        let mut manual_end: InternalKey = Default::default();

        if is_manual {
            let m: *mut ManualCompaction = self.manual_compaction_;
            c = unsafe { (*self.versions_).compact_range((*m).level, (*m).begin, (*m).end) };
            unsafe {
                (*m).done = c.is_null();
            }

            if !c.is_null() {
                let n0: i32 = unsafe { (*c).num_input_files(0) };
                if n0 > 0 {
                    manual_end = unsafe { (*(*c).input(0, n0 - 1)).largest.clone() };
                }
            }

            let begin_dbg: String = unsafe {
                if (*m).begin.is_null() {
                    "(begin)".to_string()
                } else {
                    (*(*m).begin).debug_string()
                }
            };

            let end_dbg: String = unsafe {
                if (*m).end.is_null() {
                    "(end)".to_string()
                } else {
                    (*(*m).end).debug_string()
                }
            };

            let stop_dbg: String = unsafe {
                if (*m).done {
                    "(end)".to_string()
                } else {
                    manual_end.debug_string()
                }
            };

            tracing::info!(
                level = unsafe { (*m).level },
                begin = %begin_dbg,
                end = %end_dbg,
                stop = %stop_dbg,
                "Manual compaction"
            );
        } else {
            c = unsafe { (*self.versions_).pick_compaction() };
        }

        let mut status: Status = Status::ok();

        if c.is_null() {
            // Nothing to do
        } else if !is_manual && unsafe { (*c).is_trivial_move() } {
            // Move file to next level
            assert_eq!(unsafe { (*c).num_input_files(0) }, 1);
            let f: *mut FileMetaData = unsafe { (*c).input(0, 0) };

            unsafe {
                (*(*c).edit()).delete_file((*c).level(), (*f).number);
                (*(*c).edit()).add_file(
                    (*c).level() + 1,
                    (*f).number,
                    (*f).file_size,
                    (*f).smallest.clone(),
                    (*f).largest.clone(),
                );
            }

            status = unsafe { (*self.versions_).log_and_apply((*c).edit(), &mut self.mutex) };
            if !status.is_ok() {
                self.record_background_error(&status);
            }

            let mut tmp: VersionSetLevelSummaryStorage = Default::default();
            let summary: String = unsafe { (*self.versions_).level_summary(&mut tmp) };

            tracing::info!(
                file_number = (*f).number as u64,
                to_level = unsafe { (*c).level() + 1 },
                file_size = (*f).file_size as u64,
                status = %status.to_string(),
                summary = %summary,
                "Moved file to next level"
            );
        } else {
            let compact: *mut CompactionState =
                Box::into_raw(Box::new(CompactionState::new(c)));

            status = self.do_compaction_work(compact);
            if !status.is_ok() {
                self.record_background_error(&status);
            }

            self.cleanup_compaction(compact);

            unsafe {
                (*c).release_inputs();
            }

            self.delete_obsolete_files();
        }

        if !c.is_null() {
            unsafe {
                drop(Box::from_raw(c));
            }
        }

        if status.is_ok() {
            // Done
        } else if self.shutting_down_.load(core::sync::atomic::Ordering::Acquire) {
            // Ignore compaction errors found during shutting down
        } else {
            tracing::error!(status = %status.to_string(), "Compaction error");
        }

        if is_manual {
            let m: *mut ManualCompaction = self.manual_compaction_;
            if !status.is_ok() {
                unsafe {
                    (*m).done = true;
                }
            }
            unsafe {
                if !(*m).done {
                    // We only compacted part of the requested range.  Update *m
                    // to the range that is left to be compacted.
                    (*m).tmp_storage = manual_end;
                    (*m).begin = &mut (*m).tmp_storage;
                }
            }
            self.manual_compaction_ = core::ptr::null_mut();
        }
    }
}

#[cfg(test)]
#[disable]
mod bg_compaction_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn background_compaction_is_exercised_via_manual_full_range_compaction_smoke() {
        let (dbname, mut db) =
            open_dbimpl_for_test("background_compaction_is_exercised_via_manual_full_range_compaction_smoke");

        // Create enough data to make compaction meaningful.
        fill_sequential(&mut *db, "k", 500, 256);

        // Manual full range compaction: exercises BackgroundCompaction() paths.
        force_manual_compaction_full_range(&mut *db);

        // Sanity: reads remain correct after compaction activity.
        assert_read_eq(&mut *db, "k00000000", &"v".repeat(256));
        assert_read_eq(&mut *db, "k00000499", &"v".repeat(256));

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
