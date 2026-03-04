// ---------------- [ File: bitcoinleveldb-dbiterstate/src/iterstate.rs ]
crate::ix!();

#[repr(C)]
pub struct IterState {
    mu:      *mut parking_lot::RawMutex,
    mem:     *mut MemTable,
    imm:     *mut MemTable,
    version: *mut Version,
}

impl IterState {
    pub fn new(
        mutex: *mut parking_lot::RawMutex,
        mem: *mut MemTable,
        imm: *mut MemTable,
        version: *mut Version,
    ) -> Self {
        Self {
            mu: mutex,
            mem,
            imm,
            version,
        }
    }
}

pub fn cleanup_iterator_state(arg1: *mut c_void, _arg2: *mut c_void) {
    // LevelDB registers this as an iterator cleanup callback. In normal use `arg1`
    // is a heap-allocated IterState (Box::into_raw), but make this a no-op on null.
    if arg1.is_null() {
        return;
    }

    // Small RAII guard so we always unlock even if an Unref() panics.
    struct RawMutexGuard<'a> {
        mu: &'a parking_lot::RawMutex,
    }

    impl<'a> RawMutexGuard<'a> {
        #[inline]
        fn new(mu: &'a parking_lot::RawMutex) -> Self {
            // parking_lot::RawMutex methods come from lock_api; parking_lot re-exports it.
            #[allow(unused_imports)]
            use parking_lot::lock_api::RawMutex as _;
            mu.lock();
            Self { mu }
        }
    }

    impl Drop for RawMutexGuard<'_> {
        #[inline]
        fn drop(&mut self) {
            #[allow(unused_imports)]
            use parking_lot::lock_api::RawMutex as _;
            unsafe { self.mu.unlock() };
        }
    }

    unsafe {
        // Take ownership so the IterState allocation is reclaimed even on unwind.
        let mut state: Box<IterState> = Box::from_raw(arg1 as *mut IterState);

        // Lock DB mutex (if provided) while dropping refs (matches LevelDB semantics).
        let _lock = state.mu.as_ref().map(RawMutexGuard::new);

        if !state.mem.is_null() {
            (*state.mem).unref();
        }

        if !state.imm.is_null() {
            (*state.imm).unref();
        }

        if !state.version.is_null() {
            (*state.version).unref();
        }

        // `_lock` drops here (unlock), then `state` drops here (free IterState).
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    // Bring lock_api trait methods (lock/try_lock/unlock) into scope for RawMutex.
    #[allow(unused_imports)]
    use parking_lot::lock_api::RawMutex as _;

    // Create a "null" fat pointer for `*const dyn Trait` / `*mut dyn Trait`.
    // This is only used to build minimal objects where the pointer is never dereferenced.
    #[inline]
    fn null_fat_ptr_const<T: ?Sized>(meta_source: &T) -> *const T {
        use core::ptr;

        let data: *const () = ptr::null();
        let meta = ptr::metadata(meta_source as *const T);
        ptr::from_raw_parts(data, meta)
    }

    #[inline]
    fn null_fat_ptr_mut<T: ?Sized>(meta_source: &T) -> *mut T {
        use core::ptr;

        let data: *mut () = ptr::null_mut();
        let meta = ptr::metadata(meta_source as *const T);
        ptr::from_raw_parts_mut(data, meta)
    }

    fn new_memtable_with_refs(refs: i32) -> *mut MemTable {

        let user_cmp: *const dyn SliceComparator = bitcoinleveldb_comparator::bytewise_comparator();
        let icmp = InternalKeyComparator::new(user_cmp);

        let mut mt = Box::new(MemTable::new(&icmp));
        for _ in 0..refs {
            mt.ref_();
        }
        Box::into_raw(mt)
    }

    fn new_empty_version_with_refs(refs: i32) -> *mut Version {

        let files: [Vec<*mut FileMetaData>; NUM_LEVELS] =
            core::array::from_fn(|_| Vec::new());

        let vset: *mut dyn VersionSetInterface 
            = Box::into_raw(Box::new(DummyVersionSet)) as *mut dyn VersionSetInterface;

        let mut v = VersionBuilder::default()
            .vset(vset)
            .next(core::ptr::null_mut())
            .prev(core::ptr::null_mut())
            .refs(0)
            .files(files)
            .file_to_compact(core::ptr::null_mut())
            .file_to_compact_level(0)
            .compaction_score(0.0)
            .compaction_level(0)
            .build()
            .expect("VersionBuilder should build an empty Version");

        for _ in 0..refs {
            v.ref_();
        }

        Box::into_raw(Box::new(v))
    }

    #[test]
    fn iterstate_new_stores_arguments() {
        let mut mu = parking_lot::RawMutex::INIT;

        let mem = new_memtable_with_refs(1);
        let imm = core::ptr::null_mut();
        let ver = new_empty_version_with_refs(1);

        let st = IterState::new(&mut mu as *mut _, mem, imm, ver);

        assert_eq!(st.mu, &mut mu as *mut _);
        assert_eq!(st.mem, mem);
        assert_eq!(st.imm, imm);
        assert_eq!(st.version, ver);

        unsafe {
            (*mem).unref();     // refs: 1 -> 0, drops
            (*ver).unref();     // refs: 1 -> 0, drops
        }
    }

    #[test]
    fn cleanup_null_state_is_noop() {
        cleanup_iterator_state(core::ptr::null_mut(), core::ptr::null_mut());
    }

    #[test]
    fn cleanup_decrements_refs_and_unlocks_without_imm() {
        let mut mu = parking_lot::RawMutex::INIT;

        let mem = new_memtable_with_refs(2);
        let ver = new_empty_version_with_refs(2);

        let state = Box::new(IterState::new(
            &mut mu as *mut _,
            mem,
            core::ptr::null_mut(),
            ver,
        ));
        let state_ptr = Box::into_raw(state) as *mut c_void;

        cleanup_iterator_state(state_ptr, core::ptr::null_mut());

        unsafe {
            assert_eq!(*(*mem).refs(), 1, "memtable should be Unref()'d once");
            assert_eq!(*(*ver).refs(), 1, "version should be Unref()'d once");
        }

        // Must not leave the DB mutex locked.
        assert!(mu.try_lock(), "mutex left locked after cleanup");
        unsafe { mu.unlock() };

        // Finish cleanup: refs 1 -> 0 for both, triggers drop.
        unsafe {
            (*mem).unref();
            (*ver).unref();
        }
    }

    #[test]
    fn cleanup_decrements_refs_with_imm() {
        let mut mu = parking_lot::RawMutex::INIT;

        let mem = new_memtable_with_refs(2);
        let imm = new_memtable_with_refs(2);
        let ver = new_empty_version_with_refs(2);

        let state = Box::new(IterState::new(&mut mu as *mut _, mem, imm, ver));
        let state_ptr = Box::into_raw(state) as *mut c_void;

        cleanup_iterator_state(state_ptr, core::ptr::null_mut());

        unsafe {
            assert_eq!(*(*mem).refs(), 1);
            assert_eq!(*(*imm).refs(), 1);
            assert_eq!(*(*ver).refs(), 1);
        }

        assert!(mu.try_lock(), "mutex left locked after cleanup");
        unsafe { mu.unlock() };

        unsafe {
            (*mem).unref();
            (*imm).unref();
            (*ver).unref();
        }
    }

    #[test]
    fn cleanup_unlocks_mutex_even_if_unref_panics() {
        let mut mu = parking_lot::RawMutex::INIT;

        // Make mem valid (so it gets unref'd), but make Version invalid (refs=0)
        // so Version::unref() panics. This tests that the RawMutex is still unlocked.
        let mem = new_memtable_with_refs(2);
        let ver = new_empty_version_with_refs(0);

        let state = Box::new(IterState::new(
            &mut mu as *mut _,
            mem,
            core::ptr::null_mut(),
            ver,
        ));
        let state_ptr = Box::into_raw(state) as *mut c_void;

        let res = catch_unwind(AssertUnwindSafe(|| {
            cleanup_iterator_state(state_ptr, core::ptr::null_mut());
        }));
        assert!(res.is_err(), "expected panic from Version::unref with refs=0");

        // Must not leave the DB mutex locked on unwind.
        assert!(mu.try_lock(), "mutex left locked after panic during cleanup");
        unsafe { mu.unlock() };

        // mem was unref'd once before the panic
        unsafe {
            assert_eq!(*(*mem).refs(), 1);
            (*mem).unref(); // drop it
            // Version still has refs=0; drop directly (calling unref would panic again).
            drop(Box::from_raw(ver));
        }
    }
}
