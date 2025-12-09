// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_change_options.rs ]
crate::ix!();

impl TableBuilder {

    /// Change the options used by this builder. 
    ///
    /// Note: only some of the option fields can be changed after construction.  
    ///
    /// If a field is not allowed to change dynamically and its value in the
    /// structure passed to the constructor is different from its value in the
    /// structure passed to this method, this method will return an error
    /// without changing any fields.
    ///
    pub fn change_options(&mut self, options: &Options) -> crate::Status {
        unsafe {
            let rep_ptr = self.rep_ptr_mut();
            assert!(
                !rep_ptr.is_null(),
                "TableBuilder::change_options: rep pointer is null"
            );
            let rep: &mut TableBuilderRep = &mut *rep_ptr;

            let current_interval = *rep.index_block().block_restart_interval();
            let new_interval = *options.block_restart_interval();

            trace!(
                "TableBuilder::change_options: current_block_restart_interval={}, new_block_restart_interval={}",
                current_interval,
                new_interval
            );

            let old_options_ptr = rep.options();
            assert!(
                !old_options_ptr.is_null(),
                "TableBuilder::change_options: existing options pointer is null"
            );
            let old_opts: &Options = &*old_options_ptr;

            let old_cmp_box: &Arc<dyn SliceComparator> = old_opts.comparator();
            let new_cmp_box: &Arc<dyn SliceComparator> = options.comparator();

            let old_cmp_ptr: *const dyn SliceComparator = &**old_cmp_box as *const dyn SliceComparator;
            let new_cmp_ptr: *const dyn SliceComparator = &**new_cmp_box as *const dyn SliceComparator;

            if old_cmp_ptr != new_cmp_ptr {
                let msg = b"changing comparator while building table";
                let msg_slice = Slice::from(&msg[..]);
                error!(
                    "TableBuilder::change_options: comparator changed while building table"
                );
                return crate::Status::invalid_argument(&msg_slice, None);
            }

            let new_options_ptr: *const Options = options as *const Options;
            rep.set_options(new_options_ptr);

            rep.data_block_mut().set_options(new_options_ptr);
            rep.index_block_mut().set_options(new_options_ptr);

            rep.data_block_mut().set_block_restart_interval(new_interval);
            rep.index_block_mut().set_block_restart_interval(1);

            trace!(
                "TableBuilder::change_options: options updated; block_restart_interval for index_block forced to 1"
            );

            crate::Status::ok()
        }
    }
}

#[cfg(test)]
mod table_builder_change_options_tests {
    use super::*;

    #[traced_test]
    fn change_options_with_same_options_is_ok_and_preserves_comparator() {
        let (mut builder, options_ref, file_raw) =
            create_table_builder_for_test("change_options_with_same_options_is_ok_and_preserves_comparator");

        trace!(
            "change_options_with_same_options_is_ok_and_preserves_comparator: calling change_options with original Options"
        );

        let status = builder.change_options(options_ref);

        assert!(
            status.is_ok(),
            "change_options with identical Options must succeed"
        );

        unsafe {
            let rep_ptr = builder.rep_ptr();
            let rep: &TableBuilderRep = &*rep_ptr;

            let current_interval =
                *rep.index_block().block_restart_interval();

            assert_eq!(
                current_interval,
                1,
                "index_block_restart_interval must remain 1 after change_options"
            );
        }

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }
}
