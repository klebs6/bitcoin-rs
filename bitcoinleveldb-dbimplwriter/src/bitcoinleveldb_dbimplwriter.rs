// ---------------- [ File: bitcoinleveldb-dbimplwriter/src/bitcoinleveldb_dbimplwriter.rs ]
crate::ix!();

/// Information kept for every waiting writer
#[derive(Setters, Getters, MutGetters)]
#[getset(set = "pub", get = "pub", get_mut = "pub")]
pub struct DBImplWriter {
    status: Status,
    batch:  *mut WriteBatch,
    sync:   bool,
    done:   bool,
    cv:     Condvar,
}

impl DBImplWriter {

    pub fn new(mu: *mut parking_lot::RawMutex) -> Self {
        tracing::trace!(
            target: "bitcoinleveldb_dbimplwriter::DBImplWriter",
            mu = ?mu,
            "DBImplWriter::new(start)"
        );

        let writer = Self {
            status: Status::default(),
            batch:  std::ptr::null_mut(),
            sync:   false,
            done:   false,
            cv:     Condvar::new(),
        };

        tracing::debug!(
            target: "bitcoinleveldb_dbimplwriter::DBImplWriter",
            status_code = ?writer.status.code(),
            batch_is_null = writer.batch.is_null(),
            sync = writer.sync,
            done = writer.done,
            "DBImplWriter::new(done)"
        );

        writer
    }
}

#[cfg(test)]
mod dbimpl_writer_initialization_test_suite {
    use super::*;

    #[traced_test]
    fn dbimpl_writer_new_initializes_expected_defaults_with_valid_mutex_pointer() {
        tracing::info!(
            target: "bitcoinleveldb_dbimplwriter::tests",
            "starting: dbimpl_writer_new_initializes_expected_defaults_with_valid_mutex_pointer"
        );

        let mut raw_mu = parking_lot::RawMutex::INIT;
        let mu_ptr: *mut parking_lot::RawMutex = &mut raw_mu;

        let writer = DBImplWriter::new(mu_ptr);

        tracing::debug!(
            target: "bitcoinleveldb_dbimplwriter::tests",
            status_code = ?writer.status().code(),
            batch_is_null = (*writer.batch()).is_null(),
            sync = *writer.sync(),
            done = *writer.done(),
            "constructed writer"
        );

        assert!(writer.status().is_ok());
        assert_eq!(writer.status().code(), StatusCode::Ok);
        assert!((*writer.batch()).is_null());
        assert!(!*writer.sync());
        assert!(!*writer.done());

        // Basic smoke checks: should not panic.
        writer.cv().notify_one();
        writer.cv().notify_all();
    }

    #[traced_test]
    fn dbimpl_writer_new_accepts_null_mutex_pointer_and_still_initializes_defaults() {
        tracing::info!(
            target: "bitcoinleveldb_dbimplwriter::tests",
            "starting: dbimpl_writer_new_accepts_null_mutex_pointer_and_still_initializes_defaults"
        );

        let writer = DBImplWriter::new(std::ptr::null_mut());

        tracing::debug!(
            target: "bitcoinleveldb_dbimplwriter::tests",
            status_code = ?writer.status().code(),
            batch_is_null = (*writer.batch()).is_null(),
            sync = *writer.sync(),
            done = *writer.done(),
            "constructed writer with null mutex pointer"
        );

        assert!(writer.status().is_ok());
        assert_eq!(writer.status().code(), StatusCode::Ok);
        assert!((*writer.batch()).is_null());
        assert!(!*writer.sync());
        assert!(!*writer.done());
    }

    #[traced_test]
    fn dbimpl_writer_instances_are_independent_and_mutable_via_public_accessors() {
        tracing::info!(
            target: "bitcoinleveldb_dbimplwriter::tests",
            "starting: dbimpl_writer_instances_are_independent_and_mutable_via_public_accessors"
        );

        let mut raw_mu = parking_lot::RawMutex::INIT;
        let mu_ptr: *mut parking_lot::RawMutex = &mut raw_mu;

        let mut w1 = DBImplWriter::new(mu_ptr);
        let w2 = DBImplWriter::new(mu_ptr);

        let non_null_batch_ptr: *mut WriteBatch = std::ptr::NonNull::<WriteBatch>::dangling().as_ptr();

        w1.set_sync(true);
        w1.set_done(true);
        w1.set_batch(non_null_batch_ptr);
        w1.set_status(Status::default());

        tracing::debug!(
            target: "bitcoinleveldb_dbimplwriter::tests",
            w1_status_code = ?w1.status().code(),
            w1_batch_is_null = (*w1.batch()).is_null(),
            w1_sync = *w1.sync(),
            w1_done = *w1.done(),
            w2_status_code = ?w2.status().code(),
            w2_batch_is_null = (*w2.batch()).is_null(),
            w2_sync = *w2.sync(),
            w2_done = *w2.done(),
            "post-mutation state"
        );

        assert!(w1.status().is_ok());
        assert_eq!(w1.status().code(), StatusCode::Ok);
        assert_eq!(*w1.batch(), non_null_batch_ptr);
        assert!(*w1.sync());
        assert!(*w1.done());

        assert!(w2.status().is_ok());
        assert_eq!(w2.status().code(), StatusCode::Ok);
        assert!((*w2.batch()).is_null());
        assert!(!*w2.sync());
        assert!(!*w2.done());
    }

    #[traced_test]
    fn dbimpl_writer_condvar_wait_for_times_out_without_notification() {
        tracing::info!(
            target: "bitcoinleveldb_dbimplwriter::tests",
            "starting: dbimpl_writer_condvar_wait_for_times_out_without_notification"
        );

        let mut raw_mu = parking_lot::RawMutex::INIT;
        let mu_ptr: *mut parking_lot::RawMutex = &mut raw_mu;

        let writer = DBImplWriter::new(mu_ptr);

        let lock = parking_lot::Mutex::new(());
        let mut guard = lock.lock();

        let wait_result = writer
            .cv()
            .wait_for(&mut guard, std::time::Duration::from_millis(1));

        tracing::debug!(
            target: "bitcoinleveldb_dbimplwriter::tests",
            timed_out = wait_result.timed_out(),
            "condvar wait_for result"
        );

        assert!(wait_result.timed_out());
    }
}
