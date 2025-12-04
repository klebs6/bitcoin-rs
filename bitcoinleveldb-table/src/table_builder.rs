// ---------------- [ File: bitcoinleveldb-table/src/table_builder.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/table_builder.h]

/**
  | TableBuilder provides the interface used to
  | build a Table (an immutable and sorted map from
  | keys to values).
  |
  | Multiple threads can invoke const methods on
  | a TableBuilder without external
  | synchronization, but if any of the threads may
  | call a non-const method, all threads accessing
  | the same TableBuilder must use external
  | synchronization.
  */
pub struct TableBuilder {
    rep: *mut TableBuilderRep,
}

impl TableBuilder {
    
     /**
      | Create a builder that will store the contents
      | of the table it is building in *file.  Does
      | not close the file.  It is up to the caller
      | to close the file after calling Finish().
      */
    pub fn new(options: &Options, file: *mut dyn WritableFile) -> Self {
        unsafe {
            assert!(
                !file.is_null(),
                "TableBuilder::new: file pointer is null"
            );
        }

        trace!(
            "TableBuilder::new: constructing TableBuilderRep for file={:?}",
            file
        );

        let rep_box = Box::new(TableBuilderRep::new(options, file));
        let rep_ptr: *mut TableBuilderRep = Box::into_raw(rep_box);

        unsafe {
            let rep = &mut *rep_ptr;
            if !rep.filter_block.is_null() {
                trace!(
                    "TableBuilder::new: starting first filter block at offset 0 (filter_block={:?})",
                    rep.filter_block
                );
                let fb = &mut *rep.filter_block;
                fb.start_block(0);
            } else {
                trace!(
                    "TableBuilder::new: filter_block is null; filters disabled for this table"
                );
            }
        }

        TableBuilder { rep: rep_ptr }
    }
}
