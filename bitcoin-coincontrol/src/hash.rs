crate::ix!();

/**
  | The legacy hash serializes the hashBlock
  |
  */
pub fn prepare_hash_with_hash_writer(
        ss:    &mut HashWriter,
        stats: &CoinsStats)  {
    
    todo!();
        /*
            ss << stats.hashBlock;
        */
}

/**
  | MuHash does not need the prepare step
  |
  */
pub fn prepare_hash(
        muhash: &mut MuHash3072,
        stats:  &mut CoinsStats)  {
    
    todo!();
        /*
        
        */
}

pub fn finalize_hash_with_hash_writer(
        ss:    &mut HashWriter,
        stats: &mut CoinsStats)  {
    
    todo!();
        /*
            stats.hashSerialized = ss.GetHash();
        */
}

pub fn finalize_hash(
        muhash: &mut MuHash3072,
        stats:  &mut CoinsStats)  {
    
    todo!();
        /*
            uint256 out;
        muhash.Finalize(out);
        stats.hashSerialized = out;
        */
}

