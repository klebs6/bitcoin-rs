// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_change_options.rs ]
crate::ix!();
   
impl TableBuilder {

    /**
      | Change the options used by this builder.
      | Note: only some of the option fields can be
      | changed after construction.  If a field is
      | not allowed to change dynamically and its
      | value in the structure passed to the
      | constructor is different from its value in
      | the structure passed to this method, this
      | method will return an error without changing
      | any fields.
      */
    pub fn change_options(&mut self, options: &Options) -> crate::Status {
        unsafe {
            assert!(
                !self.rep.is_null(),
                "TableBuilder::change_options: rep pointer is null"
            );
            let rep = &mut *self.rep;

            trace!(
                "TableBuilder::change_options: current_block_restart_interval={}, new_block_restart_interval={}",
                rep.index_block_options.block_restart_interval,
                options.block_restart_interval
            );

            // Note: if more fields are added to Options, update
            // this function to catch changes that should not be allowed to
            // change in the middle of building a Table.
            let old_cmp = rep.options.comparator;
            let new_cmp = options.comparator;

            if old_cmp != new_cmp {
                let msg = b"changing comparator while building table";
                let msg_slice = Slice::from(&msg[..]);
                error!(
                    "TableBuilder::change_options: comparator changed while building table"
                );
                return crate::Status::invalid_argument(&msg_slice, None);
            }

            // Note that any live BlockBuilders point to rep.options and therefore
            // will automatically pick up the updated options.
            rep.options = options.clone();
            rep.index_block_options = options.clone();
            rep.index_block_options.block_restart_interval = 1;

            trace!(
                "TableBuilder::change_options: options updated; block_restart_interval forced to 1 for index_block"
            );

            crate::Status::ok()
        }
    }
}
