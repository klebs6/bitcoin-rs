// ---------------- [ File: bitcoinleveldb-dbimpl/src/get.rs ]
crate::ix!();

impl DBGet for DBImpl {
    fn get(&mut self, options: &ReadOptions, key_: &Slice, value: *mut String) -> crate::Status {
        let mut s: Status = Status::ok();

        self.mutex.lock();

        let snapshot: SequenceNumber = match options.snapshot().as_ref() {
            Some(snap) => {
                // SAFETY: DBImpl only ever hands out SnapshotImpl instances via the public Snapshot
                // interface, so the trait object data pointer is a SnapshotImpl.
                let raw: *const dyn Snapshot = Arc::as_ptr(snap);
                let data: *const () = raw as *const ();
                let snap_impl: *const SnapshotImpl = data as *const SnapshotImpl;

                unsafe { *(*snap_impl).sequence_number() }
            }
            None => unsafe { (*self.versions).last_sequence() },
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
        unsafe { self.mutex.unlock() };

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

        if have_stat_update && unsafe { (*current).update_stats(&mut stats) } {
            self.maybe_schedule_compaction();
        }

        unsafe {
            (*mem).unref();
            if !imm.is_null() {
                (*imm).unref();
            }
            (*current).unref();
        }

        unsafe { self.mutex.unlock() };

        s
    }
}

#[cfg(test)]
mod db_get_contract_suite {
    use super::*;
    use bitcoinleveldb_dbinterface::DBGet;

    fn assert_dbimpl_implements_db_get() {
        fn _assert<T: DBGet>() {}
        _assert::<DBImpl>();
    }

    fn compile_only_accepts_db_get_trait_object(_db: &mut dyn DBGet) {}

    fn compile_only_db_get_call_via_trait_object(
        db: &mut dyn DBGet,
        options: &ReadOptions,
        key: &Slice,
        out: &mut String,
    ) -> Status {
        // Intentionally takes `&mut String` so it can coerce to a raw out-pointer
        // if the underlying interface uses `*mut String`.
        db.get(options, key, out)
    }

    #[traced_test]
    fn db_get_contract_dbimpl_implements_db_get_trait() {
        tracing::trace!("begin DBGet contract: DBImpl implements DBGet");
        assert_dbimpl_implements_db_get();
        tracing::info!("DBGet contract satisfied: DBImpl implements DBGet");
    }

    #[traced_test]
    fn db_get_contract_db_get_is_object_safe() {
        tracing::trace!("begin DBGet contract: trait object safety");
        let _accept = compile_only_accepts_db_get_trait_object as fn(&mut dyn DBGet);
        let _call = compile_only_db_get_call_via_trait_object
            as fn(&mut dyn DBGet, &ReadOptions, &Slice, &mut String) -> Status;
        tracing::info!("DBGet contract satisfied: usable as a trait object and callable via dyn dispatch");
    }

    #[traced_test]
    fn db_get_contract_method_item_is_addressable() {
        tracing::trace!("begin DBGet contract: method item addressability");
        let _method_item = <DBImpl as DBGet>::get;
        let _ = _method_item;
        tracing::info!("DBGet contract satisfied: <DBImpl as DBGet>::get method item can be referenced");
    }
}
