// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_rep.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/table_builder.cc]

#[derive(Getters,MutGetters,Setters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct TableBuilderRep {

    #[getset(skip)]
    options:             *const Options,

    #[getset(skip)]
    file:                *mut dyn WritableFile,

    offset:              u64,
    status:              Status,
    data_block:          BlockBuilder,
    index_block:         BlockBuilder,
    last_key_:           String,
    num_entries:         i64,

    /// Either Finish() or Abandon() has been called.
    /// 
    closed:              bool,
    #[getset(skip)]
    filter_block:        *mut FilterBlockBuilder,

    /// We do not emit the index entry for a block until we have seen the first
    /// key for the next data block.  
    ///
    /// This allows us to use shorter keys in the index block.  
    ///
    /// For example, consider a block boundary between the keys "the quick brown
    /// fox" and "the who".  
    ///
    /// We can use "the r" as the key for the index block entry since it is >=
    /// all entries in the first block and < all entries in subsequent blocks.
    /// 
    /// Invariant: r->pending_index_entry is true only if data_block is empty.
    ///
    pending_index_entry: bool,

    /// Handle to add to index block
    /// 
    pending_handle:      BlockHandle,
    compressed_output:   String,
}

impl TableBuilderRep {

    pub fn new(opt: &Options, f: *mut dyn WritableFile) -> Self {
        trace!(
            "TableBuilderRep::new: file={:?}, block_size={}, block_restart_interval={}",
            f,
            *opt.block_size(),
            *opt.block_restart_interval()
        );

        let options_ptr: *const Options = opt as *const Options;

        let data_block = BlockBuilder::new(options_ptr);

        let mut index_block = BlockBuilder::new(options_ptr);
        index_block.set_block_restart_interval(1);

        let filter_block_ptr = {
            trace!(
                "TableBuilderRep::new: creating FilterBlockBuilder with default NullFilterPolicy"
            );
            let policy: Box<dyn FilterPolicy> = Box::new(NullFilterPolicy::default());
            let fb = FilterBlockBuilder::new(policy);
            Box::into_raw(Box::new(fb))
        };

        TableBuilderRep {
            options:             options_ptr,
            file:                f,
            offset:              0,
            status:              Status::ok(),
            data_block,
            index_block,
            last_key_:           String::new(),
            num_entries:         0,
            closed:              false,
            filter_block:        filter_block_ptr,
            pending_index_entry: false,
            pending_handle:      BlockHandle::default(),
            compressed_output:   String::new(),
        }
    }

    #[inline]
    pub fn options(&self) -> *const Options {
        // return the raw pointer field, do NOT call self.options() again
        self.options
    }

    #[inline]
    pub fn set_options(&mut self, options: *const Options) {
        // update the raw pointer field, do NOT call self.set_options() again
        self.options = options;
    }

    #[inline]
    pub fn file(&self) -> *mut dyn WritableFile {
        self.file
    }

    #[inline]
    pub fn file_mut(&mut self) -> *mut dyn WritableFile {
        self.file
    }

    #[inline]
    pub fn set_file(&mut self, file: *mut dyn WritableFile) {
        self.file = file;
    }

    #[inline]
    pub fn filter_block(&self) -> *mut FilterBlockBuilder {
        self.filter_block
    }

    #[inline]
    pub fn filter_block_mut(&mut self) -> *mut FilterBlockBuilder {
        self.filter_block
    }

    #[inline]
    pub fn set_filter_block(&mut self, fb: *mut FilterBlockBuilder) {
        self.filter_block = fb;
    }

    #[inline]
    pub fn last_key_mut(&mut self) -> &mut String {
        self.last_key__mut()
    }
}

#[cfg(test)]
mod table_builder_rep_internal_tests {
    use super::*;

    #[traced_test]
    fn table_builder_rep_new_sets_expected_invariants() {
        trace!("table_builder_rep_new_sets_expected_invariants: constructing Options and file");

        let options_ref: &'static Options = Box::leak(Box::new(Options::default()));

        let file_box = Box::new(InMemoryWritableFile::new_for_test(
            "table_builder_rep_new_sets_expected_invariants",
        ));
        let file_raw: *mut InMemoryWritableFile = Box::into_raw(file_box);
        let file_trait: *mut dyn WritableFile = file_raw as *mut dyn WritableFile;

        let mut rep = TableBuilderRep::new(options_ref, file_trait);

        trace!("validating invariants on freshly constructed TableBuilderRep");

        assert_eq!(
            *rep.offset(),
            0,
            "offset must start at 0"
        );
        assert_eq!(
            *rep.num_entries(),
            0,
            "num_entries must start at 0"
        );
        assert!(
            !*rep.closed(),
            "closed flag must start as false"
        );
        assert!(
            rep.status().is_ok(),
            "status must start as OK"
        );
        assert!(
            rep.data_block().empty(),
            "data_block must be empty initially"
        );
        assert!(
            rep.index_block().empty(),
            "index_block must be empty initially"
        );
        assert!(
            !rep.filter_block().is_null(),
            "filter_block pointer must be non-null after construction"
        );
        assert!(
            !*rep.pending_index_entry(),
            "pending_index_entry must start as false"
        );
        assert!(
            rep.compressed_output().is_empty(),
            "compressed_output must start empty"
        );

        unsafe {
            let fb_raw = rep.filter_block();
            if !fb_raw.is_null() {
                trace!("freeing FilterBlockBuilder allocated by TableBuilderRep::new");
                let _ = Box::from_raw(fb_raw);
                rep.set_filter_block(core::ptr::null_mut());
            }

            let _ = Box::from_raw(file_raw);
        }
    }
}
