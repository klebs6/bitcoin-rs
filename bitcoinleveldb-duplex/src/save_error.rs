// ---------------- [ File: bitcoinleveldb-duplex/src/save_error.rs ]
crate::ix!();

impl TwoLevelIterator {

    pub fn save_error(&mut self, s: &Status) {
        trace!(
            "TwoLevelIterator::save_error: current_ok={}, incoming_ok={}",
            self.internal_status().is_ok(),
            s.is_ok()
        );

        if self.internal_status().is_ok() && !s.is_ok() {
            trace!(
                "TwoLevelIterator::save_error: capturing first error; code={:?}",
                s.code()
            );
            self.internal_status_mut().assign_from_other_copy(s);
        } else {
            trace!(
                "TwoLevelIterator::save_error: keeping existing status; \
                 current_code={:?}, incoming_code={:?}",
                self.internal_status().code(),
                s.code()
            );
        }
    }
}

#[cfg(test)]
mod two_level_iterator_save_error_tests {
    use super::*;
    use core::ffi::c_void;

    fn unused_block_function(
        _arg: *mut c_void,
        _options: &ReadOptions,
        _handle: &Slice,
    ) -> Option<Box<dyn LevelDBIteratorInterface>> {
        None
    }

    fn new_two_level_with_ok_index() -> TwoLevelIterator {
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

    fn make_io_error_status() -> Status {
        let msg = Slice::from("io-error");
        Status::io_error(&msg, None)
    }

    fn make_not_found_status() -> Status {
        let msg = Slice::from("not-found");
        Status::not_found(&msg, None)
    }

    #[traced_test]
    fn save_error_records_first_non_ok_status() {
        let mut two = new_two_level_with_ok_index();

        assert!(
            two.internal_status().is_ok(),
            "internal status should start in the OK state"
        );
        assert!(
            two.status().is_ok(),
            "public status should also be OK before any error is saved"
        );

        let io_error = make_io_error_status();
        two.save_error(&io_error);

        let st = two.status();
        assert!(
            !st.is_ok(),
            "TwoLevelIterator::status should surface the saved non-OK status"
        );
        assert!(
            st.is_io_error(),
            "first saved status should be the IO error"
        );
    }

    #[traced_test]
    fn save_error_does_not_override_existing_error_with_new_errors() {
        let mut two = new_two_level_with_ok_index();

        let io_error = make_io_error_status();
        let not_found = make_not_found_status();

        two.save_error(&io_error);

        assert!(
            two.internal_status().is_io_error(),
            "internal status should record the first IO error"
        );

        two.save_error(&not_found);

        let st = two.status();
        assert!(
            st.is_io_error(),
            "subsequent non-OK statuses must not overwrite the originally saved error"
        );
        assert!(
            two.internal_status().is_io_error(),
            "internal status should continue to store the first error"
        );
    }

    #[traced_test]
    fn save_error_ignores_ok_statuses() {
        let mut two = new_two_level_with_ok_index();

        let ok = Status::ok();
        two.save_error(&ok);

        assert!(
            two.internal_status().is_ok(),
            "saving an OK status must leave internal status unchanged"
        );
        assert!(
            two.status().is_ok(),
            "overall status must remain OK after saving an OK status"
        );

        let error = make_io_error_status();
        two.save_error(&error);

        assert!(
            two.internal_status().is_io_error(),
            "internal status should transition to the first non-OK status"
        );

        let ok_again = Status::ok();
        two.save_error(&ok_again);

        assert!(
            two.internal_status().is_io_error(),
            "a later OK status must not clear the recorded error"
        );
    }
}
