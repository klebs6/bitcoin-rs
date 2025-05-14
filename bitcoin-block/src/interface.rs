// ---------------- [ File: bitcoin-block/src/interface.rs ]
crate::ix!();

pub trait BlockChecked {

    /**
      | Notifies listeners of a block validation
      | result.
      | 
      | If the provided BlockValidationState
      | IsValid, the provided block is guaranteed
      | to be the current best block at the time
      | the callback was generated (not necessarily
      | now)
      |
      */
    fn block_checked(self: Arc<Self>, 
        _0: &Block,
        _1: &BlockValidationState) {}

}

pub trait NewPoWValidBlock {

    /**
      | Notifies listeners that a block which
      | builds directly on our current tip has
      | been received and connected to the headers
      | tree, though not validated yet
      |
      */
    fn new_pow_valid_block(&mut self, 
        pindex: Arc<BlockIndex>,
        block:  &Arc<Block>) {}
}


pub trait BlockConnected {

    /**
      | Notifies listeners of a block being
      | connected.
      | 
      | Provides a vector of transactions evicted
      | from the mempool as a result.
      | 
      | Called on a background thread.
      |
      */
    fn block_connected(&mut self, 
        block:  Arc<Block>,
        pindex: Arc<BlockIndex>) {}
}

pub trait BlockDisconnected {

    /**
      | Notifies listeners of a block being
      | disconnected
      | 
      | Called on a background thread.
      |
      */
    fn block_disconnected(&mut self, 
        block:  Arc<Block>,
        pindex: Arc<BlockIndex>) {}
}

pub trait UpdatedBlockTip {

    /**
      | Notifies listeners when the block chain
      | tip advances.
      | 
      | When multiple blocks are connected
      | at once, UpdatedBlockTip will be called
      | on the final tip but may not be called
      | on every intermediate tip. If the latter
      | behavior is desired, subscribe to BlockConnected()
      | instead.
      | 
      | Called on a background thread.
      |
      */
    fn updated_block_tip(&mut self, 
        pindex_new:       Option<Arc<BlockIndex>>,
        pindex_fork:      Option<Arc<BlockIndex>>,
        initial_download: bool) {}
}

pub trait ChainStateFlushed {

    /**
      | Notifies listeners of the new active
      | block chain on-disk.
      | 
      | Prior to this callback, any updates
      | are not guaranteed to persist on disk
      | (ie clients need to handle shutdown/restart
      | safety by being able to understand when
      | some updates were lost due to unclean
      | shutdown).
      | 
      | When this callback is invoked, the validation
      | changes done by any prior callback are
      | guaranteed to exist on disk and survive
      | a restart, including an unclean shutdown.
      | 
      | Provides a locator describing the best
      | chain, which is likely useful for storing
      | current state on disk in client DBs.
      | 
      | Called on a background thread.
      |
      */
    fn chain_state_flushed(&mut self, locator: &BlockLocator) {}
}

pub trait IsInitialBlockDownload {

    /**
      | Check if in IBD.
      |
      */
    fn is_initial_block_download(&self) -> bool;
}
