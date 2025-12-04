// ---------------- [ File: bitcoinleveldb-table/src/table_builder_rep.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/table_builder.cc]

pub struct TableBuilderRep {
    options:             Options,
    index_block_options: Options,
    file:                *mut dyn WritableFile,
    offset:              u64,
    status:              Status,
    data_block:          BlockBuilder,
    index_block:         BlockBuilder,
    last_key_:            String,
    num_entries:         i64,

    /**
      | Either Finish() or Abandon() has been
      | called.
      |
      */
    closed:              bool,

    filter_block:        *mut FilterBlockBuilder,

    /**
      | We do not emit the index entry for a block
      | until we have seen the first key for the next
      | data block.  This allows us to use shorter
      | keys in the index block.  For example,
      | consider a block boundary between the keys
      | "the quick brown fox" and "the who".  We can
      | use "the r" as the key for the index block
      | entry since it is >= all entries in the first
      | block and < all entries in subsequent blocks.
      |
      | Invariant: r->pending_index_entry is true
      | only if data_block is empty.
      */
    pending_index_entry: bool,

    /**
      | Handle to add to index block
      |
      */
    pending_handle:      BlockHandle,

    compressed_output:   String,
}
