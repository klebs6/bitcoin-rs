// ---------------- [ File: bitcoinleveldb-duplex/src/status.rs ]
crate::ix!();

impl LevelDBIteratorStatus for TwoLevelIterator {

    fn status(&self) -> Status {
        trace!("TwoLevelIterator::status: aggregating status");

        // Prefer index iterator status first.
        let index_status = self.index_iter().status();
        if !index_status.is_ok() {
            trace!(
                "TwoLevelIterator::status: returning index iterator status; code={:?}",
                index_status.code()
            );
            return index_status;
        }

        // Then prefer data iterator status when present.
        if self.data_iter().iter().is_some() {
            let data_status = self.data_iter().status();
            if !data_status.is_ok() {
                trace!(
                    "TwoLevelIterator::status: returning data iterator status; code={:?}",
                    data_status.code()
                );
                return data_status;
            }
        }

        // Finally, fall back to the cached internal status.
        trace!(
            "TwoLevelIterator::status: returning internal status; code={:?}",
            self.internal_status().code()
        );

        Status::new_from_other_copy(self.internal_status())
    }
}

#[cfg(test)]
mod two_level_iterator_status_tests {
    use super::*;
    use core::ffi::c_void;

    struct ErrorStatusIterator {
        status: Status,
    }

    impl ErrorStatusIterator {
        fn new_with_status(status: Status) -> Self {
            ErrorStatusIterator { status }
        }
    }

    impl LevelDBIteratorInterface for ErrorStatusIterator {}

    impl LevelDBIteratorValid for ErrorStatusIterator {
        fn valid(&self) -> bool {
            false
        }
    }

    impl LevelDBIteratorSeekToFirst for ErrorStatusIterator {
        fn seek_to_first(&mut self) {}
    }

    impl LevelDBIteratorSeekToLast for ErrorStatusIterator {
        fn seek_to_last(&mut self) {}
    }

    impl LevelDBIteratorSeek for ErrorStatusIterator {
        fn seek(&mut self, _target: &Slice) {}
    }

    impl LevelDBIteratorNext for ErrorStatusIterator {
        fn next(&mut self) {}
    }

    impl LevelDBIteratorPrev for ErrorStatusIterator {
        fn prev(&mut self) {}
    }

    impl LevelDBIteratorStatus for ErrorStatusIterator {
        fn status(&self) -> Status {
            Status::new_from_other_copy(&self.status)
        }
    }

    impl LevelDBIteratorKey for ErrorStatusIterator {
        fn key(&self) -> Slice {
            panic!("ErrorStatusIterator::key should not be called in status tests");
        }
    }

    impl LevelDBIteratorValue for ErrorStatusIterator {
        fn value(&self) -> Slice {
            panic!("ErrorStatusIterator::value should not be called in status tests");
        }
    }

    fn make_status(code: StatusCode) -> Status {
        let msg = Slice::from("status");
        match code {
            StatusCode::Ok => Status::ok(),
            StatusCode::NotFound => Status::not_found(&msg, None),
            StatusCode::Corruption => Status::corruption(&msg, None),
            StatusCode::NotSupported => Status::not_supported(&msg, None),
            StatusCode::InvalidArgument => Status::invalid_argument(&msg, None),
            StatusCode::IOError => Status::io_error(&msg, None),
        }
    }

    fn unused_block_function(
        _arg: *mut c_void,
        _options: &ReadOptions,
        _handle: &Slice,
    ) -> Option<Box<dyn LevelDBIteratorInterface>> {
        None
    }

    #[traced_test]
    fn status_prefers_index_iterator_error_over_other_sources() {
        let index_err = make_status(StatusCode::IOError);
        let index_iter: Box<dyn LevelDBIteratorInterface> =
            Box::new(ErrorStatusIterator::new_with_status(index_err));

        let options = ReadOptions::default();
        let mut two = TwoLevelIterator::new(
            index_iter,
            unused_block_function,
            core::ptr::null_mut(),
            options,
        );

        let data_err = make_status(StatusCode::NotFound);
        two.set_data_iterator(Some(Box::new(ErrorStatusIterator::new_with_status(
                        data_err,
        ))));

        let internal_err = make_status(StatusCode::Corruption);
        two.internal_status_mut().assign_from_other_copy(&internal_err);

        let st = two.status();
        assert!(
            !st.is_ok(),
            "status() should not be OK when the index iterator reports an error"
        );
        assert!(
            st.is_io_error(),
            "index iterator error must take precedence over data and internal errors"
        );
    }

    #[traced_test]
    fn status_uses_data_iterator_error_when_index_is_ok() {
        let index_iter: Box<dyn LevelDBIteratorInterface> =
            Box::new(MockStubIterator::new_empty());
        let options = ReadOptions::default();
        let mut two = TwoLevelIterator::new(
            index_iter,
            unused_block_function,
            core::ptr::null_mut(),
            options,
        );

        let data_err = make_status(StatusCode::NotFound);
        two.set_data_iterator(Some(Box::new(ErrorStatusIterator::new_with_status(
                        data_err,
        ))));

        let internal_err = make_status(StatusCode::Corruption);
        two.internal_status_mut().assign_from_other_copy(&internal_err);

        let st = two.status();
        assert!(
            !st.is_ok(),
            "status() should not be OK when the data iterator reports an error"
        );
        assert!(
            st.is_not_found(),
            "data iterator error should take precedence when the index iterator is OK"
        );
    }

    #[traced_test]
    fn status_falls_back_to_internal_status_when_both_iterators_are_ok() {
        let index_iter: Box<dyn LevelDBIteratorInterface> =
            Box::new(MockStubIterator::new_empty());
        let options = ReadOptions::default();
        let mut two = TwoLevelIterator::new(
            index_iter,
            unused_block_function,
            core::ptr::null_mut(),
            options,
        );

        assert!(
            two.status().is_ok(),
            "status() should be OK before any internal error is recorded"
        );

        let internal_err = make_status(StatusCode::Corruption);
        two.internal_status_mut().assign_from_other_copy(&internal_err);

        let st = two.status();
        assert!(
            !st.is_ok(),
            "status() should reflect the internal error when both iterators are OK"
        );
        assert!(
            st.is_corruption(),
            "internal corruption status should be visible through status()"
        );
    }
}
