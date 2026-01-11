// ---------------- [ File: bitcoinleveldb-dbimpl/src/build_batch_group.rs ]
crate::ix!();

impl DBImpl {
    /// REQUIRES: Writer list must be non-empty
    /// 
    /// REQUIRES: First writer must have a non-null
    /// batch
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn build_batch_group(
        &mut self,
        last_writer: *mut *mut DBImplWriter,
    ) -> *mut WriteBatch {
        self.mutex.assert_held();
        assert!(!self.writers_.is_empty());

        let first: *mut DBImplWriter = *self.writers_.front().unwrap();
        let mut result: *mut WriteBatch = unsafe { (*first).batch };
        assert!(!result.is_null());

        let mut size: usize = unsafe { WriteBatchInternal::byte_size((*first).batch) };

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

        let mut iter = self.writers_.iter();
        iter.next(); // Advance past "first"

        for wptr in iter {
            let w: *mut DBImplWriter = *wptr;

            if unsafe { (*w).sync } && !unsafe { (*first).sync } {
                // Do not include a sync write into a batch handled by a non-sync write.
                break;
            }

            if !unsafe { (*w).batch }.is_null() {
                size += unsafe { WriteBatchInternal::byte_size((*w).batch) };
                if size > max_size {
                    // Do not make batch too big
                    break;
                }

                // Append to *result
                if result == unsafe { (*first).batch } {
                    // Switch to temporary batch instead of disturbing caller's batch
                    result = self.tmp_batch_;
                    assert_eq!(unsafe { WriteBatchInternal::count(result) }, 0);
                    unsafe {
                        WriteBatchInternal::append(result, (*first).batch);
                    }
                }

                unsafe {
                    WriteBatchInternal::append(result, (*w).batch);
                }
            }

            unsafe {
                *last_writer = w;
            }
        }

        result
    }
}

#[cfg(test)]
#[disable]
mod build_batch_group_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn build_batch_group_single_writer_returns_original_batch_and_sets_last_writer() {
        let opts: Options = default_test_options();
        let dbname: String =
            unique_dbname("build_batch_group_single_writer_returns_original_batch_and_sets_last_writer");
        remove_db_dir_best_effort(&dbname);

        let mut db: DBImpl = DBImpl::new(&opts, &dbname);

        db.mutex_.lock();

        let mut w1: DBImplWriter = DBImplWriter::new(&mut db.mutex_);
        w1.sync = false;
        w1.done = false;

        let mut b1: WriteBatch = Default::default();
        b1.put(&Slice::from_str("k1"), &Slice::from_str("v1"));
        let mut b1_box: Box<WriteBatch> = Box::new(b1);
        w1.batch = (&mut *b1_box) as *mut WriteBatch;

        db.writers_.push_back((&mut w1) as *mut DBImplWriter);

        let mut last: *mut DBImplWriter = core::ptr::null_mut();
        let out: *mut WriteBatch = db.build_batch_group((&mut last) as *mut *mut DBImplWriter);

        tracing::info!(
            out_is_tmp = out == db.tmp_batch_,
            "build_batch_group result"
        );

        assert_eq!(last, (&mut w1) as *mut DBImplWriter, "last writer should be first");
        assert_eq!(out, w1.batch, "single writer should return original batch");

        db.writers_.clear();
        db.mutex_.unlock();

