// ---------------- [ File: bitcoinleveldb-dbimpl/src/build_batch_group.rs ]
crate::ix!();

impl DBImpl {
    /// REQUIRES: Writer list must be non-empty
    /// 
    /// REQUIRES: First writer must have a non-null
    /// batch
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn build_batch_group(
        &mut self,
        last_writer: *mut *mut DBImplWriter,
    ) -> *mut WriteBatch {
        self.mutex.assert_held();
        assert!(!self.writers.is_empty());

        let first: *mut DBImplWriter = *self.writers.front().unwrap();
        let mut result: *mut WriteBatch = unsafe { (*first).batch() };
        assert!(!result.is_null());

        let mut size: usize = unsafe { write_batch_internal::byte_size((*first).batch()) };

        // Allow the group to grow up to a maximum size, but if the
        // original write is small, limit the growth so we do not slow
        // down the small write too much.
        let mut max_size: usize = 1usize << 20;
        if size <= (128usize << 10) {
            max_size = size + (128usize << 10);
        }

        unsafe {
            *last_writer = first;
        }

        let mut iter = self.writers.iter();
        iter.next(); // Advance past "first"

        for wptr in iter {
            let w: *mut DBImplWriter = *wptr;

            if unsafe { (*w).sync() } && !unsafe { (*first).sync() } {
                // Do not include a sync write into a batch handled by a non-sync write.
                break;
            }

            if !unsafe { (*w).batch() }.is_null() {
                size += unsafe { write_batch_internal::byte_size((*w).batch()) };
                if size > max_size {
                    // Do not make batch too big
                    break;
                }

                // Append to *result
                if result == unsafe { (*first).batch() } {
                    // Switch to temporary batch instead of disturbing caller's batch
                    result = self.tmp_batch();
                    assert_eq!(unsafe { write_batch_internal::count(result) }, 0);
                    unsafe {
                        write_batch_internal::append(result, (*first).batch());
                    }
                }

                unsafe {
                    write_batch_internal::append(result, (*w).batch());
                }
            }

            unsafe {
                *last_writer = w;
            }
        }

        result
    }
}
