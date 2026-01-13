// ---------------- [ File: bitcoinleveldb-dbimpl/src/get.rs ]
crate::ix!();

impl DBGet for DBImpl {
    fn get(&mut self, options: &ReadOptions, key_: &Slice, value: *mut String) -> crate::Status {
        let mut s: Status = Status::ok();

        self.mutex.lock();

        let snapshot: SequenceNumber = if !options.snapshot().is_null() {
            unsafe { (*(options.snapshot() as *const SnapshotImpl)).sequence_number() }
        } else {
            unsafe { (*self.versions).last_sequence() }
        };

        let mem: *mut MemTable = self.mem;
        let imm: *mut MemTable = self.imm;
        let current: *mut Version = unsafe { (*self.versions).current() };

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
