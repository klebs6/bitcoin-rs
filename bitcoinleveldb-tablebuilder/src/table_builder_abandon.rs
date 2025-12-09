// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_abandon.rs ]
crate::ix!();

impl TableBuilder {
    
    /// Indicate that the contents of this builder should be abandoned.  Stops
    /// using the file passed to the constructor after this function returns.
    /// 
    /// If the caller is not going to call Finish(), it must call Abandon()
    /// before destroying this builder.
    /// 
    /// REQUIRES: Finish(), Abandon() have not been called
    ///
    pub fn abandon(&mut self) {
        unsafe {
            let rep_ptr = self.rep_ptr_mut();
            assert!(
                !rep_ptr.is_null(),
                "TableBuilder::abandon: rep pointer is null"
            );
            let r: &mut TableBuilderRep = &mut *rep_ptr;

            let offset = *r.offset();
            let num_entries = *r.num_entries();

            trace!(
                "TableBuilder::abandon: closing builder at offset={}, num_entries={}",
                offset,
                num_entries
            );

            assert!(
                !*r.closed(),
                "TableBuilder::abandon: builder already closed"
            );
            r.set_closed(true);
        }
    }
}

#[cfg(test)]
mod table_builder_abandon_behavior_tests {
    use super::*;

    #[traced_test]
    fn abandon_marks_builder_closed_without_mutating_offsets() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("abandon_marks_builder_closed_without_mutating_offsets");

        trace!(
            "abandon_marks_builder_closed_without_mutating_offsets: seeding builder with one entry"
        );

        let key  = Slice::from(b"abandon-key".as_ref());
        let val  = Slice::from(b"abandon-value".as_ref());

        builder.add(&key, &val);

        unsafe {
            let rep_ptr = builder.rep_ptr();
            let rep: &TableBuilderRep = &*rep_ptr;

            let offset_before      = *rep.offset();
            let num_entries_before = *rep.num_entries();

            trace!(
                "before abandon: offset={}, num_entries={}",
                offset_before,
                num_entries_before
            );

            builder.abandon();

            let rep_ptr_after = builder.rep_ptr();
            let rep_after: &TableBuilderRep = &*rep_ptr_after;

            assert!(
                *rep_after.closed(),
                "abandon must mark the builder as closed"
            );
            assert_eq!(
                *rep_after.offset(),
                offset_before,
                "abandon must not change the file offset"
            );
            assert_eq!(
                *rep_after.num_entries(),
                num_entries_before,
                "abandon must not change num_entries"
            );
        }

        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }

    #[traced_test]
    fn abandon_panics_when_called_after_already_closed() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("abandon_panics_when_called_after_already_closed");

        trace!(
            "abandon_panics_when_called_after_already_closed: first abandon call to close builder"
        );
        builder.abandon();

        let builder_ptr: *mut TableBuilder = &mut builder;

        let panic_result = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| unsafe {
                trace!(
                    "abandon_panics_when_called_after_already_closed: second abandon call should panic"
                );
                (*builder_ptr).abandon();
            }),
        );

        assert!(
            panic_result.is_err(),
            "second Abandon call must panic when the builder is already closed"
        );

        unsafe {
            let rep_ptr = builder.rep_ptr();
            let rep: &TableBuilderRep = &*rep_ptr;
            assert!(
                *rep.closed(),
                "builder must remain closed after failed second Abandon"
            );
        }

        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }
}
