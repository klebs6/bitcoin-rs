// ---------------- [ File: bitcoin-block/src/connect_trace.rs ]
crate::ix!();

pub struct PerBlockConnectTrace {
    pub pindex: *mut BlockIndex,
    pub pblock: Arc<Block>,
}

impl Default for PerBlockConnectTrace {
    fn default() -> Self {
        Self {
            pindex: null_mut(),
            pblock: Arc::new(Block::default()),
        }
    }
}

/**
  | Used to track blocks whose transactions
  | were applied to the UTXO state as a part
  | of a single ActivateBestChainStep
  | call.
  | 
  | This class is single-use, once you call
  | GetBlocksConnected() you have to throw
  | it away and make a new one.
  |
  */
pub struct ConnectTrace {
    pub blocks_connected: Vec<PerBlockConnectTrace>,
}

impl ConnectTrace {
    
    pub fn new() -> Self {
        Self {
            blocks_connected: Vec::with_capacity(1),
        }
    }
    
    pub fn block_connected(&mut self, 
        pindex: *mut BlockIndex,
        pblock: Arc<Block>)  {
        
        assert!(self.blocks_connected.last().as_ref().unwrap().pindex == null_mut());
        assert!(pindex != null_mut());
        assert!(Arc::<Block>::strong_count(&pblock) != 0);

        self.blocks_connected.last_mut().unwrap().pindex = pindex;
        self.blocks_connected.last_mut().unwrap().pblock = pblock;
        self.blocks_connected.push(Default::default());
    }
    
    pub fn get_blocks_connected(&mut self) -> &mut Vec<PerBlockConnectTrace> {
        
        /**
          | We always keep one extra block at the
          | end of our list because blocks are
          | added after all the conflicted
          | transactions have been filled in. Thus,
          | the last entry should always be an
          | empty one waiting for the transactions
          | from the next block. We pop the last
          | entry here to make sure the list we
          | return is sane.
          */
        assert!(self.blocks_connected.last().unwrap().pindex == null_mut());

        self.blocks_connected.pop();

        &mut self.blocks_connected
    }
}
