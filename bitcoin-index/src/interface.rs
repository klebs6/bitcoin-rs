crate::ix!();

pub trait WriteBlock {

    /**
      | Write update index entries for a newly
      | connected block.
      |
      */
    fn write_block(&mut self, 
        block:  &Block,
        pindex: *const BlockIndex) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

pub trait CommitInternal {

    /**
      | Virtual method called internally by
      | Commit that can be overridden to atomically
      | commit more index state.
      |
      */
    fn commit_internal(&mut self, batch: &mut DBBatch) -> bool;
}

pub trait Rewind {

    /**
      | Rewind index to an earlier chain tip
      | during a chain reorg. The tip must be
      | an ancestor of the current best block.
      |
      */
    fn rewind(&mut self, 
            current_tip: *const BlockIndex,
            new_tip:     *const BlockIndex) -> bool;
}

