// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_status.rs ]
crate::ix!();

impl TableBuilder {

    /// Return non-ok iff some error has been detected.
    ///
    pub fn status(&self) -> Status {
        unsafe {
            let rep_ptr = self.rep_ptr();
            assert!(
                !rep_ptr.is_null(),
                "TableBuilder::status: rep pointer is null"
            );
            let r: &TableBuilderRep = &*rep_ptr;
            Status::new_from_other_copy(r.status())
        }
    }
}

#[cfg(test)]
mod table_builder_status_tests {
    use super::*;

    #[traced_test]
    fn status_on_new_builder_is_ok() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("status_on_new_builder_is_ok");

        trace!("status_on_new_builder_is_ok: checking initial status");
        let status = builder.status();
        assert!(
            status.is_ok(),
            "status on a new builder must be OK"
        );

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }

    #[traced_test]
    fn status_returns_independent_copy_of_internal_status() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("status_returns_independent_copy_of_internal_status");

        trace!(
            "status_returns_independent_copy_of_internal_status: capturing initial status"
        );
        let original = builder.status();

        unsafe {
            let rep_ptr = builder.rep_ptr() as *mut TableBuilderRep;
            let rep: &mut TableBuilderRep = &mut *rep_ptr;

            let msg = b"status-copy-test";
            let msg_slice = Slice::from(&msg[..]);

            trace!("overwriting internal status with invalid_argument");
            rep.set_status(Status::invalid_argument(&msg_slice, None));

            assert!(
                !rep.status().is_ok(),
                "internal status must now be non-OK"
            );
        }

        assert!(
            original.is_ok(),
            "previously returned Status must remain OK even after internal mutation"
        );

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }
}
