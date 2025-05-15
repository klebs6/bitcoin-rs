// ---------------- [ File: bitcoin-blockfilter/src/init.rs ]
crate::ix!();

/**
  | Initialize a block filter index for
  | the given type if one does not already
  | exist. Returns true if a new index is
  | created and false if one has already
  | been initialized.
  |
  */
pub fn init_block_filter_index(
        filter_type:  BlockFilterType,
        n_cache_size: usize,
        memory:       Option<bool>,
        wipe:         Option<bool>) -> bool {

    let memory: bool = memory.unwrap_or(false);
    let wipe:   bool = wipe.unwrap_or(false);
    
    todo!();
        /*
            auto result = g_filter_indexes.emplace(std::piecewise_construct,
                                               std::forward_as_tuple(filter_type),
                                               std::forward_as_tuple(filter_type,
                                                                     n_cache_size, f_memory, f_wipe));
        return result.second;
        */
}
