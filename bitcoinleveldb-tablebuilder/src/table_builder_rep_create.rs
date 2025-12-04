// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_rep_create.rs ]
crate::ix!();

impl TableBuilderRep {

    pub fn new(opt: &Options, f: *mut dyn WritableFile) -> Self {
        trace!(
            "TableBuilderRep::new: file={:?}, block_size={}, block_restart_interval={}",
            f,
            opt.block_size,
            opt.block_restart_interval
        );

        let mut index_block_options = opt.clone();
        index_block_options.block_restart_interval = 1;

        let options = opt.clone();

        let data_block = BlockBuilder::new(&options);
        let index_block = BlockBuilder::new(&index_block_options);

        let filter_block_ptr = if opt.filter_policy.is_null() {
            trace!(
                "TableBuilderRep::new: filter_policy is null; filter_block disabled"
            );
            core::ptr::null_mut()
        } else {
            trace!(
                "TableBuilderRep::new: creating FilterBlockBuilder for policy={:?}",
                opt.filter_policy
            );
            let fb =
                FilterBlockBuilder::new(opt.filter_policy);
            Box::into_raw(Box::new(fb))
        };

        TableBuilderRep {
            options,
            index_block_options,
            file: f,
            offset: 0,
            status: Status::ok(),
            data_block,
            index_block,
            last_key_: String::new(),
            num_entries: 0,
            closed: false,
            filter_block: filter_block_ptr,
            pending_index_entry: false,
            pending_handle: BlockHandle::default(),
            compressed_output: String::new(),
        }
    }
}
