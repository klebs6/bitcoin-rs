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
            *latest_snapshot = (*self.versions).last_sequence();
        }

        // Collect together all needed child iterators
        let mut list: Vec<*mut LevelDBIterator> = Vec::new();

        unsafe {
            list.push((*self.mem).new_iterator());
            (*self.mem).ref_();

            if !self.imm.is_null() {
                list.push((*self.imm).new_iterator());
                (*self.imm).ref_();
            }

            let current: *mut Version = (*self.versions).current();
            (*current).add_iterators(options, &mut list);

            let n: i32 = list
                .len()
                .try_into()
                .expect("DBImpl::new_internal_iterator: too many child iterators to merge");

            let comparator: Box<dyn SliceComparator> = Box::new(InternalKeyComparator::new(
                self.internal_comparator.user_comparator(),
            ));

            let internal_iter: *mut LevelDBIterator =
                new_merging_iterator(comparator, list.as_mut_ptr(), n);

            (*current).ref_();

            let cleanup: *mut IterState = Box::into_raw(Box::new(IterState::new(
                &mut self.mutex,
                self.mem,
                self.imm,
                current,
            )));

            (*internal_iter).register_cleanup(
                cleanup_iterator_state,
                cleanup as *mut core::ffi::c_void,
                core::ptr::null_mut(),
            );

            self.seed = self.seed.wrapping_add(1);
            *seed = self.seed;

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
