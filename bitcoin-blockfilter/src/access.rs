// ---------------- [ File: bitcoin-blockfilter/src/access.rs ]
crate::ix!();

/**
  | Get a block filter index by type. Returns
  | nullptr if index has not been initialized
  | or was already destroyed.
  |
  */
pub fn get_block_filter_index(filter_type: BlockFilterType) -> Amo<BlockFilterIndex> {
    
    todo!();
        /*
            auto it = g_filter_indexes.find(filter_type);
        return it != g_filter_indexes.end() ? &it->second : nullptr;
        */
}

/**
  | Iterate over all running block filter
  | indexes, invoking fn on each.
  |
  */
pub fn for_each_block_filter_index(fn_: fn(_0: &mut BlockFilterIndex) -> ())  {
    
    todo!();
        /*
            for (auto& entry : g_filter_indexes) fn(entry.second);
        */
}
