// ---------------- [ File: bitcoinleveldb-footer/src/footer.rs ]
crate::ix!();

/**
  | Footer encapsulates the fixed information
  | stored at the tail end of every table
  | file.
  |
  */
#[derive(Default)]
pub struct Footer {
    metaindex_handle: BlockHandle,
    index_handle:     BlockHandle,
}

/**
  | Note:
  | 
  | The serialization of a Footer will always
  | occupy exactly this many bytes. It consists
  | of two block handles and a magic number.
  |
  */
pub const FOOTER_ENCODED_LENGTH: usize = 2 * BLOCK_HANDLE_MAX_ENCODED_LENGTH + 8;

impl Footer {

    /**
      | The block handle for the metaindex block
      | of the table
      |
      */
    pub fn metaindex_handle(&self) -> &BlockHandle {
        trace!(
            "Footer::metaindex_handle called (offset={}, size={})",
            self.metaindex_handle.offset(),
            self.metaindex_handle.size()
        );
        &self.metaindex_handle
    }

    pub fn set_metaindex_handle(&mut self, h: &BlockHandle) {
        trace!(
            "Footer::set_metaindex_handle: offset={} size={}",
            h.offset(),
            h.size()
        );
        self.metaindex_handle = *h;
    }

    /**
      | The block handle for the index block
      | of the table
      |
      */
    pub fn index_handle(&self) -> &BlockHandle {
        trace!(
            "Footer::index_handle called (offset={}, size={})",
            self.index_handle.offset(),
            self.index_handle.size()
        );
        &self.index_handle
    }

    pub fn set_index_handle(&mut self, h: &BlockHandle) {
        trace!(
            "Footer::set_index_handle: offset={} size={}",
            h.offset(),
            h.size()
        );
        self.index_handle = *h;
    }
}
