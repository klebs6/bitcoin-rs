crate::ix!();

pub trait FindBlock {

    /**
      | Return whether node has the block and
      | optionally return block metadata or
      | contents.
      |
      */
    fn find_block(&mut self, 
        hash:  &u256,
        block: &FoundBlock) -> bool;
}

pub trait FindFirstBlockWithTimeAndHeight {

    /**
      | Find first block in the chain with
      | timestamp >= the given time and height >=
      | than the given height, return false if
      | there is no block with a high enough
      | timestamp and height. Optionally return
      | block information.
      */
    fn find_first_block_with_time_and_height(&mut self, 
        min_time:   i64,
        min_height: i32,
        block:      &FoundBlock) -> bool;
}

pub trait FindAncestorByHeight {

    /**
      | Find ancestor of block at specified height
      | and optionally return ancestor
      | information.
      */
    fn find_ancestor_by_height(&mut self, 
        block_hash:      &u256,
        ancestor_height: i32,
        ancestor_out:    &FoundBlock) -> bool;
}

pub trait FindAncestorByHash {

    /**
      | Return whether block descends from
      | a specified ancestor, and optionally
      | return ancestor information.
      */
    fn find_ancestor_by_hash(&mut self, 
        block_hash:    &u256,
        ancestor_hash: &u256,
        ancestor_out:  &FoundBlock) -> bool;
}

pub trait FindCommonAncestor {

    /**
      | Find most recent common ancestor between
      | two blocks and optionally return block
      | information.
      */
    fn find_common_ancestor(&mut self, 
        block_hash1:  &u256,
        block_hash2:  &u256,
        ancestor_out: &FoundBlock,
        block1_out:   &FoundBlock,
        block2_out:   &FoundBlock) -> bool;
}