        remove_db_dir_best_effort(&dbname);
    }

    #[traced_test]
    fn build_batch_group_respects_sync_boundary() {
        let opts: Options = default_test_options();
        let dbname: String = unique_dbname("build_batch_group_respects_sync_boundary");
        remove_db_dir_best_effort(&dbname);

        let mut db: DBImpl = DBImpl::new(&opts, &dbname);

        db.mutex_.lock();

        let mut w1: DBImplWriter = DBImplWriter::new(&mut db.mutex_);
        let mut w2: DBImplWriter = DBImplWriter::new(&mut db.mutex_);

        w1.sync = false;
        w2.sync = true;

        let mut b1: WriteBatch = Default::default();
        b1.put(&Slice::from_str("k1"), &Slice::from_str("v1"));
        let mut b2: WriteBatch = Default::default();
        b2.put(&Slice::from_str("k2"), &Slice::from_str("v2"));

        let mut b1_box: Box<WriteBatch> = Box::new(b1);
        let mut b2_box: Box<WriteBatch> = Box::new(b2);

        w1.batch = (&mut *b1_box) as *mut WriteBatch;
        w2.batch = (&mut *b2_box) as *mut WriteBatch;

        db.writers_.push_back((&mut w1) as *mut DBImplWriter);
        db.writers_.push_back((&mut w2) as *mut DBImplWriter);

        let mut last: *mut DBImplWriter = core::ptr::null_mut();
        let out: *mut WriteBatch = db.build_batch_group((&mut last) as *mut *mut DBImplWriter);

        tracing::info!(out_is_tmp = out == db.tmp_batch_, "build_batch_group sync boundary");

        assert_eq!(last, (&mut w1) as *mut DBImplWriter, "sync boundary should stop grouping");
        assert_eq!(out, w1.batch, "should not merge across sync boundary");

        db.writers_.clear();
        db.mutex_.unlock();

        remove_db_dir_best_effort(&dbname);
    }

    #[traced_test]
    fn build_batch_group_merges_and_uses_tmp_batch_when_appending() {
        let opts: Options = default_test_options();
        let dbname: String = unique_dbname("build_batch_group_merges_and_uses_tmp_batch_when_appending");
        remove_db_dir_best_effort(&dbname);

        let mut db: DBImpl = DBImpl::new(&opts, &dbname);

        db.mutex_.lock();

        unsafe {
            (*db.tmp_batch_).clear();
        }

        let mut w1: DBImplWriter = DBImplWriter::new(&mut db.mutex_);
        let mut w2: DBImplWriter = DBImplWriter::new(&mut db.mutex_);

        w1.sync = false;
        w2.sync = false;

        let mut b1: WriteBatch = Default::default();
        b1.put(&Slice::from_str("k1"), &Slice::from_str("v1"));
        let mut b2: WriteBatch = Default::default();
        b2.put(&Slice::from_str("k2"), &Slice::from_str("v2"));

        let mut b1_box: Box<WriteBatch> = Box::new(b1);
        let mut b2_box: Box<WriteBatch> = Box::new(b2);

        w1.batch = (&mut *b1_box) as *mut WriteBatch;
        w2.batch = (&mut *b2_box) as *mut WriteBatch;

        db.writers_.push_back((&mut w1) as *mut DBImplWriter);
        db.writers_.push_back((&mut w2) as *mut DBImplWriter);

        let mut last: *mut DBImplWriter = core::ptr::null_mut();
        let out: *mut WriteBatch = db.build_batch_group((&mut last) as *mut *mut DBImplWriter);

        tracing::info!(
            out_is_tmp = out == db.tmp_batch_,
            out_count = unsafe { WriteBatchInternal::count(out) },
            "build_batch_group merge"
        );

        assert_eq!(last, (&mut w2) as *mut DBImplWriter, "should include second writer");
        assert_eq!(out, db.tmp_batch_, "should switch to tmp_batch for merged group");
        assert_eq!(unsafe { WriteBatchInternal::count(out) }, 2, "merged batch should have both puts");

        unsafe {
            (*db.tmp_batch_).clear();
        }

        db.writers_.clear();
        db.mutex_.unlock();

        remove_db_dir_best_effort(&dbname);
    }

    #[traced_test]
    fn build_batch_group_limits_max_size_to_avoid_overgrowth() {
        let opts: Options = default_test_options();
        let dbname: String = unique_dbname("build_batch_group_limits_max_size_to_avoid_overgrowth");
        remove_db_dir_best_effort(&dbname);

        let mut db: DBImpl = DBImpl::new(&opts, &dbname);

        db.mutex_.lock();

        unsafe {
            (*db.tmp_batch_).clear();
        }

        let mut w1: DBImplWriter = DBImplWriter::new(&mut db.mutex_);
        let mut w2: DBImplWriter = DBImplWriter::new(&mut db.mutex_);

        w1.sync = false;
        w2.sync = false;

        let mut b1: WriteBatch = Default::default();
        b1.put(&Slice::from_str("k1"), &Slice::from_str("v1"));

        // Make a large batch intended to exceed the growth cap.
        let big_val: String = "x".repeat(512 * 1024);
        let mut b2: WriteBatch = Default::default();
        b2.put(&Slice::from_str("k2"), &Slice::from_str(&big_val));

        let mut b1_box: Box<WriteBatch> = Box::new(b1);
        let mut b2_box: Box<WriteBatch> = Box::new(b2);

        w1.batch = (&mut *b1_box) as *mut WriteBatch;
        w2.batch = (&mut *b2_box) as *mut WriteBatch;

        db.writers_.push_back((&mut w1) as *mut DBImplWriter);
        db.writers_.push_back((&mut w2) as *mut DBImplWriter);

        let mut last: *mut DBImplWriter = core::ptr::null_mut();
        let out: *mut WriteBatch = db.build_batch_group((&mut last) as *mut *mut DBImplWriter);

        tracing::info!(
            last_is_first = last == (&mut w1) as *mut DBImplWriter,
            out_is_first = out == w1.batch,
            "build_batch_group max size check"
        );

        assert_eq!(last, (&mut w1) as *mut DBImplWriter, "should stop before oversized append");
        assert_eq!(out, w1.batch, "should keep original batch when no append occurs");

        unsafe {
            (*db.tmp_batch_).clear();
        }

        db.writers_.clear();
        db.mutex_.unlock();

        remove_db_dir_best_effort(&dbname);
    }
}
