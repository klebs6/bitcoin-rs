// ---------------- [ File: bitcoinleveldb-dbimpl/src/get.rs ]
crate::ix!();

impl DBGet for DBImpl {
    fn get(&mut self, options: &ReadOptions, key_: &Slice, value: *mut String) -> crate::Status {
        let mut s: Status = Status::ok();

        self.mutex.lock();

        let snapshot: SequenceNumber = if !options.snapshot.is_null() {
            unsafe { (*(options.snapshot as *const SnapshotImpl)).sequence_number() }
        } else {
            unsafe { (*self.versions_).last_sequence() }
        };

        let mem: *mut MemTable = self.mem_;
        let imm: *mut MemTable = self.imm;
        let current: *mut Version = unsafe { (*self.versions_).current() };

        unsafe {
            (*mem).ref_();
            if !imm.is_null() {
                (*imm).ref_();
            }
            (*current).ref_();
        }

        let mut have_stat_update: bool = false;
        let mut stats: VersionGetStats = Default::default();

        // Unlock while reading from files and memtables
        self.mutex.unlock();

        {
            // First look in the memtable, then in the immutable memtable (if any).
            let lkey: LookupKey = LookupKey::new(key_, snapshot);

            if unsafe { (*mem).get(&lkey, value, &mut s) } {
                // Done
            } else if !imm.is_null() && unsafe { (*imm).get(&lkey, value, &mut s) } {
                // Done
            } else {
                s = unsafe { (*current).get(options, &lkey, value, &mut stats) };
                have_stat_update = true;
            }
        }

        self.mutex.lock();

        if have_stat_update && unsafe { (*current).update_stats(stats) } {
            self.maybe_schedule_compaction();
        }

        unsafe {
            (*mem).unref();
            if !imm.is_null() {
                (*imm).unref();
            }
            (*current).unref();
        }

        self.mutex.unlock();

        s
    }
}

#[cfg(test)]
#[disable]
mod get_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn get_respects_snapshots_and_returns_stable_view() {
        let (dbname, mut db) = open_dbimpl_for_test("get_respects_snapshots_and_returns_stable_view");

        let s1 = write_kv(&mut *db, "k", "v1");
        assert!(s1.is_ok(), "write v1 failed: {}", s1.to_string());
        assert_read_eq(&mut *db, "k", "v1");

        // Create snapshot, then overwrite, then read via snapshot.
        let snap: Box<dyn Snapshot> = <DBImpl as DBGetSnapshot>::get_snapshot(&mut *db);

        let s2 = write_kv(&mut *db, "k", "v2");
        assert!(s2.is_ok(), "write v2 failed: {}", s2.to_string());
        assert_read_eq(&mut *db, "k", "v2");

        let mut ro: ReadOptions = Default::default();
        ro.snapshot = (&*snap) as *const dyn Snapshot;

        let (sr, v) = read_value(&mut *db, &ro, "k");
        tracing::info!(status = %sr.to_string(), value = %v, "snapshot read");
        assert!(sr.is_ok(), "snapshot read failed: {}", sr.to_string());
        assert_eq!(v, "v1", "snapshot must see old value");

        <DBImpl as DBReleaseSnapshot>::release_snapshot(&mut *db, snap);

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
