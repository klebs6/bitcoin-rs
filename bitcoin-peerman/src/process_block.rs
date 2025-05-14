// ---------------- [ File: bitcoin-peerman/src/process_block.rs ]
crate::ix!();

pub trait ProcessBlock {

    fn process_block(self: Arc<Self>, 
        node:             &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        block:            Amo<Block>,
        force_processing: bool);
}

impl ProcessBlock for PeerManager {

    /**
      | Process a new block. Perform any post-processing
      | housekeeping
      |
      */
    fn process_block(
        self:             Arc<Self>, 
        mut node:         &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        block:            Amo<Block>,
        force_processing: bool)  {
        
        let mut new_block: bool = false;

        self.chainman.get_mut().process_new_block(
            &self.chainparams, 
            block.clone(), 
            force_processing, 
            &mut new_block
        );

        if new_block {

            node.set_n_last_block_time(Some(get_datetime()));

        } else {

            let mut guard = CS_MAIN.lock();
            self.inner.lock().map_block_source.remove(&block.get().get_hash());
        }
    }
}
