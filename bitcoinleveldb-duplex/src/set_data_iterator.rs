// ---------------- [ File: bitcoinleveldb-duplex/src/set_data_iterator.rs ]
crate::ix!();

impl TwoLevelIterator {
   
    /// Set the data iterator, preserving the first non-OK status we observe.
    pub fn set_data_iterator(
        &mut self,
        data_iter: Option<Box<dyn LevelDBIteratorInterface>>,
    ) {
        trace!(
            "TwoLevelIterator::set_data_iterator: \
             existing_has_iter={}, new_has_iter={}",
            self.data_iter().has_iterator(),
            data_iter.is_some(),
        );

        if self.data_iter().iter().is_some() {
            let s = self.data_iter().status();
            if !s.is_ok() {
                trace!(
                    "TwoLevelIterator::set_data_iterator: \
                     existing data_iter has error; code={:?}",
                    s.code()
                );
            }
            self.save_error(&s);
        }

        self.data_iter_mut().set(data_iter);

        trace!(
            "TwoLevelIterator::set_data_iterator: \
             now_has_iter={}, now_valid={}",
            self.data_iter().has_iterator(),
            self.data_iter().valid(),
        );
    }
}

#[cfg(test)]
mod two_level_iterator_set_data_iterator_tests {
    use super::*;
    use core::ffi::c_void;

    fn unused_block_function(
        _arg: *mut c_void,
        _options: &ReadOptions,
        _handle: &Slice,
    ) -> Option<Box<dyn LevelDBIteratorInterface>> {
        None
    }

    struct ErrorStatusIterator {
        status: Status,
    }

    impl ErrorStatusIterator {
        fn new_with_status(status: Status) -> Self {
            ErrorStatusIterator { status }
        }

        fn new_io_error() -> Self {
            let msg = Slice::from("error-io");
            ErrorStatusIterator {
                status: Status::io_error(&msg, None),
            }
        }

        fn new_not_found_error() -> Self {
            let msg = Slice::from("error-not-found");
            ErrorStatusIterator {
                status: Status::not_found(&msg, None),
            }
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
            panic!("ErrorStatusIterator::key should not be called in set_data_iterator tests");
        }
    }

    impl LevelDBIteratorValue for ErrorStatusIterator {
        fn value(&self) -> Slice {
            panic!("ErrorStatusIterator::value should not be called in set_data_iterator tests");
        }
    }

    fn new_two_level() -> TwoLevelIterator {
        let index_iter: Box<dyn LevelDBIteratorInterface> =
            Box::new(MockStubIterator::new_empty());
        let options = ReadOptions::default();
        TwoLevelIterator::new(
            index_iter,
            unused_block_function,
            core::ptr::null_mut(),
            options,
        )
    }

    #[traced_test]
    fn set_data_iterator_captures_error_when_replacing_iterator() {
        let mut two = new_two_level();

        let error_iter: Box<dyn LevelDBIteratorInterface> =
            Box::new(ErrorStatusIterator::new_io_error());
        two.set_data_iterator(Some(error_iter));

        let status_after_first_install = two.status();
        assert!(
            !status_after_first_install.is_ok(),
            "with an error data iterator installed, status() should surface that error"
        );
        assert!(
            status_after_first_install.is_io_error(),
            "initial data iterator error should be an IO error"
        );
        assert!(
            two.internal_status().is_ok(),
            "internal status should still be OK before any replacement"
        );

        two.set_data_iterator(Some(Box::new(MockStubIterator::new_empty())));

        assert!(
            two.internal_status().is_io_error(),
            "set_data_iterator should cache the first non-OK status from the previous data iterator"
        );

        let status_after_replacement = two.status();
        assert!(
            !status_after_replacement.is_ok(),
            "status() should continue to reflect the previously observed error even after installing an OK iterator"
        );
        assert!(
            status_after_replacement.is_io_error(),
            "cached error should remain the IO error from the first iterator"
        );
    }

    #[traced_test]
    fn set_data_iterator_captures_error_when_clearing_iterator_and_does_not_override_first_error() {
        let mut two = new_two_level();

        let first_error_iter: Box<dyn LevelDBIteratorInterface> =
            Box::new(ErrorStatusIterator::new_io_error());
        two.set_data_iterator(Some(first_error_iter));

        two.set_data_iterator(None);

        assert!(
            two.internal_status().is_io_error(),
            "internal status should cache the IO error when clearing the failing data iterator"
        );
        assert!(
            two.data_iter().iter().is_none(),
            "data iterator should be cleared after set_data_iterator(None)"
        );

        let second_error_iter: Box<dyn LevelDBIteratorInterface> =
            Box::new(ErrorStatusIterator::new_not_found_error());
        two.set_data_iterator(Some(second_error_iter));

        assert!(
            two.internal_status().is_io_error(),
            "internal status must keep the first observed error even after subsequent failing iterators"
        );
    }
}
