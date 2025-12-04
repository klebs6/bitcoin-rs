// ---------------- [ File: bitcoinleveldb-tablerep/src/table_rep.rs ]
crate::ix!();

pub struct TableRep {
    options:          Options,
    status:           Status,
    file:             Rc<RefCell<dyn RandomAccessFile>>,
    cache_id:         u64,
    filter:           *mut FilterBlockReader,
    filter_data:      *mut u8,
    filter_data_len:  usize,

    /// Handle to metaindex_block: saved from
    /// footer
    /// 
    metaindex_handle: BlockHandle,
    index_block:      *mut Block,
}

impl Drop for TableRep {
    fn drop(&mut self) {
        trace!(
            "TableRep::drop: cache_id={}, index_block={:?}, filter={:?}, filter_data={:?}, filter_data_len={}",
            self.cache_id,
            self.index_block,
            self.filter,
            self.filter_data,
            self.filter_data_len
        );

        unsafe {
            if !self.filter.is_null() {
                trace!(
                    "TableRep::drop: deleting FilterBlockReader @ {:?}",
                    self.filter
                );
                let _filter_box: Box<FilterBlockReader> = Box::from_raw(self.filter);
                self.filter = core::ptr::null_mut();
            } else {
                trace!("TableRep::drop: filter pointer is null; nothing to free");
            }

            if !self.filter_data.is_null() {
                if self.filter_data_len > 0 {
                    trace!(
                        "TableRep::drop: freeing filter_data buffer @ {:?} (len={})",
                        self.filter_data,
                        self.filter_data_len
                    );
                    let slice =
                        core::ptr::slice_from_raw_parts_mut(self.filter_data, self.filter_data_len);
                    let _buf: Box<[u8]> = Box::from_raw(slice);
                    // drop(_buf) happens here
                } else {
                    debug!(
                        "TableRep::drop: filter_data non-null but len==0; skipping free to avoid UB"
                    );
                }
                self.filter_data = core::ptr::null_mut();
                self.filter_data_len = 0;
            } else {
                trace!("TableRep::drop: filter_data pointer is null; nothing to free");
            }

            if !self.index_block.is_null() {
                trace!(
                    "TableRep::drop: deleting index Block @ {:?}",
                    self.index_block
                );
                let _block_box: Box<Block> = Box::from_raw(self.index_block);
                self.index_block = core::ptr::null_mut();
            } else {
                trace!("TableRep::drop: index_block pointer is null; nothing to free");
            }
        }
    }
}
