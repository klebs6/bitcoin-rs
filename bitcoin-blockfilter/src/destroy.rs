// ---------------- [ File: bitcoin-blockfilter/src/destroy.rs ]
crate::ix!();

/**
  | Destroy the block filter index with
  | the given type. Returns false if no such
  | index exists. This just releases the
  | allocated memory and closes the database
  | connection, it does not delete the index
  | data.
  |
  */
pub fn destroy_block_filter_index(filter_type: BlockFilterType) -> bool {
    
    todo!();
        /*
            return g_filter_indexes.erase(filter_type);
        */
}

/**
  | Destroy all open block filter indexes.
  |
  */
pub fn destroy_all_block_filter_indexes()  {
    
    todo!();
        /*
            g_filter_indexes.clear();
        */
}

