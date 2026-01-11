// ---------------- [ File: bitcoinleveldb-dbimpl/src/new_internal_iterator.rs ]
crate::ix!();

impl DBImpl {

    pub fn new_internal_iterator(
        &mut self,
        options: &ReadOptions,
        latest_snapshot: *mut SequenceNumber,
        seed: *mut u32,
    ) -> *mut LevelDBIterator {
        self.mutex.lock();
        unsafe {
            *latest_snapshot = (*self.versions_).last_sequence();
        }

        // Collect together all needed child iterators
        let mut list: Vec<*mut LevelDBIterator> = Vec::new();

        unsafe {
            list.push((*self.mem_).new_iterator());
            (*self.mem_).ref_();

            if !self.imm.is_null() {
                list.push((*self.imm).new_iterator());
                (*self.imm).ref_();
            }

            let current: *mut Version = (*self.versions_).current();
            (*current).add_iterators(options, &mut list);

            let internal_iter: *mut LevelDBIterator =
                new_merging_iterator(&self.internal_comparator_, &list[0], list.len());

            (*current).ref_();

            let cleanup: *mut IterState = Box::into_raw(Box::new(IterState::new(
                &mut self.mutex,
                self.mem_,
                self.imm,
                current,
            )));

            (*internal_iter).register_cleanup(cleanup_iterator_state, cleanup as *mut core::ffi::c_void, core::ptr::null_mut());

            self.seed_ = self.seed_.wrapping_add(1);
            *seed = self.seed_;

            self.mutex.unlock();

            internal_iter
        }
    }

    /// Return an internal iterator over the current
    /// state of the database.
    /// 
    /// The keys of this iterator are internal keys
    /// (see format.h).
    /// 
    /// The returned iterator should be deleted when
    /// no longer needed.
    pub fn test_new_internal_iterator(&mut self) -> *mut LevelDBIterator {
        let mut ignored: SequenceNumber = 0;
        let mut ignored_seed: u32 = 0;
        self.new_internal_iterator(&ReadOptions::default(), &mut ignored, &mut ignored_seed)
    }
}

#[cfg(test)]
#[disable]
mod new_internal_iterator_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn new_internal_iterator_is_constructible_and_reports_ok_status() {
        let (dbname, mut db) = open_dbimpl_for_test("new_internal_iterator_is_constructible_and_reports_ok_status");

        write_kv(&mut *db, "k", "v");
        force_manual_compaction_full_range(&mut *db);

        let mut latest: SequenceNumber = 0;
        let mut seed: u32 = 0;

        let it: *mut LevelDBIterator =
            db.new_internal_iterator(&ReadOptions::default(), (&mut latest) as *mut SequenceNumber, (&mut seed) as *mut u32);

        assert!(!it.is_null(), "internal iterator must not be null");

        unsafe {
            (*it).seek_to_first();
            // We do not assert key format; we assert iterator mechanics remain healthy.
            let mut steps: usize = 0;
            while (*it).valid() && steps < 32 {
                let klen = (*it).key().size();
                let vlen = (*it).value().size();
                tracing::debug!(klen, vlen, "internal iter step");
                steps += 1;
                (*it).next();
            }
            let st = (*it).status();
            tracing::info!(status = %st.to_string(), latest, seed, steps, "internal iterator status");
            assert!(st.is_ok(), "internal iterator must end with ok status");

            drop(Box::from_raw(it));
        }

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
