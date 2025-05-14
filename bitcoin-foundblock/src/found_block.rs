// ---------------- [ File: bitcoin-foundblock/src/found_block.rs ]
crate::ix!();

/**
  | Helper for findBlock to selectively return
  | pieces of block data. If block is found, data
  | will be returned by setting specified output
  | variables. If block is not found, output
  | variables will keep their previous values.
  */
pub struct FoundBlock {
    hash:            *mut u256,
    height:          *mut i32,
    time:            *mut i64,
    max_time:        *mut i64,
    mtp_time:        *mut i64,
    in_active_chain: *mut bool,
    next_block:      Amo<FoundBlock>,
    data:            Amo<Block>,
    found:           RefCell<bool>,
}

impl Default for FoundBlock {

    fn default() -> Self {
        Self {
            hash:            null_mut(),
            height:          null_mut(),
            time:            null_mut(),
            max_time:        null_mut(),
            mtp_time:        null_mut(),
            in_active_chain: null_mut(),
            next_block:      amo_none(),
            data:            amo_none(),
            found:           RefCell::new(false), 
        }
    }
}

impl FoundBlock {
    
    pub fn hash(&mut self, hash: &mut u256) -> &mut FoundBlock {
        
        todo!();
        /*
            m_hash = &hash; return *this;
        */
    }
    
    pub fn height(&mut self, height: &mut i32) -> &mut FoundBlock {
        
        todo!();
        /*
            m_height = &height; return *this;
        */
    }
    
    pub fn time(&mut self, time: &mut i64) -> &mut FoundBlock {
        
        todo!();
        /*
            m_time = &time; return *this;
        */
    }
    
    pub fn max_time(&mut self, max_time: &mut i64) -> &mut FoundBlock {
        
        todo!();
        /*
            m_max_time = &max_time; return *this;
        */
    }
    
    pub fn mtp_time(&mut self, mtp_time: &mut i64) -> &mut FoundBlock {
        
        todo!();
        /*
            m_mtp_time = &mtp_time; return *this;
        */
    }

    /**
      | Return whether block is in the active
      | (most-work) chain.
      |
      */
    pub fn in_active_chain(&mut self, in_active_chain: &mut bool) -> &mut FoundBlock {
        
        todo!();
        /*
            m_in_active_chain = &in_active_chain; return *this;
        */
    }

    /**
      | Return next block in the active chain
      | if current block is in the active chain.
      |
      */
    pub fn next_block(&mut self, next_block: &FoundBlock) -> &mut FoundBlock {
        
        todo!();
        /*
            m_next_block = &next_block; return *this;
        */
    }

    /**
      | Read block data from disk. If the block
      | exists but doesn't have data (for example
      | due to pruning), the CBlock variable will
      | be set to null.
      */
    pub fn data(&mut self, data: &mut Block) -> &mut FoundBlock {
        
        todo!();
        /*
            m_data = &data; return *this;
        */
    }
}
